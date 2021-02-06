use crate::blockchain::block::Block;

pub struct Chain {
    blocks: Vec<Block>,
    queue: Vec<Block>,
}

impl Chain {
    pub fn spawn() -> Self {
        // Spawn genesis block

        Chain {}
    }
}
