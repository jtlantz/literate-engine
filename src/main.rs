use anyhow::Result;
use payment_processor::core::System;

fn main() -> Result<()> {
    let _system = System::new();

    Ok(())
}
