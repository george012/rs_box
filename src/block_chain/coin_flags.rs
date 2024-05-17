use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoinFlag {
    CoinFlagNone,
    CoinFlagBTC,
    CoinFlagLTC,
    CoinFlagDOGE,
    CoinFlagETC,
    CoinFlagETHW,
    CoinFlagZIL,
    CoinFlagOCTA,
    CoinFlagMETA,
    CoinFlagCAU,
}

impl CoinFlag {
    fn coin_name(&self) -> &str {
        match self {
            CoinFlag::CoinFlagBTC => "BTC",
            CoinFlag::CoinFlagLTC => "LTC",
            CoinFlag::CoinFlagDOGE => "DOGE",
            CoinFlag::CoinFlagETC => "ETC",
            CoinFlag::CoinFlagETHW => "ETHW",
            CoinFlag::CoinFlagZIL => "ZIL",
            CoinFlag::CoinFlagOCTA => "OCTA",
            CoinFlag::CoinFlagMETA => "META",
            CoinFlag::CoinFlagCAU => "CAU",
            CoinFlag::CoinFlagNone => "none",
        }
    }

    pub fn coin_full_name(&self) -> &str {
        match self {
            CoinFlag::CoinFlagBTC => "Bitcoin",
            CoinFlag::CoinFlagLTC => "Litecoin",
            CoinFlag::CoinFlagDOGE => "Dogecoin",
            CoinFlag::CoinFlagETC => "EthereumClassic",
            CoinFlag::CoinFlagETHW => "EthereumPoW",
            CoinFlag::CoinFlagZIL => "Zilliqa",
            CoinFlag::CoinFlagOCTA => "OctaSpace",
            CoinFlag::CoinFlagMETA => "MetaChain",
            CoinFlag::CoinFlagCAU => "Canxium",
            CoinFlag::CoinFlagNone => "none",
        }
    }

    pub fn get_block_node_binary_name(&self) -> &str {
        match self {
            CoinFlag::CoinFlagBTC => "btcd",
            CoinFlag::CoinFlagLTC => "litecoind",
            CoinFlag::CoinFlagDOGE => "dogecoind",
            CoinFlag::CoinFlagETC => "geth",
            CoinFlag::CoinFlagETHW => "geth",
            CoinFlag::CoinFlagZIL => "zilliqa",
            CoinFlag::CoinFlagOCTA => "geth",
            CoinFlag::CoinFlagMETA => "geth",
            CoinFlag::CoinFlagCAU => "canxium",
            CoinFlag::CoinFlagNone => "none",
        }
    }

    pub fn get_block_node_binary_systemd_service_name(&self) -> String {
        let service_name = format!("{}-{}", self.coin_name().to_lowercase(), self.get_block_node_binary_name());
        if *self == CoinFlag::CoinFlagETC {
            format!("core-{}", self.get_block_node_binary_name())
        } else {
            service_name
        }
    }
}

pub fn get_coin_flag_by_coin_name(name: &str) -> CoinFlag {
    match name {
        "BTC" | "BitCoin" | "Bitcoin" => CoinFlag::CoinFlagBTC,
        "LTC" | "LiteCoin" | "Litecoin" => CoinFlag::CoinFlagLTC,
        "DOGE" | "DogeCoin" | "Dogecoin" => CoinFlag::CoinFlagDOGE,
        "ETC" | "Ethereum Classic" | "EthereumClassic" => CoinFlag::CoinFlagETC,
        "ETHW" | "EthereumPoW" => CoinFlag::CoinFlagETHW,
        "ZIL" | "Zilliqa" => CoinFlag::CoinFlagZIL,
        "OCTA" | "OctaSpace" => CoinFlag::CoinFlagOCTA,
        "META" | "MetaChain" => CoinFlag::CoinFlagMETA,
        "CAU" | "Canxium" => CoinFlag::CoinFlagCAU,
        _ => CoinFlag::CoinFlagNone,
    }
}

pub fn is_coin_supported(coin_name: &str) -> bool {
    let supported_coins: HashMap<&str, CoinFlag> = [
        ("BTC", CoinFlag::CoinFlagBTC),
        ("ETC", CoinFlag::CoinFlagETC),
        ("ETHW", CoinFlag::CoinFlagETHW),
        ("ZIL", CoinFlag::CoinFlagZIL),
        ("OCTA", CoinFlag::CoinFlagOCTA),
        ("LTC", CoinFlag::CoinFlagLTC),
        ("DOGE", CoinFlag::CoinFlagDOGE),
        ("META", CoinFlag::CoinFlagMETA),
        ("CAU", CoinFlag::CoinFlagCAU),
    ]
        .iter()
        .cloned()
        .collect();

    supported_coins.contains_key(coin_name)
}


