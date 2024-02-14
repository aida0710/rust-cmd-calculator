use std::io::{self, Write};

mod purser;

fn main() {
    loop {
        let mut input: String = String::new();

        println!("計算式を入力してください");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        let formula: &str = input.trim();

        let result: Result<i32, String> = purser::calculate_formula(formula);
        match result {
            Ok(val) => println!("{} = {}", formula, val),
            Err(err) => println!("{}: {}", formula, err),
        }
    }
}