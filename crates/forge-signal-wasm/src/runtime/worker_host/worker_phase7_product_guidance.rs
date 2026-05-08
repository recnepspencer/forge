use serde::Serialize;

use crate::boundary::errors::ForgeSignalJsError;

use super::canonical_worker_certification_digest;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7ProductGuidanceCertificationPackage {
    pub certification_family: &'static str,
    pub guidance_status: &'static str,
    pub recommended_default_posture: &'static str,
    pub required_guidance_rule_count: u64,
    pub covered_guidance_rule_count: u64,
    pub hidden_fallback_allowed: bool,
    pub compatibility_guidance_rules: Vec<WorkerPhase7CompatibilityGuidanceRule>,
    pub product_guidance_digest: String,
    pub compatibility_guidance_digest: String,
    pub certification_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7CompatibilityGuidanceRule {
    pub posture: &'static str,
    pub when_expected: &'static str,
    pub required_artifact: &'static str,
    pub semantic_authority: &'static str,
    pub hidden_fallback_allowed: bool,
    pub product_guidance: &'static str,
}

pub fn certify_worker_phase7_product_guidance(
) -> Result<WorkerPhase7ProductGuidanceCertificationPackage, ForgeSignalJsError> {
    WorkerPhase7ProductGuidanceCertificationPackage::from_rules(required_product_guidance_rules())
}

impl WorkerPhase7ProductGuidanceCertificationPackage {
    pub(crate) fn from_rules(
        compatibility_guidance_rules: Vec<WorkerPhase7CompatibilityGuidanceRule>,
    ) -> Result<Self, ForgeSignalJsError> {
        reject_missing_guidance_rules(compatibility_guidance_rules.as_slice())?;
        reject_duplicate_guidance_rules(compatibility_guidance_rules.as_slice())?;
        reject_vague_or_hidden_fallback_guidance(compatibility_guidance_rules.as_slice())?;
        reject_noncanonical_guidance_rules(compatibility_guidance_rules.as_slice())?;
        reject_missing_worker_first_recommendation(compatibility_guidance_rules.as_slice())?;

        let product_guidance_digest = canonical_worker_certification_digest(&(
            "workerPhase7ProductGuidance",
            "workerFirstRecommendedForRuntimeOwnedWork",
            "mainThreadOnlyAsTypedHostAuthorityOrHostedCallbackLane",
            "hiddenFallbackForbidden",
        ))?;
        let compatibility_guidance_digest = canonical_worker_certification_digest(&(
            "workerPhase7CompatibilityGuidance",
            &compatibility_guidance_rules,
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerPhase7ProductGuidanceCertification",
            product_guidance_digest.as_str(),
            compatibility_guidance_digest.as_str(),
            compatibility_guidance_rules.len() as u64,
        ))?;

        Ok(Self {
            certification_family: "workerPhase7ProductGuidanceCertification",
            guidance_status: "WorkerFirstRecommendedWithExplicitCompatibilityLanes",
            recommended_default_posture: "workerFirstRuntimeOwnedGraph",
            required_guidance_rule_count: required_guidance_postures().len() as u64,
            covered_guidance_rule_count: compatibility_guidance_rules.len() as u64,
            hidden_fallback_allowed: false,
            compatibility_guidance_rules,
            product_guidance_digest,
            compatibility_guidance_digest,
            certification_digest,
        })
    }
}

pub(crate) fn required_product_guidance_rules() -> Vec<WorkerPhase7CompatibilityGuidanceRule> {
    vec![
        guidance_rule(
            "workerFirstRuntimeOwnedGraph",
            "closed runtime-owned graph state, invalidation, recomputation, observation, diagnostics, branch, and replay work",
            "workerRuntimeIdentity",
            "workerRuntime",
            "Use worker-first deployment for heavy applications; runtime-owned semantics live in the worker-owned Forge proof chain.",
        ),
        guidance_rule(
            "mainThreadHostedCallbackLane",
            "live host closures or browser-only callback capabilities that cannot be serialized into worker executable plans",
            "mainThreadHostedCallbackRequestAndResultEnvelope",
            "workerRuntimeAfterForgeProofReadmission",
            "Keep callback ergonomics by admitting hosted callback request/result envelopes instead of treating closures as portable data.",
        ),
        guidance_rule(
            "explicitPlacementDenial",
            "declarations with unsupported placement, unavailable capability posture, or attempted broad main-thread collapse",
            "placementDenialArtifact",
            "workerPlacementCertification",
            "Show the denial or unavailable artifact to product code; do not silently pin unrelated graph breadth to the main thread.",
        ),
        guidance_rule(
            "typedHostAuthorityBoundary",
            "browser facts, host capabilities, route continuity, or host effects that originate outside worker authority",
            "hostCapabilityOrHostEffectEnvelope",
            "workerRuntimeAfterTypedIngress",
            "Send host facts through typed ingress or acknowledgement envelopes so the worker remains semantic authority after admission.",
        ),
        guidance_rule(
            "workerUnavailableCompatibilityMode",
            "dedicated worker support is absent or explicitly denied by the host environment",
            "dedicatedWorkerUnavailableCompatibilityArtifact",
            "mainThreadCompatibilityRuntimeWithParityDigest",
            "Use compatibility mode only with a visible unavailable artifact and parity digest; it is not a hidden fallback.",
        ),
    ]
}

fn guidance_rule(
    posture: &'static str,
    when_expected: &'static str,
    required_artifact: &'static str,
    semantic_authority: &'static str,
    product_guidance: &'static str,
) -> WorkerPhase7CompatibilityGuidanceRule {
    WorkerPhase7CompatibilityGuidanceRule {
        posture,
        when_expected,
        required_artifact,
        semantic_authority,
        hidden_fallback_allowed: false,
        product_guidance,
    }
}

fn required_guidance_postures() -> Vec<&'static str> {
    required_product_guidance_rules()
        .into_iter()
        .map(|rule| rule.posture)
        .collect()
}

