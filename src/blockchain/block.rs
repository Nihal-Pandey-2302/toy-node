use super::transaction::Transaction;

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

pub struct BlockHeader {
    pub merkle_root: Vec<u8>, // Only merkle root for now
    // Later: timestamp, nonce, previous hash, etc.
}
