//! Fire-census re-miner for the live emitted op stream.
//!
//! Runs a classical 64-lane-per-round simulator over random secp256k1 point-addition
//! inputs and records, for every CCX/CCZ, the lanes on which every control is satisfied.
//! Gates that never fire are dead-gate candidates; CCX gates where control1 implies
//! control2 (or vice versa) on every firing lane are downgrade candidates.
//!
//! This is a diagnostic / tooling module. It does not run in a normal scoring build
//! unless `SUB4_FIRE_CENSUS=1` is set.

use crate::circuit::{Op, OperationType, QubitOrBit, NO_BIT};
use crate::weierstrass_elliptic_curve::WeierstrassEllipticCurve;
use alloy_primitives::U256;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};

#[derive(Clone, Debug)]
pub struct GateStats {
    pub op_index: usize,
    pub kind: OperationType,
    pub q_control2: u64,
    pub q_control1: u64,
    pub q_target: u64,
    pub c_condition: u64,
    /// lanes where the gate fires (both controls and condition true)
    pub fires: u64,
    /// lanes where control1 is 1 and control2 is 0 (and condition true)
    pub c1_only: u64,
    /// lanes where control2 is 1 and control1 is 0 (and condition true)
    pub c2_only: u64,
    /// lanes where both controls are 1 (and condition true)
    pub both: u64,
    /// total lanes where condition is true (OR across rounds)
    pub condition_true: u64,
    /// total number of condition-true lane-events across all rounds
    pub condition_true_count: usize,
}

