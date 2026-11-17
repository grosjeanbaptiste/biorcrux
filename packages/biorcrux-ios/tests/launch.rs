use biorcrux_ios::{check, IosLaunchError};

const SCENE: &str =
    "[ios.plist.UIApplicationSceneManifest]\nUIApplicationSupportsMultipleScenes = false\n";

fn lock(tao: &str) -> String {
    format!("[[package]]\nname = \"tao\"\nversion = \"{tao}\"\n")
}

#[test]
fn accepts_a_single_scene_manifest_and_tao_0_37() {
    assert_eq!(check(SCENE, &lock("0.37.2")), Ok(()));
}

#[test]
fn rejects_a_missing_scene_manifest() {
    assert_eq!(
        check("[bundle]\nname = \"x\"\n", &lock("0.37.2")),
        Err(IosLaunchError::SceneManifestMissing)
    );
}

#[test]
fn rejects_multiple_scenes() {
    let toml = SCENE.replace("false", "true");
    assert_eq!(
        check(&toml, &lock("0.37.2")),
        Err(IosLaunchError::MultipleScenes)
    );
}

#[test]
fn rejects_a_tao_without_scene_support() {
    assert_eq!(
        check(SCENE, &lock("0.34.8")),
        Err(IosLaunchError::TaoTooOld("0.34.8".into()))
    );
}

#[test]
fn rejects_a_lock_without_tao() {
    assert_eq!(
        check(SCENE, "[[package]]\nname = \"wry\"\nversion = \"0.53.0\"\n"),
        Err(IosLaunchError::TaoMissing)
    );
}

#[test]
fn rejects_invalid_toml() {
    assert!(matches!(
        check("[", &lock("0.37.2")),
        Err(IosLaunchError::InvalidToml(_))
    ));
}

#[test]
fn the_app_template_declares_the_scene_manifest() {
    let template = include_str!("../../../template/Dioxus.toml.tmpl");
    assert_eq!(check(template, &lock("0.37.2")), Ok(()));
}
