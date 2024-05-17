#[cfg(test)]
use super::coin_flags;

#[test]
fn test_simple_used_with_coin_flags() {
    let coin = "BTC";

    if coin_flags::is_coin_supported(coin) {
        let flag = coin_flags::get_coin_flag_by_coin_name(coin);
        println!("Coin: {}\nFull Name: {}\nblock chain node Binary Name: {}\nSystemd Service Name: {}\n",
                 coin,
                 flag.coin_full_name(),
                 flag.get_block_node_binary_name(),
                 flag.get_block_node_binary_systemd_service_name()
        );
    } else {
        println!("Coin {} is not supported.", coin);
    }
}