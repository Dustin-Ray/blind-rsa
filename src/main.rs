use crypto_bigint::modular::ConstMontyParams;
use crypto_bigint::{const_monty_form, impl_modulus, Uint, U128};
use crypto_bigint::{rand_core::OsRng, NonZero, RandomMod};
use rand::Rng;

pub fn main() {
    // RSA public exponent
    let e: Uint<2> = Uint::from(65537_u64);

    // obtained from crypto_primes::generate_safe_prime(Some(32))
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

    // public key modulus
    let n = &NonZero::new(Uint::from_be_hex(N)).unwrap();

    // prover commits to c = z^e * m mod n for
    // some random non-zero z
    let mut rng = OsRng;
    let z_rand = Uint::random_mod(&mut OsRng, n);
    let z = const_monty_form!(z_rand, Modulus);

    let c = z.pow(&e).mul(&m);

    // Soundness error = 1/2^k
    let k = 20;
    for _ in 0..k {
        // Prover picks a random r_i and keeps it secret
        let r_i_rand = Uint::random_mod(&mut OsRng, n);
        let r_i = const_monty_form!(r_i_rand, Modulus);
        // d = r^e mod n is publicly known
        let d = r_i.pow(&e);

        // Verifier picks a random bit b_i
        let b_i = rng.gen::<bool>();
        // Prover reveals r_i
        let z_i = if !b_i { r_i } else { r_i.mul(&s).mul(&z) };

        // if b = 0, Prover reveals r_i and verifier checks that
        // r_i^e = d
        if !b_i {
            assert_eq!(d, z_i.pow(&e),);
        // b_i = 1. Verifier checks that u^e = d * c mod n
        } else {
            let d_c = d.mul(&c);
            let u_e = z_i.pow(&e);
            assert_eq!(d_c, u_e)
        }
    }
}
