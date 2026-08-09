use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde_json::Value;
use sha2::{Digest, Sha256};

const ARTIFACT: &str = "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json";

#[test]
fn phase_one_compile_contract_artifact_matches_every_executed_case() {
    super::assert_protocol_revision_four_rejects_mixed_revision();
    let root = repository_root();
    let artifact: Value = serde_json::from_str(
        &std::fs::read_to_string(compile_artifact(&root)).expect("retained compile result"),
    )
    .expect("versioned compile result JSON");
    assert_eq!(artifact["schema"], "worth-ui-compile-contract-result-v1");
    assert_eq!(artifact["exit_posture"], "passed");
    assert_eq!(artifact["cargo_sessions"], 2);
    assert!(artifact["run_nonce"]
        .as_str()
        .is_some_and(|nonce| nonce.len() == 32 && nonce.bytes().all(is_lower_hex)));
    let revision = artifact["source_revision"]
        .as_str()
        .filter(|revision| revision.len() == 40 && revision.bytes().all(is_lower_hex))
        .expect("compile result source revision");
    assert_eq!(
        artifact["source_state_digest"],
        super::super::milestone_3141_phase1_ledger::source_digest::calculate_source_state(revision)
            .expect("current governed source state")
    );

    let expected = expected_cases(&root);
    let observed = observed_cases(&artifact);
    assert_eq!(observed, expected);
    assert_eq!(artifact["fail_targets"], count_kind(&expected, "fail"));
    assert_eq!(artifact["pass_targets"], count_kind(&expected, "pass"));
    let authority = count_targets(
        &observed,
        &[
            "product-native-preparation-no-builder-extraction",
            "product-native-preparation-valid",
        ],
    );
    let order_source = count_targets(&observed, &["product-paint-identities-non-orderable"]);
    let platform = count_targets(
        &observed,
        &[
            "product-cannot-bind-native-host",
            "product-native-preparation-valid",
        ],
    );
    let presentation = count_targets(
        &observed,
        &[
            "host-presentation-work-authority",
            "host-presentation-mechanics-consumer",
        ],
    );
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P1-AUTHORITY-01\":{authority},\"P1-ORDER-SOURCE-01\":{order_source},\"P1-PLATFORM-AUTHORITY-01\":{platform},\"P1-PRESENTATION-AUTHORITY-01\":{presentation},\"P1-PROTOCOL-01\":4}}"
    );
}

#[test]
fn product_native_driver_substitution_is_compiler_rejected() {
    let root = repository_root();
    let artifact: Value = serde_json::from_str(
        &std::fs::read_to_string(compile_artifact(&root)).expect("retained compile result"),
    )
    .expect("versioned compile result JSON");
    let source = "workspaces/worth-ui/crates/worth-ui/tests/ui/facade/construction/host_binding/product_cannot_substitute_native_event_loop_client.rs";
    let case = artifact["cases"]
        .as_array()
        .expect("compile case array")
        .iter()
        .find(|case| case["source"] == source)
        .expect("native driver substitution compile case");
    assert_eq!(case["kind"], "fail");
    let snapshot = case["snapshot"].as_str().expect("failure snapshot");
    let diagnostic = std::fs::read_to_string(root.join(snapshot)).expect("failure diagnostic");
    assert!(diagnostic.contains("UiNativeApplicationDefinition"));
    assert!(diagnostic.contains("ForgedNativeClient"));
}

