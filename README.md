# toy-node

A simplified blockchain node built from scratch in Rust as part of an intensive 6-month protocol engineering roadmap. This project serves as a live, evolving portfolio piece demonstrating the practical application of distributed systems and cryptography concepts.

## Guiding Philosophy: The Build-while-Learning Flywheel

The development of `toy-node` follows a strict principle: theoretical concepts from foundational texts like *Designing Data-Intensive Applications* and *Serious Cryptography* are immediately synthesized into hands-on code. The goal is not just to build a final product, but to document the journey of learning and the architectural decisions made along the way.

---

## Current Status: Milestone 1 of 12 (Week 1)

**Version: v0.1 (In Progress)**

The focus of this initial version is to establish the most fundamental data structures of a blockchain and ensure their integrity.

### Implemented Features

* **Core Data Structures:** Definition of `Block`, `BlockHeader`, and `Transaction` structs to serve as the backbone of the chain.
* **Data Serialization:** Use of `serde` and `serde_json` to allow data structures to be serialized, preparing them for network transport and on-disk storage.
* **Merkle Tree:** A from-scratch implementation of a Merkle Tree to cryptographically aggregate transactions into a single, verifiable `merkle_root`. This is the cornerstone of block integrity.
* **Modular Project Structure:** The codebase is organized into logical modules (`blockchain`, `merkle`) for clarity and maintainability.

### Work in Progress

* [ ] Implementing a `Hashable` trait for canonical hashing of blocks and transactions.
* [ ] Comprehensive unit tests for all cryptographic and data functions.

---

## How to Interact with the Project

1. **Prerequisites:** Ensure you have the Rust toolchain installed (`rustup`, `cargo`).
2. **Clone:** `git clone https://github.com/<your_username>/toy-node.git`
3. **Navigate:** `cd toy-node`
4. **Run Tests:** `cargo test`
    * This is the best way to verify the core logic, such as the Merkle Tree calculation.
5. **Run Main Executable:** `cargo run`
    * Currently, this initializes a sample block with transactions and prints its JSON representation to the console, demonstrating the serialization and data structures in action.

---

## Key Concepts Log

This section tracks the core concepts from my learning plan that have been implemented in the code.

### Week 1: Reliable Data Structures

* **Systems Design (DDIA Ch. 3):**
  * **Concept:** Hash Indexes.
  * **Application:** Understanding the fundamental key-value nature of data storage, which informs the design of our block and transaction structures.
* **Cryptography (SC Ch. 6, 15):**
  * **Concept:** Cryptographic Hashes (SHA-256) and their properties (pre-image resistance, collision resistance).
  * **Application:** Used as the fundamental building block for the Merkle Tree.
  * **Concept:** Merkle Trees.
  * **Application:** Implemented in `src/merkle.rs` to create a compact proof of all transactions within a block, stored in the `BlockHeader`.
* **Rust Language:**
  * **Concepts:** Structs, `impl` blocks, Modules, Ownership, Lifetimes, Crates (`sha2`, `serde`).
  * **Application:** Used throughout the project to build safe, well-structured, and maintainable code.

---
