use std::{path::Path, process::Command};

use crate::workspace_root;

struct OrdinaryProduct {
    label: &'static str,
    package: &'static str,
    features: &'static [&'static str],
}

const ORDINARY_PRODUCTS: &[OrdinaryProduct] = &[
    product("Store", "worth-store", &[]),
    product("blob", "worth-store-blob-chunks", &[]),
    product("maintenance", "worth-store-maintenance", &[]),
    product(
        "test-support boundary fixtures",
        "worth-store-test-support",
        &["boundary-fixtures"],
    ),
    product(
        "test-support physical-isolation fixtures",
        "worth-store-test-support",
        &["physical-isolation-fixtures"],
    ),
    product(
        "test-support layout fixtures",
        "worth-store-test-support",
        &["layout-fixtures"],
    ),
    product("scheduler", "worth-store-io-scheduler", &[]),
    product("residency", "worth-store-buffer-pool", &[]),
    product("backend", "worth-store-physical-backend", &[]),
    product("integrity", "worth-store-physical-integrity", &[]),
    product("recovery", "worth-store-recovery-physics", &[]),
];

const ORDINARY_MANIFESTS: &[(&str, bool)] = &[
    ("crates/worth-store/Cargo.toml", false),
    ("crates/worth-store-blob-chunks/Cargo.toml", false),
    ("crates/worth-store-maintenance/Cargo.toml", false),
    ("crates/worth-store-test-support/Cargo.toml", true),
    ("crates/worth-store-io-scheduler/Cargo.toml", false),
    ("crates/worth-store-buffer-pool/Cargo.toml", false),
    ("crates/worth-store-physical-backend/Cargo.toml", false),
    ("crates/worth-store-physical-integrity/Cargo.toml", false),
    ("crates/worth-store-recovery-physics/Cargo.toml", false),
];

const FORBIDDEN_FEATURES: &[&str] = &[
    "legacy-s2-models",
    "legacy-certification-models",
    "certification-test-authority",
    "certification-authority",
    "certification-test-support",
    "certification-world",
    "physical-compaction-fixtures",
];

const fn product(
    label: &'static str,
    package: &'static str,
    features: &'static [&'static str],
) -> OrdinaryProduct {
    OrdinaryProduct {
        label,
        package,
        features,
    }
}

#[test]
fn every_ordinary_product_graph_excludes_legacy_and_certification_authority() {
    for product in ORDINARY_PRODUCTS {
        let tree = ordinary_feature_tree(product);
        inspect_feature_tree(product.label, &tree)
            .unwrap_or_else(|denial| panic!("{denial}\n{tree}"));
    }
}

#[test]
fn every_ordinary_manifest_rejects_forbidden_dependency_edges() {
    for &(manifest, allow_test_json) in ORDINARY_MANIFESTS {
        let path = workspace_root().join(manifest);
        let text = std::fs::read_to_string(&path).expect("read ordinary manifest");
        inspect_ordinary_dependencies(&path, &text, allow_test_json)
            .unwrap_or_else(|denial| panic!("{denial}"));

        for forbidden in ["legacy-s2-models", "certification-test-authority"] {
            let hostile = format!(
                "[dependencies]\nhostile = {{ version = \"1\", features = [\"{forbidden}\"] }}\n"
            );
            inspect_ordinary_dependencies(&path, &hostile, allow_test_json)
                .expect_err("hostile ordinary feature edge must be denied");
        }
    }
}

#[test]
fn feature_tree_gate_rejects_forbidden_activation_in_every_product() {
    for product in ORDINARY_PRODUCTS {
        for forbidden in FORBIDDEN_FEATURES {
            let tree = format!("{} v0.0.0 [{forbidden}]", product.package);
            inspect_feature_tree(product.label, &tree)
                .expect_err("hostile ordinary feature activation must be denied");
        }
    }
}

#[test]
fn manifest_gate_covers_target_dependencies_and_test_json_allowlist() {
    let target_forbidden = r#"
[dependencies]
worth-proof.workspace = true

[target.'cfg(windows)'.dependencies]
json-carrier = { package = "serde_json", version = "1" }
"#;
    assert!(
        inspect_ordinary_dependencies(Path::new("targeted.toml"), target_forbidden, false).is_err()
    );
    assert!(
        inspect_ordinary_dependencies(Path::new("test-support.toml"), target_forbidden, true)
            .is_ok()
    );

    let dev_only = "[dev-dependencies]\nserde_json.workspace = true\n";
    assert!(inspect_ordinary_dependencies(Path::new("dev-only.toml"), dev_only, false).is_ok());
}

fn ordinary_feature_tree(product: &OrdinaryProduct) -> String {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command.current_dir(workspace_root()).args([
        "tree",
        "--manifest-path",
        "Cargo.toml",
        "-p",
        product.package,
        "-e",
        "normal,build",
        "-f",
        "{p} [{f}]",
    ]);
    if !product.features.is_empty() {
        command
            .arg("--no-default-features")
            .arg("--features")
            .arg(product.features.join(","));
    }
    let output = command
        .output()
        .expect("run ordinary product feature-tree audit");
    assert!(
        output.status.success(),
        "{} feature-tree audit failed:\n{}",
        product.label,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("cargo tree output is UTF-8")
}

fn inspect_feature_tree(label: &str, tree: &str) -> Result<(), String> {
    for forbidden in FORBIDDEN_FEATURES {
        if tree.lines().any(|line| {
            line.rsplit_once('[')
                .and_then(|(_, features)| features.strip_suffix(']'))
                .is_some_and(|features| features.split(',').any(|item| item.trim() == *forbidden))
        }) {
            return Err(format!(
                "C.5.1 sealing gate: ordinary {label} activated `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn inspect_ordinary_dependencies(
    path: &Path,
    manifest: &str,
    allow_test_json: bool,
) -> Result<(), String> {
    let dependencies = ordinary_dependency_tables(manifest);
    let mut forbidden_packages = vec![
        ("worth-query", "Query"),
        ("worth_query", "Query"),
        ("worth-relational", "Relational"),
        ("worth_relational", "Relational"),
    ];
    if !allow_test_json {
        forbidden_packages.extend([("serde_json", "JSON"), ("serde-json", "JSON")]);
    }
    for (fragment, authority) in forbidden_packages {
        if dependencies.contains(fragment) {
            return Err(format!(
                "{} has forbidden ordinary {authority} dependency `{fragment}`",
                path.display()
            ));
        }
    }
    for line in dependencies.lines() {
        for (fragment, authority) in [
            ("legacy-s2-models", "legacy S.2"),
            ("legacy-certification-models", "legacy certification"),
            ("certification-test-authority", "certification authority"),
            ("certification-authority", "certification authority"),
        ] {
            if line.contains(fragment) && !line.contains("optional = true") {
                return Err(format!(
                    "{} has forbidden non-optional ordinary {authority} edge `{fragment}`",
                    path.display()
                ));
            }
        }
    }
    Ok(())
}

fn ordinary_dependency_tables(manifest: &str) -> String {
    let mut dependencies = String::new();
    let mut ordinary = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if let Some(table) = trimmed
            .strip_prefix('[')
            .and_then(|table| table.strip_suffix(']'))
        {
            let table = table.trim();
            ordinary = table == "dependencies"
                || (table.starts_with("target.") && table.ends_with(".dependencies"));
        } else if ordinary {
            dependencies.push_str(line);
            dependencies.push('\n');
        }
    }
    dependencies
}
