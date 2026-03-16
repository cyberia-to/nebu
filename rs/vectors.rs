//! Test vectors from reference/vectors.md.

#[cfg(test)]
mod tests {
    use crate::field::{Goldilocks, P};
    use crate::extension::Fp2;
    use crate::sqrt;
    use crate::batch;

    fn g(v: u64) -> Goldilocks { Goldilocks::new(v) }

    // ── Canonical reduction ─────────────────────────────────────────

    #[test]
    fn canonical_zero() { assert_eq!(g(0).as_u64(), 0); }

    #[test]
    fn canonical_one() { assert_eq!(g(1).as_u64(), 1); }

    #[test]
    fn canonical_p_minus_one() {
        assert_eq!(g(0xFFFFFFFF00000000).as_u64(), 0xFFFFFFFF00000000);
    }

    #[test]
    fn canonical_p_is_zero() { assert_eq!(g(P).as_u64(), 0); }

    #[test]
    fn canonical_p_plus_one() { assert_eq!(g(P + 1).as_u64(), 1); }

    #[test]
    fn canonical_max_u64() {
        assert_eq!(g(0xFFFFFFFFFFFFFFFF).as_u64(), 0x00000000FFFFFFFE);
    }

    // ── Addition ────────────────────────────────────────────────────

    #[test]
    fn add_zero_zero() { assert_eq!((g(0) + g(0)).as_u64(), 0); }

    #[test]
    fn add_one_two() { assert_eq!((g(1) + g(2)).as_u64(), 3); }

    #[test]
    fn add_p_minus_one_plus_one() {
        assert_eq!((g(0xFFFFFFFF00000000) + g(1)).as_u64(), 0);
    }

    #[test]
    fn add_p_minus_one_twice() {
        assert_eq!((g(0xFFFFFFFF00000000) + g(0xFFFFFFFF00000000)).as_u64(), 0xFFFFFFFEFFFFFFFF);
    }

    #[test]
    fn add_u64_overflow() {
        assert_eq!((g(0x8000000000000000) + g(0x8000000000000000)).as_u64(), 0x00000000FFFFFFFF);
    }

    #[test]
    fn add_epsilon_pair() {
        assert_eq!((g(0x00000000FFFFFFFF) + g(0x00000000FFFFFFFF)).as_u64(), 0x00000001FFFFFFFE);
    }

    // ── Subtraction ─────────────────────────────────────────────────

    #[test]
    fn sub_basic() { assert_eq!((g(5) - g(3)).as_u64(), 2); }

    #[test]
    fn sub_zero_minus_one() {
        assert_eq!((g(0) - g(1)).as_u64(), 0xFFFFFFFF00000000);
    }

    #[test]
    fn sub_zero_minus_zero() { assert_eq!((g(0) - g(0)).as_u64(), 0); }

    #[test]
    fn sub_one_minus_p_minus_one() {
        assert_eq!((g(1) - g(0xFFFFFFFF00000000)).as_u64(), 2);
    }

    #[test]
    fn sub_p_minus_one_minus_itself() {
        assert_eq!((g(0xFFFFFFFF00000000) - g(0xFFFFFFFF00000000)).as_u64(), 0);
    }

    // ── Multiplication ──────────────────────────────────────────────

    #[test]
    fn mul_three_times_seven() { assert_eq!((g(3) * g(7)).as_u64(), 0x15); }

    #[test]
    fn mul_zero_times_any() { assert_eq!((g(0) * g(0x2A)).as_u64(), 0); }

    #[test]
    fn mul_one_times_p_minus_one() {
        assert_eq!((g(1) * g(0xFFFFFFFF00000000)).as_u64(), 0xFFFFFFFF00000000);
    }

    #[test]
    fn mul_p_minus_one_squared() {
        assert_eq!((g(0xFFFFFFFF00000000) * g(0xFFFFFFFF00000000)).as_u64(), 1);
    }

