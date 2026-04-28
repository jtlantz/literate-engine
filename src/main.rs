use anyhow::Result;
use payment_processor::core::System;

fn main() -> Result<()> {
    let path = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("Missing input filepath"))?;
    let mut system = System::new();
    // process the file, note this can actually take multiple input csv's sequentially
    // to modify the same system if we wanted to do bulk file processing before
    // updating client records.
    let _ = system.process(path)?;

    // dumps client records to std out automatically
    let _ = system.export_records()?;

    Ok(())
}
