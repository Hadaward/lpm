use core::{install::{first_run, uninstall}, is_first_run};
use std::error::Error;

// use context::Context;

pub mod core;
pub mod context;
pub mod pathutil;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let context = Context::new(); [TODO - use to share context over most methods]

    if is_first_run().await? {
        match first_run().await {
            Ok(_) => {
                println!("Successfully installed!");
            },
            Err(error) => {
                uninstall().await?;
                println!("Unable to install LPM due to some error.");
                return Err(error);
            }
        }
    }

    Ok(())
}