fn expected_cases(root: &Path) -> BTreeSet<Vec<String>> {
    let fixture = root
        .join("workspaces/worth-ui/crates/worth-ui-certification/tests/fixtures/compile_contracts");
    let bins = fixture_bins(&fixture);
    let mut cases = BTreeSet::new();
    for (owner, identity) in inventory_identities() {
        let inventory = root.join(identity);
        let crate_root = inventory
            .parent()
            .and_then(Path::parent)
            .and_then(Path::parent)
            .expect("compile inventory crate root");
        let mut reader = csv::Reader::from_path(&inventory).expect("compile inventory CSV");
        for row in reader.records() {
            let row = row.expect("compile inventory row");
            let kind = &row[0];
            let source = crate_root
                .join(&row[1])
                .canonicalize()
                .expect("fixture source");
            let target = bins.get(&source).expect("fixture bin target");
            cases.insert(case_record(root, owner, kind, target, &source));
        }
    }
    assert_eq!(cases.len(), bins.len(), "every fixture bin is inventoried");
    cases
}

fn fixture_bins(fixture: &Path) -> BTreeMap<PathBuf, String> {
    let manifest = std::fs::read_to_string(fixture.join("Cargo.toml")).expect("fixture manifest");
    let document: toml::Value = toml::from_str(&manifest).expect("fixture TOML");
    document["bin"]
        .as_array()
        .expect("fixture bins")
        .iter()
        .map(|bin| {
            let target = bin["name"].as_str().expect("bin name").to_owned();
            let source = fixture
                .join(bin["path"].as_str().expect("bin path"))
                .canonicalize()
                .expect("bin source");
            (source, target)
        })
        .collect()
}

fn case_record(root: &Path, owner: &str, kind: &str, target: &str, source: &Path) -> Vec<String> {
    let source_identity = relative(root, source);
    let mut record = vec![
        owner.to_owned(),
        kind.to_owned(),
        target.to_owned(),
        source_identity,
        digest(source),
    ];
    if kind == "fail" {
        let snapshot = source.with_extension("stderr");
        record.extend([relative(root, &snapshot), digest(&snapshot)]);
    } else {
        record.extend([String::new(), String::new()]);
    }
    record
}

fn observed_cases(artifact: &Value) -> BTreeSet<Vec<String>> {
    artifact["cases"]
        .as_array()
        .expect("compile case array")
        .iter()
        .map(|case| {
            [
                "owner",
                "kind",
                "target",
                "source",
                "source_sha256",
                "snapshot",
                "snapshot_sha256",
            ]
            .map(|field| case[field].as_str().unwrap_or_default().to_owned())
            .into_iter()
            .collect()
        })
        .collect()
}

fn inventory_identities() -> [(&'static str, &'static str); 3] {
    [
        (
            "certification",
            "workspaces/worth-ui/crates/worth-ui-certification/tests/suites/compile_contract_execution.csv",
        ),
        (
            "host",
            "workspaces/worth-ui/crates/worth-ui-host-contract/tests/suites/compile_contract_cases.csv",
        ),
        (
            "product",
            "workspaces/worth-ui/crates/worth-ui/tests/suites/compile_contract_execution.csv",
        ),
    ]
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(4)
        .expect("repository root")
        .canonicalize()
        .expect("canonical repository root")
}

fn compile_artifact(root: &Path) -> PathBuf {
    let identity =
        std::env::var("WORTH_UI_COMPILE_ARTIFACT").unwrap_or_else(|_| ARTIFACT.to_owned());
    let path = root
        .join(identity)
        .canonicalize()
        .expect("compile artifact path");
    assert!(
        path.starts_with(root),
        "compile artifact must remain in repository"
    );
    path
}

fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .expect("repository-relative case")
        .to_string_lossy()
        .replace('\\', "/")
}

fn digest(path: &Path) -> String {
    format!(
        "{:x}",
        Sha256::digest(std::fs::read(path).expect("compile evidence source"))
    )
}

fn count_kind(cases: &BTreeSet<Vec<String>>, kind: &str) -> u64 {
    cases.iter().filter(|case| case[1] == kind).count() as u64
}

fn count_targets(cases: &BTreeSet<Vec<String>>, targets: &[&str]) -> u64 {
    targets
        .iter()
        .filter(|target| cases.iter().any(|case| case[2] == **target))
        .count() as u64
}

fn is_lower_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)
}
