pub(crate) const BASIC_PLATFORM_VERSIONS: &str = "protocol=4";
pub(crate) const PROFILE_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;rustybuzz=0.20.1;swash=0.2.10;protocol=4";
pub(crate) const NATIVE_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;xcap=0.9.7;xcap-features=wgc;winsafe=0.0.28;winsafe-features=dwm+kernel+user;uiautomation=0.25.0;uiautomation-features=control+input+screenshot;win32job=2.0.3;protocol=4";
pub(crate) const TEXT_PLATFORM_VERSIONS: &str = "harfrust=0.12.0;harfrust-features=std;read-fonts=0.41.0;read-fonts-features=std+experimental_traverse;icu-segmenter=2.2.0;skrifa=0.44.0;skrifa-features=std;kurbo=0.13.1;kurbo-features=default+serde+std;linesweeper=0.4.0;linesweeper-features=none;icu-segmenter-features=compiled_data+auto;unicode-bidi=0.3.18;unicode-bidi-features=std;unicode-segmentation=1.13.3;protocol=5;text-profile=worth-ui-global-text-v2;qualification=closed";

pub(crate) fn platform_versions(requirement: &str) -> &'static str {
    if requirement.starts_with("P6-") {
        super::super::claim_contract_phase6::NATIVE_PHASE6_PLATFORM_VERSIONS
    } else if requirement.starts_with("P4-") || requirement.starts_with("P5-") {
        TEXT_PLATFORM_VERSIONS
    } else if requirement.starts_with("P2-") || super::is_p3_native(requirement) {
        NATIVE_PLATFORM_VERSIONS
    } else if requirement == "P1-PROFILE-01" {
        PROFILE_PLATFORM_VERSIONS
    } else {
        BASIC_PLATFORM_VERSIONS
    }
}

pub(crate) fn validate_platform_dependencies(requirement: &str) -> Result<(), String> {
    if requirement.starts_with("P6-") {
        return validate_phase6_platform_dependencies();
    }
    if requirement.starts_with("P4-") || requirement.starts_with("P5-") {
        return validate_text_dependencies();
    }
    if requirement != "P1-PROFILE-01"
        && !requirement.starts_with("P2-")
        && !super::is_p3_native(requirement)
    {
        return Ok(());
    }
    let manifest = std::fs::read_to_string(super::super::source_digest::repository_file(
        "workspaces/worth-ui/Cargo.toml",
    )?)
    .map_err(|error| format!("cannot read platform dependency manifest: {error}"))?;
    let historical_declarations = [
        "pollster = \"=0.4.0\"",
        "uiautomation = { version = \"=0.25.0\", default-features = false, features = [\"control\", \"input\", \"screenshot\"] }",
        "winsafe = { version = \"=0.0.28\", default-features = false, features = [\"dwm\", \"kernel\", \"user\"] }",
        "win32job = \"=2.0.3\"",
        "xcap = { version = \"=0.9.7\", default-features = false }",
        "winit = { version = \"=0.30.13\", default-features = false, features = [\"rwh_06\"] }",
        "wgpu = { version = \"=29.0.4\", default-features = false, features = [\"std\", \"parking_lot\", \"dx12\", \"wgsl\"] }",
    ];
    if historical_declarations
        .iter()
        .all(|declaration| manifest.contains(declaration))
    {
        return validate_lock_versions();
    }
    validate_phase6_platform_dependencies()
}

fn validate_phase6_platform_dependencies() -> Result<(), String> {
    let workspace = std::fs::read_to_string(super::super::source_digest::repository_file(
        "workspaces/worth-ui/Cargo.toml",
    )?)
    .map_err(|error| format!("cannot read Phase 6 platform dependency manifest: {error}"))?;
    let host = std::fs::read_to_string(super::super::source_digest::repository_file(
        "workspaces/worth-ui/crates/worth-ui-host-native/Cargo.toml",
    )?)
    .map_err(|error| format!("cannot read Phase 6 host dependency manifest: {error}"))?;
    let pulse = std::fs::read_to_string(super::super::source_digest::repository_file(
        "workspaces/worth-ui/apps/platform-pulse/Cargo.toml",
    )?)
    .map_err(|error| format!("cannot read Phase 6 Pulse dependency manifest: {error}"))?;
    for (manifest, declaration) in [
        (
            &workspace,
            "winsafe = { version = \"=0.0.28\", default-features = false }",
        ),
        (
            &host,
            "winsafe = { version = \"=0.0.28\", default-features = false, features = [\"user\"] }",
        ),
        (
            &pulse,
            "winsafe = { workspace = true, features = [\"dwm\", \"kernel\", \"user\"] }",
        ),
    ] {
        if !manifest.contains(declaration) {
            return Err(format!(
                "Phase 6 dependency declaration drifted: {declaration}"
            ));
        }
    }
    validate_exact_lock_versions(&[("winsafe", "0.0.28"), ("winit", "0.30.13")])
}

fn validate_text_dependencies() -> Result<(), String> {
    let manifest = std::fs::read_to_string(super::super::source_digest::repository_file(
        "workspaces/worth-ui/Cargo.toml",
    )?)
    .map_err(|error| format!("cannot read text dependency manifest: {error}"))?;
    for declaration in [
        "harfrust = { version = \"=0.12.0\", default-features = false, features = [\"std\"] }",
        "read-fonts = { version = \"=0.41.0\", default-features = false, features = [\"std\", \"experimental_traverse\"] }",
        "icu_segmenter = { version = \"=2.2.0\", default-features = false, features = [\"compiled_data\", \"auto\"] }",
        "unicode-bidi = { version = \"=0.3.18\", default-features = false, features = [\"std\"] }",
        "unicode-segmentation = { version = \"=1.13.3\", default-features = false }",
    ] {
        if !manifest.contains(declaration) {
            return Err(format!(
                "text dependency declaration drifted: {declaration}"
            ));
        }
    }
    validate_exact_lock_versions(&[
        ("harfrust", "0.12.0"),
        ("read-fonts", "0.41.0"),
        ("icu_segmenter", "2.2.0"),
        ("unicode-bidi", "0.3.18"),
        ("unicode-segmentation", "1.13.3"),
    ])
}

fn validate_lock_versions() -> Result<(), String> {
    validate_exact_lock_versions(&[
        ("pollster", "0.4.0"),
        ("uiautomation", "0.25.0"),
        ("winsafe", "0.0.28"),
        ("win32job", "2.0.3"),
        ("xcap", "0.9.7"),
        ("winit", "0.30.13"),
        ("wgpu", "29.0.4"),
    ])
}

fn validate_exact_lock_versions(expected: &[(&str, &str)]) -> Result<(), String> {
    let lock = std::fs::read_to_string(super::super::source_digest::repository_file(
        "workspaces/worth-ui/Cargo.lock",
    )?)
    .map_err(|error| format!("cannot read platform dependency lock: {error}"))?;
    let lock: toml::Value = toml::from_str(&lock)
        .map_err(|error| format!("invalid platform dependency lock: {error}"))?;
    let packages = lock["package"]
        .as_array()
        .ok_or_else(|| "platform lock omits packages".to_owned())?;
    for (name, version) in expected {
        let exact = packages.iter().filter(|package| {
            package["name"].as_str() == Some(*name) && package["version"].as_str() == Some(*version)
        });
        if exact.count() != 1 {
            return Err(format!("platform lock omits exact {name} {version}"));
        }
    }
    Ok(())
}
