use crate::blockchain::chain::Chain;
use crate::control::cli;

pub struct BlockchainCli<'a> {
    blockchain: &'a Chain,
}

impl BlockchainCli<'_> {
    pub fn new(blockchain: &Chain) -> BlockchainCli {
        BlockchainCli {
            blockchain: &blockchain,
        }
    }

    pub fn run(&self) {
        loop {
            // Read command
            let raw_text = self.read();

            // Parse command
            let cmd: Vec<&str> = raw_text.split(' ').collect();

            // Execute command
            match cmd[0] {
                "spawn" => println!("Spawning blockchain"),
                "exit" => break,
                _ => println!("Command not found. Enter [help] for more information."),
            };
        }
    }

    fn read(&self) -> String {
        cli::view::print_msg("obc> ");
        cli::controller::read_input()
    }
}
