---
tags: cyber, cip
crystal-type: entity
crystal-domain: cyber
alias: field encoding, byte encoding, 7-byte encoding
---

# encoding specification

how bytes map to field elements and back. this encoding is shared by every system that moves data across the byte↔field boundary.

## bytes to field elements

input bytes are packed into field elements using 7-byte little-endian chunks.

```
bytes:    [b₀, b₁, b₂, b₃, b₄, b₅, b₆,  b₇, b₈, ...]
           └───────── chunk 0 ──────────┘   └── chunk 1 ...
element:  b₀ + b₁·2⁸ + b₂·2¹⁶ + ... + b₆·2⁴⁸
```

the maximum 7-byte value is 2⁵⁶ − 1 = 72057594037927935. since 2⁵⁶ − 1 < p, every 7-byte chunk maps to a valid field element without conditional reduction. branchless. constant-time.

8 bytes would not work: the maximum 8-byte value 2⁶⁴ − 1 exceeds p. encoding would require a conditional branch to check and reduce — breaking constant-time guarantees and adding complexity.

this is the other half of the double seven (see [[goldilocks]] § the double seven): d = 7 is the minimum invertible S-box exponent, and 7 bytes is the maximum unconditional encoding width. both forced by the same prime.

## padding

the last chunk may have fewer than 7 bytes. it is zero-padded to 7 bytes. the total input length is recorded in the sponge capacity (state[10]) during finalization, which disambiguates inputs that differ only in trailing zeros.

## field elements to bytes

output field elements are serialized as 8-byte little-endian canonical u64 values.

```
element:  v (where 0 ≤ v < p)
bytes:    [v & 0xFF, (v >> 8) & 0xFF, ..., (v >> 56) & 0xFF]
```

8 bytes per element, not 7. the output encoding preserves the full field element without loss. the asymmetry (7 bytes in, 8 bytes out) is deliberate: input encoding maximizes absorption throughput, output encoding preserves the canonical representation.

## rate and throughput

with rate r = 8 field elements per sponge block, the input throughput per block is:

```
8 elements × 7 bytes/element = 56 bytes/block
```

one permutation absorbs 56 bytes of input data. for a 4096-byte chunk: ⌈4096/56⌉ = 74 permutations.
