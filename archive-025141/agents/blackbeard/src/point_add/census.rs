//! Identity-census miner: find CCX/CCZ gates that are provably-observed dead
//! (never fire) or downgradable (one control is redundant) across a large
//! number of reachable inputs, so the strip postpass can delete / downgrade
//! them with zero error.
//!
//! This is the instrument that produced the shipped `deep_strip_keys.rs` and
//! `route_042_action_mask.tsv` tables. It lives inside `src/point_add/` so it
//! can reuse `crate::sim` / `crate::circuit` / `crate::weierstrass_elliptic_curve`
//! exactly as the trusted harness does. Triggered by the `CENSUS_MINE` env var
//! at the end of `point_add::build()`; prints keys in the same TSV shape the
//! action-mask loader consumes (`index<TAB>delete|drop_q1|drop_q2`).
//!
//! Semantics (must match `apply_q1150_inverse_cswap_action_mask` and
//! `apply_deep_strip_identity`):
//!   - a CCX/CCZ is DEAD iff `cond & q1 & q2` (CCX) / `cond & t & q1 & q2`
//!     (CCZ) is zero on every reachable input  ->  `delete`
//!   - `drop_q1`: `cond & ~q1` is always zero (q1 == 1 whenever it executes)
//!     -> CCX(c2,c1,t) == CX(c2,t)
//!   - `drop_q2`: `cond & q1 & ~q2` is always zero (q1 == 1 implies q2 == 1)
//!     -> CCX(c2,c1,t) == CX(c1,t)

use crate::circuit::{Op, OperationType, QubitOrBit, NO_BIT};
use crate::weierstrass_elliptic_curve::WeierstrassEllipticCurve;
use alloy_primitives::U256;
use sha3::digest::{ExtendableOutput, Update, XofReader};
use sha3::Shake256;
use std::io::{BufReader, BufWriter, Read, Write};

const RAW_MAGIC: &[u8; 8] = b"CENSUSOP";

/// Serialize the final op stream to a raw file so parallel census shards can
/// load it without re-running the (expensive, single-threaded) build.
/// Format: MAGIC, u64 count, then count x 56-byte records (same layout as
/// ops.bin: u32 kind, u32 pad, 6 x u64 operands, all little-endian).
pub fn dump_ops_raw(ops: &[Op], path: &str) {
    let mut w = BufWriter::new(std::fs::File::create(path).expect("census dump create"));
    w.write_all(RAW_MAGIC).unwrap();
    w.write_all(&(ops.len() as u64).to_le_bytes()).unwrap();
    let mut rec = [0u8; 56];
    for op in ops {
        rec[0..4].copy_from_slice(&(op.kind as u32).to_le_bytes());
        rec[4..8].copy_from_slice(&[0u8; 4]);
        rec[8..16].copy_from_slice(&op.q_control2.0.to_le_bytes());
        rec[16..24].copy_from_slice(&op.q_control1.0.to_le_bytes());
        rec[24..32].copy_from_slice(&op.q_target.0.to_le_bytes());
        rec[32..40].copy_from_slice(&op.c_target.0.to_le_bytes());
        rec[40..48].copy_from_slice(&op.c_condition.0.to_le_bytes());
        rec[48..56].copy_from_slice(&op.r_target.0.to_le_bytes());
        w.write_all(&rec).unwrap();
    }
    w.flush().unwrap();
    eprintln!("CENSUS: dumped {} ops to {path}", ops.len());
}

