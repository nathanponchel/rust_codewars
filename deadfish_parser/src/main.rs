fn main() {}

fn parse(code: &str) -> Vec<i32> {
    let mut i: i32 = 0;
    let mut res: Vec<i32> = Vec::new();

    for c in code.chars() {
        match c {
            'i' => i += 1,
            'd' => i -= 1,
            's' => i *= i,
            'o' => res.push(i),
            _ => continue,
        };
    }

    res
}

#[test]
fn sample_tests() {
    assert_eq!(parse("iiisdoso"), vec![8, 64]);
    assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
}
