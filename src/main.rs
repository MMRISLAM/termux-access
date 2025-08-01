use serde::{Serialize, Deserialize};
use sha2::{Sha256, Digest};
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut hasher = Sha256::new();
        hasher.update(format!("{}{}{}{}", index, timestamp, &data, &previous_hash));
        let result = hasher.finalize();
        let hash = format!("{:x}", result);

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }
}

fn main() {
    let mut blockchain: Vec<Block> = Vec::new();

    let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
    blockchain.push(genesis_block);

    for i in 1..=50 {
        let prev_block = blockchain.last().unwrap();
        let new_block = Block::new(i, format!("Block number {}", i), prev_block.hash.clone());
        blockchain.push(new_block);
    }

    for block in &blockchain {
        println!("Block #{}:\nTimestamp: {}\nHash: {}\nPrev Hash: {}\nData: {}\n---",
            block.index, block.timestamp, block.hash, block.previous_hash, block.data);
    }
}
