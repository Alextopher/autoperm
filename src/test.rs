use bfi::TestResults;
use quickcheck::TestResult;
use rand::prelude::*;
use std::collections::HashMap;

fn multiple_test(code: &str, inputs: Vec<Vec<u8>>, outputs: Vec<Vec<u8>>) -> bool {
    assert!(inputs.len() == outputs.len());

    match bfi::tests_blocking(code, inputs.into_iter(), outputs.into_iter(), 1000) {
        TestResults::OutputsDontMatchInputs => false,
        TestResults::ParseError(err) => {
            eprintln!("{:?}", err);
            false
        }
        TestResults::Results(results) => {
            let mut failure = false;
            for (_i, result) in results.iter().enumerate() {
                match result {
                    bfi::TestResult::Ok => {}
                    bfi::TestResult::RunTimeError(e) => {
                        eprintln!("{:?}", e);
                        failure = true;
                    }
                    bfi::TestResult::UnexpectedOutput { expected, output } => {
                        eprintln!("Left: {:?}\nRight: {:?}", expected, output);
                    }
                }
            }
            if failure {
                eprintln!("One or more test failures");
                false
            } else {
                true
            }
        }
    }
}

const TRYS: usize = 100;

fn test_perm(inputs: usize, perm: Vec<usize>) -> bool {
    let input = format!(
        "{} -- {}",
        (0..inputs)
            .map(|i| i.to_string())
            .fold(String::new(), |a, b| format!("{} {}", a, b)),
        perm.iter()
            .map(|i| i.to_string())
            .fold(String::new(), |a, b| format!("{} {}", a, b))
    );
    let outputs = perm.len();

    let reads: String = ">,".repeat(inputs);
    let writes: String = ".<".repeat(outputs);

    let bf = format!("{}{}{}", reads, super::auto_perm(&input).unwrap(), writes);
    println!("{}", bf);

    let mut rng = rand::thread_rng();
    let mut bf_inputs: Vec<Vec<u8>> = Vec::with_capacity(TRYS);
    let mut bf_outputs: Vec<Vec<u8>> = Vec::with_capacity(TRYS);
    for _ in 0..TRYS {
        let v: Vec<u8> = (0..inputs).map(|_| rng.gen()).collect();
        //println!("{:?}", v);
        bf_inputs.push(v.clone());
        let map: HashMap<usize, u8> = (0..inputs).zip(v).collect();
        let out = perm.iter().map(|i| *map.get(i).unwrap()).rev().collect();
        //println!("{:?}", out);
        bf_outputs.push(out);
    }

    multiple_test(&bf, bf_inputs, bf_outputs)
}

#[test]
fn swap() {
    test_perm(5, vec![2, 3, 3, 0, 0, 4, 1]);
    test_perm(4, vec![0, 1]);
}

#[quickcheck]
fn quickcheck(i: u8, v: Vec<u8>) -> TestResult {
    if i == 0 || *v.iter().max().unwrap_or(&0) >= i {
        TestResult::discard()
    } else {
        println!("{} {:?}", i, v);
        TestResult::from_bool(test_perm(
            i.into(),
            v.into_iter().map(|i| i.into()).collect(),
        ))
    }
}
