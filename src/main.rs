mod basic_example;
use basic_example::basic;
mod multi_modules_example;
use multi_modules_example::multi_modules;
mod linking_example;
use linking_example::linking_modules;
mod hostcall_example;
use hostcall_example::hostcall;
use anyhow::{Result, Ok};

fn main() -> Result<()> {
    basic()?;

    multi_modules()?;

    linking_modules()?;

    hostcall()?;

    Ok(())
}
