// Placeholder struct — will later have sender, receiver, amount, etc.

#[derive(Clone, Debug)]
pub struct Transaction(pub String); // Start with a simple wrapper

// You can later expand this into:
// pub struct Transaction { from: String, to: String, amount: u64, ... }
