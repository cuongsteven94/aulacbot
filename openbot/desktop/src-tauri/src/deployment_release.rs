//! Choose a release for a new deployment and retain an existing deployment's exact version.

use std::path::Path;

use serde::Deserialize;

use crate::deployment;

/// This project's releases, newest first. Two kinds share the list: the installer's own
/// releases (`v*`) and the deployment releases (`deploy-v*`) this module is after, told apart
/// by the tag prefix in [`release_tag`]. The single "latest release" endpoint cannot be used:
/// it names whichever kind was published most recently, and an installer release must not
/// become the deployment a new install downloads.
const RELEASES: &str = "https://api.github.com/repos/cuongsteven94/aulacbot/releases?per_page=100";

/// The tag prefix that marks a release as a deployment release.
const DEPLOYMENT_TAG_PREFIX: &str = "deploy-";

/// Only new deployments consult GitHub. The fetcher records the exact tag after the source and
/// image manifest have both downloaded successfully; restarts and repairs retain that pin.
/// Uses blocking HTTP, so callers in an async runtime must use a blocking task.
pub fn resolve_version(root: &Path) -> Result<String, String> {
    resolve_version_with(root, || {
        let body = deployment::get(RELEASES)
            .map_err(|error| format!("could not find the latest AuLacBot deployment release: {error}"))?;
        release_tag(&body)
    })
}

fn resolve_version_with(
    root: &Path,
    latest: impl FnOnce() -> Result<String, String>,
) -> Result<String, String> {
    match deployment::installed(root) {
        Some(installed) => Ok(installed.version),
        None => latest(),
    }
}

fn release_tag(body: &[u8]) -> Result<String, String> {
    #[derive(Deserialize)]
    struct Release {
        tag_name: String,
        #[serde(default)]
        draft: bool,
    }

    let releases: Vec<Release> = serde_json::from_slice(body)
        .map_err(|error| format!("the AuLacBot release list is not readable: {error}"))?;
    // The list is newest first, so the first deployment release in it is the latest one.
    // Installer releases interleave with deployment releases; skipping them here is what keeps
    // a new installer release from being mistaken for a deployment.
    releases
        .into_iter()
        .find(|release| {
            !release.draft && release.tag_name.starts_with(DEPLOYMENT_TAG_PREFIX)
        })
        .map(|release| release.tag_name)
        .ok_or_else(|| "no AuLacBot deployment release has been published yet".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::temp_root;

    fn scratch(label: &str) -> std::path::PathBuf {
        let root = temp_root(label);
        std::fs::create_dir_all(&root).unwrap();
        root
    }

    #[test]
    fn a_fresh_install_selects_latest_without_recording_an_unfinished_download() {
        let root = scratch("release-fresh");
        let version = resolve_version_with(&root, || Ok("v0.0.9".into())).unwrap();
        assert_eq!(version, "v0.0.9");
        assert!(deployment::installed(&root).is_none());
        assert!(deployment::needs_fetch(&root, &version));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn an_installed_version_does_not_query_github_or_upgrade() {
        let root = scratch("release-installed");
        deployment::record(&root, "v0.0.7").unwrap();
        std::fs::write(deployment::images_path(&root), "{}").unwrap();
        let version = resolve_version_with(&root, || panic!("must work offline")).unwrap();
        assert_eq!(version, "v0.0.7");
        assert!(!deployment::needs_fetch(&root, &version));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_missing_manifest_repairs_the_pinned_version() {
        let root = scratch("release-repair");
        deployment::record(&root, "v0.0.7").unwrap();
        let version = resolve_version_with(&root, || panic!("keep the installed pin")).unwrap();
        assert_eq!(version, "v0.0.7");
        assert!(deployment::needs_fetch(&root, &version));
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn a_failed_lookup_is_reported_without_recording_a_version() {
        let root = scratch("release-lookup-failed");
        let error = resolve_version_with(&root, || Err("GitHub answered 403".into()))
            .expect_err("a failed lookup must not fall back to an old release");
        assert!(error.contains("403"));
        assert!(deployment::installed(&root).is_none());
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn githubs_exact_tag_is_used_instead_of_the_release_title() {
        assert_eq!(
            release_tag(
                br#"[{"name":"AuLacBot September release","tag_name":"deploy-v0.0.9"}]"#
            )
            .unwrap(),
            "deploy-v0.0.9"
        );
    }

    #[test]
    fn installer_releases_are_not_mistaken_for_deployments() {
        // Newest first: an installer release published after the deployment release is skipped,
        // and the deployment release below it is the one a new install gets.
        assert_eq!(
            release_tag(
                br#"[{"tag_name":"v0.2.0"},{"tag_name":"deploy-v0.3.1","draft":false},{"tag_name":"deploy-v0.3.0"}]"#
            )
            .unwrap(),
            "deploy-v0.3.1"
        );
    }

    #[test]
    fn missing_or_unreadable_release_metadata_is_refused() {
        for body in [
            b"not json".as_slice(),
            b"{}",
            br#"{"tag_name":"deploy-v0.0.9"}"#,
            b"[]",
            br#"[{"tag_name":"v0.2.0"}]"#,
            br#"[{"tag_name":"deploy-v0.0.9","draft":true}]"#,
        ] {
            assert!(release_tag(body).is_err(), "{body:?}");
        }
    }

    /// Exercises the production resolver and downloader against GitHub in an empty directory.
    #[test]
    #[ignore = "downloads the latest public release from GitHub"]
    fn live_latest_release_is_downloaded_and_pinned() {
        let root = scratch("release-live");
        let version = resolve_version(&root).expect("resolve the public release");
        assert!(deployment::installed(&root).is_none());
        deployment::fetch(&root, &version).expect("download the tagged deployment and manifest");
        assert_eq!(deployment::installed(&root).unwrap().version, version);
        let images: deployment::Images =
            serde_json::from_slice(&std::fs::read(deployment::images_path(&root)).unwrap())
                .unwrap();
        assert_eq!(images.version, version);
        let package: serde_json::Value =
            serde_json::from_slice(&std::fs::read(root.join("package.json")).unwrap()).unwrap();
        assert!(
            package["version"].is_string(),
            "the laid-out tree carries its package manifest"
        );
        for path in deployment::REQUIRED {
            assert!(root.join(path).exists(), "missing {path}");
        }
        assert_eq!(
            resolve_version_with(&root, || panic!("restart must not need GitHub")).unwrap(),
            version
        );
        assert!(!deployment::needs_fetch(&root, &version));
        println!("Downloaded and pinned {version}; tagged source, image manifest, and offline reuse verified.");
        std::fs::remove_dir_all(root).unwrap();
    }
}
