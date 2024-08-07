# Post-Quantum Encryption Experiments

This repository contains a collection of experiments with post-quantum encryption algorithms. The goal is to understand the performance of these algorithms and how they can be used in practice. The experiments are mainly going to be implemented in Rust to achieve good performance.

All algorithms are implemented in different branches. However, this will change to run under one central crate in the future.

## Algorithms

### Diffie-Hellman Key Exchange for Lattice Encryption over the Heisenberg Group

*branch name: heisenberg-encryption*

This encryption was an algorithm that is meant to be broken. Since Diffie-Hellman key exchange relies on the discrete logarithm problem, it is further reducible to the Hidden Subgroup Problem (HSP) in the Heisenberg Group. This encryption is meant to be broken by quantum computers since the Heisenberg group is one of the few non-abelian groups with a known HSP efficient algorithm. The experiment was a practice and demonstration of quantum algorithms. 
