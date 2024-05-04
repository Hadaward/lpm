use std::error::Error;
use scraper::{Html, Selector};
pub mod lpm;
pub mod environment;

pub static LUA_FTP_URL: &str = "https://www.lua.org/ftp";

pub struct Version {
    pub name: String,
    pub url: String
}

pub async fn download_lua_version_list() -> Result<Vec<Version>, Box<dyn Error>> {
    let mut versions: Vec<Version> = Vec::new();
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
    Ok(versions)
}