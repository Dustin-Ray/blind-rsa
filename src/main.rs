use crypto_bigint::{rand_core::OsRng, NonZero, RandomMod};
use rand::Rng;

pub fn main() {
    use crypto_bigint::modular::ConstMontyParams;
    use crypto_bigint::{const_monty_form, impl_modulus, Uint, U128};

    // RSA public exponent
    let e: Uint<2> = Uint::from(65537_u64);

    // obtained from crypto_primes::generate_safe_prime(32)
    let p: Uint<1> = Uint::from_be_hex("ECAF5ED6493129D7");
    let q: Uint<1> = Uint::from_be_hex("98E1FD7AE92F68F3");

    // p * q
    const N: &str = "8D5910CC89AFF00D40B1ADB4D0230F15";
    impl_modulus!(Modulus, U128, N);

    // Euler's totient
    let phi: Uint<2> =
        (p.saturating_sub(&Uint::from(1_u64))).widening_mul(&q.saturating_sub(&Uint::from(1_u64)));

    // private decryption and signing key
    let secret_d = e.inv_mod(&phi).expect("Modular inverse does not exist");

    // super secret message
    let message = U128::from(42_u64);
    let m = const_monty_form!(message, Modulus);

    // sign the message
    let s = m.pow(&secret_d);

    // verify the signature, without reducing
    assert_eq!(m, s.pow(&e), "verification failed");

    let mut rng = OsRng;
    // public key modulus
    let n: Uint<2> = p.widening_mul(&q);
    let modulus = &NonZero::new(Uint::from_be_hex(N)).unwrap();

    // prover commits to c = z^e * m mod n for
    // some random non-zero z
    let z = Uint::random_mod(&mut OsRng, modulus);
    let c = const_monty_form!(z, Modulus).pow(&e).mul(&m).retrieve();

    // Soundness error = 1/2^k
    let k = 20;
    for _ in 0..k {
        // Prover picks a random r_i
        let r_i = Uint::random_mod(&mut OsRng, modulus);

        let d = const_monty_form!(r_i, Modulus).pow(&e).retrieve();

        // Verifier picks a random bit b_i
        let b_i = rng.gen::<bool>();
        // Prover reveals r_i
        let z_i = if !b_i {
            r_i
        } else {
            r_i.mul_mod(&s.retrieve(), &n).mul_mod(&z, &n)
        };

        // if b = 0, Prover reveals r_i and verifier checks that
        // r_i^e = d
        if !b_i {
            assert_eq!(
                d,
                const_monty_form!(z_i, Modulus).pow(&e).retrieve(),
            );
        // b_i = 1. Verifier checks that u^e = d * c mod n
        } else {
            let d_c = d.mul_mod(&c, &n);
            let u_e = const_monty_form!(z_i, Modulus).pow(&e).retrieve();
            assert_eq!(d_c, u_e)
        }
    }
}
