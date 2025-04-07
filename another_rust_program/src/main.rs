// // use std::fs::File;
// // use std::io::prelude::*;
// // use std::io::{self, Read, Write};

// // // Reading from console
// // struct Person {
// //     name: String,
// //     sid: u64,
// //     port: u16,
// // }

// // fn reading_from_console() {
// //     let mut file = File::create("config.txt").unwrap();
// //     let mut buffer = String::new();

// //     print!("What's your name? ");
// //     io::stdout().flush().unwrap();
// //     io::stdin().read_line(&mut buffer).unwrap();
// //     let name = buffer.trim().to_string();
// //     buffer.clear();

// //     print!("What's your SID? ");
// //     io::stdout().flush().unwrap();
// //     io::stdin().read_line(&mut buffer).unwrap();
// //     let sid = buffer.trim().parse().unwrap();
// //     buffer.clear();

// //     print!("What's your port number? ");
// //     io::stdout().flush().unwrap();
// //     io::stdin().read_line(&mut buffer).unwrap();
// //     let port = buffer.trim().parse().unwrap();
// //     buffer.clear();
    
// //     let person = Person { name, sid, port };
// //     println!("Hi {}, your SID is {}!", person.name, person.sid);

// //     // write in config.txt file
// //     writeln!(file, "{}", person.name).unwrap();
// //     writeln!(file, "{}", person.sid).unwrap();
// //     writeln!(file, "{}", person.port).unwrap();
// // }

// // // Reading from file
// // struct Config {
// //     name: String,
// //     sid: u64,
// //     port: u16,
// // }

// // impl Config {
// //     fn from_file(path: &str) -> Config {
// //         let mut file = File::open(path).unwrap();
// //         let mut contents = String::new();
// //         file.read_to_string(&mut contents).unwrap();

// //         let mut lines = contents.lines();
// //         let name = lines.next().unwrap().to_string();
// //         let sid = lines.next().unwrap().parse().unwrap();
// //         let port = lines.next().unwrap().parse().unwrap();

// //         Config { name, sid, port }
// //     }
// // }

// // fn reading_from_file() {
// //     let config = Config::from_file("config.txt");
// //     println!("name: {}", config.name);
// //     println!("sid: {}", config.sid);
// //     println!("port: {}", config.port);
// // }

// // fn main() {
// //     reading_from_console();
// //     reading_from_file();
// // }

// // #[derive(Debug)]
// // enum Insurance {
// //     House, 
// //     Car, 
// //     Life,
// // }

// // #[derive(Debug)]
// // struct Person {
// //     name:String,
// //     insurances:Vec<Insurance>, // Vec<Insurance>
// // }

// // impl Person {
// //     fn new(n:String, i:Insurance) -> Person {
// //         Person {
// //             name:n,
// //             insurances:vec![],
// //         }

// //     }

// //     fn add_insurance(&mut self,i:Insurance) {
// //         self.insurances.push(i);
// //     }

// //     fn show_insurance(&self) {
// //     println!("Hey I am {:?}. I have a next type of insurances.", self);
// //         for i in self.insurances.iter() {
// //             match i {
// //                 Insurance::Car => println!("I am insured my {:?}", self.car),
// //                 Insurance::House => println!("I am insured my {:?}", self.house),
// //                 _ => println!(""),
// //             };
// //         }
// //     }
// // }

// // fn main() {
// //     let c = Insurance::Car;
// //     let h = Insurance::House;
// //     let l = Insurance::Life;
// //     let car = Car {
// //         model:"BMW".to_string(),
// //     };

// //     let mut person = Person::new("John".to_string(), car);
// //     person.add_insurance(c);
// //     person.add_insurance(h);
// //     person.show_insurance();
    
// // }

// enum Insurance {
//     Car(String),
//     House(u16),
// }

// impl Insurance {
//     fn show_info(&self) {
//         match self {
//             Insurance::Car(model) => println!("My car model is {:?}", model),
//             Insurance::House(year) => println!("My house was built in {:?}", year),
//         }
//     }
// }

// fn main() {
//     let car = Insurance::Car("BMW".to_string());
//     let house = Insurance::House(2024);

//     car.show_info();
//     house.show_info();
// }