use autoperm::auto_perm;
use itertools::Itertools;
use std::{env::args, process::exit};

fn main() {
    // if there are args treat them as the input
    let args = args().skip(1).join(" ");

    if !args.is_empty() {
        match auto_perm(&args) {
            Ok(a) => println!("{}", a),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    } else {
        loop {
            // read in the stack effect diagram
            let mut input: String = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => match auto_perm(&input) {
                    Ok(program) => println!("{}\n", program),
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("{}", e);
                    exit(1);
                }
            }
        }
    };
}