/// Load a raw op stream dumped by `dump_ops_raw`.
pub fn load_ops_raw(path: &str) -> Vec<Op> {
    let mut f = BufReader::new(std::fs::File::open(path).expect("census load open"));
    let mut magic = [0u8; 8];
    f.read_exact(&mut magic).unwrap();
    assert_eq!(&magic, RAW_MAGIC, "bad census raw magic");
    let mut count = [0u8; 8];
    f.read_exact(&mut count).unwrap();
    let n = u64::from_le_bytes(count) as usize;
    let mut ops = Vec::with_capacity(n);
    let mut rec = [0u8; 56];
    for _ in 0..n {
        f.read_exact(&mut rec).unwrap();
        let kind = op_kind_from_u32(u32::from_le_bytes(rec[0..4].try_into().unwrap()));
        ops.push(Op {
            kind,
            q_control2: crate::circuit::QubitId(u64::from_le_bytes(rec[8..16].try_into().unwrap())),
            q_control1: crate::circuit::QubitId(u64::from_le_bytes(rec[16..24].try_into().unwrap())),
            q_target: crate::circuit::QubitId(u64::from_le_bytes(rec[24..32].try_into().unwrap())),
            c_target: crate::circuit::BitId(u64::from_le_bytes(rec[32..40].try_into().unwrap())),
            c_condition: crate::circuit::BitId(u64::from_le_bytes(rec[40..48].try_into().unwrap())),
            r_target: crate::circuit::RegisterId(u64::from_le_bytes(rec[48..56].try_into().unwrap())),
        });
    }
    ops
}

fn op_kind_from_u32(v: u32) -> OperationType {
    match v {
        0 => OperationType::Neg,
        1 => OperationType::Register,
        2 => OperationType::AppendToRegister,
        3 => OperationType::BitInvert,
        4 => OperationType::BitStore0,
        5 => OperationType::BitStore1,
        6 => OperationType::X,
        7 => OperationType::Z,
        8 => OperationType::CX,
        9 => OperationType::CZ,
        10 => OperationType::Swap,
        11 => OperationType::R,
        12 => OperationType::Hmr,
        13 => OperationType::CCX,
        14 => OperationType::CCZ,
        15 => OperationType::PushCondition,
        16 => OperationType::PopCondition,
        17 => OperationType::DebugPrint,
        _ => panic!("unknown op kind {v}"),
    }
}

fn secp256k1() -> WeierstrassEllipticCurve {
    WeierstrassEllipticCurve {
        modulus: U256::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F",
            16,
        )
        .unwrap(),
        a: U256::from(0),
        b: U256::from(7),
        gx: U256::from_str_radix(
            "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",
            16,
        )
        .unwrap(),
        gy: U256::from_str_radix(
            "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",
            16,
        )
        .unwrap(),
        order: U256::from_str_radix(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141",
            16,
        )
        .unwrap(),
    }
}

/// One tracked CCX/CCZ gate: three accumulated bitmasks over all observed
/// shots. A mask bit is 1 iff that predicate held on that (batch,shot).
struct TrackedGate {
    index: usize,
    fired: u64,
    q1_always_one_violation: u64, // cond & ~q1   (q1 == 0 on an executing shot)
    q2_implied_violation: u64,    // cond & q1 & ~q2
}

