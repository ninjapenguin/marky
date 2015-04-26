extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use rand::Rng;

fn main() {
    // Generate the markov chain by parsing the corpus

    let path = Path::new("/tmp/alice29.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that describes the error
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("File Read.."),
    };

    // `file` goes out of scope, and the "hello.txt" file gets closed

    println!("Generating chain");

    let mut chain = HashMap::new();

    let mut last: &str = "";
    let mut last_plus: &str = "";

    for word in s.split_whitespace() {

        if word.is_empty() {
            break;
        }

        if last.is_empty() == false && last_plus.is_empty() == false {
            let key = (last_plus, last);
            let val = match chain.entry(key) {
                Vacant(entry) => entry.insert(Vec::new()),
                Occupied(entry) => entry.into_mut(),
            };
            val.push(word);
        }
        last_plus = last;
        last = word;

    }

    println!("Generated");
    //println!("{}", format!("{:?}", chain.get(&(("in", "all"))).unwrap().get(0)));
    let mut rng = rand::thread_rng();
    //let ran = rng.gen_range(0,5);
    //println!("Random: {}", ran);

    // Run the chain
    let mut target = "Alice was".to_string();

    let mut sword_1 = "Alice";
    let mut sword_2 = "was";
    let mut counter: i32 = 0;
    loop {
        match chain.get(&(sword_1, sword_2)) {
            Some(ref v) => {

                // pick an entry from this vector
                let tot = v.len();
                let indeces = rng.gen_range(0,tot);
                let picked_option = v.get(indeces);
                let picked = picked_option.unwrap();

                target = format!("{} {}", target, picked);
                sword_1 = sword_2;
                sword_2 = picked;
            },
            None => {
                println!("Missed");
                break;
            }
        };

        counter += 1;
        if counter > 15 {
            break;
        }
    }


    println!("===========\n");
    println!("{}", target);

//    hm.insert(("one","two"), "three");
//    match hm.get(&(("in", "all"))) {
//        Some(&name) => println!("== {}", name),
//        _ => println!("Nope"),
//    };


//            match hm.get(&((last, last_plus))) {
//                Some(ref mut v) => v.push(word),
//                _ => {
//                    let mut v: Vec<&str> = Vec::new();
//                    v.push(word);
//                   hm.insert((last, last_plus),v);
//                }
//            };

}
