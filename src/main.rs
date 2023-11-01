fn calculate_love(energy: f64, connection: f64) -> f64 {
    let love = energy * connection.powf(2.0);
    love
}

fn main() {
    let energy = 2.5;
    let connection = 1.8;

    let love = calculate_love(energy, connection);
    println!(
        "According to the love formula, your love value is: {}",
        love
    );
}
