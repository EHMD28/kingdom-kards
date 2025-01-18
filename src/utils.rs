use std::fmt::Display;

/// Clears terminal screen.
pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

/// Checks if two enum variants are the same without checking actual equality.
pub fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

/// Prints to `stderr` in format `An error occured in {fn_name}(): {err}`
pub fn perror_in_fn<T: Display>(fn_name: &str, err: T) {
    eprintln!("An error occured in {fn_name}(): {err}");
}