/// Run the census. `ops` is the (already-postpassed) op stream the verifier
/// will see. `num_shots` is rounded up to a multiple of 64.
pub fn census_report(
    ops: &[Op],
    regs: &[Vec<QubitOrBit>],
    total_qubits: usize,
    num_bits: usize,
    num_shots: usize,
) {
    // Sharding: CENSUS_SHARD / CENSUS_SHARDS select a deterministic slice of
    // the input+XOF space so many processes can mine disjoint batches in
    // parallel and their per-gate masks are merged afterward (OR).
    let shard: u64 = std::env::var("CENSUS_SHARD")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    let shards: u64 = std::env::var("CENSUS_SHARDS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let raw: bool = std::env::var_os("CENSUS_RAW").is_some();

    // Identify and index the CCX/CCZ gates (map op index -> slot).
    let mut slot_of: Vec<usize> = vec![usize::MAX; ops.len()];
    let mut gates: Vec<TrackedGate> = Vec::new();
    for (index, op) in ops.iter().enumerate() {
        match op.kind {
            OperationType::CCX | OperationType::CCZ => {
                slot_of[index] = gates.len();
                gates.push(TrackedGate {
                    index,
                    fired: 0,
                    q1_always_one_violation: 0,
                    q2_implied_violation: 0,
                });
            }
            _ => {}
        }
    }
    eprintln!(
        "CENSUS: shard {}/{} tracking {} CCX/CCZ gates across {} ops",
        shard,
        shards,
        gates.len(),
        ops.len()
    );

    let curve = secp256k1();

    // A single deterministic XOF drives both input generation and the Hmr/R
    // measurement stream, mirroring `eval_circuit::run_tests`. The census
    // wants far more inputs than the verifier's 9024, so we draw them all
    // from this census-specific XOF.
    let mut hasher = Shake256::default();
    hasher.update(b"quantum_ecc-census-v1");
    hasher.update(&(ops.len() as u64).to_le_bytes());
    hasher.update(&(num_shots as u64).to_le_bytes());
    hasher.update(&shard.to_le_bytes());
    hasher.update(&shards.to_le_bytes());
    let mut xof = hasher.finalize_xof();

    let batches = num_shots.div_ceil(64);
    let mut qubits = vec![0u64; total_qubits];
    let mut bits = vec![0u64; num_bits];
    let mut phase: u64 = 0;

    for batch in 0..batches {
        // --- generate 64 reachable inputs ---------------------------------
        // (target = k1*G, offset = k2*G), skip degenerate pairs like the
        // verifier does.
        qubits.iter_mut().for_each(|v| *v = 0);
        bits.iter_mut().for_each(|v| *v = 0);
        phase = 0;
        for shot in 0..64 {
            let mut rb = [[0u8; 32]; 2];
            XofReader::read(&mut xof, &mut rb[0]);
            XofReader::read(&mut xof, &mut rb[1]);
            let k1 = U256::from_le_bytes(rb[0]);
            let k2 = U256::from_le_bytes(rb[1]);
            let t = curve.mul(curve.gx, curve.gy, k1);
            let o = curve.mul(curve.gx, curve.gy, k2);
            if t.0 == o.0 || (t.0.is_zero() && t.1.is_zero()) || (o.0.is_zero() && o.1.is_zero()) {
                continue;
            }
            for (i, qb) in regs[0].iter().enumerate() {
                if let QubitOrBit::Qubit(q) = qb {
                    if t.0.bit(i) {
                        qubits[q.0 as usize] |= 1 << shot;
                    }
                }
            }
            for (i, qb) in regs[1].iter().enumerate() {
                if let QubitOrBit::Qubit(q) = qb {
                    if t.1.bit(i) {
                        qubits[q.0 as usize] |= 1 << shot;
                    }
                }
            }
            for (i, qb) in regs[2].iter().enumerate() {
                if let QubitOrBit::Bit(b) = qb {
                    if o.0.bit(i) {
                        bits[b.0 as usize] |= 1 << shot;
                    }
                }
            }
            for (i, qb) in regs[3].iter().enumerate() {
                if let QubitOrBit::Bit(b) = qb {
                    if o.1.bit(i) {
                        bits[b.0 as usize] |= 1 << shot;
                    }
                }
            }
        }

        // --- instrumented simulation (full apply_iter replica) ------------
        let mut condition_stack: Vec<u64> = Vec::new();
        let mut current_base_condition = u64::MAX;
        for (op_index, op) in ops.iter().enumerate() {
            let mut cond = current_base_condition;
            if op.c_condition != NO_BIT {
                cond &= bits[op.c_condition.0 as usize];
            }
            match op.kind {
                OperationType::CCX => {
                    let q1 = qubits[op.q_control1.0 as usize];
                    let q2 = qubits[op.q_control2.0 as usize];
                    let fired = cond & q1 & q2;
                    let slot = slot_of[op_index];
                    if slot != usize::MAX {
                        let g = &mut gates[slot];
                        g.fired |= fired;
                        g.q1_always_one_violation |= cond & !q1;
                        g.q2_implied_violation |= cond & q1 & !q2;
                    }
                    qubits[op.q_target.0 as usize] ^= fired;
                }
                OperationType::CX => {
                    let v = cond & qubits[op.q_control1.0 as usize];
                    qubits[op.q_target.0 as usize] ^= v;
                }
                OperationType::Swap => {
                    let mut q_c1 = qubits[op.q_control1.0 as usize];
                    let mut q_t = qubits[op.q_target.0 as usize];
                    q_c1 ^= q_t;
                    q_t ^= cond & q_c1;
                    q_c1 ^= q_t;
                    qubits[op.q_control1.0 as usize] = q_c1;
                    qubits[op.q_target.0 as usize] = q_t;
                }
                OperationType::X => {
                    qubits[op.q_target.0 as usize] ^= cond;
                }
                OperationType::CCZ => {
                    let q1 = qubits[op.q_control1.0 as usize];
                    let q2 = qubits[op.q_control2.0 as usize];
                    let t = qubits[op.q_target.0 as usize];
                    let fired = cond & t & q1 & q2;
                    let slot = slot_of[op_index];
                    if slot != usize::MAX {
                        let g = &mut gates[slot];
                        g.fired |= fired;
                        g.q1_always_one_violation |= cond & !q1;
                        g.q2_implied_violation |= cond & q1 & !q2;
                    }
                    phase ^= fired;
                }
                OperationType::CZ => {
                    let v = cond & qubits[op.q_target.0 as usize] & qubits[op.q_control1.0 as usize];
                    phase ^= v;
                }
                OperationType::Z => {
                    let v = cond & qubits[op.q_target.0 as usize];
                    phase ^= v;
                }
                OperationType::Neg => {
                    phase ^= cond;
                }
                OperationType::Hmr => {
                    let mut buf = [0u8; 8];
                    XofReader::read(&mut xof, &mut buf);
                    let rng_val = u64::from_le_bytes(buf);
                    bits[op.c_target.0 as usize] &= !cond;
                    bits[op.c_target.0 as usize] ^= rng_val & cond;
                    phase ^= qubits[op.q_target.0 as usize] & rng_val & cond;
                    qubits[op.q_target.0 as usize] &= !cond;
                }
                OperationType::R => {
                    let mut buf = [0u8; 8];
                    XofReader::read(&mut xof, &mut buf);
                    let rng_val = u64::from_le_bytes(buf);
                    phase ^= qubits[op.q_target.0 as usize] & rng_val & cond;
                    qubits[op.q_target.0 as usize] &= !cond;
                }
                OperationType::BitInvert => {
                    bits[op.c_target.0 as usize] ^= cond;
                }
                OperationType::BitStore0 => {
                    bits[op.c_target.0 as usize] &= !cond;
                }
                OperationType::BitStore1 => {
                    bits[op.c_target.0 as usize] |= cond;
                }
                OperationType::AppendToRegister
                | OperationType::Register
                | OperationType::DebugPrint => {}
                OperationType::PushCondition => {
                    condition_stack.push(current_base_condition);
                    current_base_condition &= bits[op.c_condition.0 as usize];
                }
                OperationType::PopCondition => {
                    if let Some(val) = condition_stack.pop() {
                        current_base_condition = val;
                    }
                }
            }
        }

        if (batch + 1) % 256 == 0 {
            eprintln!(
                "CENSUS: {}/{} batches ({} shots)",
                batch + 1,
                batches,
                (batch + 1) * 64
            );
        }
    }

    // --- classify and report ----------------------------------------------
    if raw {
        // Per-gate hex masks for cross-shard merging (OR then classify).
        for gate in &gates {
            println!(
                "{}\t{:016x}\t{:016x}\t{:016x}",
                gate.index, gate.fired, gate.q1_always_one_violation, gate.q2_implied_violation
            );
        }
        return;
    }
    let mut dead = Vec::new();
    let mut drop_q1 = Vec::new();
    let mut drop_q2 = Vec::new();
    for gate in &gates {
        if gate.fired == 0 {
            dead.push(gate.index);
        } else if gate.q1_always_one_violation == 0 {
            drop_q1.push(gate.index);
        } else if gate.q2_implied_violation == 0 {
            drop_q2.push(gate.index);
        }
    }
    eprintln!(
        "CENSUS: done. dead={} drop_q1={} drop_q2={} (of {} CCX/CCZ)",
        dead.len(),
        drop_q1.len(),
        drop_q2.len(),
        gates.len()
    );
    for &i in &dead {
        println!("{}\tdelete", i);
    }
    for &i in &drop_q1 {
        println!("{}\tdrop_q1", i);
    }
    for &i in &drop_q2 {
        println!("{}\tdrop_q2", i);
    }
}
