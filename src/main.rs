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

lazy_static! {
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
    static ref LABEL_PATTERN: Regex = Regex::new("^(moving|going) forward, ").unwrap();
    static ref OPERATIONS: [Operation; 10] = [
        Operation {
            pattern: LABEL_PATTERN.clone(),
            func: operations::no_op
        },
        Operation {
            pattern: Regex::new("^innovate ").unwrap(),
            func: operations::increment
        },
        Operation {
            pattern: Regex::new("^streamline ").unwrap(),
            func: operations::decrement
        },
        Operation {
            pattern: Regex::new("^revamp ").unwrap(),
            func: operations::negate
        },
        Operation {
            pattern: Regex::new("^amplify ").unwrap(),
            func: operations::double
        },
        Operation {
            pattern: Regex::new("^backburner ").unwrap(),
            func: operations::halve
        },
        Operation {
            pattern: Regex::new("^overhaul ").unwrap(),
            func: operations::zero
        },
        Operation {
            pattern: Regex::new("^align ").unwrap(),
            func: operations::assign
        },
        Operation {
            pattern: Regex::new("^deliver ").unwrap(),
            func: operations::print
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
#[clap(version = "0.1.0", author = "Steven Goldberg")]
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
        .collect();

    if let Err(e) = run(source) {
        eprintln!("runtime error: {}", e);
    }
}

fn run(source: Vec<String>) -> Result<(), RuntimeError> {
    let mut context = Context::new(source);
    debug!("created context: {:?}", context);
    while context.current_line_number < context.source.len() {
        context.execute_current_line()?;
    }
    Ok(())
}

#[derive(Debug)]
pub struct RuntimeError {
    line_number: usize,
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
    fn new(message: &str, context: &Context) -> RuntimeError {
        RuntimeError {
            line_number: context.current_line_number,
            message: message.to_string(),
        }
    }
}

type OpResult = Result<(), RuntimeError>;

struct Operation {
    pattern: Regex,
    func: fn(&str, &mut Context) -> OpResult,
}

#[derive(Debug)]
pub struct Context {
    source: Vec<String>,
    registers: HashMap<String, i32>,
    labels: HashMap<String, usize>,
    current_line_number: usize,
}

impl Context {
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
