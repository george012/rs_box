use crate::block_chain::hash_rate;

#[test]
fn test_simple_used_with_hash_rate() {
    let base_hash_rate = 1234567351461536146484698.012;

    let a_seed = 2;

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::Hs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::KHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);


    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::MHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::GHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::THs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::PHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::EHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::ZHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);

    let formatted_hash_rate = hash_rate::hash_rate_to_format(base_hash_rate, hash_rate::HashRateUnitFormat::YHs, a_seed);
    println!("Formatted Hash Rate: {} {}", formatted_hash_rate.value, formatted_hash_rate.unit_str);


    let formatted_hash_rate = hash_rate::hash_rate_format(base_hash_rate);
    println!("auto ---> Formatted Hash Rate: {}", formatted_hash_rate);
}