mod blockchain;
mod control;

use blockchain::chain::Chain;
use control::cli::model::BlockchainCli;

fn main() {
    let chain = Chain::spawn();
    let cli = BlockchainCli::new(&chain);
    cli.run();
}
