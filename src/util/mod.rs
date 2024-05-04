use std::{error::Error, path::Path, time::{SystemTime, UNIX_EPOCH}};
use scraper::{Html, Selector};
use self::context::Context;
use colored::Colorize;

pub mod lpm;
pub mod environment;
pub mod context;

pub static LUA_FTP_URL: &str = "https://www.lua.org/ftp";

#[derive(Clone)]
pub struct Version {
    pub name: String,
    pub url: String
}

pub fn join_path(origin: &str, piece: &str) -> String {
    let path = Path::new(origin);
    let path = path.join(piece);
    return String::from(path.to_str().expect("Failed to convert path to string"));
}

pub fn get_current_millis() -> u128 {
    let sys_time = SystemTime::now();
    let since_the_epoch = sys_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    return since_the_epoch.as_millis();
}

pub async fn download_lua_version_list(context: Context) -> Result<Vec<Version>, Box<dyn Error>> {
    let mut lua_versions_updated_at = context.lua_versions_updated_at.lock().await;
    let last_updated_at = lua_versions_updated_at.get();

    if get_current_millis() - last_updated_at < 60000 {
        let versions = context.lua_versions.lock().await;
        let mut tmp : Vec<Version> = Vec::new();

        for version in versions.iter() {
            tmp.push(version.clone());
        }

        drop(versions);
        return Ok(tmp);
    }

    let mut versions = context.lua_versions.lock().await;
    versions.clear();

    println!("{} Downloading lua version list", ">".bright_green());

    let res = reqwest::get(LUA_FTP_URL).await?;

    if res.status() != 200 {
        let error = res.error_for_status().unwrap_err();
        return Err(Box::new(error));
    }

    let body = res.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("body > table:first-of-type > tbody > tr > td.name > a").unwrap();

    for element in document.select(&selector) {
        let version = Version {
            name: element.inner_html().replace(".tar.gz", ""),
            url: format!("{}/{}", LUA_FTP_URL, element.attr("href").expect("Couldn't find href attribute for version").to_string())
        };
        versions.push(version);
    }

    lua_versions_updated_at.set(get_current_millis());

    Ok(versions.clone())
}