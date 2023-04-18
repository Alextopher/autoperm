use crate::parse::StackEffectDiagram;
use petgraph::prelude::*;

/// Represents an instruction within the computation model
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum Instruction {
    /// Clear / Zero the cell at a given index
    Clear {
        /// The index of the cell to clear
        cell: isize,
    },
    /// Delete the data in a cell and copy it to a list of other cells
    Mov {
        /// The index of the cell to Mov
        cell: isize,
        /// The list of cells to copy the data to.
        ///
        /// Note: This list will never contain `cell`
        to: Vec<isize>,
    },
    /// Start the program with the assumption that the top of the stack is at a given index
    Start {
        /// The index of the top of the stack
        cell: isize,
    },
    /// Change the top of the stack to a given index
    Top {
        /// The index of the new top of the stack
        cell: isize,
    },
}

/// Given a [`StackEffectDiagram`](crate::parse::StackEffectDiagram) generate a list of instructions
/// to apply that diagram.
///
/// # Examples
///
/// ```
/// use autoperm::{solve, Instruction, StackEffectDiagram};
///
/// // a b -- b a
/// let diagram = StackEffectDiagram {
///     inputs: 2,
///     mapping: vec![1, 0],
/// };
///
/// let instructions = solve(&diagram);
///
/// assert_eq!(instructions, vec![
///     Instruction::Start { cell: 1},
///     Instruction::Mov { cell: 1, to: vec![2] },
///     Instruction::Mov { cell: 0, to: vec![1] },
///     Instruction::Mov { cell: 2, to: vec![0] },
///     Instruction::Top { cell: 1 },
/// ]);
/// ```
pub fn solve(diagram: &StackEffectDiagram) -> Vec<Instruction> {
    let mapping = &diagram.mapping;
    let inputs = diagram.inputs;

    // The index of the temporary variable. Place it above the highest item
    let temp = std::cmp::max(inputs, mapping.len()) as isize;

    let edges: Vec<_> = mapping.iter().enumerate().map(|(i, j)| (*j, i)).collect();

    let mut digraph: DiGraph<(), (), usize> = DiGraph::from_edges(edges);

    // if the number of outputs is less than the number of inputs those nodes need to be cleared
    if mapping.len() < inputs {
        (mapping.len()..inputs).for_each(|_| {
            digraph.add_node(());
        })
    }

    // Reversing the ouput of tarjan's strongly connected components creates the program
    let tarjan = petgraph::algo::tarjan_scc(&digraph);

    let mut instructions = vec![Instruction::Start {
        cell: inputs as isize - 1,
    }];

    for component in tarjan {
        if component.len() == 1 {
            // get the neighbors as a usize
            let neighbors = get_neighbors(&digraph, component[0]);
            let index = component[0].index() as isize;

            if neighbors.is_empty() {
                if index < inputs as isize {
                    instructions.push(Instruction::Clear { cell: index });
                }
            } else if neighbors.contains(&index) {
                if neighbors.len() > 1 {
                    instructions.push(Instruction::Mov {
                        cell: index,
                        to: vec![temp],
                    });
                    instructions.push(Instruction::Mov {
                        cell: temp,
                        to: neighbors,
                    });
                }
            } else {
                instructions.push(Instruction::Mov {
                    cell: index,
                    to: neighbors,
                });
            }
        } else {
            let mut iter = component.into_iter();

            let last_index = iter.next().unwrap();
            let last_neighbors = get_neighbors(&digraph, last_index);

            instructions.push(Instruction::Mov {
                cell: last_index.index() as isize,
                to: vec![temp],
            });

            for node in iter {
                let neighbors = get_neighbors(&digraph, node);
                instructions.push(Instruction::Mov {
                    cell: node.index() as isize,
                    to: neighbors,
                });
            }

            instructions.push(Instruction::Mov {
                cell: temp,
                to: last_neighbors,
            });
        }
    }

    instructions.push(Instruction::Top {
        cell: mapping.len() as isize - 1,
    });

    instructions
}

fn get_neighbors<N, E, Ty, Ix>(graph: &Graph<N, E, Ty, Ix>, index: NodeIndex<Ix>) -> Vec<isize>
where
    Ty: petgraph::EdgeType,
    Ix: petgraph::adj::IndexType,
{
    graph.neighbors(index).map(|i| i.index() as isize).collect()
}
