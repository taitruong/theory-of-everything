fn calculate_value(interoperability: f64, uniqueness: f64) -> f64 {
    let value = interoperability * uniqueness.powf(2.0);
    value
}

fn main() {
    let uniqueness = 0.9;

    let chains = [1, 2, 3, 5, 8];

    for chain in &chains {
        let interoperability = *chain as f64;
        let value = calculate_value(interoperability, uniqueness);
        println!(
            "NFTs going interchain on {} chain(s) have a value of: {}",
            chain, value
        );
    }
}
