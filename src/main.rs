mod operations;

use clap::Clap;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;

const REGISTER_NAMES: [&str; 8] = [
    "customer experience",
    "revenue streams",
    "core competencies",
    "best practices",
    "stakeholder engagement",
    "key performance indicators",
    "return on investment",
    "assets",
];

/// Strings that can be placed between operands.
const OPERAND_CONNECTORS: [&str; 3] = [" and ", " with ", " to "];

/// Strings that can be placed between literals.
const LITERAL_CONNECTORS: [&str; 3] = [", and ", " and ", ", "];

lazy_static! {
    /// Map of literals to the values they represent.
    static ref LITERALS: HashMap<String, u8> = {
        let mut map = HashMap::new();
        map.insert("hr".to_string(), 0);
        map.insert("engineering".to_string(), 1);
        map.insert("legal".to_string(), 2);
        map.insert("pr".to_string(), 3);
        map.insert("finance".to_string(), 4);
        map.insert("marketing".to_string(), 5);
        map.insert("r&d".to_string(), 6);
        map.insert("sales".to_string(), 7);
        map.insert("manufacturing".to_string(), 8);
        map.insert("executive management".to_string(), 9);
        map
    };
    /// The pattern for lines that define a label.
    static ref LABEL_PATTERN: Regex = Regex::new("^(moving|going) forward, ").unwrap();
    static ref OPERATIONS: [Operation; 15] = [
        Operation {
            pattern: LABEL_PATTERN.clone(),
            func: operations::no_op
        },
        Operation {
            pattern: Regex::new("^(innovate|value-add) ").unwrap(),
            func: operations::increment
        },
        Operation {
            pattern: Regex::new("^(streamline|optimize) ").unwrap(),
            func: operations::decrement
        },
        Operation {
            pattern: Regex::new("^(revamp|overhaul) ").unwrap(),
            func: operations::negate
        },
        Operation {
            pattern: Regex::new("^(amplify|incentivize) ").unwrap(),
            func: operations::double
        },
        Operation {
            pattern: Regex::new("^backburner ").unwrap(),
            func: operations::halve
        },
        Operation {
            pattern: Regex::new("^paradigm shift ").unwrap(),
            func: operations::randomize
        },
        Operation {
            pattern: Regex::new("^align ").unwrap(),
            func: operations::assign
        },
        Operation {
            pattern: Regex::new("^(synergize|integrate) ").unwrap(),
            func: operations::add
        },
        Operation {
            pattern: Regex::new("^differentiate ").unwrap(),
            func: operations::subtract
        },
        Operation {
            pattern: Regex::new("^crowdsource ").unwrap(),
            func: operations::read
        },
        Operation {
            pattern: Regex::new("^(deliver|produce) ").unwrap(),
            func: operations::print
        },
        Operation {
            pattern: Regex::new("^(circle back to|revisit) ").unwrap(),
            func: operations::jump
        },
        Operation {
            pattern: Regex::new("^pivot ").unwrap(),
            func: operations::jump_if_zero
        },
        Operation {
            pattern: Regex::new("^restructure ").unwrap(),
            func: operations::jump_if_neg
        },
    ];
}

/// Interpreter for the programming language Strategic Communication.
/// More information can be found at https://github.com/rotoclone/strategic-communication/blob/master/README.md
#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Opts {
    /// The path to the file containing source code to execute
    file: String,
}

fn main() {
    env_logger::init();

    let opts = Opts::parse();

    let source = fs::read_to_string(opts.file).expect("cannot open file");
    let source: Vec<String> = source
        .split('\n')
        .map(|line| line.trim().to_lowercase())
        .filter(|line| !line.is_empty())
        .collect();

    if let Err(e) = run(source) {
        eprintln!("runtime error: {}", e);
    }
}

/// Runs a program.
///
/// # Arguments
/// * `source`: The source code of the program to run, split by line.
///
/// Returns `Err(RuntimeError)` if any errors occurred during the execution of the program.
fn run(source: Vec<String>) -> Result<(), RuntimeError> {
    let mut context = Context::new(source);
    debug!("created context: {:?}", context);
    while context.current_line_number < context.source.len() {
        context.execute_current_line()?;
    }
    Ok(())
}

/// An error during the execution of a program.
#[derive(Debug)]
pub struct RuntimeError {
    /// The 0-indexed line number the error occurred on.
    line_number: usize,
    /// A message describing the error.
    message: String,
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "error on line {}: {}",
            self.line_number + 1,
            self.message
        )
    }
}

impl RuntimeError {
    /// Creates a new `RuntimeError` with the provided message.
    fn new(message: &str, context: &Context) -> RuntimeError {
        RuntimeError {
            line_number: context.current_line_number,
            message: message.to_string(),
        }
    }
}

/// Return type for operation execution functions.
type OpResult = Result<(), RuntimeError>;

/// An operation corresponding to a line of source code.
struct Operation {
    /// The regular expression to use to determine if a given line should cause this operation to be executed.
    pattern: Regex,
    /// The function that executes this operation.
    func: fn(&str, &mut Context) -> OpResult,
}

/// A representation of the state of "memory" during the execution of a program.
#[derive(Debug)]
pub struct Context {
    /// The source code of the program, split by line.
    source: Vec<String>,
    /// Map of register names to their current values.
    registers: HashMap<String, i32>,
    /// Map of label names to the lines they are defined on.
    labels: HashMap<String, usize>,
    /// The 0-indexed line number currently being executed.
    current_line_number: usize,
}

impl Context {
    /// Creates a new `Context` for a program.
    ///
    /// # Arguments
    /// * `source`: The source code of the program, split by line.
    fn new(source: Vec<String>) -> Context {
        let labels = Context::find_labels(&source);
        Context {
            source,
            registers: REGISTER_NAMES
                .iter()
                .map(|name| (name.to_string(), 0))
                .collect(),
            labels,
            current_line_number: 0,
        }
    }

    /// Finds all the labels defined in the provided program.
    ///
    /// # Arguments
    /// * `source`: The source code of the program, split by line.
    ///
    /// Returns a map of label names to the lines they are defined on.
    fn find_labels(source: &[String]) -> HashMap<String, usize> {
        let mut labels: HashMap<String, usize> = HashMap::new();
        for (line_number, line) in source.iter().enumerate() {
            if LABEL_PATTERN.is_match(line) {
                let label_name = LABEL_PATTERN.replace(line, "").to_string();
                labels.insert(label_name, line_number);
            }
        }
        labels
    }

    /// Executes the line at `source[current_line_number]` and sets `current_line_number` to the index of the next line to execute.
    fn execute_current_line(&mut self) -> Result<(), RuntimeError> {
        if self.current_line_number >= self.source.len() {
            return Err(RuntimeError::new("invalid line number", self));
        }

        let line = &self.source[self.current_line_number];
        debug!("executing line {}: {}", self.current_line_number, line);

        for op in OPERATIONS.iter() {
            if op.pattern.is_match(line) {
                let operands = op.pattern.replace(&line, "").to_string();
                trace!("registers before: {:?}", self.registers);
                (op.func)(&operands, self)?;
                trace!("registers after: {:?}", self.registers);
                self.current_line_number += 1;
                return Ok(());
            }
        }

        Err(RuntimeError::new("unexpected expression", self))
    }
}
