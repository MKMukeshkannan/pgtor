use std::error::Error;
use app::main::init;

mod app;
mod components;

fn main() -> Result<(), Box<dyn Error>> {
    init()?;
    Ok(())
}