    #[test]
    fn mul_p_minus_one_times_two() {
        assert_eq!((g(0xFFFFFFFF00000000) * g(2)).as_u64(), 0xFFFFFFFEFFFFFFFF);
    }

    #[test]
    fn mul_large_values() {
        assert_eq!((g(0x12345678) * g(0x9ABCDEF0)).as_u64(), 0x0B00EA4E242D2080);
    }

    // ── S-box (x^7) ────────────────────────────────────────────────

    #[test]
    fn pow7_zero() { assert_eq!(g(0).pow7().as_u64(), 0); }

    #[test]
    fn pow7_one() { assert_eq!(g(1).pow7().as_u64(), 1); }

    #[test]
    fn pow7_two() { assert_eq!(g(2).pow7().as_u64(), 0x80); }

    #[test]
    fn pow7_seven() { assert_eq!(g(7).pow7().as_u64(), 0x000C90F7); }

    #[test]
    fn pow7_p_minus_one() {
        assert_eq!(g(0xFFFFFFFF00000000).pow7().as_u64(), 0xFFFFFFFF00000000);
    }

    #[test]
    fn pow7_deadbeef() {
        assert_eq!(g(0xDEADBEEF).pow7().as_u64(), 0xF49CB716AE41CF92);
    }

    #[test]
    fn pow7_large() {
        assert_eq!(g(0x123456789ABCDEF0).pow7().as_u64(), 0xA480968CDE68DB72);
    }

    // ── Negation ────────────────────────────────────────────────────

    #[test]
    fn neg_zero() { assert_eq!((-g(0)).as_u64(), 0); }

    #[test]
    fn neg_one() { assert_eq!((-g(1)).as_u64(), 0xFFFFFFFF00000000); }

    #[test]
    fn neg_p_minus_one() { assert_eq!((-g(0xFFFFFFFF00000000)).as_u64(), 1); }

    #[test]
    fn neg_42() { assert_eq!((-g(0x2A)).as_u64(), 0xFFFFFFFEFFFFFFD7); }

    #[test]
    fn neg_half() { assert_eq!((-g(0x8000000000000000)).as_u64(), 0x7FFFFFFF00000001); }

    // ── Primitive root ──────────────────────────────────────────────

    #[test]
    fn primitive_root_fermat() {
        assert_eq!(g(7).exp(P - 1).as_u64(), 1);
    }

    #[test]
    fn primitive_root_euler() {
        assert_eq!(g(7).exp((P - 1) / 2).as_u64(), 0xFFFFFFFF00000000);
    }

    #[test]
    fn two_is_qr() { assert_eq!(g(2).exp((P - 1) / 2).as_u64(), 1); }

    #[test]
    fn three_is_qr() { assert_eq!(g(3).exp((P - 1) / 2).as_u64(), 1); }

    #[test]
    fn five_is_qr() { assert_eq!(g(5).exp((P - 1) / 2).as_u64(), 1); }

    #[test]
    fn six_is_qr() { assert_eq!(g(6).exp((P - 1) / 2).as_u64(), 1); }

    // ── Inversion ───────────────────────────────────────────────────

    #[test]
    fn inv_one() { assert_eq!(g(1).inv().as_u64(), 1); }

    #[test]
    fn inv_two() { assert_eq!(g(2).inv().as_u64(), 0x7FFFFFFF80000001); }

    #[test]
    fn inv_p_minus_one() {
        assert_eq!(g(0xFFFFFFFF00000000).inv().as_u64(), 0xFFFFFFFF00000000);
    }

    #[test]
    fn inv_roundtrip() {
        let a = g(0x123456789ABCDEF0);
        assert_eq!((a * a.inv()).as_u64(), 1);
    }

    // ── Roots of unity ──────────────────────────────────────────────

    #[test]
    fn root_of_unity_2() {
        let omega2 = g(7).exp((P - 1) / 2);
        assert_eq!(omega2.as_u64(), 0xFFFFFFFF00000000);
        assert_eq!(omega2.square().as_u64(), 1);
    }

