use std::collections::{BTreeMap, BTreeSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use sha2::{Digest, Sha256};
use worth_store_process_bundle::{
    FreshRecoveryProcessBundle, ObserverProcessRole, RecoveryProcessRole, WriterProcessRole,
};

#[path = "support_binary_feature_sets/expected_feature_lanes.rs"]
mod expected_feature_lanes;

#[test]
fn process_bundle_exposes_role_typed_artifacts() {
    with_finalized_bundle(|binaries| {
        let _: &worth_store_process_bundle::BoundArtifact<WriterProcessRole> = binaries.writer();
        let _: &worth_store_process_bundle::BoundArtifact<ObserverProcessRole> =
            binaries.observer();
        let _: &worth_store_process_bundle::BoundArtifact<RecoveryProcessRole> =
            binaries.recovery();
    });
}

#[test]
fn production_support_binaries_keep_authority_features_in_their_lanes() {
    with_finalized_bundle(|binaries| {
        let package_names = local_package_names();
        let writer = parse_compiler_features(binaries.writer().raw_cargo_stdout(), &package_names);
        let observer =
            parse_compiler_features(binaries.observer().raw_cargo_stdout(), &package_names);
        let recovery =
            parse_compiler_features(binaries.recovery().raw_cargo_stdout(), &package_names);

        assert_eq!(
            local_feature_map(&writer),
            expected_feature_lanes::writer_local_map()
        );
        assert_eq!(
            local_feature_map(&observer),
            expected_feature_lanes::observer_local_map()
        );
        assert_eq!(
            local_feature_map(&recovery),
            expected_feature_lanes::recovery_local_map()
        );
        assert_eq!(
            digest_hex(&writer),
            "2ec07a92fa032222ec0b50f2376a434e61d1cc3b9d0cc83db9de037bc7c7d527"
        );
        assert_eq!(
            digest_hex(&observer),
            "eb55e29ea0b533ddff8c0a5ddcdc356a2d7517db4aeb7b4b79bc75a264f5d281"
        );
        assert_eq!(
            digest_hex(&recovery),
            "ba040c841e6f989946918a644d816bf0aa550dd1cfd4f46b44fa3778a3ea966d"
        );

        assert_eq!(authority_projection(&writer), BTreeMap::new());
        assert_eq!(authority_projection(&observer), BTreeMap::new());
        assert_eq!(
            authority_projection(&recovery),
            expected_feature_lanes::recovery_projection()
        );
        assert!(
            !writer.is_empty(),
            "writer Cargo transcript had no artifacts"
        );
        assert!(
            !observer.is_empty(),
            "observer Cargo transcript had no artifacts"
        );
        assert!(
            !recovery.is_empty(),
            "recovery Cargo transcript had no artifacts"
        );
        assert_eq!(
            target_features(&writer, "worth-store", "physical_store_c8_writer"),
            BTreeSet::new()
        );
        assert_eq!(
            target_features(
                &observer,
                "worth-store-offline-verifier",
                "physical_store_offline_observer"
            ),
            BTreeSet::new()
        );
        assert_eq!(
            target_features(
                &recovery,
                "worth-store-recovery-runtime",
                "physical_store_recover"
            ),
            BTreeSet::from(["certification-test-authority".to_owned()])
        );
    });
}

fn with_finalized_bundle(test: impl FnOnce(&FreshRecoveryProcessBundle)) {
    let workspace = workspace_root();
    let repository = repository_root(&workspace);
    let finalized = FreshRecoveryProcessBundle::build_production_finalized(&workspace, &repository)
        .unwrap_or_else(|error| panic!("build role-typed process bundle: {error}"));
    let result = catch_unwind(AssertUnwindSafe(|| test(finalized.bundle()))).map_err(panic_message);
    finalized
        .finish(result)
        .unwrap_or_else(|error| panic!("role-typed process bundle proof failed: {error}"));
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("Phase 8 workspace root")
}

fn repository_root(workspace: &Path) -> PathBuf {
    workspace
        .join("../..")
        .canonicalize()
        .expect("Phase 8 repository root")
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| {
            payload
                .downcast_ref::<&str>()
                .map(|message| (*message).to_owned())
        })
        .unwrap_or_else(|| "role-typed process bundle proof panicked".to_owned())
}

type FeatureMap = BTreeMap<(String, String), BTreeSet<String>>;

