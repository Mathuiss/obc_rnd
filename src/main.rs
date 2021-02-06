mod blockchain;
mod control;

use control::cli::model::BlockchainCli;

fn main() {
    let cli = BlockchainCli::new();
}
