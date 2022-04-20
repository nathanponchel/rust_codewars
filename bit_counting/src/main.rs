fn main() {}

fn count_bits(n: i64) -> u32 {
    let n_binary = format!("{:b}", n);
    let mut count: u32 = 0;

    for bit in n_binary.chars() {
        if bit == '1' {
            count += 1;
        }
    }

    return count;
}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}