use std::collections::BTreeSet;

use crate::publication::cdc::data::{
    NormalizedContinuationProof, SubscriberBoundaryAssessment, SubscriberContinuationAssessment,
    SubscriberContinuationSummary, SubscriberContractDeclaration, SubscriberRecoveryDisposition,
    SubscriberStreamFailure, SubscriberStreamFailureClass, MAX_NORMALIZED_CONTINUATION_BOUNDARIES,
};
use crate::publication::cdc::diagnostics::continuation_rejection_artifact;
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::data::{
    DescriptorSemanticsVersion, SchemaBoundaryFingerprint, SchemaContinuationClassification,
};

pub(crate) fn select_execution_envelopes(
    source: &[CanonicalCommitEnvelope],
    start_after_position: Option<PatchStreamPosition>,
    max_commits: usize,
) -> Vec<CanonicalCommitEnvelope> {
    source
        .iter()
        .filter(|envelope| {
            start_after_position.is_none_or(|position| envelope.patch.position > position)
        })
        .take(max_commits)
        .cloned()
        .collect()
}

pub(crate) fn assess_subscriber_continuity(
    runtime: &crate::logic::runtime::RelationalRuntime,
    selected_envelopes: &[CanonicalCommitEnvelope],
    subscriber_contract: &SubscriberContractDeclaration,
    prior_proof: &NormalizedContinuationProof,
    fallback_descriptor_semantics_version: DescriptorSemanticsVersion,
) -> Result<SubscriberContinuationAssessment, SubscriberStreamFailure> {
    let mut crossed_boundaries = Vec::new();
    let mut seen_boundaries = BTreeSet::new();
    let mut continuation_outcome = SchemaContinuationClassification::ContinueUnchanged;
    let mut contract_upgrade_applied = false;
    let mut boundary_assessments = Vec::new();

    for envelope in selected_envelopes {
        let Some(descriptor) = &envelope.schema_continuation_descriptor else {
            continue;
        };

        let fingerprint = descriptor.boundary_fingerprint;
        if seen_boundaries.insert(fingerprint) {
            crossed_boundaries.push(fingerprint);
        }

        let contract_consumes_boundary =
            subscriber_contract.consumes_any_strata(&descriptor.bridge.changed_strata);
        let subscriber_specific_outcome = if !contract_consumes_boundary {
            SchemaContinuationClassification::ContinueUnchanged
        } else {
            descriptor.bridge.continuation
        };
        boundary_assessments.push(SubscriberBoundaryAssessment::new(
            fingerprint,
            descriptor.bridge.continuation,
            subscriber_specific_outcome,
            descriptor.bridge.changed_strata.clone(),
            contract_consumes_boundary,
        ));

        match subscriber_specific_outcome {
            SchemaContinuationClassification::ContinueUnchanged => {
                if !subscriber_contract.accepts_continuation(subscriber_specific_outcome) {
                    return Err(unsupported_continuation_failure(
                        SubscriberStreamFailureClass::UnsupportedContinuation,
                        "subscriber contract does not accept unchanged continuation classification",
                        subscriber_contract,
                        &boundary_assessments,
                        prior_proof.normalized_boundary_count(),
                        descriptor_semantics_version(
                            selected_envelopes,
                            fallback_descriptor_semantics_version,
                        ),
                    ));
                }
            }
            SchemaContinuationClassification::ContinueWithTransparentBridge
            | SchemaContinuationClassification::ContinueWithVisibleBridge => {
                if !subscriber_contract.accepts_continuation(subscriber_specific_outcome) {
                    return Err(unsupported_continuation_failure(
                        SubscriberStreamFailureClass::UnsupportedContinuation,
                        format!(
                            "subscriber contract does not accept {:?}",
                            subscriber_specific_outcome
                        ),
                        subscriber_contract,
                        &boundary_assessments,
                        prior_proof.normalized_boundary_count(),
                        descriptor_semantics_version(
                            selected_envelopes,
                            fallback_descriptor_semantics_version,
                        ),
                    ));
                }
            }
            SchemaContinuationClassification::ContinueWithContractUpgrade => {
                if !subscriber_contract.accepts_upgrade(subscriber_specific_outcome) {
                    return Err(unsupported_continuation_failure(
                        SubscriberStreamFailureClass::ContractUpgradeUnsupported,
                        "subscriber contract does not accept contract upgrade continuation",
                        subscriber_contract,
                        &boundary_assessments,
                        prior_proof.normalized_boundary_count(),
                        descriptor_semantics_version(
                            selected_envelopes,
                            fallback_descriptor_semantics_version,
                        ),
                    ));
                }
                contract_upgrade_applied = true;
            }
            SchemaContinuationClassification::RequireRenegotiation => {
                return Err(unsupported_continuation_failure(
                    SubscriberStreamFailureClass::RenegotiationRequired,
                    "schema boundary requires subscriber renegotiation before continuation",
                    subscriber_contract,
                    &boundary_assessments,
                    prior_proof.normalized_boundary_count(),
                    descriptor_semantics_version(
                        selected_envelopes,
                        fallback_descriptor_semantics_version,
                    ),
                ));
            }
            SchemaContinuationClassification::Rejected => {
                return Err(unsupported_continuation_failure(
                    SubscriberStreamFailureClass::SchemaIncompatible,
                    "schema boundary rejected subscriber continuation",
                    subscriber_contract,
                    &boundary_assessments,
                    prior_proof.normalized_boundary_count(),
                    descriptor_semantics_version(
                        selected_envelopes,
                        fallback_descriptor_semantics_version,
                    ),
                ));
            }
        }

        continuation_outcome = strongest_outcome(continuation_outcome, subscriber_specific_outcome);
    }

    let descriptor_semantics_version = selected_envelopes
        .last()
        .map(|envelope| envelope.descriptor_semantics_version)
        .unwrap_or(fallback_descriptor_semantics_version);
    let normalized_continuation_proof = compose_normalized_proof(
        runtime,
        prior_proof,
        &crossed_boundaries,
        descriptor_semantics_version,
        subscriber_contract,
        &boundary_assessments,
    )?;
    let continuation_summary = SubscriberContinuationSummary::new(
        subscriber_contract.contract_id.clone(),
        continuation_outcome,
        crossed_boundaries.len(),
        normalized_continuation_proof.normalized_boundary_count(),
        descriptor_semantics_version,
        contract_upgrade_applied,
    );
    runtime
        .performance_access()
        .count_subscriber_resume_evaluation(continuation_outcome);

    Ok(SubscriberContinuationAssessment::new(
        crossed_boundaries,
        continuation_outcome,
        contract_upgrade_applied,
        normalized_continuation_proof,
        continuation_summary,
        boundary_assessments,
    ))
}

