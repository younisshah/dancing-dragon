# dancing-dragon

**dancing dragon** is a simple Blockchain implementation in Rust.

This repository is merely for academic purposes for learning Blockchain technology and the Rust language as well.
The code is not optimized and in no way should be used for production.

## Installation

The code in not on [crates.io](https://crates.io) yet. Once all the Blockchain features are implemented, I will release it as a crate.

```text
git clone https://github.com/younisshah/dancing-dragon
```

## Usage

In ```main.rs```

```rust
    extern crate dancing_dragon;

    use dancing_dragon::blockchain::*;

    fn main() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block("Transfer 1 BTC to John".to_string());
        blockchain.add_block("Transfer 2 BTC to John".to_string());

        for b in blockchain.blocks.borrow().iter() {
            println!("Data: {:?}", b.data);
            println!("Hash: {:?}", b.hash);
            println!("Previous hash: {:?}", b.prev_block_hash);
        }
    }
```

## TODO

- [ ] Add Proof of Work.
- [ ] Add data store and CLI.
- [ ] Add Transactions.
- [ ] Add coin addresses.
- [ ] Add more transactions and mining.
- [ ] Add networking.

### License
MIT
