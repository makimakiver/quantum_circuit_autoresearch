#!/usr/bin/env python3
"""Exact two-AND obstruction for 2-bit no-carry controlled addition.

The model is an XOR-AND graph (XAG): each AND has affine inputs over the primary
inputs and preceding AND outputs; final outputs are affine.  This is a superset
of CCX/CNOT/X circuits with clean work bits when only the number of nonlinear
computations is charged.  It deliberately does not model HMR cleanup because
HMR cannot create a missing Boolean nonlinear value.

For the map (c,a,b) -> (c,a,b+c*a mod 4), this script proves that two ANDs do
not suffice.  The result is a bounded obstruction, not a general controlled-
adder lower bound.
"""

from __future__ import annotations

import json
from itertools import combinations_with_replacement

INPUT_NAMES = ("c", "a0", "a1", "b0", "b1")


def variable_truth_table(bit: int) -> int:
    return sum(((assignment >> bit) & 1) << assignment for assignment in range(1 << len(INPUT_NAMES)))


def reduced_basis(vectors: list[int]) -> dict[int, int]:
    """Return a GF(2) pivot map keyed by highest set bit."""
    pivots: dict[int, int] = {}
    for value in vectors:
        while value:
            pivot = value.bit_length() - 1
            old = pivots.get(pivot)
            if old is None:
                pivots[pivot] = value
                break
            value ^= old
    return pivots


def in_span(value: int, pivots: dict[int, int]) -> bool:
    while value:
        old = pivots.get(value.bit_length() - 1)
        if old is None:
            return False
        value ^= old
    return True


def affine_functions(basis: list[int]) -> list[int]:
    functions: list[int] = []
    for coefficients in range(1 << len(basis)):
        value = 0
        for i, signal in enumerate(basis):
            if (coefficients >> i) & 1:
                value ^= signal
        functions.append(value)
    return functions


def unique_affine_products(basis: list[int]) -> dict[int, tuple[int, int]]:
    affine = affine_functions(basis)
    products: dict[int, tuple[int, int]] = {}
    for left, right in combinations_with_replacement(affine, 2):
        products.setdefault(left & right, (left, right))
    return products


def certificate() -> dict[str, int | bool]:
    c, a0, a1, b0, _b1 = (variable_truth_table(i) for i in range(5))
    constant = (1 << (1 << len(INPUT_NAMES))) - 1
    affine_basis = [constant, c, a0, a1, b0, _b1]
    affine_span = reduced_basis(affine_basis)

    # Non-affine parts of b0' and b1' after the linear b terms are removed.
    f0 = c & a0
    f1 = (c & a1) ^ (c & a0 & b0)
    assert not in_span(f0, affine_span)
    assert not in_span(f1, affine_span)
    assert not in_span(f0 ^ f1, affine_span)

    first_products = unique_affine_products(affine_basis)
    # With only two ANDs, their two quotient vectors must span exactly
    # span{f0,f1} modulo affine functions.  The first AND has degree at most
    # two, so it cannot represent f1 or f0^f1 (both have a cubic term); it must
    # represent f0 modulo affine terms.  This explicit enumeration checks that
    # reduction rather than relying only on the degree observation.
    compatible_first = [
        product
        for product in first_products
        if in_span(product ^ f0, affine_span)
    ]
    assert all(
        not in_span(product ^ f1, affine_span)
        and not in_span(product ^ f0 ^ f1, affine_span)
        for product in first_products
    )

    second_candidates = 0
    witnesses = 0
    for first in compatible_first:
        second_products = unique_affine_products([*affine_basis, first])
        for second in second_products:
            second_candidates += 1
            # The second quotient must supply f1, possibly shifted by f0.
            if in_span(second ^ f1, affine_span) or in_span(second ^ f1 ^ f0, affine_span):
                witnesses += 1

    return {
        "inputs": len(INPUT_NAMES),
        "first_and_functions": len(first_products),
        "compatible_first_and_functions": len(compatible_first),
        "second_and_functions_examined": second_candidates,
        "two_and_witnesses": witnesses,
        "two_and_unsat": witnesses == 0,
    }


def main() -> None:
    report = certificate()
    assert report["two_and_unsat"]
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
