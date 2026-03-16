---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: test vectors, known-answer tests, field vectors
---

# test vectors

known-answer tests for Goldilocks field arithmetic. generated from the [[hemera]] reference implementation. any conforming implementation must produce identical results.

```
p = 0xFFFFFFFF00000001
ε = 0x00000000FFFFFFFF
```

## canonical reduction

| input | canonical |
|---|---|
| `0x0000000000000000` | `0x0000000000000000` |
| `0x0000000000000001` | `0x0000000000000001` |
| `0xFFFFFFFF00000000` | `0xFFFFFFFF00000000` |
| `0xFFFFFFFF00000001` | `0x0000000000000000` |
| `0xFFFFFFFF00000002` | `0x0000000000000001` |
| `0xFFFFFFFFFFFFFFFF` | `0x00000000FFFFFFFE` |

p itself reduces to 0. values in [p, 2⁶⁴) subtract p once.

## addition

| a | b | a + b mod p |
|---|---|---|
| `0x0000000000000000` | `0x0000000000000000` | `0x0000000000000000` |
| `0x0000000000000001` | `0x0000000000000002` | `0x0000000000000003` |
| `0xFFFFFFFF00000000` | `0x0000000000000001` | `0x0000000000000000` |
| `0xFFFFFFFF00000000` | `0xFFFFFFFF00000000` | `0xFFFFFFFEFFFFFFFF` |
| `0x8000000000000000` | `0x8000000000000000` | `0x00000000FFFFFFFF` |
| `0x00000000FFFFFFFF` | `0x00000000FFFFFFFF` | `0x00000001FFFFFFFE` |

row 3: (p−1) + 1 = 0. row 4: (p−1) + (p−1) = p − 2. row 5: triggers u64 overflow, correction adds ε.

## subtraction

| a | b | a − b mod p |
|---|---|---|
| `0x0000000000000005` | `0x0000000000000003` | `0x0000000000000002` |
| `0x0000000000000000` | `0x0000000000000001` | `0xFFFFFFFF00000000` |
| `0x0000000000000000` | `0x0000000000000000` | `0x0000000000000000` |
| `0x0000000000000001` | `0xFFFFFFFF00000000` | `0x0000000000000002` |
| `0xFFFFFFFF00000000` | `0xFFFFFFFF00000000` | `0x0000000000000000` |

row 2: 0 − 1 = p − 1. row 4: 1 − (p−1) = 2.

## multiplication

| a | b | a × b mod p |
|---|---|---|
| `0x0000000000000003` | `0x0000000000000007` | `0x0000000000000015` |
| `0x0000000000000000` | `0x000000000000002A` | `0x0000000000000000` |
| `0x0000000000000001` | `0xFFFFFFFF00000000` | `0xFFFFFFFF00000000` |
| `0xFFFFFFFF00000000` | `0xFFFFFFFF00000000` | `0x0000000000000001` |
| `0xFFFFFFFF00000000` | `0x0000000000000002` | `0xFFFFFFFEFFFFFFFF` |
| `0x0000000012345678` | `0x000000009ABCDEF0` | `0x0B00EA4E242D2080` |

row 4: (p−1)² = 1. this is the key identity — the negative unit squares to the unit.

## S-box (x⁷)

| x | x⁷ mod p |
|---|---|
| `0x0000000000000000` | `0x0000000000000000` |
| `0x0000000000000001` | `0x0000000000000001` |
| `0x0000000000000002` | `0x0000000000000080` |
| `0x0000000000000007` | `0x00000000000C90F7` |
| `0xFFFFFFFF00000000` | `0xFFFFFFFF00000000` |
| `0x00000000DEADBEEF` | `0xF49CB716AE41CF92` |
| `0x123456789ABCDEF0` | `0xA480968CDE68DB72` |

row 3: 2⁷ = 128. row 5: (p−1)⁷ = p−1 (odd power of −1 is −1).

## negation

| x | −x mod p |
|---|---|
| `0x0000000000000000` | `0x0000000000000000` |
| `0x0000000000000001` | `0xFFFFFFFF00000000` |
| `0xFFFFFFFF00000000` | `0x0000000000000001` |
| `0x000000000000002A` | `0xFFFFFFFEFFFFFFD7` |
| `0x8000000000000000` | `0x7FFFFFFF00000001` |

neg is self-inverse: −(−x) = x. row 2 and row 3 demonstrate this.

## primitive root

| expression | value |
|---|---|
| 7^(p−1) mod p | `0x0000000000000001` |
| 7^((p−1)/2) mod p | `0xFFFFFFFF00000000` |
| 2^((p−1)/2) mod p | `0x0000000000000001` |
| 3^((p−1)/2) mod p | `0x0000000000000001` |
| 5^((p−1)/2) mod p | `0x0000000000000001` |
| 6^((p−1)/2) mod p | `0x0000000000000001` |

7^((p−1)/2) = p−1 ≠ 1, confirming 7 is a quadratic non-residue and thus a generator. candidates 2, 3, 5, 6 all yield 1, confirming they are quadratic residues (not generators).
