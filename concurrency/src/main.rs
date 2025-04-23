// Intro to  Concurrency: Base Examples

// fn simple_spawn_join() {
//     println!("Intro to Concurrency");
//     let steps =  Box::new(5);
//     let thread = std::thread::spawn(move ||{
//         // important to notice usage of closure
//         // it captures the environment, and steps
//         // variable becomes available in our new thread
//         for step in 1..=*steps {
//             std::thread::sleep(std::time::Duration::from_secs(1));
//             println!("Thread step {}",step);
//         }
//         panic!("Something bad!");
//         "Goodbye!" // important thread could return values
//     });
//     println!("Spawned a thread!");

//     // Very important moment to understand closure captures
//     // the environment
    
//     //println!("steps now unavailable {}", steps);
//     std::thread::sleep(std::time::Duration::from_secs(3));
//     println!("Main thread slept for 3 seconds");
//     // Now we join our spawned thread with it's returned value, if we don't our function just returns
//     // without waiting for spawned thread
//     let result = thread.join().unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

//     println!("Thread returned: {:?}", result);
    
// }

// fn share_data_between_threads_with_arc() {
//     use std::sync::Arc; // atomic reference counter(smart pointer)
    
//     println!("Intro to Concurrency");
//     let steps =  Arc::new(5);
//     let thread = {
//         let steps = steps.clone();
//         std::thread::spawn(move ||{
//             // cannot mutate this variable (steps)
//             for step in 1..=*steps {
//                 std::thread::sleep(std::time::Duration::from_secs(1));
//                 println!("Thread step {}",step);
//             }
//             "Goodbye!" // important thread could return values
//         })
//     };

//     println!("Spawned a thread!");

//     // Very important moment to understand closure captures
//     // the environment
    
//     println!("steps now available {}", steps);
//     std::thread::sleep(std::time::Duration::from_secs(3));
//     println!("Main thread slept for 3 seconds");
//     // Now we join our spawned thread with it's returned value, if we don't our function just returns
//     // without waiting for spawned thread
//     let result = thread.join().unwrap(); // we need to unwrap result enum,because potentially thread could panick and we end up with err

//     println!("Thread returned: {:?}", result);
// }

// fn share_data_between_threads_and_mutate() {
//     use std::sync::Arc; // atomic reference counter(smart pointer)
//     use std::sync::Mutex; // mutex -> mutual exclusive
//     // smart pointer which guarantess that only one thread with lock
//     // acquired will be able to mutate the value inside
    
//     println!("Intro to Concurrency");
//     let steps =  Arc::new(Mutex::new(5));
//     // wrapping to make sure multiple threads are able to access that data
//     let thread = {
//         let steps = steps.clone();
//         std::thread::spawn(move ||{
//             while *steps.lock().unwrap() > 0{
//                 std::thread::sleep(std::time::Duration::from_secs(1));
//                 println!("Thread step {}",steps.lock().unwrap());
//                 *steps.lock().unwrap() -=1 ;
//                 // unlock() as soon as execution finishes
//             }
//             "Goodbye!" // important thread could return values
//         })
//     };

//     println!("Spawned a thread!");

//     // Very important moment to understand closure captures
//     // the environment
    
//     std::thread::sleep(std::time::Duration::from_secs(3));
//     println!("Main thread slept for 3 seconds");
    
//     let result = thread.join().unwrap(); 
//     println!("Thread returned: {:?}", result);
// }

// fn share_data_between_threads_through_channel() {
//     // multiple producer, single consumer
//     use std::sync::mpsc; 

//     println!("Concurrency");
//     let (sender,receiver) = mpsc::channel(); // notice tuple unpacking

//     let thread = {
//         std::thread::spawn(move ||{
//             let steps = receiver.recv().unwrap();
//             for step in 1..=steps {
//                 std::thread::sleep(std::time::Duration::from_secs(1));
//                 println!("Thread step {}",step);
//             }
//             "Goodbye!" // important thread could return values
//         })
//     };

//     println!("Spawned a thread!");
//     sender.send(5).unwrap();// if message did not reach reciever, you get err
//     std::thread::sleep(std::time::Duration::from_secs(3));
//     println!("Main thread slept for 3 seconds");
//     let result = thread.join().unwrap();
//     println!("Thread returned: {:?}", result);
// }
    
// fn fortune_cookie() {
//     extern crate rand;
//     use rand::Rng;
//     use std::thread;
//     // multiproducer, single consumer
//     use std::sync::mpsc::channel;
    
//     use std::time;

//     let ten_millis = time::Duration::from_millis(1000);
    
//     const DISCONNECT: &str = "Come back tomorrow!";
    
//     let (sender,reciever) = channel();
    
    
    
//     thread::spawn(move || {
//         let mut rng = rand::thread_rng();
//         loop {
            
//             let msg = match rng.gen_range(0..5)  {
//                 0 => "Fortune favors the brave.",
//                 1 => DISCONNECT,
//                 2 => "You will travel to many exotic places in your lifetime.",
//                 3 => "You can make your own happiness.",
//                 4 => "You are very talented in many ways.",
//                 _ => unreachable!(),
//             };
            
//             println!("Sending cookie: {}",msg);
//             //thread::sleep(ten_millis);
//             sender.send(msg).unwrap();
//             if msg == DISCONNECT {
//                 break;
//             }
//         }
//     });
    
//     for recieved_msg in reciever {
//         println!("What a day. Your fortune cookie : {}",recieved_msg);
//         thread::sleep(ten_millis);
        
//     }
    
// }

fn rw_locks() {
    // comparing with mutex which does not separate with reading and writing
    // to the data inside of mutex Read and Write lock, allows to separate that logic
    // like many readers and single writer, very close to RefCell.
    // but this idea is not new or unique to Rust, this idea existed before dawn
    // in Java and C++
    
    use std::sync::{Arc, RwLock};
    use std::thread;
    
    let data = Arc::new(RwLock::new("Hello World".to_string()));
    use std::time;
    let ten_millis = time::Duration::from_millis(10);
    let twenty_millis = time::Duration::from_millis(40);
    
    let reader_a = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_A: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let reader_b = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let data_for_read = data.read().unwrap();
                println!("Data from reader_B: {} ",data_for_read);
                thread::sleep(ten_millis);
            }
        })
    };
    
    let writer = {
        let data = data.clone();
        thread::spawn(move || {
            for _ in 0..10 {
                let mut data_to_write = data.write().unwrap();
                data_to_write.push('!');
                println!("Updating data {} ",data_to_write);
                thread::sleep(twenty_millis);
                
            }
        })
        };
        
    reader_a.join().unwrap();
    reader_b.join().unwrap();
    writer.join().unwrap();
    
    println!("{:?}",data);
    
}

fn main() {
    rw_locks();
}