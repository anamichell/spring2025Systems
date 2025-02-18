/// Module 2
// 04 Rust Ownership
/// Ownership and functions 
fn main() {
    let s = String::from("hello");
    let s = takes_ownership(s);
    // s is no longer valid here
    println!("{}", s)
}

fn takes_ownership(some_string: String) -> String {
    println!("{}", some_string);
    some_string
} // some_string goes out of scope and is dropped

/// 05 Rust Borrowing
/// Borrowing 
fn main() {
    let word = "UTRGV".to_string();
    let borrow_word = &word;

    let borrow_word1 = &word;

    getString(&word);
}

fn getString(some_string: &String) {
    println!("{}", some_string)
}