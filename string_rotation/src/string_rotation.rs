fn left_rotate(input: &str, n: usize) -> String {
    let mut result: String = String::new();

    result = input[0..n].chars().rev().collect();
    result = input[n..].chars().rev().collect();
    result = input.chars().rev().collect();

    result
}

fn right_rotate(input: &str, n: usize) -> String {
    left_rotate(input, input.len() - n)
}
