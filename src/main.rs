use alloy_core::json_abi::JsonAbi;
use foundry_compilers::artifacts::Evm;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{read_dir, read_to_string};

#[derive(Serialize, Deserialize, Debug)]
struct Export {
    abi: JsonAbi,
    evm: Evm,
    // #[serde(rename = "chanId")]
    // chain_id: u32,
    // name: String,
}

fn read_json<T>(path: &str) -> Result<T, Box<dyn Error>>
where
    T: DeserializeOwned,
{
    println!("Reading ABI from {}", path);

    let data = read_to_string(path)?;
    let content = serde_json::from_str::<T>(&data)?;

    Ok(content)
}

fn find_json_dir(dir: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = vec![];
    let dir = read_dir(dir)?;
    println!("Listing files in {:?}", dir);

    for entry in dir {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if let Some(p) = path.to_str() {
                    if p.ends_with(".json") {
                        result.push(p.to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }

    Ok(result)
}

fn main() {
    println!("Hello, world!");

    let cartesi_dapp_def = "wallet/node_modules/@cartesi/rollups/export/artifacts/contracts/dapp/CartesiDApp.sol/CartesiDApp.json";
    // let erc1155 = "wallet/smart_contracts/erc1155-abi.json";

    let cartesi_dapp = read_json::<Export>(cartesi_dapp_def).unwrap();
    let tr = cartesi_dapp
        .abi
        .function("transferOwnership")
        .unwrap()
        .get(0)
        .unwrap();
    dbg!(tr.full_signature());
    dbg!(cartesi_dapp);

    // let erc1155_abi = read_abi::<JsonAbi>(erc1155).unwrap();
    // dbg!(erc1155_abi);

    // let abi = read_abi(path).unwrap();
    //
    // let function = abi.function("balanceOf");
    //
    // dbg!(function);

    // for item in abi.items() {
    //     println!("{:?}", item);
    // }
}
