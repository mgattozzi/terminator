use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    fs::read_to_string("path/does/not/exist")?;
    Ok(())
}
