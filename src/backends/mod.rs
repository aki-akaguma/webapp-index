use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use anyhow::Context;

#[cfg(feature = "server")]
use tokio::fs::{read_dir, read_to_string, try_exists};

mod conf;

#[cfg(feature = "server")]
use conf::{ConfApp, Config};

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct AppInfo {
    name: String,
    desc: String,
    web: bool,
    apk_fnms: Vec<String>,
    appimage_fnms: Vec<String>,
}

impl AppInfo {
    #[cfg(feature = "server")]
    async fn from_conf_app(conf_app: &ConfApp) -> Result<AppInfo> {
        let name = conf_app.name().to_string();
        let desc = conf_app.desc().to_string();
        let web = conf_app.is_web();
        let apk_fnms = {
            let mut vec: Vec<String> = vec![];
            if conf_app.is_android_wva() {
                let fnm = find_fnm_apk_wva(&name).await?;
                if !fnm.is_empty() {
                    vec.push(fnm);
                }
            }
            if conf_app.is_android_aarch64() {
                let fnm = find_fnm_apk_aarch64(&name).await?;
                if !fnm.is_empty() {
                    vec.push(fnm);
                }
            }
            if conf_app.is_android_x86_64() {
                let fnm = find_fnm_apk_x86_64(&name).await?;
                if !fnm.is_empty() {
                    vec.push(fnm);
                }
            }
            vec
        };
        let appimage_fnms = {
            let mut vec: Vec<String> = vec![];
            if conf_app.is_desktop_linux() {
                let fnm = find_fnm_appimage(&name).await?;
                if !fnm.is_empty() {
                    vec.push(fnm);
                }
            }
            vec
        };
        Ok(AppInfo {
            name,
            desc,
            web,
            apk_fnms,
            appimage_fnms,
        })
    }
    pub fn name(&self) -> &str {
        self.name.as_str()
    }
    pub fn desc(&self) -> &str {
        self.desc.as_str()
    }
    pub fn apk_fnms(&self) -> &[String] {
        &self.apk_fnms
    }
    pub fn appimage_fnms(&self) -> &[String] {
        &self.appimage_fnms
    }
}

#[cfg(feature = "server")]
async fn find_fnm_apk_wva(name: &str) -> Result<String> {
    // appimage file name format:
    //   memboost-wva-app-release-signed.apk
    let file_name = format!("{name}-wva-app-release-signed.apk");
    find_fnm_apk_file_name(name, file_name).await
}

#[cfg(feature = "server")]
async fn find_fnm_apk_aarch64(name: &str) -> Result<String> {
    // appimage file name format:
    //   memboost-aarch64-app-release-signed.apk
    let file_name = format!("{name}-aarch64-app-release-signed.apk");
    find_fnm_apk_file_name(name, file_name).await
}

#[cfg(feature = "server")]
async fn find_fnm_apk_x86_64(name: &str) -> Result<String> {
    // appimage file name format:
    //   memboost-x86_64-app-release-signed.apk
    let file_name = format!("{name}-x86_64-app-release-signed.apk");
    find_fnm_apk_file_name(name, file_name).await
}

#[cfg(feature = "server")]
async fn find_fnm_apk_file_name(name: &str, file_name: String) -> Result<String> {
    let file_path = format!("/opt/webapp-{name}/android/{}", &file_name);
    if try_exists(&file_path).await.unwrap_or_default() {
        Ok(file_name)
    } else {
        Ok("".to_string())
    }
}

#[cfg(feature = "server")]
async fn find_fnm_appimage(name: &str) -> Result<String> {
    use semver::Version;
    // appimage file name format:
    //   memboost_0.1.1_x86_64.AppImage
    let ends = "_x86_64.AppImage";
    let mut vec = vec![];
    let dir_path = format!("/opt/webapp-{name}/desktop");
    let mut entries = read_dir(&dir_path)
        .await
        .with_context(|| format!("Failed to read dir from '{}'", &dir_path))?;
    while let Some(entry) = entries.next_entry().await? {
        let fnm = entry.file_name().to_string_lossy().to_string();
        if fnm.starts_with(name) && fnm.ends_with(ends) {
            let fnm_version = &fnm[(name.len() + 1)..(fnm.len() - ends.len())];
            let version_s = fnm_version.to_string();
            let version = Version::parse(version_s.as_str())?;
            vec.push((version, version_s));
        }
    }
    if vec.is_empty() {
        Ok("".to_string())
    } else {
        let last_version = if vec.len() == 1 {
            &vec[0].1
        } else {
            vec.sort_by(|a, b| a.0.cmp(&b.0));
            &vec[vec.len() - 1].1
        };
        Ok(format!("{name}_{last_version}{ends}"))
    }
}

#[post("/api/v1/apps")]
pub async fn list_apps(is_devel: bool) -> Result<Vec<AppInfo>> {
    let mut apps = Vec::new();
    let file_path = "/opt/webapp-akiapp/web/config.toml";
    let conf_string = read_to_string(file_path)
        .await
        .with_context(|| format!("Failed to read from '{}'", file_path))?;
    let conf: Config = toml::from_str(conf_string.as_str())?;
    let iter = if !is_devel {
        conf.apps.iter()
    } else {
        conf.dev_apps.iter()
    };
    for conf_app in iter {
        if !conf_app.name().is_empty() {
            apps.push(AppInfo::from_conf_app(conf_app).await?);
        }
    }
    apps.sort_by(|a, b| a.name().cmp(b.name()));
    Ok(apps)
}
