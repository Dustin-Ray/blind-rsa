# Zero-Knowledge Proof Protocol for RSA Signature without Revealing Signature or Message

**Initial Setup:**
- Alice's RSA public key is $(e, n)$.
- The prover knows a signature $s$ such that $s^e \equiv m \mod n$.
- The prover picks some random non-zero $z \mod{n}$.
- The prover commits to $c = z^e \cdot m \mod{n}$. This directly implies that $c = (s \cdot z)^e \mod{n}$, and that $s \cdot z \mod{n}$ is a valid signature for $c$.

**The ZK Proof Process (Repeated $k$ Times for soundness error $2^{-k}$):**
1. *Prover's Randomization:*
   - The prover picks a random number $r_i \mod{n}$, computes $d = r_i^e \mod{n}$.

2. *Verifier's Response:*
   - The Verifier picks a random bit $b_i$ and sends it back to the prover.

3. *Prover's Response:*
   - If $b_i = 0$: The prover reveals $r_i$. The Verifier checks that $r_i^e = d$.
   - If $b_i = 1$: The prover reveals $u = r \cdot s \cdot z \mod{n}$. The verifier checks that $u^e = d \cdot c \mod{n}$.
