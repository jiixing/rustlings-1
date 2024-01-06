// move_semantics2.rs
//
// Make the test pass by finding a way to keep both Vecs separate!
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DON

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let mut vec1 = fill_vec(vec0.clone());
    vec1.push(33);
    assert_eq!(vec0, vec![22, 44, 66]);
    assert_eq!(vec1, vec![22, 44, 66, 88, 33]);
}
 
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = vec;

    new_vec.push(88);

    new_vec
}
