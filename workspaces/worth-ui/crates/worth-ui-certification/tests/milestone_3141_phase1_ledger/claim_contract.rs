pub(super) const BASIC_PLATFORM_VERSIONS: &str = "protocol=4";
pub(super) const PROFILE_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;rustybuzz=0.20.1;swash=0.2.10;protocol=4";
pub(super) const NATIVE_PLATFORM_VERSIONS: &str = "pollster=0.4.0;winit=0.30.13;winit-features=rwh_06;wgpu=29.0.4;wgpu-features=std+parking_lot+dx12+wgsl;xcap=0.9.7;xcap-features=wgc;winsafe=0.0.28;winsafe-features=dwm+kernel+user;uiautomation=0.25.0;uiautomation-features=control+input+screenshot;win32job=2.0.3;protocol=4";
pub(super) const TEXT_PLATFORM_VERSIONS: &str = "harfrust=0.12.0;harfrust-features=std;read-fonts=0.41.0;read-fonts-features=std+experimental_traverse;icu-segmenter=2.2.0;skrifa=0.44.0;skrifa-features=std;kurbo=0.13.1;kurbo-features=default+serde+std;linesweeper=0.4.0;linesweeper-features=none;icu-segmenter-features=compiled_data+auto;unicode-bidi=0.3.18;unicode-bidi-features=std;unicode-segmentation=1.13.3;protocol=5;text-profile=worth-ui-global-text-v2;qualification=closed";

use sha2::{Digest, Sha256};

pub(super) fn baseline_path(requirement: &str) -> Option<&'static str> {
    if requirement.starts_with("P2-")
        || is_p3_native(requirement)
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
    ) || matches!(
        requirement,
        "P3-DELTA-SOURCE-01"
            | "P3-HEADLESS-COST-01"
            | "P3-PRODUCER-SLOPE-01"
            | "P3-RECONSTRUCTION-01"
            | "P3-STALE-DELTA-01"
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
        "P3-PREDECESSOR-01" => "stale-phase-two-source",
        "P3-BASELINE-REPLAY-01" => "opaque-baseline-clear",
        "P3-CLIPPED-DELTA-01" => "zero-paint-as-indeterminate",
        "P3-CLOSE-01" => "open-requirement",
        "P3-DAMAGE-INDEX-01" => "full-retained-scan",
        "P3-DAMAGE-REPLAY-01" => "omitted-vacated-replay",
        "P3-DELTA-SOURCE-01" => "successor-rediscovery",
        "P3-DRAW-LIST-01" => "complete-map-clone",
        "P3-HEADLESS-COST-01" => "complete-transcript-clone",
        "P3-HP02-WORLD-01" => "synthetic-successor",
        "P3-PHYSICAL-AMPLIFICATION-01" => "hidden-full-surface-copy",
        "P3-PRODUCER-SLOPE-01" => "complete-successor-scan",
        "P3-RECONSTRUCTION-01" => "stale-derived-state",
        "P3-STALE-DELTA-01" => "stale-affinity-acceptance",
        "P3-TOTAL-ORDER-01" => "identity-ordering",
        "P3-TRANSACTION-01" => "commit-before-handoff",
        "P3-UNCHANGED-01" => "fresh-unchanged-epoch",
        "P4-FONT-COLLECTION-01" => {
            "ambient-or-single-family-or-stale-generation-or-registration-order-substitution"
        }
        "P4-PREDECESSOR-01" => "stale-phase-three-source",
        "P4-TEXT-PROFILE-01" => "font-or-unicode-digest-drift",
        "P4-COLOR-FONT-ADMISSION-01" => "unsupported-svg-or-layer-drop",
        "P4-UNICODE-SEGMENTATION-01" => "zwj-or-flag-split",
        "P4-EMOJI-SEQUENCE-01" => "variation-or-zwj-decomposition",
        "P4-BIDI-01" => "logical-order-rendering",
        "P4-FALLBACK-01" => "emoji-or-indic-split",
        "P4-SHAPING-01" => "one-run-latin",
        "P4-LINE-LAYOUT-01" => "mid-cluster-wrap",
        "P4-CAPACITY-01" => "shape-before-capacity-denial",
        "P4-MEASUREMENT-IDENTITY-01" => "independent-measurement-pass",
        "P4-ORIGINAL-RANGE-01" => "normalized-offset-substitution",
        "P4-BIDI-INTERACTION-01" => "swapped-bidi-caret-affinity",
        "P4-ACCESSIBILITY-GEOMETRY-01" => "accessibility-reshape",
        "P4-TEXT-CONTENT-LOCALITY-01" => "content-only-global-rescan",
        "P4-TEXT-WIDTH-LOCALITY-01" => "paragraph-width-global-rescan",
        "P4-TEXT-RECONSTRUCTION-01" => "stale-layout-reuse",
        "P4-UNCHANGED-01" => "unchanged-paragraph-rescan",
        "P4-TEXT-COST-01" => "complete-document-rescan",
        "P4-CLOSE-01" => "open-requirement",
        _ => return super::claim_contract_phase5::scenario_delta(requirement),
    })
}

