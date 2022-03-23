// fn likes(names: &[&str]) -> String {
//     match names.len() {
//         0 => format!("no one likes this"),
//         1 => format!("{} likes this", names[0]),
//         2 => format!("{} and {} like this", names[0], names[1]),
//         3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
//         _ => format!("{}, {} and {} others like this", names[0], names[1], names.len()-2),
//     }
// }

fn likes(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}

fn main() {}

#[test]
fn example_tests() {
    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}
