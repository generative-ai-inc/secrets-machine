use std::{error::Error, os::unix::fs::PermissionsExt, path::PathBuf};

use crate::{built_info, library::system};
use bytes::Bytes;
use tokio::process::Command;

use super::logging;

use tokio::fs;

async fn schedule_replace_and_restart(
    current_binary_path: &std::path::Path,
    new_binary_path: &std::path::Path,
) {
    // Use a shell script to handle the replacement after the current process exits
    let script = format!(
        r#"
        #!/bin/bash
        sleep 1
        mv "{new}" "{current}"
        chmod +x "{current}"
        "#,
        new = new_binary_path.display(),
        current = current_binary_path.display()
    );

    let rand_num = fastrand::i32(..);
    let script_path = PathBuf::from(format!("/tmp/sm-{rand_num}.sh"));
    fs::write(&script_path, script)
        .await
        .expect("Failed to write update script");
    fs::set_permissions(&script_path, PermissionsExt::from_mode(0o755))
        .await
        .expect("Failed to set script permissions");

    // Execute the script in a new process and exit the current process
    Command::new("sh")
        .arg(script_path)
        .spawn()
        .expect("Failed to execute update script");

    // Exit the current process
    std::process::exit(0);
}

async fn download_update(
    client: &reqwest::Client,
    download_url: &str,
) -> Result<Bytes, Box<dyn Error>> {
    let Ok(response) = client
        .get(download_url)
        .header("Accept", "application/octet-stream")
        .header("User-Agent", "sm")
        .send()
        .await
    else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to download update",
        )));
    };

    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to download update: {}", response.status()),
        )));
    }

    match response.bytes().await {
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to get bytes: {e}"),
        ))),
    }
}

pub async fn update() {
    logging::info("Updating Secrets Machine...").await;

    let current_version: String = built_info::PKG_VERSION.to_string();

    logging::info(&format!("Current version: {current_version}")).await;

    // Find where the binary is located
    let Ok(binary_path) = std::env::current_exe() else {
        logging::error("Failed to find secrets machine binary path").await;
        std::process::exit(1);
    };

    // Find os and arch
    let Ok(os_string) = system::command::run("uname -s").await else {
        logging::error("Failed to read output of command which identifies os").await;
        std::process::exit(1);
    };
    let Ok(arch_string) = system::command::run("uname -m").await else {
        logging::error("Failed to read output of command which identifies arch").await;
        std::process::exit(1);
    };

    let asset_name = format!(
        "secrets-machine-{}-{}",
        arch_string.to_lowercase().trim(),
        os_string.to_lowercase().trim()
    );

    let client = reqwest::Client::new();

    // Find the latest version
    let response = client
        .get("https://api.github.com/repos/generative-ai-inc/secrets-machine/releases/latest")
        .header("Accept", "application/json")
        .header("User-Agent", "sm")
        .send()
        .await;

    let response = match response {
        Ok(res) => res,
        Err(e) => {
            logging::error(&format!("Failed to get latest version: {e}")).await;
            std::process::exit(1);
        }
    };

    if !response.status().is_success() {
        logging::error(&format!(
            "Failed to get latest version: {}",
            response.status()
        ))
        .await;
        std::process::exit(1);
    }

    let Ok(data) = response.json::<serde_json::Value>().await else {
        logging::error("Failed to deserialize latest release response").await;
        std::process::exit(1);
    };

    let latest_version = if let Some(tag) = data["tag_name"].as_str() {
        tag.replace('v', "")
    } else {
        logging::error("Tag name not found in latest release response").await;
        std::process::exit(1);
    };

    logging::info(&format!("Latest version: {latest_version}")).await;

    if current_version == latest_version {
        logging::info("You are already on the latest version").await;
        std::process::exit(0);
    }

    let Some(assets) = data["assets"].as_array() else {
        logging::error("Assets not found in latest release response").await;
        std::process::exit(1);
    };

    let Some(asset) = assets.iter().find(|a| {
        let Some(name) = a["name"].as_str() else {
            return false;
        };
        name.contains(&asset_name)
    }) else {
        logging::error(&format!("No asset found for {asset_name}")).await;
        std::process::exit(1);
    };

    let Some(download_url) = asset["url"].as_str() else {
        logging::error("Download URL not found in asset").await;
        std::process::exit(1);
    };

    let Ok(bytes) = download_update(&client, download_url).await else {
        logging::error("Failed to download update").await;
        std::process::exit(1);
    };

    let rand_num = fastrand::i32(..);
    let tmp_binary_path = PathBuf::from(format!("/tmp/sm-{rand_num}"));

    if let Err(e) = fs::write(&tmp_binary_path, bytes).await {
        logging::error(&format!("Failed to write bytes: {e}")).await;
        std::process::exit(1);
    };

    logging::info(&format!(
        "Updated Secrets Machine to version {latest_version}"
    ))
    .await;

    // Schedule the replacement and restart
    schedule_replace_and_restart(&binary_path, &tmp_binary_path).await;
}
