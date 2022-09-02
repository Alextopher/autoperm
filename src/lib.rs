use std::{cmp::Ordering, collections::HashMap};

use petgraph::prelude::*;

#[derive(Debug)]
pub enum Instruction {
    Clear(isize),
    Mov(isize, Vec<isize>),
}

impl Instruction {
    pub fn ptr(&self) -> isize {
        match self {
            Instruction::Clear(i) => *i,
            Instruction::Mov(i, _) => *i,
        }
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Clear(_) => write!(f, "[-]")?,
            Instruction::Mov(i, moves) => {
                write!(f, "[-")?;

                let mut ptr = i;
                for m in moves {
                    let diff = m - ptr;
                    write!(
                        f,
                        "{}+",
                        match diff.cmp(&0) {
                            Ordering::Less => "<".repeat(diff.unsigned_abs()),
                            Ordering::Equal => String::new(),
                            Ordering::Greater => ">".repeat(diff.unsigned_abs()),
                        }
                    )?;
                    ptr = m
                }

                let diff = i - ptr;
                write!(
                    f,
                    "{}",
                    match diff.cmp(&0) {
                        Ordering::Less => "<".repeat(diff.unsigned_abs()),
                        Ordering::Equal => String::new(),
                        Ordering::Greater => ">".repeat(diff.unsigned_abs()),
                    }
                )?;

                write!(f, "]")?;
            }
        }

        Ok(())
    }
}

pub fn auto_perm(stack_effect: &str) -> Result<String, String> {
    let mut iter = stack_effect.split("--");
    let pops = match iter.next() {
        Some(p) => p,
        None => {
            return Err(String::from(
                "stack effect is wrong. it should look like 'a b -- a b a b`",
            ));
        }
    };
    let pushes = match iter.next() {
        Some(p) => p,
        None => {
            return Err(String::from(
                "stack effect is wrong. it should look like 'a b -- a b a b`",
            ));
        }
    };

    if iter.next().is_some() {
        return Err(String::from(
            "stack effect is wrong. it should look like 'a b -- a b a b`",
        ));
    }

    // map symbols to their input postitions
    let mut symbols_to_positions = HashMap::new();
    for (i, symbol) in (0..).zip(pops.split_whitespace()) {
        // each symbol must only appear once in the input
        if let Some(pos) = symbols_to_positions.get(symbol) {
            return Err(format!(
                "symbol {} is defined twice. At postition {} and {}",
                symbol, i, pos
            ));
        } else {
            symbols_to_positions.insert(symbol, i);
        }
    }
    let input_size = symbols_to_positions.len() as isize;

    // the mapping of results to inputs is a function
    // ie:
    let mut mapping = vec![];
    for symbol in pushes.split_whitespace() {
        if let Some(pos) = symbols_to_positions.get(symbol) {
            mapping.push(*pos);
        } else {
            return Err(format!("symbol {} in the output is not defined", symbol));
        }
    }
    let output_size = mapping.len() as isize;

    // The index of the temporary variable. Place it above the last item
    let temp = mapping.len() as isize;

    let edges: Vec<_> = mapping.iter().enumerate().map(|(i, j)| (*j, i)).collect();

    let mut digraph: DiGraph<(), (), usize> = DiGraph::from_edges(edges);

    // if the number of outputs is less than the number of inputs those nodes need to be cleared
    if mapping.len() < symbols_to_positions.len() {
        (mapping.len()..symbols_to_positions.len()).for_each(|_| {
            digraph.add_node(());
        })
    }

    // Reversing the ouput of tarjan's strongly connected components creates the program
    let tarjan = petgraph::algo::tarjan_scc(&digraph);

    let mut instructions = vec![];

    for component in tarjan {
        if component.len() == 1 {
            // get the neighbors as a usize
            let neighbors = get_neighbors(&digraph, component[0]);
            let index = component[0].index() as isize;

            if neighbors.is_empty() {
                if index < input_size {
                    instructions.push(Instruction::Clear(index as isize));
                }
            } else if neighbors.contains(&index) {
                if neighbors.len() > 1 {
                    instructions.push(Instruction::Mov(index, vec![temp]));
                    instructions.push(Instruction::Mov(temp, neighbors));
                }
            } else {
                instructions.push(Instruction::Mov(index, neighbors));
            }
        } else {
            let mut iter = component.into_iter();

            let last_index = iter.next().unwrap();
            let last_neighbors = get_neighbors(&digraph, last_index);

            instructions.push(Instruction::Mov(last_index.index() as isize, vec![temp]));

            for node in iter {
                let neighbors = get_neighbors(&digraph, node);
                instructions.push(Instruction::Mov(node.index() as isize, neighbors));
            }

            instructions.push(Instruction::Mov(temp, last_neighbors));
        }
    }

    let mut result = String::new();
    let mut ptr = (input_size - 1) as isize;
    for i in instructions {
        let diff = i.ptr() - ptr;
        result += &match diff.cmp(&0) {
            Ordering::Less => "<".repeat(diff.unsigned_abs()),
            Ordering::Equal => String::new(),
            Ordering::Greater => ">".repeat(diff.unsigned_abs()),
        };

        result = format!("{}{}", result, i);
        ptr = i.ptr();
    }

    let diff = output_size - ptr - 1;
    result += &match diff.cmp(&0) {
        Ordering::Less => "<".repeat(diff.unsigned_abs()),
        Ordering::Equal => String::new(),
        Ordering::Greater => ">".repeat(diff.unsigned_abs()),
    };

    Ok(result)
}

fn get_neighbors<N, E, Ty, Ix>(graph: &Graph<N, E, Ty, Ix>, index: NodeIndex<Ix>) -> Vec<isize>
where
    Ty: petgraph::EdgeType,
    Ix: petgraph::adj::IndexType,
{
    graph.neighbors(index).map(|i| i.index() as isize).collect()
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod test;
