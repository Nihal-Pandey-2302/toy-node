mod blockchain;

use blockchain::{Transaction, Block, BlockHeader, calculate_merkle_root, merkle_tree};
use chrono::Utc;

fn main() {
    // Sample transactions
    let tx1 = Transaction::new("Alice".into(), "Bob".into(), 10, "sig1".into());
    let tx2 = Transaction::new("Bob".into(), "Charlie".into(), 5, "sig2".into());
    let tx3 = Transaction::new("Charlie".into(), "Dave".into(), 20, "sig3".into());

    let transactions = vec![tx1, tx2, tx3];

    // Compute Merkle root
    let merkle_root = calculate_merkle_root(&transactions);
    println!("Merkle Root: {}", merkle_root);

    // Build block
    let header = BlockHeader {
        previous_hash: "000000000".into(),
        merkle_root,
        timestamp: Utc::now().timestamp(),
        nonce: 0,
    };

    let block = Block {
        header,
        transactions,
    };
    println!("{:#?}", block);
    let merkle_tree = merkle_tree(&block.transactions);
    println!("Merkle Tree: {}", merkle_tree);
    println!("Block created with {} transactions", block.transactions.len());
}
