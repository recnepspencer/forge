pub(super) const BASIC_PLATFORM_VERSIONS: &str = "protocol=4";
pub(super) const PROFILE_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;rustybuzz=0.20.1;swash=0.2.10;protocol=4";
pub(super) const NATIVE_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;xcap=0.9.7;xcap-features=wgc;winsafe=0.0.28;winsafe-features=dwm+kernel+user;uiautomation=0.25.0;uiautomation-features=control+input+screenshot;win32job=2.0.3;protocol=4";

use sha2::{Digest, Sha256};

pub(super) fn baseline_path(requirement: &str) -> Option<&'static str> {
    if requirement.starts_with("P2-")
        || requirement.contains("PROFILE")
        || requirement.contains("BACKEND")
    {
        Some("workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-windows-dx12-v1.toml")
    } else if matches!(
        requirement,
        "P1-AFFINITY-01"
            | "P1-BASELINE-01"
            | "P1-CONSUMERS-01"
            | "P1-DAMAGE-01"
            | "P1-HEADLESS-01"
            | "P1-HEADLESS-COST-01"
            | "P1-ORDER-01"
            | "P1-ORDER-SOURCE-01"
            | "P1-PRESENTATION-AUTHORITY-01"
            | "P1-PRODUCER-01"
            | "P1-PRODUCER-COST-01"
            | "P1-PROTOCOL-01"
            | "P1-WORLDS-01"
    ) {
        Some("workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/control_points.toml")
    } else {
        None
    }
}

pub(super) fn baseline_digest(requirement: &str) -> Result<String, String> {
    baseline_path(requirement).map_or_else(
        || {
            Ok(format!(
                "{:x}",
                Sha256::digest(format!("not-applicable:{requirement}"))
            ))
        },
        super::source_digest::file_digest,
    )
}

pub(super) fn scenario_delta(requirement: &str) -> Option<&'static str> {
    Some(match requirement {
        "P1-AFFINITY-01" => "stale-predecessor",
        "P1-AUTHORITY-01" => "public-construction",
        "P1-BACKEND-FEATURES-01" => "vulkan-default",
        "P1-BASELINE-01" => "forged-known-empty",
        "P1-CLOSE-01" => "open-requirement",
        "P1-CONSUMERS-01" => "agreement-validation-bypass",
        "P1-DAMAGE-01" => "widened-damage",
        "P1-HEADLESS-01" => "performed-external-effect",
        "P1-HEADLESS-COST-01" => "unchanged-carriage",
        "P1-ORDER-01" => "identity-tie-break",
        "P1-ORDER-SOURCE-01" => "public-ordering",
        "P1-PLATFORM-AUTHORITY-01" => "downstream-bind",
        "P1-PREPARATION-LIFECYCLE-01" => "host-during-prepare",
        "P1-PRESENTATION-AUTHORITY-01" => "external-work-issue",
        "P1-PRODUCER-01" => "dropped-removal",
        "P1-PRODUCER-COST-01" => "unchanged-payload",
        "P1-PROFILE-01" => "qualified-capacity-drift",
        "P1-PROTOCOL-01" => "mixed-revision",
        "P1-TOPOLOGY-01" => "target-dependency-alias",
        "P1-WORLDS-01" => "damage-and-order-mutants",
        "P2-APPLICATION-01" => "fake-client",
        "P2-CLOSE-01" => "held-readback",
        "P2-EVENT-LOOP-01" => "off-thread-run",
        "P2-GRAPHICS-01" => "vulkan-or-small-limit",
        "P2-PIXELS-01" => "wrong-client-pixel",
        "P2-PORTS-01" => "indeterminate-as-before-effects",
        "P2-PRESENT-01" => "geometry-or-color-drift",
        "P2-READINESS-01" => "duplicate-generation",
        "P2-WINDOW-01" => "dpi-basis-drift",
        "P2-WORLD-01" => "os-backend-client-or-close",
        _ => return None,
    })
}

