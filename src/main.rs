use std::error::Error;

use cli::start_cli;
use util::lpm::check_lpm_install;

pub mod util;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    check_lpm_install().await?;
    start_cli().await?;

    Ok(())
}