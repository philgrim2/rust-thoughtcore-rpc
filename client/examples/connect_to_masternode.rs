extern crate dashcore_rpc;

use dashcore_rpc::{Auth, Client, RpcApi};

fn main() {
    let rpc = Client::new(
        "localhost:19998",
        Auth::UserPass("dashrpc".to_string(), "rpcpassword".to_string()),
    )
    .unwrap();

    // Get Dash network info
    let network_info = rpc.get_network_info().unwrap();
    println!("\nDash network info: \n{:?}", network_info);

    // Get best block hash
    let best_block_hash = rpc.get_best_block_hash().unwrap();
    println!("\n\nBest block hash: \n{}", best_block_hash);

    // Get block count
    let block_count = rpc.get_block_count().unwrap();
    println!("\n\nBlock count: \n{}", block_count);

    // Get block hash (for the a specified block height)
    let block_hash = rpc.get_block_hash(block_count).unwrap();
    println!("\n\nBlock hash at block height {}: \n{}", block_count, block_hash);

    // Get masternode count
    let masternode_count = rpc.get_masternode_count().unwrap();
    println!("\n\nMasternode Count: \n{:?}", masternode_count);


    // Get masternode list
    let mn_list = rpc.get_masternode_list(Some("json"), None).unwrap();
    println!("\n\nMasternode List: \n{:?}", mn_list);

    // Get masternode outputs
    let mn_outputs = rpc.get_masternode_outputs().unwrap();
    println!("\n\nMasternode Outputs: \n{:?}", mn_outputs);

    // Get masternode payments 
    let mn_payments = rpc.get_masternode_payments(None, None).unwrap();
    println!("\n\nMasternode Payments: \n{:?}", mn_payments);

    // Get masternode status
    let mn_status = rpc.get_masternode_status().unwrap();
    println!("\n\nMasternode Status: \n{:?}", mn_status);

    // Get masternode winners
    let mn_winners = rpc.get_masternode_winners(None, None).unwrap();
    println!("\n\nMasternode Winners: \n{:?}", mn_winners);

}
