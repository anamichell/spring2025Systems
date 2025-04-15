// // Task 1
// // Write a closure named operation that multiplies two integers and returns the result. Test it with 10 * 5 and print the result.
// fn main() {
//     let operation = |a: i32, b: i32| {
//        a * b
//     };

//     println!("Result: {}", operation(10, 5));
// }

// // Task 2 
// // Write a closure named update inside a function track_changes. The closure should increment and print a counter each time it is called.
// fn track_changes() {
//     let mut tracker = 0;
//     let mut update = || {
//         tracker += 1;
//         println!("Counter: {}", tracker);
//     };

//     update();
//     update();
// }

// fn main() {
//     track_changes();
// }

// Task 3
// Write a function process_vector that applies a closure to transform each element of a vector. 
// Implement it in both ways:
// 1. Using map and collect
// 2. Using a for loop
fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    // Option 1
    // vec.into_iter().map(f).collect();

    // Option 2
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        // Implement: multiply each number by 2
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        // Implement: if number > 2, replace with 0, else keep number
        if x > 2 {
            0
        } else {
            x
        }
    });

    println!("Doubled: {:?}", doubled);
    println!("Replaced: {:?}", replaced);
}

// // Task 5: Lazy Computation
// // Write a struct ComputeCache that accepts a closure during initialization. 
// // Cache the result after the first computation. Use thread::sleep to simulate an expensive computation.
// use std::{thread, time::Duration};

// struct ComputeCache<T>
// where
//     T: Fn() -> String,
// {
//     computation: T,
//     value: Option<String>,
// }

// impl<T> ComputeCache<T>
// where
//     T: Fn() -> String,
// {
//     fn new(computation: T) -> Self {
//         ComputeCache {
//             computation,
//             value: None,
//         }
//     }

//     fn get_result(&mut self) -> String {
//         match &self.value {
//             Some(v) => {
//                 println!("Retrieved from cache instantly!");
//                 v.to_string()
//             },
//             None => {
//                 let v = (self.computation)();
//                 self.value = Some(v.clone());
//                 v.to_string()
//             }
//         }
//     }
// }

// fn main() {
//     let mut cache = ComputeCache::new(|| {
//         println!("Computing (this will take 2 seconds)...");
//         thread::sleep(Duration::from_secs(2));
//         "Hello, world!".to_string()
//     });

//     println!("First call:");
//     println!("Result: {}", cache.get_result());
    
//     println!("\nSecond call:");
//     println!("Result (cached): {}", cache.get_result());
// }

