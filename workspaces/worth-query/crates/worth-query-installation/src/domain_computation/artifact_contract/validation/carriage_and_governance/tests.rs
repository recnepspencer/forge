use worth_foundational::facade::RetentionDeliveryProfile;

use crate::domain_computation_artifact_fixture::{active_compatibility, base_builder};
use crate::facade::*;

#[test]
fn decision_classification_cannot_exceed_its_artifact() {
    let denial = contract(
        WorthQueryArtifactClassification::Internal,
        RetentionDeliveryProfile::Durable,
        WorthQueryArtifactClassification::Restricted,
        RetentionDeliveryProfile::Retained,
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryArtifactContractValidationDenialKind::DecisionGovernanceExceedsArtifact
    );
}

#[test]
fn decision_retention_cannot_outlive_its_artifact() {
    let denial = contract(
        WorthQueryArtifactClassification::Restricted,
        RetentionDeliveryProfile::Retained,
        WorthQueryArtifactClassification::Internal,
        RetentionDeliveryProfile::Durable,
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        WorthQueryArtifactContractValidationDenialKind::DecisionGovernanceExceedsArtifact
    );
}

#[test]
fn stricter_longer_lived_artifact_can_contain_decision_records() {
    contract(
        WorthQueryArtifactClassification::Restricted,
        RetentionDeliveryProfile::Durable,
        WorthQueryArtifactClassification::Confidential,
        RetentionDeliveryProfile::Retained,
    )
    .unwrap();
}

fn contract(
    artifact_classification: WorthQueryArtifactClassification,
    artifact_retention: RetentionDeliveryProfile,
    decision_classification: WorthQueryArtifactClassification,
    decision_retention: RetentionDeliveryProfile,
) -> Result<WorthQueryPortableArtifactContract, WorthQueryArtifactContractValidationDenial> {
    base_builder()
        .decisions(WorthQueryDecisionRecordContract::declared([decision(
            decision_classification,
            decision_retention,
        )]))
        .governance(WorthQueryArtifactGovernanceContract::new(
            ["internal"],
            artifact_classification,
            WorthQueryArtifactRedactionPosture::CanonicalProjectionOnly,
            artifact_retention,
            WorthQueryArtifactDeletionPosture::DeleteAfterRetention,
            WorthQueryArtifactLegalHoldPosture::DomainControlled,
        ))
        .compatibility(active_compatibility())
        .finish()
}

fn decision(
    classification: WorthQueryArtifactClassification,
    retention: RetentionDeliveryProfile,
) -> WorthQueryDecisionSchema {
    WorthQueryDecisionSchema::new(
        WorthQueryDecisionIdentity::new(
            WorthQueryDecisionKind::new("decision").unwrap(),
            WorthQueryDecisionReasonFamily::new("reason").unwrap(),
            WorthQueryArtifactKeyFamily::new("artifact-key").unwrap(),
        ),
        WorthQueryDecisionCausalParentShape::OptionalSingle,
        WorthQueryDecisionPayloadVersion::new(1),
        WorthQueryDecisionGovernance::new(classification, retention),
    )
}
