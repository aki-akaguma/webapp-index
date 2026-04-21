use anyhow::Result;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use anyhow::Context;

#[cfg(feature = "server")]
use tokio::fs::{read_dir, read_to_string, try_exists};

#[cfg(feature = "server")]
use std::time::SystemTime;

#[cfg(feature = "server")]
use tokio::sync::RwLock;

mod conf;

#[cfg(feature = "server")]
use conf::{ConfApp, Config};

#[cfg(feature = "server")]
struct ConfigCache {
    config: Config,
    mtime: SystemTime,
}

#[cfg(feature = "server")]
static CACHE: RwLock<Option<ConfigCache>> = RwLock::const_new(None);

#[cfg(feature = "server")]
async fn get_config() -> Result<Config> {
    let file_path = "/opt/webapp-akiapp/web/config.toml";

    // Get file metadata (to check modification date and time)
    let metadata = tokio::fs::metadata(file_path)
        .await
        .with_context(|| format!("Failed to get metadata for '{}'", file_path))?;
    let mtime = metadata.modified()?;

    // 1. First, check if the cache is enabled with a read lock.
    {
        let cache = CACHE.read().await;
        if let Some(c) = &*cache {
            if c.mtime >= mtime {
                return Ok(c.config.clone());
            }
        }
    }

    // 2. If cache is missing or stale, acquire write lock and update
    let mut cache = CACHE.write().await;

    // Double-checked locking:
    // may have been updated by another thread while waiting for lock acquisition.
    if let Some(c) = &*cache {
        if c.mtime >= mtime {
            return Ok(c.config.clone());
        }
    }

    // Load and parse the file
    let conf_string = read_to_string(file_path).await?;
    let config: Config = toml::from_str(&conf_string)?;

    // Update cache
    *cache = Some(ConfigCache {
        config: config.clone(),
        mtime,
    });

    Ok(config)
}

#[post("/api/v1/apps/bc")]
#[tracing::instrument(fields(is_devel))]
pub async fn get_base_config() -> Result<String, ServerFnError> {
    let conf = get_config()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    // Returns [base] public_url in config.toml (needs addition to structure)
    Ok(conf.base.public_url.clone())
}

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
    //
    // Error handling in case the directory does not exist
    let mut entries = read_dir(&dir_path)
        .await
        .with_context(|| format!("Failed to read dir from '{}'", &dir_path))?;
    //
    while let Some(entry) = entries.next_entry().await? {
        let fnm = entry.file_name().to_string_lossy().to_string();
        if fnm.starts_with(name) && fnm.ends_with(ends) {
            let fnm_version = &fnm[(name.len() + 1)..(fnm.len() - ends.len())];
            let version_s = fnm_version.to_string();
            if let Ok(version) = Version::parse(version_s.as_str()) {
                vec.push((version, version_s));
            } else {
                tracing::warn!("Skipping invalid version format: {}", fnm);
            }
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
#[tracing::instrument(fields(is_devel))]
pub async fn list_apps(is_devel: bool) -> Result<Vec<AppInfo>> {
    // Record the IP in the current tracing span.
    tracing::Span::current().record("is_devel", is_devel);

    // Get settings from cache
    let conf = get_config().await?;

    let mut apps = Vec::new();
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
