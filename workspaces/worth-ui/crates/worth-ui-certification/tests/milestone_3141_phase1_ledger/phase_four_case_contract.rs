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

pub(super) fn validate(requirement: &str, artifact: &Value) -> Result<(), String> {
    if requirement != "P4-FONT-COLLECTION-01" {
        return Ok(());
    }
    require_array(artifact, "governed_cases", &owned(FONT_COLLECTION_POSITIVE))?;
    let control = artifact
        .get("hostile_control")
        .filter(|value| value.is_object())
        .ok_or_else(|| "font-collection evidence omits hostile control".to_owned())?;
    require_array(control, "mutation_cases", &owned(FONT_COLLECTION_HOSTILE))
}

fn owned(cases: &[&str]) -> Vec<String> {
    cases.iter().map(|case| (*case).to_owned()).collect()
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{owned, validate, FONT_COLLECTION_HOSTILE, FONT_COLLECTION_POSITIVE};

    fn artifact() -> serde_json::Value {
        json!({
            "governed_cases": owned(FONT_COLLECTION_POSITIVE),
            "hostile_control": {"mutation_cases": owned(FONT_COLLECTION_HOSTILE)},
        })
    }

    #[test]
    fn font_collection_requires_every_exact_positive_and_hostile_case() {
        validate("P4-FONT-COLLECTION-01", &artifact()).unwrap();
        for (field, hostile) in [("governed_cases", false), ("mutation_cases", true)] {
            let mut missing = artifact();
            let target = if hostile {
                missing["hostile_control"].get_mut(field).unwrap()
            } else {
                missing.get_mut(field).unwrap()
            };
            target.as_array_mut().unwrap().pop();
            assert!(validate("P4-FONT-COLLECTION-01", &missing)
                .unwrap_err()
                .contains(field));

            let mut substituted = artifact();
            let target = if hostile {
                substituted["hostile_control"].get_mut(field).unwrap()
            } else {
                substituted.get_mut(field).unwrap()
            };
            target.as_array_mut().unwrap()[0] = json!("cooperative-substitute");
            assert!(validate("P4-FONT-COLLECTION-01", &substituted)
                .unwrap_err()
                .contains(field));
        }
    }
}
