use std::cmp::min;
use std::fmt;
use std::io::{Read, Write};
use std::{env, fs, path, process};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use lazy_static::lazy_static;
use serde_json::Value;

pub static VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";
lazy_static! {
    pub static ref MCSERVERS_DIR: String = match env::var("MCS_DIR") {
        Ok(var) => var,
        Err(_e) => default_mcservers_dir(),
    };
}

pub fn get_latest_version() -> Value {
    // download version manifest
    let body = reqwest::blocking::get(VERSION_MANIFEST_URL).unwrap();

    let manifest_json: Value = serde_json::from_str(&body.text().unwrap()).unwrap();
    manifest_json["latest"]["release"].clone()
}

pub fn default_mcservers_dir() -> String {
    let mcservers_dir: String = match home::home_dir() {
        Some(path) => format!("{}/.mcservers", path.display()),
        None => {
            eprintln!("Cannot find your home directory!");
            process::exit(1)
        }
    };

    if path::Path::new(&mcservers_dir).exists() {
        return mcservers_dir;
    }

    fs::create_dir(&mcservers_dir).unwrap();
    mcservers_dir
}

pub fn download_with_pb(url: &str, path: &String) {
    let mut response = reqwest::blocking::get(url).unwrap();
    let total_size: u64 = response.content_length().unwrap();
    let mut downloaded: u64 = 0;

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn fmt::Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));

    let mut file = fs::File::create(path).unwrap();

    loop {
        let mut buffer = [0; 1024];
        let bytes_read = response.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read]).unwrap();

        let new = min(downloaded + 1024, total_size);
        downloaded = new;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("downloaded");
}

pub fn fetch_server_jar_url(desired_version: &str) -> String {
    // download version manifest
    let body = reqwest::blocking::get(VERSION_MANIFEST_URL).unwrap();

    let manifest_json: Value = serde_json::from_str(&body.text().unwrap()).unwrap();
    let versions: &Value = &manifest_json["versions"];

    let array = versions.as_array().unwrap();

    for version in array {
        if version["id"] == desired_version {
            let version_url = version["url"].as_str().unwrap();
            let resp = reqwest::blocking::get(version_url).unwrap();

            let version_json: Value = serde_json::from_str(&resp.text().unwrap()).unwrap();

            return version_json["downloads"]["server"]["url"]
                .as_str()
                .unwrap()
                .to_string();
        };
    }

    eprintln!("Cannot find a server file for version {desired_version}!");
    process::exit(0);
}
