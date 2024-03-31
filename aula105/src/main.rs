struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: Option<String>,
}

impl Block {
    fn new(index: u64, timestamp: u64, data: String, hash: String, prev_hash: Option<String>) -> Block {
        Block { index, timestamp, data, hash, prev_hash }
    }


    fn data_size(&self) -> usize { self.data.len() }

    fn creation_time(&self) -> u64 { self.timestamp / 1000 }
}


fn main() {
    let block = Block::new(0, 1630000000000, "Genesis Block".to_string(), "0".to_string(), None);
    println!("Block created at: {}", block.creation_time());
    println!("Data size: {}", block.data_size());
}
