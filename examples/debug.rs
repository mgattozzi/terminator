use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    fs::read_to_string("path/does/not/exist")?;
    Ok(())
}
