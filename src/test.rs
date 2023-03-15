use bfi::TestResults;
use quickcheck::TestResult;

use crate::{generate, models::Brainfuck, parse, solve, StackEffectDiagram};

fn test_brainfuck(code: &str, inputs: Vec<u8>, outputs: Vec<u8>) -> bool {
    match bfi::test_blocking(code, inputs, outputs, 10000) {
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

fn test_stackeffect(effect: &StackEffectDiagram) -> bool {
    if !effect.mapping.is_empty() {
        assert!(
            *effect.mapping.iter().max().unwrap() < effect.inputs,
            "Problem with test creation"
        );
    }

    // Solve the stack effect diagram
    let function = generate(solve(effect), Brainfuck::new());

    // Create a testing harness
    let reads: String = ">,".repeat(effect.inputs);
    let writes: String = ".<".repeat(effect.mapping.len());

    let bf = format!("{}{}{}", reads, function, writes);
    println!("Brainfuck: {}", bf);

    test_brainfuck(
        &bf,
        (0..effect.inputs).map(|i| i as u8).collect(),
        effect.mapping.iter().map(|&i| i as u8).collect(),
    )
}

/// Checks that the functions used in https://github.com/Alextopher/serotonin stdlib work
#[test]
fn serotonin() {
    // dup (a -- a a)
    println!("dup (a -- a a)");
    let dup = StackEffectDiagram {
        inputs: 1,
        mapping: vec![0, 0],
    };
    test_stackeffect(&dup);
    assert_eq!(parse("a -- a a"), Ok(dup));

    // dup2 (a b -- a b a b)
    println!("dup2 (a b -- a b a b)");
    let dup2 = StackEffectDiagram {
        inputs: 2,
        mapping: vec![0, 1, 0, 1],
    };
    test_stackeffect(&dup2);
    assert_eq!(parse("a b -- a b a b"), Ok(dup2));

    // drop (a --)
    println!("drop (a --)");
    let drop = StackEffectDiagram {
        inputs: 1,
        mapping: vec![],
    };
    test_stackeffect(&drop);
    assert_eq!(parse("a --"), Ok(drop));

    // drop2 (a b --)
    println!("drop2 (a b --)");
    let drop2 = StackEffectDiagram {
        inputs: 2,
        mapping: vec![],
    };
    test_stackeffect(&drop2);
    assert_eq!(parse("a b --"), Ok(drop2));

    // swap (a b -- b a)
    println!("swap (a b -- b a)");
    let swap = StackEffectDiagram {
        inputs: 2,
        mapping: vec![1, 0],
    };
    test_stackeffect(&swap);
    assert_eq!(parse("a b -- b a"), Ok(swap));

    // swap2 (a b c d -- c d a b)
    println!("swap2 (a b c d -- c d a b)");
    let swap2 = StackEffectDiagram {
        inputs: 4,
        mapping: vec![2, 3, 0, 1],
    };
    test_stackeffect(&swap2);
    assert_eq!(parse("a b c d -- c d a b"), Ok(swap2));

    // over (a b -- a b a)
    println!("over (a b -- a b a)");
    let over = StackEffectDiagram {
        inputs: 2,
        mapping: vec![0, 1, 0],
    };
    test_stackeffect(&over);
    assert_eq!(parse("a b -- a b a"), Ok(over));

    // over2 (a b c d -- a b c d a b)
    println!("over2 (a b c d -- a b c d a b)");
    let over2 = StackEffectDiagram {
        inputs: 4,
        mapping: vec![0, 1, 2, 3, 0, 1],
    };
    test_stackeffect(&over2);

    // rot (a b c -- b c a)
    println!("rot (a b c -- b c a)");
    let rot = StackEffectDiagram {
        inputs: 3,
        mapping: vec![1, 2, 0],
    };
    test_stackeffect(&rot);
    assert_eq!(parse("a b c -- b c a"), Ok(rot));

    // rot2 (a b c d e f -- c d e f a b)
    println!("rot2 (a b c d e f -- c d e f a b)");
    let rot2 = StackEffectDiagram {
        inputs: 6,
        mapping: vec![2, 3, 4, 5, 0, 1],
    };
    test_stackeffect(&rot2);
    assert_eq!(parse("a b c d e f -- c d e f a b"), Ok(rot2));

    // -rot (a b c -- c a b)
    println!("-rot (a b c -- c a b)");
    let minus_rot = StackEffectDiagram {
        inputs: 3,
        mapping: vec![2, 0, 1],
    };
    test_stackeffect(&minus_rot);
    assert_eq!(parse("a b c -- c a b"), Ok(minus_rot));

    // -rot2 (a b c d e f -- e f a b c d)
    println!("-rot2 (a b c d e f -- e f a b c d)");
    let minus_rot2 = StackEffectDiagram {
        inputs: 6,
        mapping: vec![4, 5, 0, 1, 2, 3],
    };
    test_stackeffect(&minus_rot2);
    assert_eq!(parse("a b c d e f -- e f a b c d"), Ok(minus_rot2));

    // nip (a b -- b)
    println!("nip (a b -- b)");
    let nip = StackEffectDiagram {
        inputs: 2,
        mapping: vec![1],
    };
    test_stackeffect(&nip);
    assert_eq!(parse("a b -- b"), Ok(nip));

    // nip2 (a b c d -- c d)
    println!("nip2 (a b c d -- c d)");
    let nip2 = StackEffectDiagram {
        inputs: 4,
        mapping: vec![2, 3],
    };
    test_stackeffect(&nip2);
    assert_eq!(parse("a b c d -- c d"), Ok(nip2));

    // tuck (a b -- b a b)
    println!("tuck (a b -- b a b)");
    let tuck = StackEffectDiagram {
        inputs: 2,
        mapping: vec![1, 0, 1],
    };
    test_stackeffect(&tuck);
    assert_eq!(parse("a b -- b a b"), Ok(tuck));

    // tuck2 (a b c d -- c d a b c d)
    println!("tuck2 (a b c d -- c d a b c d)");
    let tuck2 = StackEffectDiagram {
        inputs: 4,
        mapping: vec![2, 3, 0, 1, 2, 3],
    };
    test_stackeffect(&tuck2);
    assert_eq!(parse("a b c d -- c d a b c d"), Ok(tuck2));
}

/// it would be embarrassing if the README examples didn't work
#[test]
fn readme() {
    // (a b -- b a)
    println!("(a b -- b a)");
    let one = StackEffectDiagram {
        inputs: 2,
        mapping: vec![1, 0],
    };
    test_stackeffect(&one);
    assert_eq!(parse("a b -- b a"), Ok(one));

    // (a b c -- c a b)
    println!("(a b c -- c a b)");
    let two = StackEffectDiagram {
        inputs: 3,
        mapping: vec![2, 0, 1],
    };
    test_stackeffect(&two);
    assert_eq!(parse("a b c -- c a b"), Ok(two));

    // (a -- a a a a)
    println!("(a -- a a a a)");
    let three = StackEffectDiagram {
        inputs: 1,
        mapping: vec![0, 0, 0, 0],
    };
    test_stackeffect(&three);
    assert_eq!(parse("a -- a a a a"), Ok(three));

    // (a b c d -- d c a b)
    println!("(a b c d -- d c a b)");
    let four = StackEffectDiagram {
        inputs: 4,
        mapping: vec![3, 2, 0, 1],
    };
    test_stackeffect(&four);
    assert_eq!(parse("a b c d -- d c a b"), Ok(four));

    // (a b c -- c)
    println!("(a b c -- c)");
    let five = StackEffectDiagram {
        inputs: 3,
        mapping: vec![2],
    };
    test_stackeffect(&five);
    assert_eq!(parse("a b c -- c"), Ok(five));

    // (a b c d e f -- c d d f e e b)
    println!("(a b c d e f -- c d d f e e b)");
    let six = StackEffectDiagram {
        inputs: 6,
        mapping: vec![2, 3, 3, 5, 4, 4, 1],
    };
    test_stackeffect(&six);
    assert_eq!(parse("a b c d e f -- c d d f e e b"), Ok(six));
}

#[quickcheck]
fn quickcheck(i: u8, v: Vec<u8>) -> TestResult {
    if i == 0 || *v.iter().max().unwrap_or(&0) >= i {
        TestResult::discard()
    } else {
        println!("\nTest: {} {:?}", i, v);
        TestResult::from_bool(test_stackeffect(&StackEffectDiagram {
            inputs: i as usize,
            mapping: v.into_iter().map(|i| i as usize).collect(),
        }))
    }
}
