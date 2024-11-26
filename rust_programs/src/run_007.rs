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

    let block_height = 123321;
    let verbosity = 2;

    let block_hash = base_command("getblockhash", vec![&block_height.to_string()])
        .output()
        .expect("failed to execute process")
        .stdout;

    let block_hash_str = String::from_utf8(block_hash).unwrap();

    let block = base_command("getblock", vec![block_hash_str.trim(), &verbosity.to_string()])
        .output()
        .expect("failed to execute process")
        .stdout;

    let block_data = str::from_utf8(&block).unwrap();
    let block_json: Value = serde_json::from_str(block_data).unwrap();

    let txs = block_json["tx"].as_array().unwrap();
    
    for tx in txs {
        let txid = tx["txid"].as_str().unwrap().trim();
        let vout = tx["vout"].as_array().unwrap();
        for i in 0..vout.len() {
            let txout = base_command("gettxout", vec![txid, &i.to_string()])
                .output()
                .expect("failed to execute process")
                .stdout;
            let txout_data = str::from_utf8(&txout).unwrap().trim();
            if txout_data == "" {
                continue;
            }
            else {
                let txout_json: Value = serde_json::from_str(txout_data).unwrap();
                let address = &txout_json["scriptPubKey"]["address"];
                let str_addr = address.as_str().unwrap().to_string();
                print!("{}", str_addr.trim_end_matches("%"));
            }
        }
    }
}