    #[test]
    fn half_mod_p() {
        assert_eq!(g(2).inv().as_u64(), 0x7FFFFFFF80000001);
    }

    // ── Square root ─────────────────────────────────────────────────

    #[test]
    fn sqrt_zero() { assert_eq!(sqrt::sqrt(g(0)), Some(g(0))); }

    #[test]
    fn sqrt_one() { assert_eq!(sqrt::sqrt(g(1)), Some(g(1))); }

    #[test]
    fn sqrt_four() { assert_eq!(sqrt::sqrt(g(4)), Some(g(2))); }

    #[test]
    fn sqrt_nine() { assert_eq!(sqrt::sqrt(g(9)), Some(g(3))); }

    #[test]
    fn sqrt_two() {
        let r = sqrt::sqrt(g(2)).unwrap();
        assert_eq!(r.as_u64(), 0x000000FFFEFFFF00);
        assert_eq!(r.square().as_u64(), 2);
    }

    #[test]
    fn sqrt_seven_is_none() {
        assert_eq!(sqrt::sqrt(g(7)), None);
    }

    #[test]
    fn legendre_zero() { assert_eq!(sqrt::legendre(g(0)).as_u64(), 0); }

    #[test]
    fn legendre_qr() { assert_eq!(sqrt::legendre(g(4)).as_u64(), 1); }

    #[test]
    fn legendre_qnr() {
        assert_eq!(sqrt::legendre(g(7)).as_u64(), 0xFFFFFFFF00000000);
    }

    // ── Batch inversion ─────────────────────────────────────────────

    #[test]
    fn batch_inv_3_5_7() {
        let a = [g(3), g(5), g(7)];
        let mut result = [Goldilocks::ZERO; 3];
        batch::batch_inv(&a, &mut result);
        assert_eq!(result[0].as_u64(), 0xAAAAAAAA00000001);
        assert_eq!(result[1].as_u64(), 0xCCCCCCCC00000001);
        assert_eq!(result[2].as_u64(), 0x249249246DB6DB6E);
        // Verify a[i] * result[i] = 1
        for i in 0..3 {
            assert_eq!((a[i] * result[i]).as_u64(), 1);
        }
    }

    // ── Extension field ─────────────────────────────────────────────

    #[test]
    fn fp2_mul_small() {
        let x = Fp2::new(g(2), g(3));
        let y = Fp2::new(g(4), g(5));
        let z = x * y;
        assert_eq!(z.re.as_u64(), 0x71);  // 113
        assert_eq!(z.im.as_u64(), 0x16);  // 22
    }

    #[test]
    fn fp2_mul_large() {
        let x = Fp2::new(g(0x123456789ABCDEF0), g(0xFEDCBA9876543210));
        let y = Fp2::new(g(0xAAAAAAAA), g(0x55555555));
        let z = x * y;
        assert_eq!(z.re.as_u64(), 0x25ED096D7B425EDC);
        assert_eq!(z.im.as_u64(), 0xD7CC6BAE7839A5C3);
    }

    #[test]
    fn fp2_inv() {
        let x = Fp2::new(g(2), g(3));
        let xi = x.inv();
        assert_eq!(xi.re.as_u64(), 0x49C341156822B63D);
        assert_eq!(xi.im.as_u64(), 0x115B1E5F63CBEEA5);
        // Verify x * x^-1 = (1, 0)
        let one = x * xi;
        assert_eq!(one.re.as_u64(), 1);
        assert_eq!(one.im.as_u64(), 0);
    }

    #[test]
    fn fp2_conj_and_norm() {
        let x = Fp2::new(g(1), g(1));
        let c = x.conj();
        assert_eq!(c.re.as_u64(), 1);
        assert_eq!(c.im.as_u64(), P - 1);
        assert_eq!(x.norm().as_u64(), 0xFFFFFFFEFFFFFFFB);
    }
}
