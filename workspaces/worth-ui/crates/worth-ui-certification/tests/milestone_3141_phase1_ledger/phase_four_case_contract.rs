use serde_json::Value;

use super::result_artifact_binding::require_array;

const FONT_COLLECTION_POSITIVE: &[&str] = &[
    "owned-ttf",
    "owned-otf",
    "owned-ttc-multi-index",
    "owned-otc-multi-index",
    "ordered-multi-family-stack",
    "static-regular-bold-italic-oblique",
    "pack-scoped-family-name-collision",
    "variable-weight",
    "variable-width",
    "variable-slant",
    "explicit-opentype-feature",
    "whole-cluster-default-emoji-last-resort-fallback",
    "whole-cluster-khmer-shaping-syllable",
    "independent-per-span-stack",
    "generation-replace-remove-pins-predecessor-bytes",
    "exact-generation-reconstruction",
];

const FONT_COLLECTION_HOSTILE: &[&str] = &[
    "unsupported-web-container",
    "unsupported-aat-shaping-table",
    "unsupported-explicit-feature",
    "registration-order-substitution",
    "malformed-localized-name",
    "malformed-ambiguous-unsupported-over-capacity-pack",
    "generation-exhaustion-alias",
    "same-number-different-lineage",
    "face-definition-order-substitution",
    "worse-face-skips-later-family",
    "variable-axis-range-substitution",
    "missing-unicode-coverage",
    "pack-family-boundary-alias",
];

const ATLAS_POSITIVE: &[&str] = &[
    "exact-signal-basis",
    "independent-model",
    "real-dx12-alpha-color",
    "bounded-capacity",
    "temporal-recovery",
    "terminal-census",
];

const ATLAS_HOSTILE: &[&str] = &[
    "callback-before-effects",
    "partial-upload-indeterminate",
    "replayed-completion",
    "capacity-before-raster",
    "cancellation-recovery",
    "equal-epoch-registration-order",
];

const PINNING_POSITIVE: &[&str] = &[
    "shared-layout-pins",
    "runtime-transaction-owner",
    "native-signal-settlement",
    "pressure-saturation",
    "deterministic-unpinned-replacement",
    "last-owner-release",
    "atlas-capacity-dependency",
    "terminal-census",
];

const PINNING_HOSTILE: &[&str] = &["shared-owner-preservation", "last-owner-release"];

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
        "P4-FONT-COLLECTION-01" => Some((FONT_COLLECTION_POSITIVE, FONT_COLLECTION_HOSTILE)),
        "P5-ATLAS-01" => Some((ATLAS_POSITIVE, ATLAS_HOSTILE)),
        "P5-ATLAS-PINNING-01" => Some((PINNING_POSITIVE, PINNING_HOSTILE)),
        _ => None,
    }
}

fn owned(cases: &[&str]) -> Vec<String> {
    cases.iter().map(|case| (*case).to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{owned, required_cases, validate};

    fn artifact(requirement: &str) -> serde_json::Value {
        let (positive, hostile) = required_cases(requirement).unwrap();
        json!({
            "governed_cases": owned(positive),
            "hostile_control": {"mutation_cases": owned(hostile)},
        })
    }

    #[test]
    fn registered_case_sets_reject_missing_reordered_and_substituted_cases() {
        for requirement in [
            "P4-FONT-COLLECTION-01",
            "P5-ATLAS-01",
            "P5-ATLAS-PINNING-01",
        ] {
            validate(requirement, &artifact(requirement)).unwrap();
            for (field, hostile) in [("governed_cases", false), ("mutation_cases", true)] {
                for mutation in ["missing", "reordered", "substituted"] {
                    let mut evidence = artifact(requirement);
                    let target = if hostile {
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
                        _ => cases[0] = json!("cooperative-substitute"),
                    }
                    assert!(validate(requirement, &evidence)
                        .unwrap_err()
                        .contains(field));
                }
            }
        }
    }
}
