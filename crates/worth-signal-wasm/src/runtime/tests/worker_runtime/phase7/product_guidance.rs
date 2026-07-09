use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{
    certify_worker_phase7_product_guidance, required_product_guidance_rules,
    WorkerPhase7CompatibilityGuidanceRule, WorkerPhase7ProductGuidanceCertificationPackage,
};

#[test]
fn worker_phase7_product_guidance_certifies_worker_first_recommendation() {
    let package = certify_worker_phase7_product_guidance().unwrap();

    assert_eq!(
        package.certification_family,
        "workerPhase7ProductGuidanceCertification"
    );
    assert_eq!(
        package.guidance_status,
        "WorkerFirstRecommendedWithExplicitCompatibilityLanes"
    );
    assert_eq!(
        package.recommended_default_posture,
        "workerFirstRuntimeOwnedGraph"
    );
    assert_eq!(package.required_guidance_rule_count, 5);
    assert_eq!(package.covered_guidance_rule_count, 5);
    assert!(!package.hidden_fallback_allowed);
    assert!(package.compatibility_guidance_rules.iter().any(|rule| {
        rule.posture == "workerUnavailableCompatibilityMode"
            && rule.required_artifact == "dedicatedWorkerUnavailableCompatibilityArtifact"
            && !rule.hidden_fallback_allowed
    }));
    assert_digest_shape(&package.product_guidance_digest);
    assert_digest_shape(&package.compatibility_guidance_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_phase7_product_guidance_rejects_missing_worker_first_rule() {
    let mut rules = required_product_guidance_rules();
    rules.retain(|rule| rule.posture != "workerFirstRuntimeOwnedGraph");

    let error = WorkerPhase7ProductGuidanceCertificationPackage::from_rules(rules).unwrap_err();

    assert!(error.message.contains("workerFirstRuntimeOwnedGraph"));
}

#[test]
fn worker_phase7_product_guidance_rejects_duplicate_posture() {
    let mut rules = required_product_guidance_rules();
    rules.push(rules[0].clone());

    let error = WorkerPhase7ProductGuidanceCertificationPackage::from_rules(rules).unwrap_err();

    assert!(error.message.contains("duplicate posture"));
}

#[test]
fn worker_phase7_product_guidance_rejects_hidden_fallback() {
    let mut rules = required_product_guidance_rules();
    rules[0].hidden_fallback_allowed = true;

    let error = WorkerPhase7ProductGuidanceCertificationPackage::from_rules(rules).unwrap_err();

    assert!(error.message.contains("non-fallback guidance"));
}

#[test]
fn worker_phase7_product_guidance_rejects_noncanonical_artifact_or_authority() {
    let mut rules = required_product_guidance_rules();
    rules[4] = WorkerPhase7CompatibilityGuidanceRule {
        required_artifact: "genericMainThreadFallback",
        ..rules[4].clone()
    };

    let error = WorkerPhase7ProductGuidanceCertificationPackage::from_rules(rules).unwrap_err();

    assert!(error.message.contains("canonical artifact and authority"));
}

#[test]
fn worker_phase7_product_guidance_rejects_vague_compatibility_guidance() {
    let mut rules = required_product_guidance_rules();
    rules[1] = WorkerPhase7CompatibilityGuidanceRule {
        product_guidance: "Depends on Best Effort host behavior",
        ..rules[1].clone()
    };

    let error = WorkerPhase7ProductGuidanceCertificationPackage::from_rules(rules).unwrap_err();

    assert!(error.message.contains("non-fallback guidance"));
}
