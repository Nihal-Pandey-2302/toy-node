use super::transaction::Transaction;
use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct BlockHeader{
    pub previous_hash: String,
    pub merkle_root: String,
    pub timestamp: i64,
    pub nonce: u64,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Block{
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}