pub(crate) fn disposition_for_assessment(
    start_after_position: Option<PatchStreamPosition>,
    continuation_outcome: SchemaContinuationClassification,
) -> SubscriberRecoveryDisposition {
    match continuation_outcome {
        SchemaContinuationClassification::ContinueUnchanged => {
            if start_after_position.is_some() {
                SubscriberRecoveryDisposition::ResumeAfterCheckpoint
            } else {
                SubscriberRecoveryDisposition::StartFromBeginning
            }
        }
        SchemaContinuationClassification::ContinueWithTransparentBridge => {
            SubscriberRecoveryDisposition::ContinueWithTransparentBridge
        }
        SchemaContinuationClassification::ContinueWithVisibleBridge => {
            SubscriberRecoveryDisposition::ContinueWithVisibleBridge
        }
        SchemaContinuationClassification::ContinueWithContractUpgrade => {
            SubscriberRecoveryDisposition::ContinueWithContractUpgrade
        }
        SchemaContinuationClassification::RequireRenegotiation => {
            SubscriberRecoveryDisposition::RequireRenegotiation
        }
        SchemaContinuationClassification::Rejected => {
            SubscriberRecoveryDisposition::RequireRenegotiation
        }
    }
}

fn compose_normalized_proof(
    runtime: &crate::logic::runtime::RelationalRuntime,
    prior_proof: &NormalizedContinuationProof,
    crossed_boundaries: &[SchemaBoundaryFingerprint],
    descriptor_semantics_version: DescriptorSemanticsVersion,
    subscriber_contract: &SubscriberContractDeclaration,
    boundary_assessments: &[SubscriberBoundaryAssessment],
) -> Result<NormalizedContinuationProof, SubscriberStreamFailure> {
    let mut boundary_fingerprints =
        Vec::with_capacity(prior_proof.boundary_fingerprints().len() + crossed_boundaries.len());
    let mut seen = BTreeSet::new();

    for fingerprint in prior_proof
        .boundary_fingerprints()
        .iter()
        .chain(crossed_boundaries.iter())
        .copied()
    {
        if seen.insert(fingerprint) {
            boundary_fingerprints.push(fingerprint);
        }
    }
    runtime
        .performance_access()
        .count_schema_normalized_descriptor_composition(crossed_boundaries.len());

    if boundary_fingerprints.len() > MAX_NORMALIZED_CONTINUATION_BOUNDARIES {
        return Err(unsupported_continuation_failure(
            SubscriberStreamFailureClass::RenegotiationRequired,
            format!(
                "normalized continuation proof exceeded boundary complexity ceiling of {}",
                MAX_NORMALIZED_CONTINUATION_BOUNDARIES
            ),
            subscriber_contract,
            boundary_assessments,
            boundary_fingerprints.len(),
            descriptor_semantics_version,
        ));
    }

    Ok(NormalizedContinuationProof::new(
        boundary_fingerprints,
        descriptor_semantics_version,
    ))
}

