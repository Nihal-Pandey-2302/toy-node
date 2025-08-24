# toy-node: A Blockchain Protocol in Rust

This repository documents my journey of building a blockchain node from scratch in Rust, following a comprehensive protocol engineering roadmap. This project serves as a hands-on implementation of concepts from distributed systems, cryptography, and peer-to-peer networking.

## Week 1: Data Integrity with Merkle Trees

The first component is a robust Merkle Tree implementation. A Merkle Tree provides a cryptographic fingerprint of all the transactions in a block, allowing for efficient and secure verification of data integrity.

### How to Run

1. **Clone the repository:**

    ```bash
    git clone [your-repo-link]
    cd toy-node
    ```

2. **Run tests:**

    ```bash
    cargo test
    ```

3. **Run benchmarks:**

    ```bash
    cargo bench
    ```

### Benchmark Results

This benchmark measures the time taken to construct a Merkle Tree for 1,000 transactions.

```
// PASTE THE OUTPUT OF `cargo bench` HERE
```
