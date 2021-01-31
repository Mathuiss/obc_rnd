pub struct Header {
    id: u64,
    timestamp: u64,
    difficulty: u8,
    nonce: u64,
    previous_hash: [u8; 256],
}

impl Header {
    pub fn new(id: u64, timestamp: u64, difficulty: u8, previous_hash: [u8; 256]) -> Self {
        Header {
            id: id,
            timestamp: timestamp,
            difficulty: difficulty,
            nonce: 0,
            previous_hash: previous_hash,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_time(&self) -> u64 {
        self.timestamp
    }

    pub fn get_difficulty(&self) -> u8 {
        self.difficulty
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }

    pub fn get_previous_hash(&self) -> [u8; 256] {
        self.previous_hash
    }
}
