use std::io::{self, Write};
use std::process::{self, Command};
use std::{fs, path};

use serde_json::Value;

use crate::utils::{fetch_server_jar_url, get_latest_version, download_with_pb};
use crate::utils::{MCSERVERS_DIR, VERSION_MANIFEST_URL};

pub fn create(name: &String, version: &str) {
    let server_folder = format!("{}/{}", *MCSERVERS_DIR, name);

    if path::Path::new(&server_folder).exists() {
        eprintln!("Server {name} already exists!");
        process::exit(0);
    }

    let latest = &get_latest_version();
    let server_version = if version == "latest" {
        latest.as_str().unwrap()
    } else {
        version
    };

    let server_jar_link: String = fetch_server_jar_url(server_version);
    let server_jar_path = format!("{server_folder}/server.jar");
    fs::create_dir(&server_folder).unwrap();

    download_with_pb(&server_jar_link, &server_jar_path);

    println!(
        "Created server '{name}' on version {server_version}! Launch it with `mcs launch {name}`."
    )
}

pub fn launch(name: &String) {
    let server_folder = format!("{}/{}", *MCSERVERS_DIR, name);

    if !path::Path::new(&server_folder).exists() {
        eprintln!("Server {name} does not exist!");
        process::exit(0);
    }

    let eula_path = format!("{server_folder}/eula.txt");
    if !path::Path::new(&eula_path).exists() {
        println!("To start this Minecraft server, you need to agree to the Minecraft EULA");
        println!("For more info, visit https://aka.ms/MinecraftEULA");

        let mut eula_agree = String::new();
        print!("Do you agree to the EULA? (y/n) ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut eula_agree).unwrap();

        if eula_agree == "y\n" {
            // Mimic normal EULA
            let mut eula_content = String::new();
            eula_content.push_str("#By changing the setting below to TRUE ");
            eula_content.push_str("you are indicating your agreement to ");
            eula_content.push_str("our EULA (https://aka.ms/MinecraftEULA).\n");

            let now = chrono::Utc::now();
            let dt_str = now.format("#%a %b %d %H:%M:%S %Z %Y\n").to_string();
            eula_content.push_str(&dt_str);

            eula_content.push_str("eula=true\n");

            fs::write(&eula_path, eula_content).unwrap();
        } else {
            println!("You did not agree to the EULA; exiting");
            process::exit(0);
        }
    }

    let server_jar_path = format!("{server_folder}/server.jar");
    Command::new("java")
        .arg("-jar")
        .arg(&server_jar_path)
        .arg("nogui")
        .current_dir(server_folder)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

pub fn list() {
    let mcservers_dir = (*MCSERVERS_DIR).to_string();

    let entries = fs::read_dir(mcservers_dir.clone()).unwrap();
    let count = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().is_dir())
        .count();
    // no servers; folder has no subfolders
    if count == 0 {
        println!("You have no servers.");
        return;
    }

    let folders = fs::read_dir(mcservers_dir).unwrap();

    println!("Servers:");
    for folder in folders {
        let folder_name = folder.unwrap();
        let folder_name = folder_name.file_name();

        let server_path = format!(
            "{}/{}/server.jar",
            *MCSERVERS_DIR,
            folder_name.to_string_lossy()
        );

        if path::Path::new(&server_path).exists() {
            println!("{}", folder_name.to_string_lossy())
        }
    }
}

pub fn remove(name: &String) {
    let server_folder = format!("{}/{}", *MCSERVERS_DIR, name);

    if !path::Path::new(&server_folder).exists() {
        eprintln!("Server {name} does not exist!");
        process::exit(0);
    }

    fs::remove_dir_all(&server_folder).unwrap();
    println!("Removed server '{name}'!")
}

pub fn update(name: &String, version: &str) {
    let server_folder = format!("{}/{}", *MCSERVERS_DIR, name);

    if !path::Path::new(&server_folder).exists() {
        eprintln!("Server {name} does not exist!");
        process::exit(0);
    }

    let latest = &get_latest_version();
    let server_version = if version == "latest" {
        latest.as_str().unwrap()
    } else {
        version
    };

    let server_jar_link = fetch_server_jar_url(server_version);
    let server_jar_path = format!("{server_folder}/server.jar");
    fs::remove_file(server_jar_path.clone()).unwrap();

    download_with_pb(&server_jar_link, &server_jar_path);

    println!("Updated server {name} to version {server_version}!");
}

pub fn versions() {
    let version_manifest = reqwest::blocking::get(VERSION_MANIFEST_URL).unwrap();
    let version_manifest_text = version_manifest.text().unwrap();
    let version_manifest_json: Value = serde_json::from_str(&version_manifest_text).unwrap();
    let versions = &version_manifest_json["versions"];

    println!("Available server versions: ");

    for version in versions.as_array().unwrap() {
        println!("{}", version["id"].as_str().unwrap())
    }
}
