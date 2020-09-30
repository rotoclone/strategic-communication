mod operations;
#[cfg(feature = "llvm")]
mod codegen;
#[cfg(not(feature = "llvm"))]
mod interpreter;
mod lib;

use clap::Clap;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;

#[cfg(feature = "llvm")]
type Context<'ctx> = codegen::CodeGen<'ctx>;
#[cfg(not(feature = "llvm"))]
type Context<'ctx> = interpreter::Context<'ctx>;

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
            func: operations::label
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
pub struct Opts {
    /// Print the LLVM IR to the console
    #[clap(short('i'), long)]
    #[cfg(feature = "llvm")]
    print_ir: bool,
    /// View the control flow graph
    #[clap(long)]
    #[cfg(feature = "llvm")]
    view_cfg: bool,
    #[clap(short('O'), long, possible_values(&["0","1","2","3"]), default_value("2"))]
    #[cfg(feature = "llvm")]
    optimization_level: u8,
    /// The path to the file containing source code to execute
    file: String,
}

fn main() {
    env_logger::init();

    let opts = Opts::parse();

    let path = Path::new(&opts.file);
    let source = fs::read_to_string(path).expect("cannot open file");
    let source: Vec<String> = source
        .split('\n')
        .map(|line| line.trim().to_lowercase())
        .filter(|line| !line.is_empty())
        .collect();

    let program = Program::new(path.file_name().unwrap().to_str().unwrap().to_string(), source);
    match program {
        Err(e) => {
            eprintln!("error: {}", e);
        }
        Ok(p) => {
            if let Err(e) = Context::run(&p, &opts) {
                eprintln!("error: {}", e);
            }
        }
    }
}

/// An error during the compilation or execution of a program.
#[derive(Debug)]
pub struct Error {
    /// The 0-indexed line number the error occurred on.
    line_number: usize,
    /// A message describing the error.
    message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "line {}: {}",
            self.line_number + 1,
            self.message
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl Error {
    /// Creates a new `Error` with the provided message.
    fn new(message: &str, line_number: usize) -> Error {
        Error {
            line_number: line_number,
            message: message.to_string(),
        }
    }
}

/// Return type for operation execution functions.
type OpResult = Result<(), Error>;

/// An operation corresponding to a line of source code.
struct Operation {
    /// The regular expression to use to determine if a given line should cause this operation to be executed.
    pattern: Regex,
    /// The function that compiles this operation.
    func: fn(&str, usize, &mut Context) -> OpResult,
}

/// A representation of a program.
#[derive(Debug)]
pub struct Program {
    /// The name of the program
    name: String,
    /// The source code of the program, split by line.
    source: Vec<String>,
    /// Map of label names to the lines they are defined on.
    labels: HashMap<String, usize>,
}

impl Program {
    /// Creates a new `Program`.
    ///
    /// # Arguments
    /// * `source`: The source code of the program, split by line.
    fn new(name: String, source: Vec<String>) -> Result<Program, Error> {
        let labels = Program::find_labels(&source)?;
        Ok(Program {
            name,
            source,
            labels,
        })
    }

    /// Finds all the labels defined in the provided program.
    ///
    /// # Arguments
    /// * `source`: The source code of the program, split by line.
    ///
    /// Returns a map of label names to the lines they are defined on.
    fn find_labels(source: &[String]) -> Result<HashMap<String, usize>, Error> {
        let mut labels: HashMap<String, usize> = HashMap::new();
        for (line_number, line) in source.iter().enumerate() {
            if LABEL_PATTERN.is_match(line) {
                let label_name = LABEL_PATTERN.replace(line, "").to_string();
                match labels.get(&label_name) {
                    Some(other_line_number) => {
                        return Err(Error::new(
                            &format!("duplicate label “{}”, previously defined on line {}", label_name, other_line_number + 1), 
                            line_number));
                        }
                    None => {
                        labels.insert(label_name, line_number);
                    }
                }
            }
        }
        Ok(labels)
    }
}
