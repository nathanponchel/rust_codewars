fn duplicate_encode(word: &str) -> String {
    let word = word.to_lowercase();

    word.chars()
        .map(|c| {
            if word.matches(c).count() == 1 {
                '('
            } else {
                ')'
            }
        })
        .collect()
}

fn main() {}

#[test]
fn tests() {
    assert_eq!(duplicate_encode("din"), "(((");
    assert_eq!(duplicate_encode("recede"), "()()()");
    assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
    assert_eq!(duplicate_encode("(( @"), "))((");
    assert_eq!(duplicate_encode("CodeWarrior"), "()(((())())");
    assert_eq!(duplicate_encode("Supralapsarian"), ")()))()))))()(");
    assert_eq!(
        duplicate_encode("iiiiii"),
        "))))))",
        "duplicate-only-string"
    );
    assert_eq!(duplicate_encode(" ( ( )"), ")))))(");
}
