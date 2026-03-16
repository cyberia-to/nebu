---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: Goldilocks field, Goldilocks prime, F_p, field specification
---

# field specification

the Goldilocks prime field. arithmetic substrate for [[hemera]], [[trident]], [[nox]], and every computational domain in [[cyber]].

## the prime

```
p = 2⁶⁴ − 2³² + 1 = 0xFFFFFFFF00000001 = 18446744069414584321
```

the Goldilocks prime. the name comes from its structure: a 64-bit prime with a 32-bit "hole" that enables fast reduction. the reduction identity:

```
2⁶⁴ ≡ 2³² − 1 (mod p)
```

this identity eliminates division from modular reduction — every reduction is subtracts and adds on 32-bit halves.

## field elements

a Goldilocks field element is an integer in the range [0, p). every element is represented as a canonical u64 in little-endian byte order.

canonical form: a u64 value `v` is canonical if `v < p`. non-canonical values (where `v ≥ p`) are reduced by subtracting p.

## arithmetic

all operations are modular arithmetic over F_p.

### addition

```
a + b = (a + b) mod p
```

implemented using overflowing u64 addition. when the sum overflows 2⁶⁴, the correction is:

```
2⁶⁴ mod p = 2³² − 1
```

so an overflow adds `2³² − 1` to the result. if the corrected result is still ≥ p, subtract p once.

### subtraction

```
a − b = (a − b + p) mod p
```

implemented using overflowing u64 subtraction with the same correction constant.

### multiplication

```
a × b = (a × b) mod p
```

the product is computed as a u128, then reduced using the identity:

```
2⁶⁴ ≡ 2³² − 1 (mod p)
```

the 128-bit result is split into high and low 64-bit halves. the high half is further split at bit 32. reduction replaces the high portion by multiplying with `2³² − 1` and combining — no division, no trial subtraction loop.

### inversion

```
a⁻¹ = a^(p − 2) mod p
```

Fermat's little theorem. computed via a square-and-multiply chain optimized for the Goldilocks prime. zero has no inverse — the caller must handle this.

### negation

```
−a = p − a (when a ≠ 0), 0 (when a = 0)
```

### exponentiation (S-box)

```
x⁷ = x³ · x⁴
```

where `x³ = x² · x` and `x⁴ = x² · x²`. 3 multiplications total. d = 7 is the minimum invertible exponent for this field — see beauty in [[why-hemera]].

## properties

| property | value |
|---|---|
| prime | p = 2⁶⁴ − 2³² + 1 |
| size | 64 bits |
| characteristic | p (prime field) |
| order | p − 1 = 2³² × (2³² − 1) |
| factorization of p − 1 | 2³² × 3 × 5 × 17 × 257 × 65537 |
| two-adicity | 32 (largest k where 2ᵏ divides p−1) |
| primitive root | 7 (smallest generator of the multiplicative group) |

the high two-adicity (32) makes the field efficient for [[NTT]] (Number Theoretic Transform), which is why it is widely used in STARK proof systems.

## see also

- [[why-goldilocks]] — rationale for field choice: native u64, STARK compatibility, universal substrate, the double seven

## references

- [1] Plonky2: "Plonky2: Fast Recursive Arguments with PLONK and FRI." Polygon Zero Team, 2022.
- [2] Goldilocks field analysis in the context of STARK systems: documented in Polygon, Plonky3, and SP1 implementations.
