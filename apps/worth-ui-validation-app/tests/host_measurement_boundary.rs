use std::fs;
use std::path::Path;

#[allow(dead_code)]
mod support;

use support::{
    native_boundary_markers::{
        HOST_MEASUREMENT_ADAPTER_CAPABILITIES, HOST_MEASUREMENT_OBSERVATION_TOKENS,
    },
    native_boundary_scanning::rust_files,
};

#[test]
fn host_measurement_reads_stay_in_declared_adapter_capabilities() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut offenders = Vec::new();
    for path in rust_files(&src_root) {
        let relative = relative_path(&path, &src_root);
        let Some(text) = fs::read_to_string(&path).ok() else {
            continue;
        };
        for token in HOST_MEASUREMENT_OBSERVATION_TOKENS {
            if text.contains(token) && !adapter_capability_allows(relative.as_str(), token) {
                offenders.push(format!(
                    "{relative} uses `{token}` outside declared adapter role"
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "host measurements/allocation APIs must stay in declared adapter capabilities: {offenders:?}"
    );
}

#[test]
fn declared_host_measurement_adapter_capabilities_are_used() {
    let src_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let missing: Vec<_> = HOST_MEASUREMENT_ADAPTER_CAPABILITIES
        .iter()
        .filter_map(|capability| {
            let path = src_root.join(capability.file);
            let text = fs::read_to_string(path).expect("source should be readable");
            capability
                .tokens
                .iter()
                .find(|token| !text.contains(**token))
                .map(|token| {
                    format!(
                        "{} role `{}` missing `{token}`",
                        capability.file, capability.role
                    )
                })
        })
        .collect();

    assert!(
        missing.is_empty(),
        "declared adapter capabilities must correspond to real adapter code: {missing:?}"
    );
}

fn relative_path(path: &Path, src_root: &Path) -> String {
    path.strip_prefix(src_root)
        .expect("path should be under src root")
        .display()
        .to_string()
}

fn adapter_capability_allows(file: &str, token: &str) -> bool {
    HOST_MEASUREMENT_ADAPTER_CAPABILITIES
        .iter()
        .any(|capability| capability.file == file && capability.tokens.contains(&token))
}