fn parse_compiler_features(
    raw_stdout: &str,
    package_names: &BTreeMap<String, String>,
) -> FeatureMap {
    raw_stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let message: Value = serde_json::from_str(line).expect("Cargo JSON transcript");
            (message.get("reason").and_then(Value::as_str) == Some("compiler-artifact"))
                .then_some(message)
        })
        .map(|message| {
            let package_id = message
                .get("package_id")
                .and_then(Value::as_str)
                .expect("Cargo compiler artifact package id");
            let package = package_names
                .get(package_id)
                .cloned()
                .unwrap_or_else(|| package_id.to_owned());
            let target = message
                .get("target")
                .and_then(|target| target.get("name"))
                .and_then(Value::as_str)
                .expect("Cargo compiler artifact target name")
                .to_owned();
            let features = message
                .get("features")
                .and_then(Value::as_array)
                .expect("Cargo compiler artifact feature set")
                .iter()
                .map(|feature| feature.as_str().expect("Cargo feature name").to_owned())
                .collect();
            ((package, target), features)
        })
        .fold(FeatureMap::new(), |mut map, (key, features)| {
            if let Some(existing) = map.get(&key) {
                assert_eq!(
                    existing, &features,
                    "Cargo target feature drift across package identities {key:?}"
                );
            } else {
                map.insert(key, features);
            }
            map
        })
}

fn local_feature_map(map: &FeatureMap) -> FeatureMap {
    map.iter()
        .filter(|((package, _), _)| package_base(package).starts_with("worth-"))
        .map(|((package, target), features)| {
            ((package_base(package), target.clone()), features.clone())
        })
        .collect()
}

fn digest_hex(map: &FeatureMap) -> String {
    let mut hasher = Sha256::new();
    for ((package, target), features) in map {
        hasher.update(package.as_bytes());
        hasher.update([0]);
        hasher.update(target.as_bytes());
        hasher.update([0]);
        for feature in features {
            hasher.update(feature.as_bytes());
            hasher.update([0]);
        }
        hasher.update([0xff]);
    }
    hex(&hasher.finalize())
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn local_package_names() -> BTreeMap<String, String> {
    let workspace = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("Phase 8 workspace root");
    let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
        .current_dir(&workspace)
        .args([
            "metadata",
            "--locked",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(workspace.join("Cargo.toml"))
        .output()
        .expect("Cargo metadata for independent feature oracle");
    assert!(
        output.status.success(),
        "independent Cargo metadata failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let document: Value = serde_json::from_slice(&output.stdout).expect("Cargo metadata JSON");
    document
        .get("packages")
        .and_then(Value::as_array)
        .expect("Cargo metadata packages")
        .iter()
        .filter_map(|package| {
            Some((
                package.get("id")?.as_str()?.to_owned(),
                format!(
                    "{}@{}",
                    package.get("name")?.as_str()?,
                    package.get("version")?.as_str()?
                ),
            ))
        })
        .collect()
}

fn package_base(package: &str) -> String {
    package
        .rsplit_once('@')
        .map_or_else(|| package.to_owned(), |(name, _)| name.to_owned())
}

fn target_features(map: &FeatureMap, package: &str, target: &str) -> BTreeSet<String> {
    map.iter()
        .find(|((candidate, candidate_target), _)| {
            package_base(candidate) == package && candidate_target == target
        })
        .map(|(_, features)| features.clone())
        .unwrap_or_else(|| panic!("Cargo transcript omitted {package}::{target}"))
}

fn authority_projection(map: &FeatureMap) -> BTreeMap<String, BTreeSet<String>> {
    let mut projection = BTreeMap::new();
    for ((package, _), features) in map {
        let authority = features
            .iter()
            .filter(|feature| {
                matches!(
                    feature.as_str(),
                    "certification-test-authority" | "recovery-runtime-owner"
                )
            })
            .cloned()
            .collect::<BTreeSet<_>>();
        if !authority.is_empty() {
            projection
                .entry(package_base(package))
                .or_insert_with(BTreeSet::new)
                .extend(authority);
        }
    }
    projection
}

#[cfg(test)]
mod tests {
    use super::{expected_feature_lanes, parse_compiler_features};
    use std::collections::BTreeMap;

    #[test]
    fn feature_oracle_rejects_missing_extra_and_swapped_target_maps() {
        let writer = expected_feature_lanes::writer_local_map();
        let observer = expected_feature_lanes::observer_local_map();
        assert_ne!(
            writer, observer,
            "writer and observer maps must not be interchangeable"
        );

        let mut missing = writer.clone();
        missing.remove(&("worth-store".to_owned(), "worth_store".to_owned()));
        assert_ne!(
            missing, writer,
            "omitted target must change the expected map"
        );

        let mut extra = writer;
        extra.insert(
            ("worth-store".to_owned(), "forged_target".to_owned()),
            std::collections::BTreeSet::new(),
        );
        assert_ne!(
            extra,
            expected_feature_lanes::writer_local_map(),
            "extra target must change the expected map"
        );
    }

    #[test]
    fn independent_parser_rejects_conflicting_duplicate_target_transcripts() {
        let line = r#"{"reason":"compiler-artifact","package_id":"path+file:///workspace/Cargo.toml#0.1.0","target":{"name":"writer","kind":["bin"]},"features":[]}"#;
        let conflicting = format!(
            "{line}\n{}",
            line.replace("\"features\":[]", "\"features\":[\"forged\"]")
        );
        let result =
            std::panic::catch_unwind(|| parse_compiler_features(&conflicting, &BTreeMap::new()));
        assert!(
            result.is_err(),
            "conflicting duplicate target transcript was accepted"
        );
    }
}
