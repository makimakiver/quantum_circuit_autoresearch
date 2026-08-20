// Scout-only instrumented census of the FINAL op stream (ops.bin).
// Mirrors sim.rs::apply_iter semantics exactly (same op order, same xof reads),
// observing per-CCX/CCZ: admission mask, fire mask, control-implication masks.
use alloy_primitives::U256;
use quantum_ecc::circuit::{analyze_ops, Op, OperationType, QubitId, QubitOrBit, NO_BIT};
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;
use std::io::Read;

fn secp256k1() -> quantum_ecc::weierstrass_elliptic_curve::WeierstrassEllipticCurve {
    quantum_ecc::weierstrass_elliptic_curve::WeierstrassEllipticCurve {
        modulus: U256::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F", 16).unwrap(),
        a: U256::from(0),
        b: U256::from(7),
        gx: U256::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798", 16).unwrap(),
        gy: U256::from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8", 16).unwrap(),
        order: U256::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap(),
    }
}

fn load_ops(path: &str) -> Vec<Op> {
    let mut raw = std::fs::read(path).unwrap();
    let n = u64::from_le_bytes(raw[8..16].try_into().unwrap()) as usize;
    let mut dec = zstd::stream::read::Decoder::new(&raw[16..]).unwrap();
    let mut body = vec![0u8; n * 56];
    dec.read_exact(&mut body).unwrap();
    let mut ops = Vec::with_capacity(n);
    for i in 0..n {
        let off = i * 56;
        let kind = u32::from_le_bytes(body[off..off + 4].try_into().unwrap());
        let g = |o: usize| u64::from_le_bytes(body[off + o..off + o + 8].try_into().unwrap());
        let op = Op {
            kind: match kind { 0=>OperationType::Neg,1=>OperationType::Register,2=>OperationType::AppendToRegister,3=>OperationType::BitInvert,4=>OperationType::BitStore0,5=>OperationType::BitStore1,6=>OperationType::X,7=>OperationType::Z,8=>OperationType::CX,9=>OperationType::CZ,10=>OperationType::Swap,11=>OperationType::R,12=>OperationType::Hmr,13=>OperationType::CCX,14=>OperationType::CCZ,15=>OperationType::PushCondition,16=>OperationType::PopCondition,_=>OperationType::DebugPrint },
            q_control2: QubitId(g(8)),
            q_control1: QubitId(g(16)),
            q_target: QubitId(g(24)),
            c_target: quantum_ecc::circuit::BitId(g(32)),
            c_condition: quantum_ecc::circuit::BitId(g(40)),
            r_target: quantum_ecc::circuit::RegisterId(g(48)),
        };
        ops.push(op);
    }
    raw.clear();
    ops
}

struct Obs {
    // masks accumulated across all batches (bit i = lane i ever admitted/fired/violated)
    admit: u64,
    fired: u64,
    viol_c1_implies_c2: u64, // admit & q1 & !q2  (q_control1 implies q_control2)
    viol_c2_implies_c1: u64,
    admit_pop: u64, // total lane-executions
}

