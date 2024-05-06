use std::error::Error;

pub mod core;
pub mod pathutil;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    Ok(())
}