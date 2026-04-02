use anyhow::Result;
use dioxus::prelude::*;

#[cfg(feature = "server")]
use tokio::fs::{read_dir, DirEntry};

//#[cfg(feature = "server")]
//use std::path::PathBuf;

/// Query the apps
#[get("/api/v1/apps")]
pub async fn list_apps() -> Result<Vec<(String, String, Vec<String>)>> {
    let mut apps = Vec::new();
    let mut entries = read_dir("/opt").await?;
    while let Some(entry) = entries.next_entry().await? {
        //let path = entry.path().to_string_lossy().to_string();
        let app_info = get_app_info_from_dir_entry(entry).await;
        if !app_info.0.is_empty() {
            apps.push(app_info);
        }
    }
    apps.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(apps)
}

#[cfg(feature = "server")]
async fn get_app_info_from_dir_entry(entry: DirEntry) -> (String, String, Vec<String>) {
    let r = (String::new(), String::new(), Vec::new());
    //
    let file_type;
    let app_name;
    if let Ok(ftype) = entry.file_type().await {
        file_type = ftype;
    } else {
        return r;
    };
    if !file_type.is_dir() {
        return r;
    }
    let file_name = entry.file_name().to_string_lossy().to_string();
    //dioxus_logger::tracing::info!("dir fnm: {file_name}");
    if file_name == "webapp-index" {
        return r;
    }
    let prefix = "webapp-";
    if !file_name.starts_with(prefix) {
        return r;
    }
    app_name = file_name[prefix.len()..].to_string();
    //
    let path = entry.path().to_string_lossy().to_string();
    let desc = if let Ok(v) = std::fs::read(format!("{path}/description.txt")) {
        String::from_utf8_lossy(&v).to_string()
    } else {
        String::new()
    };
    //
    let apk_fnms = if let Ok(v) = get_apk_from_android_path(path).await {
        v
    } else {
        Vec::new()
    };
    //
    (app_name, desc, apk_fnms)
}

#[cfg(feature = "server")]
async fn get_apk_from_android_path(path: String) -> Result<Vec<String>> {
    let mut r = Vec::new();
    let dir = format!("{path}/android");
    let mut entries = read_dir(dir).await?;
    while let Some(entry) = entries.next_entry().await? {
        let file_name = entry.file_name().to_string_lossy().to_string();
        if file_name.ends_with(".apk") {
            r.push(file_name);
        }
    }
    r.sort();
    Ok(r)
}
