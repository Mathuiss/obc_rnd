pub struct Transaction {
    sender: u64,
    recipient: u64,
    payload_type: String,
    json_payload: String,
}

impl Transaction {
    pub fn new(sender: u64, recipient: u64, payload_type: String, json_payload: String) -> Self {
        Transaction {
            sender: sender,
            recipient: recipient,
            payload_type: payload_type,
            json_payload: json_payload,
        }
    }

    pub fn get_sender(&self) -> u64 {
        self.sender
    }

    pub fn get_recipient(&self) -> u64 {
        self.recipient
    }

    pub fn get_payload_type(&self) -> &String {
        &self.payload_type
    }

    pub fn get_json_payload(&self) -> &String {
        &self.json_payload
    }
}
