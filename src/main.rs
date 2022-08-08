mod multi_modules_example;
use multi_modules_example::multi_modules;
use anyhow::{Result, Ok};

fn main() -> Result<()> {
    multi_modules()?;

    Ok(())
}
