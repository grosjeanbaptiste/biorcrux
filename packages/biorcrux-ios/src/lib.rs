//! iOS 27 kills an app without the UIKit scene lifecycle at launch (SIGTRAP in
//! `EvaluateRuntimeIssueForNoSceneLifecycleAdoption`). Two conditions, checked here
//! so an app's own tests fail before a device does.

#[derive(Debug, PartialEq, Eq)]
pub enum IosLaunchError {
    InvalidToml(String),
    SceneManifestMissing,
    MultipleScenes,
    TaoMissing,
    TaoTooOld(String),
}

/// `dioxus_toml` is the app's `Dioxus.toml`, `cargo_lock` its `Cargo.lock`.
pub fn check(dioxus_toml: &str, cargo_lock: &str) -> Result<(), IosLaunchError> {
    check_scene_manifest(dioxus_toml)?;
    check_tao(cargo_lock)
}

fn parse(text: &str) -> Result<toml::Table, IosLaunchError> {
    text.parse()
        .map_err(|e: toml::de::Error| IosLaunchError::InvalidToml(e.to_string()))
}

fn check_scene_manifest(dioxus_toml: &str) -> Result<(), IosLaunchError> {
    let doc = parse(dioxus_toml)?;
    let multiple = doc
        .get("ios")
        .and_then(|v| v.get("plist"))
        .and_then(|v| v.get("UIApplicationSceneManifest"))
        .and_then(|v| v.get("UIApplicationSupportsMultipleScenes"))
        .ok_or(IosLaunchError::SceneManifestMissing)?;
    match multiple.as_bool() {
        Some(false) => Ok(()),
        _ => Err(IosLaunchError::MultipleScenes),
    }
}

/// tao gained UIScene support in 0.37 (tauri-apps/tao#1340).
fn check_tao(cargo_lock: &str) -> Result<(), IosLaunchError> {
    let doc = parse(cargo_lock)?;
    let version = doc
        .get("package")
        .and_then(|p| p.as_array())
        .into_iter()
        .flatten()
        .find(|p| p.get("name").and_then(|n| n.as_str()) == Some("tao"))
        .and_then(|p| p.get("version")?.as_str())
        .ok_or(IosLaunchError::TaoMissing)?;
    match version
        .split('.')
        .nth(1)
        .and_then(|m| m.parse::<u32>().ok())
    {
        Some(minor) if minor >= 37 => Ok(()),
        _ => Err(IosLaunchError::TaoTooOld(version.to_string())),
    }
}
