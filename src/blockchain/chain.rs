use crate::blockchain::block::Block;
use crate::blockchain::header::Header;

pub struct Chain {
    blocks: Vec<Block>,
    queue: Vec<Block>,
}

impl Chain {
    pub fn spawn() -> Self {
        // Spawn genesis block
        let header = Header::new(0, 1, [0x0; 256]);
        let genesis_block = Block::new(header);
        let mut blocks = Vec::new();
        blocks.push(genesis_block);

        let queue = Vec::new();

        Chain {
            blocks: blocks,
            queue: queue,
        }
    }
}
