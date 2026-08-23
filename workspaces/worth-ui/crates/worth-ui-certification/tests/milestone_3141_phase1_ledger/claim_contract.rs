#[path = "platform_dependencies.rs"]
mod platform_dependencies;

pub(super) use platform_dependencies::{
    platform_versions, validate_platform_dependencies, TEXT_PLATFORM_VERSIONS,
};

use sha2::{Digest, Sha256};

pub(super) fn baseline_path(requirement: &str) -> Option<&'static str> {
    if requirement.starts_with("P2-")
        || is_p3_native(requirement)
        || requirement.starts_with("P6-")
        || requirement.contains("PROFILE")
        || requirement.contains("BACKEND")
    {
        Some(
            "workspaces/worth-ui/crates/worth-ui-host-native/profiles/worth-ui-windows-dx12-v1.toml",
        )
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
        Some(
            "workspaces/worth-ui/crates/worth-ui-certification/tests/application_contracts/host_platform/control_points.toml",
        )
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
        _ => {
            return super::claim_contract_phase6::scenario_delta(requirement)
                .or_else(|| super::claim_contract_phase5::scenario_delta(requirement));
        }
    })
}

pub(super) fn construction_cost(requirement: &str) -> &'static str {
    if let Some(cost) = super::claim_contract_phase6::construction_cost(requirement) {
        return cost;
    }
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
            (true, _) => {
                "main-tests=1;hostile-controls=1;product-processes=1;compile-sessions=0;courtroom-worlds=1"
            }
            (_, true) => {
                "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=1"
            }
            _ => {
                "main-tests=1;hostile-controls=1;product-processes=0;compile-sessions=0;courtroom-worlds=0"
            }
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
    if let Some(cost) = super::claim_contract_phase6::execution_cost(requirement) {
        return cost;
    }
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