fn reject_missing_guidance_rules(
    rules: &[WorkerPhase7CompatibilityGuidanceRule],
) -> Result<(), ForgeSignalJsError> {
    for required in required_guidance_postures() {
        if !rules.iter().any(|rule| rule.posture == required) {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "worker Phase 7 product guidance requires posture {required}",
            )));
        }
    }
    Ok(())
}

fn reject_duplicate_guidance_rules(
    rules: &[WorkerPhase7CompatibilityGuidanceRule],
) -> Result<(), ForgeSignalJsError> {
    for (index, rule) in rules.iter().enumerate() {
        if rules[(index + 1)..]
            .iter()
            .any(|candidate| candidate.posture == rule.posture)
        {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "worker Phase 7 product guidance duplicate posture {}",
                rule.posture,
            )));
        }
    }
    Ok(())
}

fn reject_noncanonical_guidance_rules(
    rules: &[WorkerPhase7CompatibilityGuidanceRule],
) -> Result<(), ForgeSignalJsError> {
    for required in required_product_guidance_rules() {
        let rule = rules
            .iter()
            .find(|rule| rule.posture == required.posture)
            .ok_or_else(|| {
                ForgeSignalJsError::invalid_input(format!(
                    "worker Phase 7 product guidance requires posture {}",
                    required.posture,
                ))
            })?;
        if rule != &required {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "worker Phase 7 product guidance requires canonical artifact and authority for {}",
                rule.posture,
            )));
        }
    }
    Ok(())
}

fn reject_missing_worker_first_recommendation(
    rules: &[WorkerPhase7CompatibilityGuidanceRule],
) -> Result<(), ForgeSignalJsError> {
    let worker_first = rules
        .iter()
        .find(|rule| rule.posture == "workerFirstRuntimeOwnedGraph")
        .ok_or_else(|| {
            ForgeSignalJsError::invalid_input(
                "worker Phase 7 product guidance requires worker-first recommendation",
            )
        })?;
    if worker_first.semantic_authority != "workerRuntime"
        || !worker_first
            .product_guidance
            .contains("worker-first deployment")
    {
        return Err(ForgeSignalJsError::invalid_input(
            "worker Phase 7 product guidance requires worker-first as the recommended runtime posture",
        ));
    }
    Ok(())
}

fn reject_vague_or_hidden_fallback_guidance(
    rules: &[WorkerPhase7CompatibilityGuidanceRule],
) -> Result<(), ForgeSignalJsError> {
    for rule in rules {
        if rule.hidden_fallback_allowed
            || rule.when_expected.is_empty()
            || rule.required_artifact.is_empty()
            || rule.semantic_authority.is_empty()
            || rule.product_guidance.is_empty()
            || contains_vague_guidance(rule)
        {
            return Err(ForgeSignalJsError::invalid_input(format!(
                "worker Phase 7 product guidance requires explicit non-fallback guidance for {}",
                rule.posture,
            )));
        }
    }
    Ok(())
}

fn contains_vague_guidance(rule: &WorkerPhase7CompatibilityGuidanceRule) -> bool {
    [
        rule.when_expected,
        rule.required_artifact,
        rule.semantic_authority,
        rule.product_guidance,
    ]
    .iter()
    .any(|value| {
        let normalized_value = value.to_ascii_lowercase();
        normalized_value.contains("best effort")
            || normalized_value.contains("best-effort")
            || normalized_value.contains("depends")
            || normalized_value.contains("maybe")
            || normalized_value.contains("implicit")
    })
}
