fn main() {}

fn disemvowel(s: &str) -> String {
    s.chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}

#[test]
fn example_test() {
    assert_eq!(
        disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}
