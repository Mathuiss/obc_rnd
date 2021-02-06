use crate::blockchain::header::Header;
use crate::blockchain::transaction::Transaction;

pub struct Block {
    header: Header,
    transactions: Vec<Transaction>,
    hash: [u8; 256],
}

impl Block {
    pub fn new(header: Header) -> Block {
        Block {
            header: header,
            transactions: Vec::new(),
            hash: [0x0; 256],
        }
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_transactions(&self) -> &Vec<Transaction> {
        &self.transactions
    }

    pub fn get_hash(&self) -> [u8; 256] {
        self.hash
    }

    pub fn set_hash(&mut self, hash: [u8; 256]) {
        self.hash = hash;
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn calculate_hash(&self) -> [u8; 256] {}
}
