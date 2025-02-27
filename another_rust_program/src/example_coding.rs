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

// Problem 1: String concatenation
fn concat_strings(s1: &String, s2: &String) -> String {
    let mut borrow_word = s1.to_string();
    borrow_word.push_str(&s2);
    borrow_word
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}

// // Problem 2: Clone and Modify
// fn clone_and_modify(s: &String) -> String {
//     let mut word = s.clone();
//     word.push_str("World");
//     word
// }

// fn main() {
//     let s = String::from("Hello, ");
//     let modified = clone_and_modify(&s);
//     println!("Original: {}", s); // Should print: "Original: Hello, "
//     println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
// }

// // Problem 3: Mutable Reference Sum
// #[allow(unused_variables, unused_mut)]
// fn sum(total: &mut i32, low: i32, high: i32) {
//     for x in low..=high {
//         *total += x;
//     }
    
// }

// fn main() {
//     // create necessary variables and test your function for low 0 high 100
//     // total should be 5050
//     let low = 0;
//     let high = 100;
//     let mut total = 0;
    
//     sum(&mut total, low, high);

//     println!("Total: {}", total);
// }