use std::path::Path;

use super::fault_cases::ManifestPosture;

pub(super) fn observe_tree(root: &Path) -> String {
    observer_output("--tree", root)
}

pub(super) fn observe_namespace(root: &Path) -> String {
    observer_output("--namespace", root)
}

fn observer_output(mode: &str, root: &Path) -> String {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_physical_media_os_observer"))
        .arg(mode)
        .arg(root)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

pub(super) fn manifest_projection(manifest: &str) -> Vec<String> {
    manifest
        .lines()
        .filter_map(|line| line.strip_prefix("entry="))
        .map(|entry| {
            let mut fields = entry.splitn(4, '|');
            let kind = fields.next().unwrap();
            let length = fields.next().unwrap();
            let digest = fields.next().unwrap();
            let path = normalize_ephemeral_path(fields.next().unwrap());
            let stable_length =
                if path == "namespace/mutation.lock" && digest == "<valid-mutation-observation>" {
                    "*"
                } else {
                    length
                };
            format!("{kind}:{stable_length}:{digest}:{path}")
        })
        .collect()
}

pub(super) fn expected_manifest(posture: ManifestPosture) -> Vec<String> {
    let entries: &[&str] = match posture {
        ManifestPosture::Absent => &[],
        ManifestPosture::ScaffoldOnly => &[
            "directory:0:-:families",
            "directory:0:-:namespace",
            "directory:0:-:staging",
        ],
        ManifestPosture::ScaffoldWithLock => &[
            "directory:0:-:families",
            "directory:0:-:namespace",
            "file:*:<valid-mutation-observation>:namespace/mutation.lock",
            "directory:0:-:staging",
        ],
        ManifestPosture::EmptyStagedIdentity => &[
            "directory:0:-:families",
            "directory:0:-:namespace",
            "file:0:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855:namespace/identity-<ephemeral>.staged",
            "file:*:<valid-mutation-observation>:namespace/mutation.lock",
            "directory:0:-:staging",
        ],
        ManifestPosture::FullStagedIdentity => &[
            "directory:0:-:families",
            "directory:0:-:namespace",
            "file:72:<valid-identity-record>:namespace/identity-<ephemeral>.staged",
            "file:*:<valid-mutation-observation>:namespace/mutation.lock",
            "directory:0:-:staging",
        ],
        ManifestPosture::Published => &[
            "directory:0:-:families",
            "directory:0:-:namespace",
            "file:72:<valid-identity-record>:namespace/identity",
            "file:*:<valid-mutation-observation>:namespace/mutation.lock",
            "directory:0:-:staging",
        ],
        ManifestPosture::PublishedWithQualificationResidue => &[
            "directory:0:-:families",
            "directory:0:-:namespace",
            "file:72:<valid-identity-record>:namespace/identity",
            "file:16:c2642dda112a616f52168b2a5e7be38e328d5da80c4c347866bb646d1030bf0f:namespace/identity-<ephemeral>.staged",
            "file:*:<valid-mutation-observation>:namespace/mutation.lock",
            "directory:0:-:staging",
        ],
    };
    entries.iter().map(|entry| (*entry).into()).collect()
}

fn normalize_ephemeral_path(path: &str) -> String {
    let Some(identity) = path
        .strip_prefix("namespace/identity-")
        .and_then(|value| value.strip_suffix(".staged"))
    else {
        return path.into();
    };
    assert_eq!(identity.len(), 32, "staged identity nonce width drifted");
    assert!(
        identity.bytes().all(|byte| byte.is_ascii_hexdigit()),
        "staged identity nonce grammar drifted"
    );
    "namespace/identity-<ephemeral>.staged".into()
}
