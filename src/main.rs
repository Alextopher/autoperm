use autoperm::auto_perm;
use itertools::Itertools;
use std::{env::args, process::exit};

fn main() {
    // if there are args treat them as the input
    let args = args().skip(1).join(" ");

    let input = if !args.is_empty() {
        args
    } else {
        // read in the stack effect diagram
        let mut input: String = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input
    };

    match auto_perm(&input) {
        Ok(a) => println!("{}", a),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
