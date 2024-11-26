use core::str;
use std::process::Command;
use serde_json::Value;

pub fn run() {
    let base_command = |cmd: &str, args: Vec<&str>| -> Command {
        let mut command = Command::new("bitcoin-cli");
        command
            .arg("-rpcconnect=84.247.182.145")
            .arg("-rpcuser=user_106")
            .arg("-rpcpassword=4wjt7NoqJHld")
            .arg(cmd);
        for arg in args {
            command.arg(arg);
        }
        command
    };
    
    let block1_height = 256128;
    let block2_height = 257343;
    let verbosity = 2;
    let block1_hash = base_command("getblockhash", vec![&block1_height.to_string()])
        .output()
        .expect("failed to execute process")
        .stdout;

    let block2_hash = base_command("getblockhash", vec![&block2_height.to_string()])
        .output()
        .expect("failed to execute process")
        .stdout;

    let block1_hash_str = String::from_utf8(block1_hash).unwrap();
    let block2_hash_str = String::from_utf8(block2_hash).unwrap();

    let block1 = base_command("getblock", vec![block1_hash_str.trim(), &verbosity.to_string()])
        .output()
        .expect("failed to execute process")
        .stdout;

    let block2 = base_command("getblock", vec![block2_hash_str.trim(), &verbosity.to_string()])
        .output()
        .expect("failed to execute process")
        .stdout;

    let block1_data = str::from_utf8(&block1).unwrap();
    let block2_data = str::from_utf8(&block2).unwrap();

    let block1_json: Value = serde_json::from_str(block1_data).unwrap();
    let block2_json: Value = serde_json::from_str(block2_data).unwrap();

    let txs1 = block1_json["tx"].as_array().unwrap();
    let txs2 = block2_json["tx"].as_array().unwrap();

    let coinbase_tx1 = &txs1[0];
    let coinbase_tx1_txid = coinbase_tx1["txid"].as_str().unwrap().trim();

    // up to here is ok

    let mut txid_expends_coinbase_tx1: Option<&str> = None;

    for tx in txs2 {
        let vins = tx["vin"].as_array().unwrap();

        for i in 1..vins.len() - 1 {
            let txid_vin = vins[i]["txid"].as_str().expect("not found").trim();
            //print!("txid_vin: {}\n", txid_vin);
            if txid_vin == coinbase_tx1_txid {
                txid_expends_coinbase_tx1 = Some(tx["txid"].as_str().unwrap().trim());
            }
        }
    }

    if txid_expends_coinbase_tx1.is_none() {
        println!("0");
    } else {
        let txid = txid_expends_coinbase_tx1.unwrap();
        println!("{}", txid.trim());
    }
}

