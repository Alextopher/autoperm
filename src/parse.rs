use std::collections::HashMap;

/// Represents a stack effect diagram.
///
/// # Example
///
/// ```
/// use autoperm::{parse, StackEffectDiagram};
///
/// let diagram = parse("a b -- b a").unwrap();
///
/// assert_eq!(diagram, StackEffectDiagram {
///     inputs: 2,
///     mapping: vec![1, 0],
/// });
/// ```

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct StackEffectDiagram {
    /// Mapping of output symbols to their starting positions
    pub mapping: Vec<usize>,
    /// The number of input symbols
    pub inputs: usize,
}

/// Returned when [`parse`](crate::parse::parse) fails
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ParseError {
    /// Returned where there is no "--" symbol in the input
    MissingDoubleDash,
    /// Returned where there is more than one "--" symbol in the input
    AdditionalDoubleDash,
    /// Returned when a symbol is defined twice.
    ///
    /// # Example
    ///
    /// ```
    /// use autoperm::{parse, ParseError};
    ///
    /// let diagram = parse("a b a -- a");
    ///
    /// assert_eq!(diagram, Err(ParseError::SymbolDefinedTwice{ symbol: "a".to_string(), first: 0, second: 2 }));
    /// ```
    SymbolDefinedTwice {
        /// The symbol that is defined twice
        symbol: String,
        /// The symbol number of the first occurrence
        first: usize,
        /// The symbol number of the second occurrence
        second: usize,
    },
    /// Returned when a symbol is used but not defined
    ///
    /// # Example
    ///
    /// ```
    /// use autoperm::{parse, ParseError};
    ///
    /// let diagram = parse("a b -- a c");
    ///
    /// assert_eq!(diagram, Err(ParseError::SymbolNotDefined{ symbol: "c".to_string(), id: 1 }));
    /// ```
    SymbolNotDefined {
        /// The symbol that is used but not defined
        symbol: String,
        /// The symbol number of the first occurrence
        id: usize,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingDoubleDash => write!(f, "Missing --"),
            ParseError::AdditionalDoubleDash => write!(f, "Additional --"),
            ParseError::SymbolDefinedTwice {
                symbol,
                first,
                second,
            } => write!(
                f,
                "Symbol {} defined twice at {} and {}",
                symbol, first, second
            ),
            ParseError::SymbolNotDefined { symbol, id } => {
                write!(f, "Symbol {} not defined at {}", symbol, id)
            }
        }
    }
}

/// A very simple parser for parsing [`StackEffectDiagram`](crate::StackEffectDiagram)s.
pub fn parse(stack_effect: &str) -> Result<StackEffectDiagram, ParseError> {
    let mut iter = stack_effect.split("--");

    let pops = match iter.next() {
        Some(p) => p,
        None => return Err(ParseError::MissingDoubleDash),
    };

    let pushes = match iter.next() {
        Some(p) => p,
        None => return Err(ParseError::MissingDoubleDash),
    };

    if iter.next().is_some() {
        return Err(ParseError::AdditionalDoubleDash);
    }

    // map symbols to their input postitions
    let mut symbols_to_positions = HashMap::new();
    for (i, symbol) in pops.split_whitespace().enumerate() {
        // each symbol must only appear once in the input
        if let Some(pos) = symbols_to_positions.get(symbol) {
            return Err(ParseError::SymbolDefinedTwice {
                symbol: symbol.to_string(),
                first: *pos,
                second: i,
            });
        } else {
            symbols_to_positions.insert(symbol, i);
        }
    }

    let input_size = symbols_to_positions.len();

    // map output symbols to their starting positions
    let mut mapping = Vec::with_capacity(pushes.len());
    for symbol in pushes.split_whitespace() {
        if let Some(pos) = symbols_to_positions.get(symbol) {
            mapping.push(*pos);
        } else {
            return Err(ParseError::SymbolNotDefined {
                symbol: symbol.to_string(),
                id: mapping.len(),
            });
        }
    }

    Ok(StackEffectDiagram {
        mapping,
        inputs: input_size,
    })
}
