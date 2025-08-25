# toy-node: A Blockchain Protocol in Rust

This repository documents my journey of building a blockchain node from scratch in Rust, following a comprehensive protocol engineering roadmap. This project serves as a hands-on implementation of concepts from distributed systems, cryptography, and peer-to-peer networking.

## Week 1: Data Integrity with Merkle Trees

The first component is a robust Merkle Tree implementation. A Merkle Tree provides a cryptographic fingerprint of all the transactions in a block, allowing for efficient and secure verification of data integrity.

### How to Run

1. **Clone the repository:**

    ```bash
    git clone [https://github.com/Nihal-Pandey-2302/toy-node]
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
merkle_root_1000        time:   [633.46 µs 635.98 µs 638.17 µs]
                        change: [−8.5603% −7.9958% −7.4185%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  2 (2.00%) high severe

merkle_tree_1000        time:   [771.18 µs 775.72 µs 779.90 µs]
                        change: [−5.0214% −4.5717% −4.1434%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe
```
