use std::{error::Error, fs::File, io::Cursor, io::copy};
use clap::Parser;
use colored::Colorize;
use crate::util::{context::Context, download_lua_version_list, join_path};


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
pub async fn start_cli(context: Context) -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    if cli.version_list {
        for version in download_lua_version_list(context.clone()).await? {
            println!("{} {}{}", ">".bright_green(), "@".bright_blue(), version.name.bright_blue());
        }
        println!("{} use '{} {}' to install {}", ">".bright_green(), "lpm -i".green(), "@version".bright_blue(), "lua".bright_blue());
    }

    if let Some(package) = cli.install.as_deref() {
        if package.starts_with("@lua-") {
            let package = &package[1..];
            let mut url = String::new();

            for version in download_lua_version_list(context.clone()).await? {
                if version.name == package {
                    url = version.url;
                }
            }

            if url == "" {
                println!("{} Invalid lua version {}", ">".bright_green(), package.bright_blue());
            } else {
                println!("{} Downloading from {}",  ">".bright_green(), url.yellow());
                let downloads_dir = context.lpm_downloads_dir.lock().await;

                let mut dest = File::create(join_path(downloads_dir.get().as_str(), format!("{}.tar.gz", package).as_str()))?;
                let response = reqwest::get(url).await?;
                let mut content =  Cursor::new(response.bytes().await?);
                copy(&mut content, &mut dest)?;
                println!("{} Download completed",  ">".bright_green());
            }
        } else {
            // installing lua package
            println!("{} Installing package {}", ">".bright_green(), package.yellow())
        }
    }

    Ok(())
}