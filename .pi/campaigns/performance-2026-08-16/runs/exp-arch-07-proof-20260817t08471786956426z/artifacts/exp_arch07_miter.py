#!/usr/bin/env python3
"""EXP-ARCH-07 exact local cone miter; output-only, no circuit build."""
import argparse, hashlib, json, re, sys
from pathlib import Path

P = 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F
W = 256
MASK = (1 << W) - 1
TERM = {"name":"b_hi_f_32_sub", "shift":32, "outer_op":"Sub", "naf_op":"Sub"}
EDGE = [0, 1, P - 2, P - 1]
# Fixed finite source-value corpus for hi=lambda[128..256], interpreted little-endian.
HI = [1, 2, 3, (1 << 64) + 1, (1 << 127) - 1, 1 << 127, (1 << 128) - 2]

def canon(x): return f"0x{x:064x}"
def sha_bytes(b): return hashlib.sha256(b).hexdigest()
def dump(path, obj):
    Path(path).write_text(json.dumps(obj, sort_keys=True, indent=2) + "\n")

def relevant_sources(root):
    names = ["src/point_add/trailmix_ludicrous/square.rs", "src/point_add/trailmix_ludicrous/gcd.rs", "src/point_add/trailmix_ludicrous/codec.rs", "src/point_add/trailmix_ludicrous/ec_add.rs"]
    return {n: sha_bytes((root/n).read_bytes()) for n in names}

def extraction(root):
    square = (root / "src/point_add/trailmix_ludicrous/square.rs").read_text()
    gcd = (root / "src/point_add/trailmix_ludicrous/gcd.rs").read_text()
    codec = (root / "src/point_add/trailmix_ludicrous/codec.rs").read_text()
    ec = (root / "src/point_add/trailmix_ludicrous/ec_add.rs").read_text()
    required = {
      "b_prod_is_square_of_hi": "symmetric_square_into_prod(circ, hi, &mut b_prod);",
      "b_hi_f_sub_call": "apply_f_times_value(circ, &b_prod, output_reg, ShiftOp::Sub);",
      "selected_naf_32_sub": "(32, ShiftOp::Sub)",
      "full_width_path": "if value.len() == N {",
      "selected_hi_term_helper": "apply_shifted_hi_term(circ, value, output_reg, shift, term_op);",
      "shifted_low_sub": "mod_sub_shifted_low(circ, &hi[..n - shift], output_reg, shift)",
      "high_bit_correction_sub": "ShiftOp::Sub => sub_f_window_shifted(circ, ctrl, output_reg, t)",
      "square_then_forward": "mod_square_sub_pm_secp256k1_symmetric(circ, &y2[..N], x2);\n\n    circ.set_phase(\"tlm_forward_multiply\")",
      "forward_walk": "let mut tape = forward_gcd_jump(circ, &mut xv, None);",
      "step0_t1_shift": "circ.cx(v[0], t1);\n            circ.x(t1);\n            controlled_right_shift(circ, &t1, &v[..current_n]);",
      "step0_s2_shift": "circ.cx(v[0], s2);\n        circ.x(s2);\n        controlled_right_shift(circ, &s2, &v[..current_n]);",
      "step0_sub": "circ.cx(v[0], subtracted);",
      "step0_raw": "circ.cx(slots[0], slots[1]);",
      "step0_encode": "let data = super::codec::compress_step0_with_t1(circ, t1, &slots);",
      "step0_decode": "let (t1, raw) = super::codec::decompress_step0_with_t1(circ, &data);",
      "encoder_s2_pair": "circ.ccx(t1, s2, sub);",
    }
    found = {k: (v in (square + gcd + codec + ec)) for k,v in required.items()}
    # Algebra from F_NAF_TERMS: -(1 + 2^4 - 2^6 + 2^10 + 2^32) = -(2^32+977).
    naf = -(1 + (1<<4) - (1<<6) + (1<<10) + (1<<32))
    return {"passed": all(found.values()) and naf == -( (1<<32)+977),
      "selected_term": TERM, "source_hashes": relevant_sources(root), "required_textual_anchors":found,
      "algebra":{"naf_sum":naf,"expected":-((1<<32)+977),"term_semantics":"selected source path subtracts (b_prod << 32) modulo p; high bits receive the source correction loop"},
      "interface":{"square":"hi=lambda[128..256]; b_prod=hi^2 (256-bit raw product); selected term is b_hi_f_32_sub", "walk":"post-square x2 is passed as xv to Direction::Forward -> forward_gcd_jump", "controls":"Step0 t1=!v0; shift; s2=!v0; shift; subtracted=v0; swp=subtracted", "codec":"raw=[subtracted,swp,s2], Step0 encoded=[t1 xor subtracted,s2], decoder restores raw and t1"}}