fn draw_index(reader: &mut impl XofReader, upper: usize) -> usize {
    assert!(upper >= 2, "point pool must contain at least two entries");
    let upper_u64 = u64::try_from(upper).expect("point pool does not fit in u64");
    let acceptance = u64::MAX - (u64::MAX % upper_u64);
    loop {
        let mut bytes = [0u8; 8];
        XofReader::read(reader, &mut bytes);
        let value = u64::from_le_bytes(bytes);
        if value < acceptance {
            return (value % upper_u64) as usize;
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let ops_path = args.get(1).map(|s| s.as_str()).unwrap_or("ops.bin");
    let threads: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(8);
    let batches_per_thread: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(200);
    let pool: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(8192);
    assert!(pool >= 2, "pool must be at least 2");
    let census_seed = args.get(5).map(String::as_str).unwrap_or("20260803");
    let ops = load_ops(ops_path);
    eprintln!("loaded {} ops", ops.len());
    let (total_qubits, num_bits, _nr, regs) = analyze_ops(ops.iter());
    eprintln!("qubits={} bits={} regs={}", total_qubits, num_bits, regs.len());

    let curve = secp256k1();
    // point pool from random scalars (per-thread disjoint seeds via master shake)
    let mut master = Shake256::default();
    master.update(b"stream-census-v2");
    master.update(census_seed.as_bytes());
    let mut mxof = master.finalize_xof();

    let n_ccx = ops.iter().filter(|o| o.kind == OperationType::CCX).count();
    let n_ccz = ops.iter().filter(|o| o.kind == OperationType::CCZ).count();

    let obs: Vec<std::sync::Mutex<Vec<Obs>>> = {
        let mut v = Vec::new();
        for _ in 0..2 {
            let mut col = Vec::with_capacity(ops.len());
            for _ in 0..ops.len() { col.push(Obs { admit:0, fired:0, viol_c1_implies_c2:u64::MAX, viol_c2_implies_c1:u64::MAX, admit_pop:0 }); }
            v.push(std::sync::Mutex::new(col));
        }
        v
    };
    // note: viol masks start MAX meaning "no violation yet"; we AND violations in.
    // admit/fired start 0; we OR in.

    let total_tof = std::sync::atomic::AtomicU64::new(0);
    let inputs_done = std::sync::atomic::AtomicU64::new(0);

    std::thread::scope(|scope| {
        for t in 0..threads {
            let ops = &ops;
            let regs = &regs;
            let obs = &obs;
            let curve = &curve;
            let total_tof = &total_tof;
            let inputs_done = &inputs_done;
            let mut seedbuf = [0u8; 32];
            XofReader::read(&mut mxof, &mut seedbuf);
            let mut txof = {
                let mut h = Shake256::default();
                h.update(&seedbuf);
                h.finalize_xof()
            };
            scope.spawn(move || {
                // build point pool
                let mut pts: Vec<(U256, U256)> = Vec::with_capacity(pool);
                let mut rb = [0u8; 32];
                while pts.len() < pool {
                    XofReader::read(&mut txof, &mut rb);
                    let k = U256::from_le_bytes(rb);
                    let p = curve.mul(curve.gx, curve.gy, k);
                    if !(p.0.is_zero() && p.1.is_zero()) { pts.push(p); }
                }
                let mut qubits = vec![0u64; total_qubits as usize];
                let mut bits = vec![0u64; num_bits as usize];
                let mut local: Vec<Obs> = ops.iter().map(|_| Obs { admit:0, fired:0, viol_c1_implies_c2:u64::MAX, viol_c2_implies_c1:u64::MAX, admit_pop:0 }).collect();
                let mut tof_local = 0u64;
                for _b in 0..batches_per_thread {
                    // sample 64 distinct-ish pairs
                    let mut ti = [0usize; 64]; let mut oi = [0usize; 64];
                    for s in 0..64 {
                        loop {
                            let a = draw_index(&mut txof, pool);
                            let c = draw_index(&mut txof, pool);
                            if a != c { ti[s] = a; oi[s] = c; break; }
                        }
                    }
                    // clear
                    for q in qubits.iter_mut() { *q = 0; }
                    for b in bits.iter_mut() { *b = 0; }
                    let mut phase = 0u64;
                    // set registers per shot (lane s)
                    for s in 0..64 {
                        for (reg, (px, py)) in [(0, pts[ti[s]]), (1, pts[ti[s]]), (2, pts[oi[s]]), (3, pts[oi[s]])] {
                            let (v, which) = if reg % 2 == 0 { (px, 0) } else { (py, 1) };
                            for (k, item) in regs[reg].iter().enumerate() {
                                let bitval = v.bit(k) as u64;
                                match item {
                                    QubitOrBit::Qubit(q) => { if bitval == 1 { qubits[q.0 as usize] |= 1u64 << s; } }
                                    QubitOrBit::Bit(b) => { if bitval == 1 { bits[b.0 as usize] |= 1u64 << s; } }
                                }
                            }
                        }
                    }
                    // instrumented apply
                    let mut cond_stack: Vec<u64> = Vec::new();
                    let mut base = u64::MAX;
                    let dbg = t == 0 && std::env::var("CENSUS_DEBUG").is_ok();
                    if dbg { println!("DBG batch inputs: xt0={} xo0={} xt1={} xo1={}", pts[ti[0]].0.bit(0), pts[oi[0]].0.bit(0), pts[ti[0]].0.bit(1), pts[oi[0]].0.bit(1)); }
                    for (i, op) in ops.iter().enumerate() {
                        if dbg && matches!(i, 1 | 767 | 768 | 1278 | 1535) {
                            println!("DBG i={} q0={} q1={} q512={} q513={} lane0", i, (qubits[0]>>0)&1, (qubits[1]>>0)&1, (qubits[512]>>0)&1, (qubits[513]>>0)&1);
                        }
                        let mut cond = base;
                        if op.c_condition != NO_BIT { cond &= bits[op.c_condition.0 as usize]; }
                        match op.kind {
                            OperationType::CCX => {
                                tof_local += cond.count_ones() as u64;
                                let q1 = qubits[op.q_control1.0 as usize];
                                let q2 = qubits[op.q_control2.0 as usize];
                                let o = &mut local[i];
                                o.admit |= cond;
                                o.admit_pop += cond.count_ones() as u64;
                                let fire = cond & q1 & q2;
                                o.fired |= fire;
                                o.viol_c1_implies_c2 &= !(cond & q1 & !q2);
                                o.viol_c2_implies_c1 &= !(cond & q2 & !q1);
                                let v = fire;
                                qubits[op.q_target.0 as usize] ^= v;
                            }
                            OperationType::CCZ => {
                                tof_local += cond.count_ones() as u64;
                                let q1 = qubits[op.q_control1.0 as usize];
                                let q2 = qubits[op.q_control2.0 as usize];
                                let qt = qubits[op.q_target.0 as usize];
                                let o = &mut local[i];
                                o.admit |= cond;
                                o.admit_pop += cond.count_ones() as u64;
                                let fire = cond & q1 & q2 & qt;
                                o.fired |= fire;
                                // for CCZ record implications between (c1,c2) only (target kept)
                                o.viol_c1_implies_c2 &= !(cond & q1 & !q2);
                                o.viol_c2_implies_c1 &= !(cond & q2 & !q1);
                                phase ^= fire;
                            }
                            OperationType::CX => { let v = cond & qubits[op.q_control1.0 as usize]; qubits[op.q_target.0 as usize] ^= v; }
                            OperationType::Swap => {
                                let mut a = qubits[op.q_control1.0 as usize]; let mut b = qubits[op.q_target.0 as usize];
                                a ^= b; b ^= cond & a; a ^= b;
                                qubits[op.q_control1.0 as usize] = a; qubits[op.q_target.0 as usize] = b;
                            }
                            OperationType::X => { qubits[op.q_target.0 as usize] ^= cond; }
                            OperationType::Z => { phase ^= cond & qubits[op.q_target.0 as usize]; }
                            OperationType::CZ => { phase ^= cond & qubits[op.q_target.0 as usize] & qubits[op.q_control1.0 as usize]; }
                            OperationType::Neg => { phase ^= cond; }
                            OperationType::Hmr | OperationType::R => {
                                let mut buf = [0u8; 8];
                                XofReader::read(&mut txof, &mut buf);
                                let rng_val = u64::from_le_bytes(buf);
                                if op.kind == OperationType::Hmr {
                                    let cb = &mut bits[op.c_target.0 as usize];
                                    *cb &= !cond; *cb ^= rng_val & cond;
                                }
                                let q = op.q_target.0 as usize;
                                phase ^= qubits[q] & rng_val & cond;
                                qubits[q] &= !cond;
                            }
                            OperationType::BitInvert => { bits[op.c_target.0 as usize] ^= cond; }
                            OperationType::BitStore0 => { bits[op.c_target.0 as usize] &= !cond; }
                            OperationType::BitStore1 => { bits[op.c_target.0 as usize] |= cond; }
                            OperationType::PushCondition => { cond_stack.push(base); base &= bits[op.c_condition.0 as usize]; }
                            OperationType::PopCondition => { if let Some(v) = cond_stack.pop() { base = v; } }
                            _ => {}
                        }
                    }
                    inputs_done.fetch_add(64, std::sync::atomic::Ordering::Relaxed);
                }
                total_tof.fetch_add(tof_local, std::sync::atomic::Ordering::Relaxed);
                // merge
                let mut g0 = obs[0].lock().unwrap();
                for (i, o) in local.iter().enumerate() {
                    if o.admit == 0 && o.viol_c1_implies_c2 == u64::MAX && o.viol_c2_implies_c1 == u64::MAX && o.fired == 0 && o.admit_pop == 0 { continue; }
                    let g = &mut g0[i];
                    g.admit |= o.admit; g.fired |= o.fired;
                    g.viol_c1_implies_c2 &= o.viol_c1_implies_c2;
                    g.viol_c2_implies_c1 &= o.viol_c2_implies_c1;
                    g.admit_pop += o.admit_pop;
                }
            });
        }
    });

    let n_inputs = inputs_done.load(std::sync::atomic::Ordering::Relaxed);
    let g = obs[0].lock().unwrap();
    let mut ccx_dead = 0usize; let mut ccz_dead = 0usize;
    let mut ccx_down_c1i = 0usize; let mut ccx_down_c2i = 0usize; let mut ccx_down_eq = 0usize;
    let mut ccz_down = 0usize;
    let mut dead_admit_pop = 0u64; let mut down_admit_pop = 0u64;
    let mut tot_admit_pop = 0u64;
    let mut admitted_ccx = 0usize;
    for (i, op) in ops.iter().enumerate() {
        let o = &g[i];
        match op.kind {
            OperationType::CCX => {
                if o.admit_pop == 0 { continue; } // never admitted in census: unobserved
                admitted_ccx += 1;
                tot_admit_pop += o.admit_pop;
                if o.fired == 0 { ccx_dead += 1; dead_admit_pop += o.admit_pop; }
                else {
                    let a = o.viol_c1_implies_c2 == u64::MAX; // q1 -> q2 HOLDS (no violating lane ever)
                    let b = o.viol_c2_implies_c1 == u64::MAX; // q2 -> q1 HOLDS
                    if a { ccx_down_c1i += 1; down_admit_pop += o.admit_pop; }
                    if b { ccx_down_c2i += 1; }
                    if a && b { ccx_down_eq += 1; }
                }
            }
            OperationType::CCZ => {
                if o.admit_pop == 0 { continue; }
                tot_admit_pop += o.admit_pop;
                if o.fired == 0 { ccz_dead += 1; dead_admit_pop += o.admit_pop; }
                else if o.viol_c1_implies_c2 == u64::MAX || o.viol_c2_implies_c1 == u64::MAX { ccz_down += 1; down_admit_pop += o.admit_pop; }
            }
            _ => {}
        }
    }
    let tt = total_tof.load(std::sync::atomic::Ordering::Relaxed);
    println!("inputs={n_inputs} pool={pool} threads={threads} batches_per_thread={batches_per_thread} seed={census_seed}");
    println!("emitted ccx={n_ccx} ccz={n_ccz}");
    println!("census executed-toffoli total={} avg={:.3}", tt, tt as f64 / n_inputs as f64);
    println!("admitted ccx/ccz (admit_pop>0): {}", admitted_ccx + { let _=0; 0 });
    println!("ccx admitted={} total_admit_pop={}", admitted_ccx, tot_admit_pop);
    println!("NEW dead candidates: ccx={} ccz={} (admit_pop sum {})", ccx_dead, ccz_dead, dead_admit_pop);
    println!("NEW downgrade candidates (HOLD): ccx q1->q2={} ccx q2->q1={} ccx both-equiv={} ccz={} (admit_pop sum {})", ccx_down_c1i, ccx_down_c2i, ccx_down_eq, ccz_down, down_admit_pop);
    println!("est executed-T saving: dead {:.1} + downgrade {:.1}", dead_admit_pop as f64 / n_inputs as f64, down_admit_pop as f64 / n_inputs as f64);
    // admission histograms for candidates
    let mut d_a = [0usize; 5]; let mut g_a = [0usize; 5]; // <0.1, <0.4, <0.9, <1.0, ==1.0
    let mut d_p = [0u64; 5]; let mut g_p = [0u64; 5];
    for (i, op) in ops.iter().enumerate() {
        let o = &g[i];
        if o.admit_pop == 0 { continue; }
        let adm = o.admit_pop as f64 / n_inputs as f64;
        let bucket = if adm < 0.1 {0} else if adm < 0.4 {1} else if adm < 0.9 {2} else if adm < 1.0 {3} else {4};
        let is_cc = op.kind == OperationType::CCX || op.kind == OperationType::CCZ;
        if !is_cc { continue; }
        if o.fired == 0 { d_a[bucket]+=1; d_p[bucket]+=o.admit_pop; }
        else if op.kind == OperationType::CCX && o.viol_c1_implies_c2 == u64::MAX { g_a[bucket]+=1; g_p[bucket]+=o.admit_pop; }
    }
    println!("dead  by admission [<0.1,<0.4,<0.9,<1.0,==1.0]: {:?} pops {:?}", d_a, d_p);
    println!("downg by admission [<0.1,<0.4,<0.9,<1.0,==1.0]: {:?} pops {:?}", g_a, g_p);
    // dump examples of both-equal fired gates
    if std::env::var("DUMP").is_ok() {
        let mut shown = 0;
        for (i, op) in ops.iter().enumerate() {
            if op.kind != OperationType::CCX { continue; }
            let o = &g[i];
            if o.admit_pop == 0 || o.fired == 0 { continue; }
            if o.viol_c1_implies_c2 == u64::MAX && o.viol_c2_implies_c1 == u64::MAX {
                println!("EQ idx={} c2={} c1={} t={} admit={:#x} fired={:#x} admit_pop={}", i, op.q_control2.0, op.q_control1.0, op.q_target.0, o.admit, o.fired, o.admit_pop);
                shown += 1;
                if shown >= 12 { break; }
            }
        }
        // also: distribution of fired vs admit
        let mut full = 0; let mut never_admitted = 0;
        for (i, op) in ops.iter().enumerate() {
            if op.kind != OperationType::CCX { continue; }
            let o = &g[i];
            if o.admit_pop == 0 { never_admitted += 1; }
            else if o.fired == o.admit { full += 1; }
        }
        println!("ccx fired==admit always: {} never_admitted: {}", full, never_admitted);
    }
}
