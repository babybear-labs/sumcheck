# Sumcheck

This project implements the Sumcheck Protocol, a fundamental zero-knowledge proof protocol used in various cryptographic applications. The implementation is written in Rust and uses the Arkworks library for finite field operations.

## Overview

The Sumcheck Protocol allows a prover to convince a verifier that the sum of a multivariate polynomial over a hypercube is equal to a claimed value, without revealing the actual polynomial evaluation. This implementation includes:

- Multivariate and univariate polynomial representations
- Prover implementation for generating proof messages
- Verifier implementation for checking proof validity

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)

## Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd sumcheck
cargo test
```

2. Build the project:
```bash
cargo build
```

## Usage

The library provides three main components:

1. `MultiVariatePolynomial`: For representing and manipulating multivariate polynomials
2. `Prover`: For generating proof messages in the sumcheck protocol
3. `Verifier`: For verifying the proof messages


## Project Structure

## Dependencies

- `ark-ff`: For finite field operations

## Contributing

See [issues](https://github.com/babybear-labs/sumcheck/issues).