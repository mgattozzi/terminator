extern crate terminator;
use terminator::Terminator;
use std::fs;

fn main() -> Result<(), Terminator> {
    fs::read_to_string("path/does/not/exist")?;
    Ok(())
}
