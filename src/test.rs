use bfi::TestResults;
use quickcheck::TestResult;

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

fn test_perm(inputs: usize, perm: Vec<usize>) -> bool {
    if !perm.is_empty() {
        assert!(*perm.iter().max().unwrap() < inputs);
    }

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
    println!("{}", input);

    let reads: String = ">,".repeat(inputs);
    let writes: String = ".<".repeat(outputs);

    let bf = format!("{}{}{}", reads, super::auto_perm(&input).unwrap(), writes);
    println!("{}", bf);

    multiple_test(
        &bf,
        vec![(0..inputs).map(|i| i as u8).collect()],
        vec![perm.into_iter().map(|i| i as u8).collect()],
    )
}

/// checks that the stack functions used in serotonin work
#[test]
fn serotonin() {
    // dup
    test_perm(1, vec![0, 0]);
    test_perm(2, vec![0, 1, 0, 1]);
    // drop
    test_perm(1, vec![]);
    test_perm(2, vec![]);
    // swap
    test_perm(2, vec![1, 0]);
    test_perm(4, vec![2, 3, 0, 1]);
    // over
    test_perm(2, vec![0, 1, 0]);
    test_perm(4, vec![0, 1, 2, 3, 0, 1]);
    // rot
    test_perm(3, vec![1, 2, 0]);
    test_perm(6, vec![2, 3, 4, 5, 0, 1]);
    // -rot
    test_perm(3, vec![2, 0, 1]);
    test_perm(6, vec![4, 5, 0, 1, 2, 3]);
    // nip
    test_perm(2, vec![1]);
    test_perm(4, vec![2, 3]);
    // tuck
    test_perm(2, vec![1, 0, 1]);
    test_perm(4, vec![2, 3, 0, 1, 2, 3]);
}

/// it would be embarrassing if the README examples didn't work
#[test]
fn readme() {
    test_perm(2, vec![1, 0]);
    test_perm(3, vec![2, 0, 1]);
    test_perm(1, vec![0, 0, 0, 0]);
    test_perm(4, vec![3, 2, 0, 1]);
    test_perm(3, vec![2]);
    test_perm(6, vec![2, 3, 3, 5, 4, 4, 1]);
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
