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
    let block_height = "123456";

    let block_hash = base_command("getblockhash", vec![block_height]).output().unwrap().stdout;
    let block_hash_str = str::from_utf8(&block_hash).unwrap().trim(); // remove the newline character

    let verbosity = "2";
    let block = base_command("getblock", vec![block_hash_str,verbosity]).output().expect("Failed getblock");

    if !block.status.success() {
        let error = str::from_utf8(&block.stderr).unwrap();
        println!("Error: {}", error);
        return;
    }

    let block_data = str::from_utf8(&block.stdout).unwrap();

    let block_data: Value = serde_json::from_str(block_data).unwrap();

    let txs = block_data["tx"].as_array().unwrap();
    let vouts = txs.iter().map(|tx| tx["vout"].as_array().unwrap().len()).collect::<Vec<usize>>();

    let mut count = 0;
    for vout in vouts {
        count += vout;
    }

    let output = count;
    
    println!("{}", output.to_string());
}
