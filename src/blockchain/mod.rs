pub mod transaction;
pub mod block;
pub mod merkle;

// Re-export commonly used types so you can do blockchain::Transaction directly
pub use transaction::Transaction;
pub use block::{Block, BlockHeader};
pub use merkle::{calculate_merkle_root, merkle_tree, build_merkle_tree};
