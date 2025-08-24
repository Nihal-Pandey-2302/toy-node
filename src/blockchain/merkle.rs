use sha2::{Sha256, Digest};
use serde_json;
use super::transaction::Transaction;

pub fn calculate_merkle_root(transactions: &[Transaction]) -> String {
    if transactions.is_empty() {
        return String::from("0"); // empty tree
    }

    // Step 1: Hash each transaction into Vec<String>
    let mut hashes: Vec<String> = transactions.iter().map(|tx| {
        let serialized = serde_json::to_string(tx).unwrap();
        let hash = Sha256::digest(serialized.as_bytes());
        hex::encode(hash)
    }).collect();

    // Step 2: Pair up and hash until one remains
    while hashes.len() > 1 {
        let mut new_hashes = Vec::new();

        for i in (0..hashes.len()).step_by(2) {
            if i + 1 < hashes.len() {
                // Pair exists
                let combined = format!("{}{}", hashes[i], hashes[i+1]);
                let hash = Sha256::digest(combined.as_bytes());
                new_hashes.push(hex::encode(hash));
            } else {
                // Odd one → duplicate last
                let combined = format!("{}{}", hashes[i], hashes[i]);
                let hash = Sha256::digest(combined.as_bytes());
                new_hashes.push(hex::encode(hash));
            }
        }

        hashes = new_hashes;
    }

    hashes[0].clone() // Merkle root
}

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct MerkleTree {
    pub root: String,
    pub levels: Vec<Vec<String>>, // each level is a list of hashes
}

fn hash_transaction(tx: &Transaction) -> String {
    let serialized = serde_json::to_string(tx).unwrap();
    let hash = Sha256::digest(serialized.as_bytes());
    hex::encode(hash)
}

fn hash(data: &str) -> String {
    let hash = Sha256::digest(data.as_bytes());
    hex::encode(hash)
}

pub fn build_merkle_tree(transactions: &[Transaction]) -> MerkleTree {
    if transactions.is_empty() {
        return MerkleTree {
            root: String::from("0"),
            levels: vec![],
        };
    }
    // convert transactions into leaf hashes
    let mut current_level: Vec<String> = transactions
        .iter()
        .map(|tx| hash_transaction(tx)) // your hashing function for a Transaction
        .collect();

    let mut levels = Vec::new();
    levels.push(current_level.clone());

    // build upwards until we get the root
    while current_level.len() > 1 {
        let mut next_level = Vec::new();
        for i in (0..current_level.len()).step_by(2) {
            if i + 1 < current_level.len() {
                let combined = format!("{}{}", current_level[i], current_level[i + 1]);
                next_level.push(hash(&combined)); // your hash function for string
            } else {
                // duplicate last hash if odd number of elements
                let combined = format!("{}{}", current_level[i], current_level[i]);
                next_level.push(hash(&combined));
            }
        }
        levels.push(next_level.clone());
        current_level = next_level;
    }

    let root = current_level[0].clone();

    MerkleTree { root, levels }
}

pub fn merkle_tree(transactions: &[Transaction]) -> String {
    let tree = build_merkle_tree(transactions);
    serde_json::to_string_pretty(&tree).unwrap()
}
