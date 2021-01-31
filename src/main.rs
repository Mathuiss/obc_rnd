mod block;

use block::Block;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::io::stdin;

struct Chain {
    nodes: Vec<Block<Transaction>>,
}

impl Chain {
    fn spawn() -> Chain {
        let mut chain = Self { nodes: Vec::new() };
        // Instantiate genesis block
        let gb = Block::new(0, 0, Transaction::new(0, 1, 999999.0));
        chain.append(gb);
        return chain;
    }

    fn append(&mut self, mut block: Block<Transaction>) {
        let len = self.nodes.len();

        if len == 0 {
            self.nodes.push(block);
        } else {
            // Determine previous hash
            let last_index = len - 1;
            let previous_hash = self.nodes[last_index].get_hash();

            // Set previous hash on new block
            block.set_previous_hash(previous_hash);
            self.nodes.push(block);
        }
    }
}

struct Transaction {
    send_address: u64,
    rec_address: u64,
    amount: f64,
}

impl Transaction {
    fn new(send_address: u64, rec_address: u64, amount: f64) -> Self {
        Self {
            send_address: send_address,
            rec_address: rec_address,
            amount: amount,
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "[Send Address: {}, Rec Address: {}, Amount {}]",
            self.send_address, self.rec_address, self.amount
        )
    }
}

fn main() {
    // Construct genesis block and chain
    let mut chain = Chain::spawn();

    loop {
        let cmd_code: u8 = read_input().parse::<u8>().expect("Enter an integer");

        match cmd_code {
            // Create new transaction
            0 => create_transaction(&mut chain, Transaction::new(0, 1, 99.01)),
            _ => println!("Enter a value between 0 - 1"),
        }

        for i in 0..chain.nodes.len() {
            println!("Node: {}", chain.nodes[i].serialize());
            println!("Hash: {}", chain.nodes[i].get_hash());
            println!("-----------------------------------------");
            println!();
        }
    }
}

fn create_transaction(chain: &mut Chain, transaction: Transaction) {
    let new_id = chain.nodes.len() as u64;
    let index = chain.nodes.len() - 1;

    let block = Block::new(new_id, chain.nodes[index].get_hash(), transaction);
    chain.append(block);
}

fn read_input() -> String {
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("Failed to open stream to stdin");

    buf = buf.replace("\n", "");
    buf = buf.replace("\r", "");

    return buf;
}
