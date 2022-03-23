use std::io;

fn high_and_low(numbers: &str) -> String {
    let result: Vec<i32> = numbers
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    format!(
        "{} {}",
        result.iter().max().unwrap(),
        result.iter().min().unwrap()
    )
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("read_line failed :(");

    println!("{}", high_and_low(input.as_str()));
}

#[test]
fn test() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
}
