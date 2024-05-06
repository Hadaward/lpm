use core::{install::first_run, is_first_run};
use std::error::Error;

pub mod core;
pub mod pathutil;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if is_first_run().await? {
        first_run().await?;
    }

    Ok(())
}