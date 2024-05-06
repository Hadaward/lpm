use std::error::Error;

use cli::start_cli;
use util::{context::Context, join_path, lpm::check_lpm_install};

pub mod util;
pub mod cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new();
    
    let mut lpm_home = context.lpm_home.lock().await;
    lpm_home.set(check_lpm_install().await?);

    let mut lpm_downloads_dir = context.lpm_downloads_dir.lock().await;
    lpm_downloads_dir.set(join_path(lpm_home.get().as_str(), "downloads"));
    
    drop(lpm_home);
    drop(lpm_downloads_dir);
    
    start_cli(context.clone()).await?;

    Ok(())
}