pub(super) fn construction_cost(requirement: &str) -> &'static str {
    if requirement == "P3-PREDECESSOR-01" {
        return "main-tests=21;hostile-controls=12;product-processes=1;compile-sessions=2;courtroom-worlds=2";
    }
    if requirement.starts_with("P3-") {
        if requirement == "P3-CLIPPED-DELTA-01" {
            return "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0";
        }
        if requirement == "P3-HP02-WORLD-01" {
            return "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1;shared-mounted-worlds=1";
        }
        if is_p3_shared_native(requirement) {
            return "main-tests=0;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0;shared-native-worlds=1";
        }
        if is_p3_shared_mixed(requirement) {
            return "main-tests=0;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0;shared-mounted-worlds=1";
        }
        let native = matches!(
            requirement,
            "P3-BASELINE-REPLAY-01"
                | "P3-DAMAGE-REPLAY-01"
                | "P3-DRAW-LIST-01"
                | "P3-HP02-WORLD-01"
                | "P3-PHYSICAL-AMPLIFICATION-01"
                | "P3-TRANSACTION-01"
                | "P3-UNCHANGED-01"
        );
        let mixed = matches!(
            requirement,
            "P3-DELTA-SOURCE-01" | "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01"
        );
        return match (native, mixed) {
            (true, _) => "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1",
            (_, true) => "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=1",
            _ => "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0",
        };
    }
    if let Some(cost) = super::claim_contract_phase5::construction_cost(requirement) {
        return cost;
    }
    if requirement.starts_with("P4-") {
        if requirement == "P4-PREDECESSOR-01" {
            return "main-tests=26;hostile-controls=28;product-processes=3;compile-sessions=2;courtroom-worlds=6";
        }
        if requirement == "P4-FONT-COLLECTION-01" {
            return "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=1;courtroom-worlds=0";
        }
        return "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0";
    }
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
    } else if requirement == "P2-WORLD-01" {
        "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1"
    } else if requirement.starts_with("P2-") || is_p3_native(requirement) {
        "main-tests=0;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0;shared-native-worlds=1"
    } else if requirement == "P1-HEADLESS-COST-01" {
        "main-tests=0;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=0;shared-mounted-worlds=1"
    } else if requirement == "P1-WORLDS-01" {
        "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=1"
    } else {
        "main-tests=1;hostile-controls=0;product-processes=0;compile-sessions=0;courtroom-worlds=0"
    }
}

