use std::collections::{HashMap, HashSet};

use wasm_bindgen::prelude::*;

type U256 = [u128; 2];
struct Address(String);

trait Operator {
    fn balance_of() -> U256;
    fn transfer(to: &str, value: U256) -> bool;
    fn withdraw(to: &str, value: U256) -> bool;
}

trait Wallet {
    fn getWalletOrNew(address: &str) -> WalletApp;
}

#[wasm_bindgen]
#[derive(Default)]
struct WalletApp {
    ether: U256,
    erc20: HashMap<Address, U256>,
    erc721: HashMap<Address, HashSet<U256>>,
    erc1155: HashMap<Address, HashMap<U256, U256>>,
}

struct WalletImpl {
    dapp: Option<Address>,
    wallets: WalletApp,
}


#[wasm_bindgen(module = "viem")]
extern "C" {
    fn isAddress(address: &str) -> bool;
}

#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[wasm_bindgen]
pub fn is_real(address: &str) -> bool {
    isAddress(address)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
