use accounts::tax::*;
use essembly_core::entity::*;

fn main() {
    println!("Hello, world!");

    let result = calc_tax(2.0, 0.05);
    println!("Tax Result: {:?}", result);
}
