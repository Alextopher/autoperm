use autoperm::autoperm_bf;
use itertools::Itertools;
use std::{env::args, process::exit};

fn main() {
    // if there are args treat them as the input
    let args = args().skip(1).join(" ");

    if !args.is_empty() {
        match autoperm_bf(&args) {
            Ok(a) => println!("{}", a),
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    } else {
        // If there are no args run a REPL for interactive use
        loop {
            // read in the stack effect diagram
            let mut input: String = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => match autoperm_bf(&input) {
                    Ok(program) => println!("{}\n", program),
                    Err(e) => {
                        eprintln!("{}\n", e);
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