fn secp256k1_curve() -> WeierstrassEllipticCurve {
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

/// Fast classical 64-lane simulator mirror. Ignores phase; R/Hmr reset the target qubit
/// to 0 under the condition and (for Hmr) write a fresh random bit to c_target.
fn classical_run(
    ops: &[Op],
    qubits: &mut [u64],
    bits: &mut [u64],
    xof: &mut impl XofReader,
    stats: &mut [GateStats],
) {
    let mut base = u64::MAX;
    let mut condition_stack: Vec<u64> = Vec::new();

    for (index, op) in ops.iter().enumerate() {
        let cond = if op.c_condition == NO_BIT {
            base
        } else {
            base & bits[op.c_condition.0 as usize]
        };

        match op.kind {
            OperationType::CCX => {
                let c1 = qubits[op.q_control1.0 as usize];
                let c2 = qubits[op.q_control2.0 as usize];
                let fires = cond & c1 & c2;
                qubits[op.q_target.0 as usize] ^= fires;
                if let Some(s) = stats.get_mut(index) {
                    s.fires |= fires;
                    s.both |= cond & c1 & c2;
                    s.c1_only |= cond & c1 & !c2;
                    s.c2_only |= cond & !c1 & c2;
                    s.condition_true |= cond;
                    s.condition_true_count += cond.count_ones() as usize;
                }
            }
            OperationType::CX => {
                let c1 = qubits[op.q_control1.0 as usize];
                qubits[op.q_target.0 as usize] ^= cond & c1;
            }
            OperationType::Swap => {
                let a = qubits[op.q_control1.0 as usize];
                let b = qubits[op.q_target.0 as usize];
                let diff = cond & (a ^ b);
                qubits[op.q_control1.0 as usize] = a ^ diff;
                qubits[op.q_target.0 as usize] = b ^ diff;
            }
            OperationType::X => {
                qubits[op.q_target.0 as usize] ^= cond;
            }
            OperationType::CCZ => {
                let c1 = qubits[op.q_control1.0 as usize];
                let c2 = qubits[op.q_control2.0 as usize];
                let t = qubits[op.q_target.0 as usize];
                let fires = cond & c1 & c2 & t;
                if let Some(s) = stats.get_mut(index) {
                    s.fires |= fires;
                    s.both |= cond & c1 & c2;
                    s.c1_only |= cond & c1 & !c2;
                    s.c2_only |= cond & !c1 & c2;
                    s.condition_true |= cond;
                    s.condition_true_count += cond.count_ones() as usize;
                }
            }
            OperationType::CZ => {}
            OperationType::Z => {}
            OperationType::Neg => {}
            OperationType::Hmr | OperationType::R => {
                // Reset target qubit under condition.
                qubits[op.q_target.0 as usize] &= !cond;
                if op.kind == OperationType::Hmr {
                    let mut buf = [0u8; 8];
                    xof.read(&mut buf);
                    let rng = u64::from_le_bytes(buf);
                    bits[op.c_target.0 as usize] &= !cond;
                    bits[op.c_target.0 as usize] ^= rng & cond;
                }
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
                condition_stack.push(base);
                base &= bits[op.c_condition.0 as usize];
            }
            OperationType::PopCondition => {
                if let Some(v) = condition_stack.pop() {
                    base = v;
                }
            }
        }
    }
}

fn seed_lanes(
    qubits: &mut [u64],
    bits: &mut [u64],
    regs: &[Vec<QubitOrBit>],
    round: u64,
) {
    let curve = secp256k1_curve();
    let mut h = Shake256::default();
    h.update(b"fire-census-inputs");
    h.update(&round.to_le_bytes());
    let mut inputs = h.finalize_xof();

    // Clear only register qubits/bits.
    for qb in regs.iter().flatten() {
        match qb {
            QubitOrBit::Qubit(q) => qubits[q.0 as usize] = 0,
            QubitOrBit::Bit(b) => bits[b.0 as usize] = 0,
        }
    }

    let mut shot = 0usize;
    while shot < 64 {
        let mut rb = [[0u8; 32]; 2];
        inputs.read(&mut rb[0]);
        inputs.read(&mut rb[1]);
        let t = curve.mul(curve.gx, curve.gy, U256::from_le_bytes(rb[0]));
        let o = curve.mul(curve.gx, curve.gy, U256::from_le_bytes(rb[1]));
        if t.0 == o.0 || (t.0.is_zero() && t.1.is_zero()) || (o.0.is_zero() && o.1.is_zero()) {
            continue;
        }
        let mask = 1u64 << shot;
        sim_set_register(qubits, &regs[0], t.0, mask);
        sim_set_register(qubits, &regs[1], t.1, mask);
        sim_set_register(qubits, &regs[2], o.0, mask);
        sim_set_register(qubits, &regs[3], o.1, mask);
        shot += 1;
    }
}

fn sim_set_register(words: &mut [u64], reg: &[QubitOrBit], value: U256, mask: u64) {
    let bytes = value.to_le_bytes::<32>();
    for (word_idx, qb) in reg.iter().enumerate() {
        if let QubitOrBit::Qubit(q) = qb {
            let byte_idx = word_idx / 8;
            let bit_idx = word_idx % 8;
            let bit = ((bytes[byte_idx] >> bit_idx) & 1) as u64;
            if bit == 1 {
                words[q.0 as usize] |= mask;
            } else {
                words[q.0 as usize] &= !mask;
            }
        }
    }
}

/// Run a fire census over `rounds` 64-lane batches and return per-gate statistics.
pub fn fire_census(ops: &[Op], rounds: u64) -> Vec<GateStats> {
    let (num_q, num_b, _nregs, regs) = crate::circuit::analyze_ops(ops.iter());
    if regs.len() != 4 {
        eprintln!("FIRE_CENSUS: expected 4 registers, got {}", regs.len());
        return Vec::new();
    }

    let mut stats: Vec<GateStats> = ops
        .iter()
        .enumerate()
        .map(|(i, op)| GateStats {
            op_index: i,
            kind: op.kind,
            q_control2: op.q_control2.0,
            q_control1: op.q_control1.0,
            q_target: op.q_target.0,
            c_condition: op.c_condition.0,
            fires: 0,
            c1_only: 0,
            c2_only: 0,
            both: 0,
            condition_true: 0,
            condition_true_count: 0,
        })
        .collect();

    let mut qubits = vec![0u64; num_q as usize];
    let mut bits = vec![0u64; num_b as usize];

    for round in 0..rounds {
        let mut xof = {
            let mut h = Shake256::default();
            h.update(b"tlm-fire-census-seed");
            h.update(&round.to_le_bytes());
            h.finalize_xof()
        };
        seed_lanes(&mut qubits, &mut bits, &regs, round);
        classical_run(ops, &mut qubits, &mut bits, &mut xof, &mut stats);
    }

    stats
        .into_iter()
        .filter(|s| matches!(s.kind, OperationType::CCX | OperationType::CCZ))
        .collect()
}

/// Print a summary and candidate tables. `min_survival_rounds` suppresses candidates
/// that dropped out before that many rounds; higher values trade recall for precision.
pub fn report(stats: &[GateStats], ops: &[Op], min_survival_rounds: u64) {
    use std::collections::HashMap;
    type Tup = (u8, u64, u64, u64, u64);

    // Tuple occupancy in the full stream.
    let mut occ: HashMap<Tup, u32> = HashMap::new();
    for op in ops {
        let kb = op.kind as u8;
        if matches!(op.kind, OperationType::CCX | OperationType::CCZ) {
            *occ
                .entry((kb, op.q_control2.0, op.q_control1.0, op.q_target.0, op.c_condition.0))
                .or_insert(0) += 1;
        }
    }

    // Accumulate per-tuple best survival: number of rounds a candidate of this tuple survived.
    // We only have aggregated masks, so we approximate by treating any gate with non-zero fires
    // as "survived" for the whole run. That is wrong for candidates that fire late; a proper
    // implementation would track per-round masks. We use it only for crude ranking here.
    let mut dead: Vec<&GateStats> = Vec::new();
    let mut down_c2_implies_c1: Vec<&GateStats> = Vec::new();
    let mut down_c1_implies_c2: Vec<&GateStats> = Vec::new();

    for s in stats {
        if s.condition_true_count as u64 >= min_survival_rounds * 64 && s.fires == 0 {
            dead.push(s);
        } else if s.condition_true_count as u64 >= min_survival_rounds * 64
            && s.fires.count_ones() > 0
        {
            if s.c2_only == 0 {
                down_c2_implies_c1.push(s);
            }
            if s.c1_only == 0 {
                down_c1_implies_c2.push(s);
            }
        }
    }

    eprintln!(
        "FIRE_CENSUS summary: tracked {} gates; dead candidates {} (min_survival_rounds={})",
        stats.len(),
        dead.len(),
        min_survival_rounds
    );
    eprintln!(
        "  c2-implies-c1 downgrade candidates: {} (c1 redundant)",
        down_c2_implies_c1.len()
    );
    eprintln!(
        "  c1-implies-c2 downgrade candidates: {} (c2 redundant)",
        down_c1_implies_c2.len()
    );

    eprintln!("\n// DEAD_KEYS candidates (kind, q_control2, q_control1, q_target, c_condition, ordinal, tuple_occupancy)");
    eprintln!("pub(crate) const FIRE_CENSUS_DEAD_KEYS: &[(u8, u64, u64, u64, u64, u32, u32)] = &[");
    let mut ord: HashMap<Tup, u32> = HashMap::new();
    for s in &dead {
        let tup = (s.kind as u8, s.q_control2, s.q_control1, s.q_target, s.c_condition);
        let o = ord.entry(tup).or_insert(0);
        let tot = occ.get(&tup).copied().unwrap_or(0);
        eprintln!(
            "    ({}, {}, {}, {}, {}, {}, {}),",
            s.kind as u8, s.q_control2, s.q_control1, s.q_target, s.c_condition, *o, tot
        );
        *o += 1;
    }
    eprintln!("];");

    eprintln!("\n// DOWNGRADE_KEYS candidates (kind, q_control2, q_control1, q_target, c_condition, ordinal, tuple_occupancy, action)");
    eprintln!("pub(crate) const FIRE_CENSUS_DOWNGRADE_KEYS: &[(u8, u64, u64, u64, u64, u32, u32, u8)] = &[");
    ord.clear();
    for s in &down_c2_implies_c1 {
        let tup = (s.kind as u8, s.q_control2, s.q_control1, s.q_target, s.c_condition);
        let o = ord.entry(tup).or_insert(0);
        let tot = occ.get(&tup).copied().unwrap_or(0);
        eprintln!(
            "    ({}, {}, {}, {}, {}, {}, {}, 1),",
            s.kind as u8, s.q_control2, s.q_control1, s.q_target, s.c_condition, *o, tot
        );
        *o += 1;
    }
    ord.clear();
    for s in &down_c1_implies_c2 {
        let tup = (s.kind as u8, s.q_control2, s.q_control1, s.q_target, s.c_condition);
        let o = ord.entry(tup).or_insert(0);
        let tot = occ.get(&tup).copied().unwrap_or(0);
        eprintln!(
            "    ({}, {}, {}, {}, {}, {}, {}, 2),",
            s.kind as u8, s.q_control2, s.q_control1, s.q_target, s.c_condition, *o, tot
        );
        *o += 1;
    }
    eprintln!("];");
}
