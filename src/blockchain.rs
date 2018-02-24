extern crate crypto;

use blockchain::crypto::sha2::Sha256;
use blockchain::crypto::digest::Digest;
use std::time::{SystemTime, UNIX_EPOCH};
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Block {
    pub time_stamp: u64,
    pub data: String,
    pub prev_block_hash: String,
    pub hash: String,
}

pub trait Blockable {
    fn new(data: String, previous_hash: String) -> Self;
}

impl Blockable for Block {
    fn new(data: String, previous_hash: String) -> Self {
        let ts = get_current_time_stamp();
        let mut sha = Sha256::new();
        sha.input_str(&format!("{}{}{}", ts, data, previous_hash));
        let block = Block {
            time_stamp: get_current_time_stamp(),
            data,
            prev_block_hash: previous_hash,
            hash: sha.result_str(),
        };
        block
    }
}

pub struct Blockchain<> {
    pub blocks: RefCell<Vec<Block>>,
    latest_block: RefCell<Block>,
}

pub trait Blockchainable {
    fn new() -> Self;
    fn add_block(&mut self, data: String);
}

impl Blockchainable for Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new("genesis block".to_string(), "".to_string());
        let blockchain: Blockchain = Blockchain {
            blocks: RefCell::new(Vec::new()),
            latest_block: RefCell::new(genesis_block.clone()),
        };
        blockchain.blocks.borrow_mut().push(genesis_block);
        blockchain
    }
    fn add_block(&mut self, data: String) {
        let new_block = Block::new(data, self.latest_block.borrow().hash.clone());
        self.latest_block = RefCell::new(new_block.clone());
        self.blocks.borrow_mut().push(new_block.clone());
    }
}

fn get_current_time_stamp() -> u64 {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("err: time went backwards");
    return since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000;
}
