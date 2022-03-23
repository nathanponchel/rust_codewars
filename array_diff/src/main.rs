fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut a = a;

    // for y in &b {
    //    a.retain(|x| !(x == y));
    // }
    a.retain(|x| !b.contains(x));

    a
}

fn main() {}

#[test]
fn returns_expected() {
    assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
    assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    assert_eq!(array_diff(vec![1, 2, 3], vec![1, 2]), vec![3]);
}
