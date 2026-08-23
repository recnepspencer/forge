use serde_json::Value;

use super::result_artifact_binding::require_array;

const GLYPH_POSITIVE: &[&str] = &[
    "exact-demand-identity",
    "fractional-origin",
    "variable-outline",
    "last-resort-outline",
    "cross-layout-raster-reuse",
    "qualified-alpha-color-batches",
];
const GLYPH_HOSTILE: &[&str] = &["consumer-reshape", "ambient-system-font"];
const COLOR_POSITIVE: &[&str] = &[
    "colrv0-cpal",
    "colrv1-cpal",
    "cbdt-cblc",
    "sbix-png-dupe",
    "selector-lane",
    "exhaustive-rgi",
    "gradient-composite",
    "nonseparable-composite",
    "bitmap-composite",
];
const COLOR_HOSTILE: &[&str] = &[
    "foreground-tint",
    "cluster-split",
    "source-substitution",
    "malformed-graph",
    "unsupported-bitmap",
    "unbounded-current-color",
];
const ATLAS_POSITIVE: &[&str] = &[
    "exact-signal-basis",
    "independent-model",
    "real-dx12-alpha-color",
    "bounded-capacity",
    "temporal-recovery",
    "retry-correlation",
    "retained-content-extent",
    "production-supersession",
    "terminal-census",
];
const ATLAS_HOSTILE: &[&str] = &[
    "callback-before-effects",
    "partial-upload-indeterminate",
    "replayed-completion",
    "capacity-before-raster",
    "cancellation-recovery",
    "equal-epoch-registration-order",
    "alpha-color-owner-merger",
];
const PIN_POSITIVE: &[&str] = &[
    "shared-layout-pins",
    "runtime-transaction-owner",
    "native-signal-settlement",
    "alpha-color-event-loop-progression",
    "last-owner-release",
    "preclose-pin-transition",
    "terminal-census",
];
const PIN_HOSTILE: &[&str] = &["shared-owner-preservation", "last-owner-release"];

pub(super) fn validate(requirement: &str, artifact: &Value) -> Result<(), String> {
    let Some((positive, hostile)) = required_cases(requirement) else {
        return Ok(());
    };
    require_array(artifact, "governed_cases", &owned(positive))?;
    let control = artifact
        .get("hostile_control")
        .filter(|value| value.is_object())
        .ok_or_else(|| format!("{requirement} evidence omits hostile control"))?;
    require_array(control, "mutation_cases", &owned(hostile))
}

fn required_cases(requirement: &str) -> Option<(&'static [&'static str], &'static [&'static str])> {
    match requirement {
        "P5-GLYPH-RASTER-01" => Some((GLYPH_POSITIVE, GLYPH_HOSTILE)),
        "P5-COLOR-EMOJI-01" => Some((COLOR_POSITIVE, COLOR_HOSTILE)),
        "P5-ATLAS-01" => Some((ATLAS_POSITIVE, ATLAS_HOSTILE)),
        "P5-ATLAS-PINNING-01" => Some((PIN_POSITIVE, PIN_HOSTILE)),
        _ => None,
    }
}

fn owned(cases: &[&str]) -> Vec<String> {
    cases.iter().map(|case| (*case).to_owned()).collect()
}

#[test]
fn registered_phase_five_cases_reject_missing_reordered_and_substituted_values() {
    for requirement in [
        "P5-GLYPH-RASTER-01",
        "P5-COLOR-EMOJI-01",
        "P5-ATLAS-01",
        "P5-ATLAS-PINNING-01",
    ] {
        let (positive, hostile) = required_cases(requirement).unwrap();
        let baseline = serde_json::json!({"governed_cases": owned(positive), "hostile_control": {"mutation_cases": owned(hostile)}});
        validate(requirement, &baseline).unwrap();
        for (field, nested) in [("governed_cases", false), ("mutation_cases", true)] {
            for mutation in ["missing", "reordered", "substituted"] {
                let mut evidence = baseline.clone();
                let target = if nested {
                    evidence["hostile_control"].get_mut(field).unwrap()
                } else {
                    evidence.get_mut(field).unwrap()
                };
                let cases = target.as_array_mut().unwrap();
                match mutation {
                    "missing" => {
                        cases.pop();
                    }
                    "reordered" => cases.reverse(),
                    _ => cases[0] = serde_json::json!("substitute"),
                }
                assert!(validate(requirement, &evidence).is_err());
            }
        }
    }
}
