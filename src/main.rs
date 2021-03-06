// Test script to generate a markov chain from Alice in Wonderland using Rust
//
// Marcov Chain takes form of
//
//   {
//       (&str, &str) => Vec<&str>,
//       ("word1", "word2") => ["word", "word", "word"...]
//       ...
//   }
//
//  matt <matt@ninjapenguin.co.uk>
extern crate rand;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

use rand::Rng;

fn parse_file(file_path: &str) -> String {
    let path = Path::new(file_path);
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

    s
}

fn build_chain<'a>(corpus: &'a str) -> HashMap<(&'a str, &'a str), Vec<&'a str>> {
    let mut chain: HashMap<(&str, &str), Vec<&str>> = HashMap::new();

    let mut last: &str = "";
    let mut last_plus: &str = "";

    for word in corpus.split_whitespace() {

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
    chain
}

fn run_chain<'a>(chain: HashMap<(&str, &str), Vec<&str>>, config: Config) -> String {

    let mut rng = rand::thread_rng();

    let mut sword_1 = &*config.first_seed;
    let mut sword_2 = &*config.second_seed;
    let mut target = format!("{} {}", sword_1, sword_2);

    let mut counter: i32 = 0;
    loop {
        match chain.get(&(sword_1, sword_2)) {
            Some(ref v) => {

                // pick an entry from this vector
                let tot = v.len();
                let indeces = rng.gen_range(0,tot);
                let picked_option = v.get(indeces);
                let picked = picked_option.unwrap();

                target = target + " " + picked;
                sword_1 = sword_2;
                sword_2 = picked;

                // if the current state ends with punctuation, terminate
                if picked.contains(".") || picked.contains("!") {
                    println!("Breaking on punctuation");
                    break;
                }
            },
            None => {
                println!("Missed");
                break;
            }
        };

        counter += 1;
        if counter > 50 {
            break;
        }
    }
    target
}

fn main() {

    let args: Vec<String> = env::args().collect();
    //let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Generate the markov chain by parsing the corpus
    let s = parse_file(&config.filename);

    println!("Generating chain from {}", config.filename);
    let chain = build_chain(s.as_str());
    println!("Markov Chain Generated");

    // Run the chain
    let target = run_chain(chain, config);

    println!("===========\n");
    println!("{}", target);

}


struct Config {
    filename: String,
    first_seed: String,
    second_seed: String
}

impl Config {

    fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 4 {
            return Err("Not enough arguments")
        }

        let filename = args[1].clone();
        let first_seed = args[2].clone();
        let second_seed = args[3].clone();

        Ok(Config { filename, first_seed, second_seed } )
    }
}