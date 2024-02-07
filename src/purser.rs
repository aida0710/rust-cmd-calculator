pub fn calculate_formula(formula: &str) -> i32 {
    let formula: String = formula.replace(" ", "");

    let mut result: i32 = 0;
    let mut operator: char = '+';
    let mut num: i32 = 0;
    for c in formula.chars() {
        if c.is_digit(10) {
            num = num * 10 + c.to_digit(10).unwrap() as i32;
        } else {
            result = match_operator(operator, num, result);
            operator = c;
            num = 0;
        }
    }
    result = match_operator(operator, num, result);
    result
}

fn match_operator(operator: char, num: i32, mut result: i32) -> i32 {
    match operator {
        '+' => result + num,
        '-' => result - num,
        '*' => result * num,
        '/' => {
            if num == 0 {
                panic!("Division by zero");
            } else {
                result /= num;
                result
            }
        }
        _ => panic!("Invalid operator"),
    }
}