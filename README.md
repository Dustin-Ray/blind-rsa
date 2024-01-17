# Zero-Knowledge Proof Protocol for RSA Signature without Revealing Signature or Message

Today on another episode of "solving impossible problems with cryptography":

Suppose agent Bob gives you a sealed envelope. Contained within is a message, and Bob claims the message is signed by President Alice. However, the message _and_ the signature are top-secret and cannot be revealed. Can we verify that a secret message and its signature are valid, without ever opening the envelope?

<p align="center">
  <img src="https://github.com/drcapybara/blind-rsa/assets/40841027/2e069be5-649c-48af-931f-3b1de3246d23" width="350" height="350">
</p>



**Initial Setup:**
- Alice's RSA public key is $(e, n)$.
- The prover (Bob) knows a signature $s$ such that $s^e \equiv m \mod n$.
- The prover picks some random non-zero $z \mod{n}$.
- The prover commits to $c = z^e \cdot m \mod{n}$. This directly implies that $c = (s \cdot z)^e \mod{n}$, and that $s \cdot z \mod{n}$ is a valid signature for $c$.

**The ZK Proof Process (Repeated $k$ Times for soundness error $2^{-k}$ ):**
1. *Prover's Randomization:*
   - The prover picks a random number $r_i \mod{n}$, computes $d = r_i^e \mod{n}$.

2. *Verifier's Challenge:*
   - The Verifier picks a random bit $b_i$ and sends it back to the prover.

3. *Prover's Response:*
   - If $b_i = 0$: The prover reveals $r_i$. The Verifier checks that $r_i^e = d$.
   - If $b_i = 1$: The prover reveals $u = r \cdot s \cdot z \mod{n}$. The verifier checks that $u^e = d \cdot c \mod{n}$.

**Zero-Knowledge:**
The protocol is blinding for both the message and the signature. If $z$ is chosen at random, then $c$ is perfectly blinded as a one-time-pad. The only assumption is that RSA is secure and signatures cannot be forged. The partially-homomorphic property of RSA allows for multiplications on blinded values.

Thank you once again to Dr. Barreto for the riveting exercise.