pub(super) fn construction_cost(requirement: &str) -> &'static str {
    if matches!(
        requirement,
        "P1-AUTHORITY-01"
            | "P1-ORDER-SOURCE-01"
            | "P1-PLATFORM-AUTHORITY-01"
            | "P1-PRESENTATION-AUTHORITY-01"
            | "P1-PROTOCOL-01"
    ) {
        "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=2;courtroom-worlds=0"
    } else if requirement == "P1-CONSUMERS-01" {
        "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0"
    } else if requirement.starts_with("P2-") {
        "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1"
    } else if matches!(requirement, "P1-HEADLESS-COST-01" | "P1-WORLDS-01") {
        "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=1"
    } else {
        "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=0"
    }
}

pub(super) fn execution_cost(requirement: &str) -> &'static str {
    if matches!(requirement, "P1-HEADLESS-COST-01" | "P1-WORLDS-01") {
        "executed-tests=1;presentations=7"
    } else if requirement == "P1-CONSUMERS-01" {
        "executed-tests=2;presentations=0"
    } else if requirement.starts_with("P2-") {
        "executed-tests=2;presentations=1"
    } else {
        "executed-tests=1;presentations=0"
    }
}

pub(super) fn platform_versions(requirement: &str) -> &'static str {
    if requirement.starts_with("P2-") {
        NATIVE_PLATFORM_VERSIONS
    } else if requirement == "P1-PROFILE-01" {
        PROFILE_PLATFORM_VERSIONS
    } else {
        BASIC_PLATFORM_VERSIONS
    }
}

pub(super) fn validate_platform_dependencies(requirement: &str) -> Result<(), String> {
    if requirement != "P1-PROFILE-01" && !requirement.starts_with("P2-") {
        return Ok(());
    }
    let manifest = std::fs::read_to_string(super::source_digest::repository_file(
        "workspaces/worth-ui/Cargo.toml",
    )?)
    .map_err(|error| format!("cannot read platform dependency manifest: {error}"))?;
    for declaration in [
        "pollster = \"=0.4.0\"",
        "uiautomation = { version = \"=0.25.0\", default-features = false, features = [\"control\", \"input\", \"screenshot\"] }",
        "winsafe = { version = \"=0.0.28\", default-features = false, features = [\"dwm\", \"kernel\", \"user\"] }",
        "win32job = \"=2.0.3\"",
        "xcap = { version = \"=0.9.7\", default-features = false }",
        "winit = { version = \"=0.30.13\", default-features = false, features = [\"rwh_06\"] }",
        "wgpu = { version = \"=29.0.4\", default-features = false, features = [\"std\", \"parking_lot\", \"dx12\", \"wgsl\"] }",
    ] {
        if !manifest.contains(declaration) {
            return Err(format!("platform dependency declaration drifted: {declaration}"));
        }
    }
    validate_lock_versions()
}

fn validate_lock_versions() -> Result<(), String> {
    let lock = std::fs::read_to_string(super::source_digest::repository_file(
        "workspaces/worth-ui/Cargo.lock",
    )?)
    .map_err(|error| format!("cannot read platform dependency lock: {error}"))?;
    let lock: toml::Value = toml::from_str(&lock)
        .map_err(|error| format!("invalid platform dependency lock: {error}"))?;
    let packages = lock["package"]
        .as_array()
        .ok_or_else(|| "platform lock omits packages".to_owned())?;
    for (name, version) in [
        ("pollster", "0.4.0"),
        ("uiautomation", "0.25.0"),
        ("winsafe", "0.0.28"),
        ("win32job", "2.0.3"),
        ("xcap", "0.9.7"),
        ("winit", "0.30.13"),
        ("wgpu", "29.0.4"),
    ] {
        let exact = packages.iter().filter(|package| {
            package["name"].as_str() == Some(name) && package["version"].as_str() == Some(version)
        });
        if exact.count() != 1 {
            return Err(format!("platform lock omits exact {name} {version}"));
        }
    }
    Ok(())
}
