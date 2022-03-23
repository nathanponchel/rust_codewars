// fn multiply_digits(mut num: u64) -> u64 {
//     let mut res: u64 = 1;
//     while num > 0 {
//         res *= num % 10;
//         num /= 10;
//     }
//     res
// }

// fn persistence(num: u64) -> u64 {
//     // num is already one digit
//     if num < 10 {
//         return 0;
//     }

//     // first iteration
//     let mut res = multiply_digits(num);
//     let mut count = 1;

//     // nexts iterations
//     while res > 9 {
//         res = multiply_digits(res);
//         count += 1;
//     }
//     count
// }

fn persistence(mut num: u64) -> u64 {
    if num < 10 {
        return 0;
    }

    let mut count: u64 = 0;
    while num > 9 {
        num = num
            .to_string()
            .chars()
            // .map(|x| x as u64 - 48)
            .map(|x| x.to_digit(10).unwrap() as u64)
            .product();
        count += 1;
    }
    count
}

fn main() {
    // in ASCII 0 => 48, 1 => 49, 2 => 50, etc..
    // because cast from char to int return the ascii value of it,
    // with this substraction (50 - 48 = 2), we got our number
    let c: char = '2';
    let _n = c as u64 - 48;
}

#[test]
fn sample_tests() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(4), 0);
    assert_eq!(persistence(25), 2);
    assert_eq!(persistence(999), 4);
}