pub(super) fn execution_cost(requirement: &str) -> &'static str {
    if requirement == "P3-PREDECESSOR-01" {
        return "executed-tests=35;presentations=8";
    }
    if requirement.starts_with("P3-") {
        if requirement == "P3-CLIPPED-DELTA-01" {
            return "executed-tests=2;presentations=0";
        }
        if is_p3_shared_native(requirement) {
            return "executed-tests=1;presentations=0;shared-presentations=7";
        }
        if is_p3_shared_mixed(requirement) {
            return "executed-tests=1;presentations=0;shared-presentations=5";
        }
        return if requirement == "P3-HP02-WORLD-01" {
            "executed-tests=2;presentations=7;shared-presentations=5"
        } else if matches!(
            requirement,
            "P3-BASELINE-REPLAY-01"
                | "P3-DAMAGE-REPLAY-01"
                | "P3-DRAW-LIST-01"
                | "P3-PHYSICAL-AMPLIFICATION-01"
                | "P3-TRANSACTION-01"
                | "P3-UNCHANGED-01"
        ) {
            "executed-tests=2;presentations=7"
        } else if matches!(
            requirement,
            "P3-DELTA-SOURCE-01" | "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01"
        ) {
            "executed-tests=2;presentations=5"
        } else {
            "executed-tests=2;presentations=0"
        };
    }
    if requirement == "P1-HEADLESS-COST-01" {
        "executed-tests=0;presentations=0;shared-presentations=7"
    } else if requirement == "P1-WORLDS-01" {
        "executed-tests=1;presentations=7"
    } else if requirement == "P1-CONSUMERS-01" {
        "executed-tests=2;presentations=0"
    } else if requirement == "P2-WORLD-01" {
        "executed-tests=2;presentations=1"
    } else if requirement.starts_with("P2-") || is_p3_native(requirement) {
        "executed-tests=1;presentations=0;shared-presentations=1"
    } else {
        if let Some(cost) = super::claim_contract_phase5::execution_cost(requirement) {
            cost
        } else if requirement == "P4-PREDECESSOR-01" {
            "executed-tests=56;presentations=28"
        } else if requirement.starts_with("P4-") {
            "executed-tests=2;presentations=0"
        } else {
            "executed-tests=1;presentations=0"
        }
    }
}

pub(super) fn platform_versions(requirement: &str) -> &'static str {
    if requirement.starts_with("P4-") || requirement.starts_with("P5-") {
        TEXT_PLATFORM_VERSIONS
    } else if requirement.starts_with("P2-") || is_p3_native(requirement) {
        NATIVE_PLATFORM_VERSIONS
    } else if requirement == "P1-PROFILE-01" {
        PROFILE_PLATFORM_VERSIONS
    } else {
        BASIC_PLATFORM_VERSIONS
    }
}

pub(super) fn validate_platform_dependencies(requirement: &str) -> Result<(), String> {
    if requirement.starts_with("P4-") || requirement.starts_with("P5-") {
        return validate_text_dependencies();
    }
    if requirement != "P1-PROFILE-01"
        && !requirement.starts_with("P2-")
        && !is_p3_native(requirement)
    {
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

fn validate_text_dependencies() -> Result<(), String> {
    let manifest = std::fs::read_to_string(super::source_digest::repository_file(
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
            return Err(format!("text dependency declaration drifted: {declaration}"));
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

fn is_p3_native(requirement: &str) -> bool {
    matches!(
        requirement,
        "P3-BASELINE-REPLAY-01"
            | "P3-CLIPPED-DELTA-01"
            | "P3-DAMAGE-INDEX-01"
            | "P3-DAMAGE-REPLAY-01"
            | "P3-DRAW-LIST-01"
            | "P3-HP02-WORLD-01"
            | "P3-PHYSICAL-AMPLIFICATION-01"
            | "P3-TOTAL-ORDER-01"
            | "P3-TRANSACTION-01"
            | "P3-UNCHANGED-01"
    )
}

fn is_p3_shared_native(requirement: &str) -> bool {
    matches!(
        requirement,
        "P3-BASELINE-REPLAY-01"
            | "P3-DAMAGE-REPLAY-01"
            | "P3-DRAW-LIST-01"
            | "P3-PHYSICAL-AMPLIFICATION-01"
            | "P3-TRANSACTION-01"
            | "P3-UNCHANGED-01"
    )
}

fn is_p3_shared_mixed(requirement: &str) -> bool {
    matches!(requirement, "P3-HEADLESS-COST-01" | "P3-PRODUCER-SLOPE-01")
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
    let lock = std::fs::read_to_string(super::source_digest::repository_file(
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
