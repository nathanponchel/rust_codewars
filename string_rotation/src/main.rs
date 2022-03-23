fn main() {}

// fn last_to_first(mut input: String) -> String {
//     let mut result = String::new();

//     result.push(input.chars().last().unwrap());
//     input.pop();
//     result.push_str(input.get(0..).unwrap());
//     result
// }

// fn shifted_diff(first: &str, second: &str) -> Option<usize> {
//     // already sames
//     if first.eq(second) {
//         return Some(0);
//     }
//     // Not sames because of case sensitive
//     if first.eq_ignore_ascii_case(second) {
//         return None;
//     }

//     let mut temp = last_to_first(first.to_string());
//     let mut count: usize = 1;

//     while temp != second {
//         //after a complete permutation cycle, they arent variations of each others
//         if count > second.len() {
//             return None;
//         }

//         count += 1;
//         temp = last_to_first(temp.to_string());
//     }
//     Some(count)
// }

fn shifted_diff(first: &str, second: &str) -> Option<usize> {
    if first.len() > second.len() {
        return None;
    }
    second.repeat(2).find(first)
}

#[test]
fn test_example() {
    assert_eq!(shifted_diff("hoop", "pooh"), None);
    assert_eq!(shifted_diff("eecoff", "coffee"), Some(4));
    assert_eq!(shifted_diff("Moose", "moose"), None);
    assert_eq!(shifted_diff("isn't", "'tisn"), Some(2));
    assert_eq!(shifted_diff("Esham", "Esham"), Some(0));
    assert_eq!(shifted_diff(" ", " "), Some(0));
}