fn strongest_outcome(
    current: SchemaContinuationClassification,
    candidate: SchemaContinuationClassification,
) -> SchemaContinuationClassification {
    if continuation_priority(candidate) > continuation_priority(current) {
        candidate
    } else {
        current
    }
}

fn continuation_priority(classification: SchemaContinuationClassification) -> u8 {
    match classification {
        SchemaContinuationClassification::ContinueUnchanged => 0,
        SchemaContinuationClassification::ContinueWithTransparentBridge => 1,
        SchemaContinuationClassification::ContinueWithVisibleBridge => 2,
        SchemaContinuationClassification::ContinueWithContractUpgrade => 3,
        SchemaContinuationClassification::RequireRenegotiation => 4,
        SchemaContinuationClassification::Rejected => 5,
    }
}

fn unsupported_continuation_failure(
    class: SubscriberStreamFailureClass,
    detail: impl Into<String>,
    subscriber_contract: &SubscriberContractDeclaration,
    boundary_assessments: &[SubscriberBoundaryAssessment],
    normalized_boundary_count_at_failure: usize,
    descriptor_semantics_version: DescriptorSemanticsVersion,
) -> SubscriberStreamFailure {
    let detail = detail.into();
    let continuation_outcome = boundary_assessments
        .last()
        .map(|assessment| assessment.subscriber_outcome)
        .unwrap_or(SchemaContinuationClassification::ContinueUnchanged);
    let contract_upgrade_applied = boundary_assessments.iter().any(|assessment| {
        assessment.subscriber_outcome
            == SchemaContinuationClassification::ContinueWithContractUpgrade
    });
    let assessment = SubscriberContinuationAssessment::new(
        boundary_assessments
            .iter()
            .map(|assessment| assessment.boundary_fingerprint)
            .collect(),
        continuation_outcome,
        contract_upgrade_applied,
        NormalizedContinuationProof::default(),
        SubscriberContinuationSummary::new(
            subscriber_contract.contract_id.clone(),
            continuation_outcome,
            boundary_assessments.len(),
            normalized_boundary_count_at_failure,
            descriptor_semantics_version,
            contract_upgrade_applied,
        ),
        boundary_assessments.to_vec(),
    );
    SubscriberStreamFailure::new(
        class,
        detail.clone(),
        None,
        vec![continuation_rejection_artifact(
            class,
            &detail,
            subscriber_contract,
            &assessment,
            normalized_boundary_count_at_failure,
        )],
    )
}

fn descriptor_semantics_version(
    selected_envelopes: &[CanonicalCommitEnvelope],
    fallback_descriptor_semantics_version: DescriptorSemanticsVersion,
) -> DescriptorSemanticsVersion {
    selected_envelopes
        .last()
        .map(|envelope| envelope.descriptor_semantics_version)
        .unwrap_or(fallback_descriptor_semantics_version)
}
