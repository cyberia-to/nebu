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

the Goldilocks prime. the name comes from its structure: a 64-bit prime with a 32-bit "hole" that enables fast reduction.

## reduction identity

```
2⁶⁴ ≡ 2³² − 1 (mod p)
```

define the correction constant:

```
ε = 2³² − 1 = 0xFFFFFFFF
```

this is also `p.wrapping_neg()` in two's complement u64. every reduction uses ε — no division, no trial subtraction loops.

## field elements

a Goldilocks field element is an integer in the range [0, p). every element is represented as a canonical u64 in little-endian byte order.

canonical form: a u64 value `v` is canonical if `v < p`. non-canonical values (where `v ≥ p`) are reduced by subtracting p once.

equality: two elements are equal if and only if their canonical forms are identical.

## arithmetic

all operations are modular arithmetic over F_p. all algorithms are constant-time (no secret-dependent branches).

### addition

```
add(a, b):
  (sum, carry₁) = a + b                    // overflowing u64 add
  (sum, carry₂) = sum + carry₁ · ε         // correction for overflow
  if carry₂: sum = sum + ε                  // double overflow (rare)
  return sum
```

when the u64 addition overflows, the discarded 2⁶⁴ is replaced by ε = 2³² − 1. a second overflow can occur from the correction, handled identically.

the result may be non-canonical (in [p, 2⁶⁴)). canonicalization subtracts p if needed, but is deferred — intermediate results tolerate non-canonical form.

### subtraction

```
sub(a, b):
  (diff, borrow₁) = a − b                  // overflowing u64 sub
  (diff, borrow₂) = diff − borrow₁ · ε     // correction for underflow
  if borrow₂: diff = diff − ε               // double underflow (rare)
  return diff
```

underflow borrows 2⁶⁴, which equals p + ε. subtracting ε from the result corrects for the borrowed 2⁶⁴ modulo p. this is the mirror of addition — underflow subtracts ε where overflow adds ε.

### multiplication

```
mul(a, b):
  x = a × b                                // u128 full product
  x_lo = x[0:64]                           // low 64 bits
  x_hi = x[64:128]                         // high 64 bits
  x_hi_hi = x_hi >> 32                     // high 32 bits of x_hi
  x_hi_lo = x_hi & ε                       // low 32 bits of x_hi

  (t₀, borrow) = x_lo − x_hi_hi           // 2⁶⁴ → subtract x_hi_hi
  if borrow: t₀ = t₀ − ε                   // borrow correction

  t₁ = x_hi_lo × ε                         // 2³² → multiply by ε

  (result, carry) = t₀ + t₁                // combine
  return result + carry · ε                 // carry correction
```

the 128-bit product splits into high and low halves. the high 64 bits represent multiples of 2⁶⁴. the reduction identity replaces 2⁶⁴ with ε:

- `x_hi_hi` (bits 96–127): contributes `x_hi_hi · 2⁹⁶ ≡ x_hi_hi · ε² ≡ x_hi_hi · (2⁶⁴ − 2³² + 1) ≡ −x_hi_hi` (mod p), hence the subtraction
- `x_hi_lo` (bits 64–95): contributes `x_hi_lo · 2⁶⁴ ≡ x_hi_lo · ε`, hence the multiplication

no division. no trial subtraction loop. three 64-bit operations after the u128 multiply.

### inversion

```
a⁻¹ = a^(p − 2) mod p
```

Fermat's little theorem. zero has no inverse — the caller must handle this.

p − 2 = 0xFFFFFFFF_FFFFFFFF in binary has 63 set bits. a naive square-and-multiply chain requires 63 squarings and 62 multiplications. an optimized addition chain reduces this — the optimal chain for Goldilocks p−2 uses the structure of p:

```
p − 2 = 2⁶⁴ − 2³² − 1 = (2³² − 1) · 2³² − 1 = ε · 2³² − 1
```

which suggests computing via `a^ε`, then squaring 32 times, then multiplying by `a⁻¹`. since `a⁻¹` is what we are computing, this is expressed as:

```
inv(a):
  t  = a                    // a
  t  = t² · t               // a³
  t  = t² · t               // keep doubling via square-and-multiply for a^ε
  ... (build a^(2³²−1) using addition chain for 2³²−1)
  t  = t^(2³²)              // 32 squarings: a^((2³²−1)·2³²) = a^(2⁶⁴−2³²)
  t  = t · a^(p−2−(2⁶⁴−2³²))  // final correction
  return t
```

the exact addition chain is implementation-defined. the reference implementation uses repeated squaring on the binary expansion of p−2.

### negation

```
neg(a):
  if a = 0: return 0
  return p − a
```

### exponentiation (S-box)

```
pow7(x):
  x² = x · x
  x³ = x² · x
  x⁴ = x² · x²
  x⁷ = x³ · x⁴
  return x⁷
```

3 multiplications. d = 7 is the minimum invertible exponent for this field: gcd(d, p−1) = 1 requires d coprime to p−1 = 2³² × 3 × 5 × 17 × 257 × 65537. d=2 fails (even). d=3 fails (divides p−1). d=5 fails (divides p−1). d=7 succeeds.

## primitive root

g = 7 is the smallest generator of the multiplicative group F_p*.

verification: a generator must satisfy g^((p−1)/q) ≠ 1 for every prime factor q of p−1. the prime factors of p−1 are {2, 3, 5, 17, 257, 65537}.

```
7^(p−1)     = 1                             // Fermat's little theorem
7^((p−1)/2) = p − 1 = 0xFFFFFFFF00000000   // not 1 → passes
```

smaller candidates fail: 2^((p−1)/2) = 1, 3^((p−1)/2) = 1, 5^((p−1)/2) = 1, 6^((p−1)/2) = 1. these are quadratic residues, not generators.

g = 7 is used to derive all roots of unity for [[NTT]].

## properties

| property | value |
|---|---|
| prime | p = 2⁶⁴ − 2³² + 1 |
| size | 64 bits |
| characteristic | p (prime field) |
| order | p − 1 = 2³² × (2³² − 1) |
| factorization of p − 1 | 2³² × 3 × 5 × 17 × 257 × 65537 |
| two-adicity | 32 (largest k where 2ᵏ divides p−1) |
| primitive root | 7 (smallest generator of F_p*) |
| correction constant | ε = 2³² − 1 = 0xFFFFFFFF |

the high two-adicity (32) makes the field efficient for [[NTT]] (Number Theoretic Transform), which is why it is widely used in STARK proof systems.

## see also

- [[why-goldilocks]] — rationale for field choice: native u64, STARK compatibility, universal substrate, the double seven
- [[vectors]] — known-answer test vectors for all operations

## references

- [1] Plonky2: "Plonky2: Fast Recursive Arguments with PLONK and FRI." Polygon Zero Team, 2022.
- [2] Goldilocks field analysis in the context of STARK systems: documented in Polygon, Plonky3, and SP1 implementations.
