pub fn calculate_formula(formula: &str) -> Result<i32, String> {
    let formula: String = formula.replace(" ", "");

    let tokens: Vec<_> = formula.chars().collect();
    match evaluate_expression(&tokens, 0, tokens.len()) {
        Ok((result, _)) => Ok(result),
        Err(e) => Err(e),
    }
}

fn evaluate_expression(tokens: &[char], pos: usize, end: usize) -> Result<(i32, usize), String> {
    if pos >= end {
        return Err("エラー：式の終わりには予期されていない文字列です。".to_string());
    }

    let (mut lhs, mut pos) = evaluate_term(tokens, pos, end)?;

    while pos < end {
        match tokens[pos] {
            '+' => {
                let (rhs, new_pos) = evaluate_term(tokens, pos + 1, end)?;
                lhs += rhs;
                pos = new_pos;
            }
            '-' => {
                let (rhs, new_pos) = evaluate_term(tokens, pos + 1, end)?;
                lhs -= rhs;
                pos = new_pos;
            }
            _ => break,
        }
    }

    Ok((lhs, pos))
}

fn evaluate_term(tokens: &[char], pos: usize, end: usize) -> Result<(i32, usize), String> {
    if pos >= end {
        return Err("エラー：式の終わりには予期されていない文字列です。".to_string());
    }

    let (mut lhs, mut pos) = evaluate_factor(tokens, pos, end)?;

    while pos < end {
        match tokens[pos] {
            '*' => {
                let (rhs, new_pos) = evaluate_factor(tokens, pos + 1, end)?;
                lhs *= rhs;
                pos = new_pos;
            }
            '/' => {
                let (rhs, new_pos) = evaluate_factor(tokens, pos + 1, end)?;
                if rhs == 0 {
                    return Err("エラー：0で除算することはできません。".to_string());
                }
                lhs /= rhs;
                pos = new_pos;
            }
            _ => break,
        }
    }

    Ok((lhs, pos))
}

fn evaluate_factor(tokens: &[char], pos: usize, end: usize) -> Result<(i32, usize), String> {
    if pos >= end {
        return Err("エラー：式の終わりには予期されていない文字列です。".to_string());
    }

    if tokens[pos] == '(' {
        let (value, new_pos) = evaluate_expression(tokens, pos + 1, end)?;
        Ok((value, new_pos + 1))
    } else {
        tokens[pos].to_digit(10)
            .map(|num| (num as i32, pos + 1))
            .ok_or(format!("エラー：想定外の文字列 '{}' が含まれています。", tokens[pos]))
    }
}