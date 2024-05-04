use std::error::Error;
use clap::Parser;
use crate::util::download_lua_version_list;

#[derive(Parser)]
#[command(about, long_about = None)]
struct Cli {
    /// Install a lua package
    #[arg(short, long, value_name = "PACKAGE")]
    install: Option<String>,

    /// List lua versions
    #[arg(short, long)]
    version_list: bool,
    
}
pub async fn start_cli() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if let Some(package) = cli.install.as_deref() {
        println!("Installing: {}", package);
    }

    if cli.version_list {
        for version in download_lua_version_list().await? {
            println!("{}", version.name);
        }
    }
    Ok(())
}