// // In class Assignment

// // Create a struct Student (major)
// struct Student {
//     major: String,
// }

// // Higher order functions update majors 
// fn update_major(collection: &mut Vec<Student>,behavior:fn(&mut Student,String)) {
//     for student in collection.iter_mut() {
//         behavior(student, "Biology".to_string());
//     }
// }

// // First order functions, assign_major(student, major_declared)
// fn assign_major(s:&mut Student, major:String) {
//     s.major = major;
// }

// // Helper function to print values without closures
// fn print_values(items: &[Student]) {
//     print!("Values: ");
//     for (i, item) in items.iter().enumerate() {
//         if i > 0 {
//             print!(", ");
//         }
//         print!("{}", item.major);
//     }
//     println!();
// }

// // Create a vector of students1,2,3 and update all students major
// fn main() {
//     let mut students = vec![
//         Student { major: "Computer Science".to_string() },
//         Student { major: "English".to_string() },
//         Student { major: "Music".to_string() },
//     ];
//     // Before update
//     print_values(&students);

//     // Update the major
//     update_major(&mut students, assign_major);
//     print_values(&students);
// }

// // // // fn motivation_example() {
// // // //     // Problem: find the largest element
// // // //     // For integers 32
// // // //     fn largest<T>(list: &[T]) -> T {
// // // //         let mut largest = list[0];
// // // //         for &item in list.iter() {
// // // //             if item > largest {
// // // //                 largest = item;
// // // //             }
// // // //         }
// // // //         largest
// // // //     }

// // // //     println!("{}", largest(&[1, 2, 5]));
// // // //     println!("{}", largest(&[1.5, 2.6, 5.9]));
// // // //     println!("{}", largest(&['A', 'B', 'C']));
// // // // }

// // // // fn main() {
// // // //     motivation_example();
// // // // }


// // // #[allow(dead_code)]
// // // fn generics_in_struct() {
// // //     #[derive(Debug)]
// // //     struct Point<T> {
// // //         x: T,
// // //         y: T,
// // //     }

// // //     let integer = Point { x: 5, y: 10 };
// // //     let float = Point { x: 1.0, y: 4.0 };

// // //     #[derive(Debug)]
// // //     struct Dot {
// // //         alpha: i32,
// // //         beta: i32,
// // //     }

// // //     let imaginary_data_type = Point{
// // //         x: Dot{alpha:64,beta:84},
// // //         y: Dot{alpha:64,beta:84},
// // //     };

// // //     println!("int Point: {:?} float Point: {:?}", integer, float);
// // //     println!("imaginary_data_type: {:?}", imaginary_data_type);

// // //     #[derive(Debug)]
// // //     struct User<T, U> {
// // //         name: T,
// // //         y: U,
// // //     }

// // //     let user1 = User { name: "Vandam", y: 35 };
// // //     let user2 = User { name: "James Bond".to_string(), y: "===> 007" };

// // //     println!("User1: {:?} User2: {:?}", user1, user2);
// // // }

// // // fn main() {
// // //     generics_in_struct();
// // // }

// // fn benefits_of_logical_entity(){
        
// //     pub trait Summary { // Trait should be public if we want to allow others to implement it
// //         fn summarize(&self) -> String; // no body just declaration like interface
// //     }
    
// //     pub struct NewsArticle {
// //         pub headline: String,
// //         pub location: String,
// //         pub author: String,
// //         pub content: String,
// //     }
    
// //     impl Summary for NewsArticle { 
// //         fn summarize(&self) -> String { 
// //             format!("{}, by {} ({})", self.headline, self.author, self.location)
// //         }
// //     }
    
// //     pub struct Tweet {
// //         pub username: String,
// //         pub content: String,
// //         pub reply: bool,
// //         pub retweet: bool,
// //     }
    
// //     impl Summary for Tweet {
// //         fn summarize(&self) -> String {
// //             format!("{}: {}", self.username, self.content)
// //         }
// //     }
    
// //     let tw = Tweet {
// //              username: String::from("Elon"),
// //              content: String::from("to the Moon"),
// //              reply: false,
// //              retweet: false,
// //          };
// //     println!("{}",tw.summarize());
        
// //     let article = NewsArticle {
// //              headline: String::from("Elon sells Bitcoin"),
// //              location: String::from("Menlo Park, CA, USA"),
// //              author: String::from("CNN"),
// //              content: String::from("FBI investigates"),
// //          };
    
// //     println!("{}", article.summarize());

// //     // Real reason we need to use traits or interfaces
// //     // Change you coding paradigm, start to programm to behavior!

// //     pub fn notify_sugar_syntax(item: impl Summary) { // your function will accept any parameter that implements Summary trait. (so all parameters will have the common behavior)
// //         println!("Breaking news! {}", item.summarize());
// //     }

// //     // Same logic as above but explicit, this is refereed to the idea TRAIT BOUNDS
// //     // or in simple language, sometimes you want to accept parameters, which implement more than one trait(have more than one common method to call on it)
    
// //     pub fn notify_real_syntax<T: Summary>(item: T){ // please notice generics you are saying. My function will accept as a parameter a generic type T which implements Summary trait, because I just want to make sure that I can call the methods safely.
        
// //         println!("Breaking news! {}", item.summarize());
// //     }


// //     notify_real_syntax(tw);
// //     notify_sugar_syntax(article);

// // }

// // fn main() {
// //     benefits_of_logical_entity();
// // }

// // Lambda allows you to see what the calculation is wihtout
// // looking for a seperate funtction in the code
// //** print(calculator(lambda x,y: x**y, 5, 10));

// // Define our data structure
// struct Data {
//     value: i32,
// }

// // Higher-order function: defines what needs to be done
// fn process_data(data: &mut [Data], operation: fn(&mut Data)) {
//     for item in data.iter_mut() {
//         operation(item);
//     }
// }

// // Specific operations: actual functions which do the work
// fn double_value(data: &mut Data) {
//     data.value *= 2;
// }

// fn square_value(data: &mut Data) {
//     data.value = data.value * data.value;
// }

// // Helper function to print values without closures
// fn print_values(items: &[Data]) {
//     print!("Values: ");
//     for (i, item) in items.iter().enumerate() {
//         if i > 0 {
//             print!(", ");
//         }
//         print!("{}", item.value);
//     }
//     println!();
// }

// fn main() {
//     let mut items = vec![
//         Data { value: 1 },
//         Data { value: 2 },
//         Data { value: 3 },
//         Data { value: 4 },
//         Data { value: 5 },
//     ];
    
//     // The specific operation is decided here
//     print!("Original ");
//     print_values(&items);
    
//     process_data(&mut items, double_value);
//     print!("After doubling: ");
//     print_values(&items);
    
//     // We can easily switch to a different operation
//     process_data(&mut items, square_value);
//     print!("After squaring: ");
//     print_values(&items);
// }

fn using_function_as_parameter() {
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calculator(x: i32, y: i32, operation: fn(i32, i32) -> i32) {
        let result = operation(x, y);
        println!("Result of operation: {}", result);    
    }

    calculator(1, 2, add);
    calculator(1, 2, |x, y| x + y);
    calculator(1, 2, |x, y| x - y);
    calculator(1, 2, |x, y| x * y);
}

fn main() {
    using_function_as_parameter();
}