use autoperm::auto_perm;
use std::process::exit;

fn main() {
    // read in the stack effect diagram
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match auto_perm(&input) {
        Ok(a) => println!("{}", a),
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