def step0(v):
    """Exact 256-bit i=0 state per gcd.rs; all values are registers, not field reductions."""
    u = P
    t1 = 1 ^ (v & 1)
    v1 = v >> t1
    s2 = 1 ^ (v1 & 1)
    v2 = v1 >> s2
    sub = v2 & 1
    swp = sub
    # for i=0, only bit positions 1..255 swap; both bit 0s are one in sub branch.
    if swp:
        u, v2 = ((v2 & ~1) | (u & 1)), ((u & ~1) | (v2 & 1))
    # `for q in v {x}; controlled_add_active(... ForwardKnownOneAfterCx)`.
    # In the sub branch y0 goes complement(1)^1=1 and the high 255-bit adder is modulo 2^255.
    inv_v = (~v2) & MASK
    if sub:
        vpost = (((inv_v >> 1) + (u >> 1)) & ((1 << 255) - 1)) << 1 | 1
    else:
        vpost = inv_v
    raw = [sub, sub, s2]
    encoded = [t1 ^ sub, s2]
    # Direct Boolean decoder in codec.rs.
    dec_sub = 1 ^ (encoded[0] & encoded[1])
    dec_t1 = encoded[0] ^ dec_sub
    dec_raw = [dec_sub, dec_sub, encoded[1]]
    assert raw == dec_raw and t1 == dec_t1
    return {"input_v":canon(v), "tuple":{"t1":t1,"s2":s2,"subtracted":sub,"swp":swp}, "raw_symbol":raw, "encoded_symbol":encoded, "decoded":{"t1":dec_t1,"raw_symbol":dec_raw}, "post_i0":{"u":canon(u),"v":canon(vpost)}}

def case(hi, on):
    b = hi * hi
    assert b < 1 << 256
    delta = (b << 32) % P
    # Baseline selected source term is subtract delta. Removing/defering just this term makes input +delta.
    off = (on + delta) % P
    a,bstate = step0(on),step0(off)
    return {"hi":canon(hi), "b_prod":canon(b), "term_delta":canon(delta), "term_on_input":canon(on), "term_off_input":canon(off), "term_on":a,"term_off":bstate,
      "mismatch": a["tuple"] != bstate["tuple"] or a["raw_symbol"] != bstate["raw_symbol"] or a["encoded_symbol"] != bstate["encoded_symbol"] or a["post_i0"] != bstate["post_i0"]}

def d1(root, out):
    ext=extraction(root)
    cases=[case(h,r) for h in HI for r in EDGE]
    witnesses=[c for c in cases if c["mismatch"]]
    corpus={"kind":"declared finite cross-product", "edge_residues":[canon(x) for x in EDGE],"hi_values":[canon(x) for x in HI],"size":len(cases),"seed":"EXP-ARCH-07-fixed-v1", "sha256":sha_bytes(json.dumps({"edge":EDGE,"hi":HI},sort_keys=True).encode())}
    result={"gate":"D1","d0_extraction_passed":ext["passed"],"model_scope":"256-bit exact register arithmetic at forward_gcd_jump i=0; selected b_hi f/shift32/Sub term only; finite local-cone corpus, no claim of full EC-shot reachability", "invariants":["same complete tuple (t1,s2,subtracted,swp)","same Step0 raw symbol", "same Step0 encoded symbol and Boolean decoder", "same post-i=0 (u,v) register state", "paired s2 conditionality in encoder/decoder"], "corpus":corpus,"source_hashes":ext["source_hashes"],"case_count":len(cases),"mismatch_count":len(witnesses),"witnesses":witnesses,"outcome":"VERIFIED_WITNESS" if witnesses and ext["passed"] else "NO_VERIFIED_WITNESS"}
    dump(out,result)
    return result

def replay(root, inp, out):
    doc=json.loads(Path(inp).read_text())
    w=doc.get("witnesses", [None])[0]
    if not w: raise SystemExit("no witness in replay input")
    h=int(w["hi"],16); on=int(w["term_on_input"],16)
    fresh=case(h,on)
    keys=["hi","b_prod","term_delta","term_on_input","term_off_input","term_on","term_off","mismatch"]
    same=all(fresh[k]==w[k] for k in keys)
    result={"gate":"REPLAY","input":inp,"source_hashes":relevant_sources(root),"recomputed":fresh,"replay_match":same,"outcome":"INDEPENDENT_INTEGER_REPLAY_PASSED" if same and fresh["mismatch"] else "REPLAY_FAILED"}
    dump(out,result)
    return result

def main():
 ap=argparse.ArgumentParser(); ap.add_argument("--source-root",required=True); ap.add_argument("--d0",action="store_true"); ap.add_argument("--d1",action="store_true"); ap.add_argument("--replay"); ap.add_argument("--emit",required=True); a=ap.parse_args()
 root=Path(a.source_root)
 if a.d0: obj=extraction(root)
 elif a.d1: obj=d1(root,a.emit)
 elif a.replay: obj=replay(root,a.replay,a.emit)
 else: ap.error("select --d0, --d1, or --replay")
 if a.d0: dump(a.emit,obj)
 print(json.dumps({"outcome":obj.get("outcome", "D0_PASSED" if obj.get("passed") else "D0_FAILED"),"emit":a.emit},sort_keys=True))

if __name__ == "__main__": main()
