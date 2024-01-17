# Today on another episode of _impossible problems solved with cryptography_:

Suppose agent Bob has a sealed envelope. Contained within is a message, and Bob claims the message is signed by President Alice. However, the message _and_ the signature are top-secret and cannot be revealed. Can we verify that a secret message and its signature are valid, without ever opening the envelope?

<p align="center">
  <img src="https://github.com/drcapybara/blind-rsa/assets/40841027/e47caab3-2541-4d38-b422-3e079fe23be7" width="400" height="400">
</p>

**Initial Setup:**
- Alice's RSA public key is $(e, n)$.
- The prover (Bob) knows a signature $s$ such that $s^e \equiv m \mod n$.
- The prover picks some random non-zero $z \mod{n}$.
- The prover commits to $c = z^e \cdot m \mod{n}$. This directly implies that $c = (s \cdot z)^e \mod{n}$, and that $s \cdot z \mod{n}$ is a valid signature for $c$.

**The ZK Proof Process (Repeated $k$ Times for soundness error $2^{-k}$ ):**
1. *Prover's Randomization:*
   - The prover picks a random number $r_i \mod{n}$, and computes $d = r_i^e \mod{n}$. $r_i$ is secret, and $d$ is publicly known.

2. *Verifier's Challenge:*
   - The Verifier picks a random bit $b_i$ and sends it back to the prover.

3. *Prover's Response:*
   - If $b_i = 0$: The prover reveals $r_i$. The Verifier checks that $r_i^e = d$.
   - If $b_i = 1$: The prover reveals $u = r \cdot s \cdot z \mod{n}$. The verifier checks that $u^e = d \cdot c \mod{n}$.

**Zero-Knowledge:**
The protocol is blinding for both the message and the signature. If $z$ is chosen at random, then $c$ is perfectly blinded as a one-time-pad. The only assumption is that RSA is secure and signatures cannot be forged. The partially-homomorphic property of RSA allows for multiplications on blinded values.

**Fixed-time:**
Although entirely insecure, this toy version of RSA is powered by [crypto-bigint ](https://github.com/RustCrypto/crypto-bigint). All operations are thus performed in fixed-time. Additionally, note that values are represented in Montgomery form, reducing the need for expensive modular reductions.

**Thanks:**
Thank you once again to Dr. Barreto for the riveting exercise.
