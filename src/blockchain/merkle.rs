use super::transaction::Transaction;
use sha2::{Digest, Sha256};
pub struct MerkleTree {
    pub root: Vec<u8>, // Hex or raw bytes
}

impl MerkleTree {
    pub fn new(transactions: &[Transaction]) -> Self {
        let mut current_layer: Vec<Vec<u8>> = Vec::new();
        for tx in transactions {
            current_layer.push(Self::hash_transaction(tx));
        }

        // 2. Build the tree
        while current_layer.len() > 1 {
            let mut next_layer = Vec::new();

            // If odd number of nodes, duplicate the last
            if current_layer.len() % 2 != 0 {
                current_layer.push(current_layer.last().unwrap().clone());
            }

            // Pair up hashes
            for pair in current_layer.chunks(2) {
                let parent_hash = Self::hash_pair(&pair[0], &pair[1]);
                next_layer.push(parent_hash);
            }

            current_layer = next_layer;
        }

        // 3. The final hash is the Merkle root
        let merkle_root = current_layer[0].clone();

        // 4. Return the MerkleTree
        MerkleTree { root: merkle_root }
    }

    fn hash_transaction(tx: &Transaction) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(tx.0.as_bytes());
        hasher.finalize().to_vec()
    }

    fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(left);
        hasher.update(right);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blockchain::transaction::Transaction; // optional, helps with readable hashes

    #[test]
    fn test_merkle_root_4_leaves() {
        // 1. Define your sample transactions
        let transactions = vec![
            Transaction("a".into()),
            Transaction("b".into()),
            Transaction("c".into()),
            Transaction("d".into()),
        ];

        // 2. Build the tree
        let tree = MerkleTree::new(&transactions);

        // 3. Print or manually verify root (we'll skip assert_eq for now)
        println!("Merkle root: {}", hex::encode(&tree.root));
    }
}
