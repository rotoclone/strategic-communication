use rand::Rng;
use std::io::Write;

/// Functions called by Strategic Communication programs

#[no_mangle]
pub extern fn print_value(to_print: i32) {
    if to_print < 0 {
        panic!(
                "{} does not correspond to a valid UTF-8 character",
                to_print
            );
    }

    match std::char::from_u32(to_print as u32) {
        Some(c) => {
            print!("{}", c);
            std::io::stdout().flush().unwrap();
        }
        _ => {
            panic!(
                "{} does not correspond to a valid UTF-8 character",
                to_print
            );
        }
    }
}

#[no_mangle]
/// Returns a random number between 0 and 9 (inclusive).
pub extern fn randomize() -> i32 {
    rand::thread_rng().gen_range(0, 10)
}

// Adding the functions above to static,
// so Rust compiler won't remove them.
#[used]
static PRINT_VALUE_FUNC: extern "C" fn(i32) = print_value;

#[used]
static RANDOMIZE_FUNC: extern "C" fn() -> i32 = randomize;
