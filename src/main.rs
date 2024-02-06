mod purser;

use std::io::{self, Write};

fn main() {
    loop {
        let mut input: String = String::new();

        println!("計算式を入力してください");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let formula: &str = input.trim();

        let result: i32 = purser::calculate_formula(formula);
        println!("{} = {}", formula, result);
    }
}