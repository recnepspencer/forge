use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn public_boundary_seals_measurement_request_authority_surfaces() {
    let cases = [
        (
            "request_constructor_private",
            r#"
use worth_ui_host_contract::{
    UiMeasurementCapabilityGrant, UiMeasurementEvidenceFamily, UiMeasurementRequest,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity, WorthUiHostCapability,
};

fn main() {
    let _request = UiMeasurementRequest {
        identity: UiMeasurementRequestIdentity::new(1),
        family: UiMeasurementRequestFamily::ViewportExtent,
        evidence_family: UiMeasurementEvidenceFamily::ViewportExtent,
        capability_grant: UiMeasurementCapabilityGrant {
            required_capabilities: Box::<[WorthUiHostCapability]>::default(),
        },
        payload: unsafe { std::mem::zeroed() },
    };
}
"#,
            &["UiMeasurementRequest", "private"][..],
        ),
        (
            "capability_grant_private",
            r#"
use worth_ui_host_contract::{UiMeasurementCapabilityGrant, WorthUiHostCapability};

fn main() {
    let _grant = UiMeasurementCapabilityGrant {
        required_capabilities: Box::<[WorthUiHostCapability]>::default(),
    };
}
"#,
            &["UiMeasurementCapabilityGrant", "private"][..],
        ),
        (
            "host_observation_private",
            r#"
use worth_ui_host_contract::{
    UiHostObservation, UiHostObservationValue, UiMeasurementEvidenceFamily,
    UiMeasurementRequestFamily, UiMeasurementRequestIdentity, UiViewportExtentObservation,
};

fn main() {
    let _observation = UiHostObservation {
        request_identity: UiMeasurementRequestIdentity::new(1),
        family: UiMeasurementRequestFamily::ViewportExtent,
        evidence_family: UiMeasurementEvidenceFamily::ViewportExtent,
        value: UiHostObservationValue::ViewportExtent(UiViewportExtentObservation {
            width: 1.0,
            height: 1.0,
        }),
    };
}
"#,
            &["UiHostObservation", "private"][..],
        ),
    ];

    for (name, source, expected_fragments) in cases {
        let stderr = compile_fail_case(name, source);
        for expected in expected_fragments {
            assert!(
                stderr.contains(expected),
                "expected stderr for {name} to contain {expected:?}, got:\n{stderr}"
            );
        }
    }
}

fn compile_fail_case(name: &str, source: &str) -> String {
    let case_root = compile_fail_root().join(name);
    if case_root.exists() {
        std::fs::remove_dir_all(&case_root).expect("remove stale compile-fail case");
    }
    std::fs::create_dir_all(case_root.join("src")).expect("create compile-fail src");

    std::fs::write(case_root.join("Cargo.toml"), cargo_toml()).expect("write Cargo.toml");
    std::fs::write(case_root.join("src/main.rs"), source).expect("write main.rs");

    let output = Command::new(cargo_binary())
        .arg("check")
        .arg("--manifest-path")
        .arg(case_root.join("Cargo.toml"))
        .current_dir(&case_root)
        .output()
        .expect("run cargo check");

    assert!(
        !output.status.success(),
        "expected compile-fail case {name} to fail, but it succeeded"
    );
    String::from_utf8_lossy(&output.stderr).into_owned()
}

fn cargo_toml() -> String {
    format!(
        "[package]\nname = \"worth-ui-host-contract-public-boundary-check\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n\n[dependencies]\nworth-ui-host-contract = {{ path = {:?} }}\n",
        normalized_manifest_dir()
    )
}

fn cargo_binary() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned())
}

fn compile_fail_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("public-boundary-compile-fail");
    std::fs::create_dir_all(&root).expect("create compile fail root");
    root
}

fn normalized_manifest_dir() -> String {
    env!("CARGO_MANIFEST_DIR").replace('\\', "/")
}
