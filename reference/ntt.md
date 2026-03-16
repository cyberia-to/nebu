---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: NTT, Number Theoretic Transform, roots of unity
---

# NTT specification

the Number Theoretic Transform over the Goldilocks field. the algebraic engine behind STARK proving, polynomial arithmetic, and [[TFHE]] operations in [[cyber]].

## roots of unity

the multiplicative group F_p* has order p − 1 = 2³² × (2³² − 1). the factor 2³² provides a principal 2³²-th root of unity:

```
ω = g^((p−1) / 2³²)
```

where g is a primitive root of F_p*. this root enables NTT of length up to 2³² — sufficient for any polynomial arithmetic in the STARK prover.

for NTT of length N = 2ᵏ (where k ≤ 32), the N-th root of unity is:

```
ω_N = g^((p−1) / N)
```

the inverse root ω_N⁻¹ exists because gcd(N, p−1) = N (N divides p−1). this guarantees the inverse NTT is well-defined.

## butterfly operation

the NTT butterfly is the core operation:

```
a' = a + ω · b
b' = a − ω · b
```

two field additions and one field multiplication. over Goldilocks, each operation completes in one or two machine cycles — no multi-limb arithmetic, no Montgomery reduction.

the full NTT of length N = 2ᵏ performs N/2 × k butterflies. for N = 2³² (maximum length): 2³¹ × 32 = 2³⁶ butterflies.

## Cooley-Tukey decomposition

the NTT follows the standard radix-2 decimation-in-time algorithm:

```
for s in 0..k:
    m = 2^(s+1)
    ω_m = g^((p−1) / m)
    for j in (0..N).step_by(m):
        w = 1
        for i in 0..m/2:
            t = w · a[j + i + m/2]
            a[j + i + m/2] = a[j + i] − t
            a[j + i]       = a[j + i] + t
            w = w · ω_m
```

input is in bit-reversed order. output is in natural order. the inverse NTT uses ω_m⁻¹ and scales the result by N⁻¹.

## hardware support

the [[GFP]] dedicates the `ntt` primitive to the butterfly operation. a single `ntt` instruction performs:

```
(a, b, ω) → (a + ω·b, a − ω·b)
```

one fused operation: one multiplication and two additions. the twiddle factor ω is a register operand. this eliminates the multiply-then-add pipeline stall that limits software NTT throughput.

## usage in cyber

| system | NTT role | typical length |
|--------|----------|----------------|
| STARK prover | polynomial evaluation/interpolation | 2¹⁸ − 2²⁴ |
| [[WHIR]] | polynomial commitment | 2¹⁶ − 2²⁰ |
| [[TFHE]] | ciphertext multiplication in R_p | 2¹⁰ − 2¹⁴ |
| [[hemera]] bootstrap | round constant generation (indirect) | — |
