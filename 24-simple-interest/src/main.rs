fn main() {
    let stdin = std::io::stdin();

    let mut principal = String::new();
    let mut rate = String::new();
    let mut num_of_years = String::new();

    println!("Enter the principal: ");

    stdin.read_line(&mut principal).unwrap();

    let principal = principal
        .trim()
        .parse::<f64>()
        .expect("Please enter a valid number");

    println!("Enter the rate of interest: ");

    stdin.read_line(&mut rate).unwrap();

    let rate = rate
        .trim()
        .parse::<f32>()
        .map(|v| v / 100.0)
        .expect("Please enter the rate as a percentage (without percent sign), e.g. 15(%)");

    println!("Enter the number of years: ");

    stdin.read_line(&mut num_of_years).unwrap();

    let num_of_years = num_of_years
        .trim()
        .parse::<usize>()
        .expect("Please enter an integer");

    let interest = calculate_simple_interest(rate, principal, num_of_years);

    println!(
        "After {num_of_years} years at {rate}%, the investment will be worth ${interest:.2}",
        num_of_years = num_of_years,
        rate = rate * 100.0,
        interest = interest
    );

    print_yearly_simple_interest(rate, principal, num_of_years);
}

fn calculate_simple_interest(rate: f32, principal: f64, num_of_years: usize) -> f64 {
    principal * (1.0 + rate * (num_of_years as f32)) as f64
}

fn print_yearly_simple_interest(rate: f32, principal: f64, num_of_years: usize) {
    println!("\nThe yearly breakdown is: \n");

    for year in 1..=num_of_years {
        let amount = calculate_simple_interest(rate, principal, year);
        println!("Year {year}: {amount:.2}", year = year, amount = amount)
    }
}
