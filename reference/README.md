---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: aurum reference, aurum specification
---

# aurum specification

canonical reference for the Goldilocks prime field, its arithmetic, and its hardware.

## spec pages

| page | defines |
|------|---------|
| [[field]] | prime, elements, arithmetic, properties, why Goldilocks |
| [[ntt]] | Number Theoretic Transform, roots of unity, butterfly, Cooley-Tukey |
| [[encoding]] | 7-byte input encoding, 8-byte output encoding, padding, throughput |
| [[vectors]] | known-answer test vectors for all field operations |
| [[hardware]] | GFP primitives: fma, ntt, p2r, lut (proposal) |

## see also

- [[hemera]] — hash function over this field
- [[trident]] — language compiled to circuits over this field
- [[nox]] — VM executing over this field
