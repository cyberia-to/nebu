---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: why Goldilocks, field rationale, field choice
---

# why Goldilocks

three properties make this the right field for [[cyber]].

## native u64 arithmetic

every field element fits in a single machine register. addition is one add + one conditional subtract. multiplication is one u128 multiply + fast reduction. no multi-limb arithmetic, no Montgomery form.

the reduction identity `2⁶⁴ ≡ 2³² − 1 (mod p)` means modular reduction is subtracts and adds on 32-bit halves — no division, no trial loops. this is not an optimization applied to an arbitrary prime. it is a structural property of the prime itself.

compare with BN254 (254-bit prime, 4-limb arithmetic) or BLS12-381 (381-bit prime, 6-limb arithmetic). every field operation on those primes requires multi-precision arithmetic. Goldilocks completes the same operation in one machine instruction.

## STARK compatibility

the two-adicity of 32 enables efficient FFT/NTT for polynomial arithmetic. the multiplicative group has order p − 1 = 2³² × (2³² − 1), providing 2³²-th roots of unity — sufficient for any polynomial degree in the STARK prover.

the same field used for hashing is the field used for proving. no field conversion at the hash-to-proof boundary. a [[hemera]] hash output is 8 Goldilocks field elements — these elements enter the STARK prover directly. no decomposition, no re-encoding, no constraint overhead. ~1,200 constraints per hash vs ~15,000 for BLAKE3.

## universal substrate

the Goldilocks field is the arithmetic home for every computational domain in [[cyber]]:

```
         ┌──────────┬──────────┬──────────┬──────────┬──────────┐
         │  Hashing │  Proving │    FHE   │  Neural  │ Quantum  │
         │ (Hemera) │  (STARK) │   (LWE)  │(inference)│(circuits)│
         └────┬─────┴────┬─────┴────┬─────┴────┬─────┴────┬─────┘
              │          │          │          │          │
              └──────────┴──────────┴──────────┴──────────┘
                        Goldilocks field (p = 2⁶⁴ − 2³² + 1)
```

| domain | algebraic home | how F_p helps |
|--------|---------------|---------------|
| ZK proofs | arithmetic circuits over F_p | programs are circuits by construction |
| AI | matrix operations over F_p | weights and activations are field elements, no quantization |
| FHE | polynomial ring R_p = F_p[X]/(X^N+1) | when ciphertext modulus q = p, proof impedance vanishes |
| quantum | unitary matrices over F_{p²} | prime dimension eliminates gate decomposition overhead |

one field. no conversion at any boundary. [[trident]] demonstrates this with [[Trinity]]: five computational domains executing inside one STARK trace, all over the Goldilocks field.

LWE ciphertexts are Goldilocks vectors. neural weights are Goldilocks elements. [[hemera]] round constants are Goldilocks elements. [[WHIR]] commitments are hemera hashes. the field is the universal substrate.

## the double seven

the number 7 appears twice in [[hemera]], for two independent reasons — both forced by this prime.

the S-box must be a bijection over F_p, requiring gcd(d, p−1) = 1. for Goldilocks: p−1 = 2³² × (2³² − 1), which has factors 2, 3, 5. d=3 fails (gcd=3). d=5 fails (gcd=5). d=7 is the minimum invertible exponent.

the input encoding must map bytes to field elements without conditional reduction. the maximum 7-byte value is 2⁵⁶ − 1 < p. the maximum 8-byte value can exceed p. 7 bytes is the maximum whole-byte count that fits [0, p) unconditionally.

the same prime constrains both the nonlinear layer and the encoding layer to the same number. this is a consequence of the field, not a design choice.
