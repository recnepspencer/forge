use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryBranchIntentReceiptInspection, WorthQueryIntentDenialInspection,
    WorthQueryIntentReceiptInspection, WorthQueryPreviewOutcomeInspection, WorthQueryReadReceipt,
    WorthQueryWriteReceiptInspection,
};

use super::inventory::CausalEvidenceFamily;
use super::observation_identity::{CausalObservationTargetHandle, CausalResultShapeContextHandle};
use super::receipt_helpers::{
    causal_evidence_reference_identity_digest, causal_observation_basis_evidence_identity,
    causal_observation_query_evidence_identity, causal_observation_receipt_evidence_identity,
    read_observation_query_identity, read_observation_receipt_identity,
    read_observation_result_reference_digest, write_observation_query_identity,
};
use super::receipt_types::{
    CausalObservationBasisPosture, CausalObservationEvidenceIdentity, CausalObservationOutcome,
    ObservationReceiptParts, QueryObservationReceipt, QueryObservationReceiptFamily,
};

impl QueryObservationReceipt {
    pub fn from_write_receipt_inspection(
        inspection: &WorthQueryWriteReceiptInspection,
        inspection_basis: crate::basis_lifecycle::ScopedInspectionBasis,
    ) -> Self {
        let commit_identity = inspection.commit_identity().evidence_identity();
        let mut evidence_identities = vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_identity(),
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::RelationalAuthority,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::RelationalAuthority,
                    &commit_identity,
                ),
            ),
        ];
        if let Some(causality) = inspection.causality_evidence() {
            let causality_digest = causality.causality_digest().evidence_identity();
            let route_digest = causality.route_digest().evidence_identity();
            let evaluation_surface_digest =
                causality.evaluation_surface_digest().evidence_identity();
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryMutationCausality,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::QueryMutationCausality,
                    &causality_digest,
                ),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::BridgeRoute,
                    &route_digest,
                ),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeEvaluation,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::BridgeEvaluation,
                    &evaluation_surface_digest,
                ),
            ));
        }
        if let Some(provenance) = inspection.provenance_evidence() {
            let feedback_provenance_digest =
                provenance.feedback_provenance_digest().evidence_identity();
            let execution_record_digest = provenance.execution_record_digest().evidence_identity();
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryMutationProvenance,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::QueryMutationProvenance,
                    &feedback_provenance_digest,
                ),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::Provenance,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::Provenance,
                    &execution_record_digest,
                ),
            ));
        }
        let snapshot_identity = inspection.snapshot_identity().evidence_identity();
        let target_identity = inspection
            .target_entity_identity()
            .or_else(|| inspection.declared_entity_identity())
            .map(|identity| identity.evidence_identity());
        let fallback_target_identity = target_handle(
            "write_receipt_unspecified_target",
            inspection.inspection_identity(),
            "mutation-target-unspecified",
        );
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::WriteReceipt,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::WriteReceipt,
                inspection.inspection_identity(),
            ),
            query_identity: write_observation_query_identity(inspection),
            basis_posture: CausalObservationBasisPosture::AuthorityLane(inspection.basis_lane()),
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::AuthorityLane(inspection.basis_lane()),
                &snapshot_identity,
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "write_receipt_mutation_family",
                inspection.inspection_identity(),
                inspection.mutation_family(),
            ),
            observation_target: target_identity
                .as_ref()
                .map(CausalObservationTargetHandle::from_evidence_identity)
                .unwrap_or(fallback_target_identity),
            outcome: CausalObservationOutcome::Changed,
            evidence_identities,
        })
    }

    pub fn from_intent_receipt_inspection(
        inspection: &WorthQueryIntentReceiptInspection,
        inspection_basis: crate::basis_lifecycle::ScopedInspectionBasis,
    ) -> Self {
        let snapshot_identity = inspection.snapshot_identity().evidence_identity();
        let commit_identity = inspection.commit_identity().evidence_identity();
        let outcome = if inspection.produced_mutation_digest().is_some() {
            CausalObservationOutcome::Changed
        } else {
            CausalObservationOutcome::Suppressed
        };
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::IntentReceipt,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::IntentReceipt,
                inspection.receipt_identity(),
            ),
            query_identity: causal_observation_query_evidence_identity(
                "intent_receipt",
                inspection.receipt_identity(),
            ),
            basis_posture: CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
                &snapshot_identity,
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "intent_receipt_strategy",
                inspection.receipt_identity(),
                inspection.strategy_descriptor_digest(),
            ),
            observation_target: target_handle(
                "intent_receipt_intent",
                inspection.receipt_identity(),
                inspection.intent_name(),
            ),
            outcome,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    causal_evidence_reference_identity_digest(
                        CausalEvidenceFamily::QueryInspection,
                        inspection.inspection_identity(),
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::RelationalAuthority,
                    causal_evidence_reference_identity_digest(
                        CausalEvidenceFamily::RelationalAuthority,
                        &commit_identity,
                    ),
                ),
            ],
        })
    }

    pub fn from_intent_denial_inspection(
        inspection: &WorthQueryIntentDenialInspection,
        inspection_basis: crate::basis_lifecycle::ScopedInspectionBasis,
    ) -> Self {
        let snapshot_basis_identity = inspection
            .snapshot_evidence_identity()
            .unwrap_or_else(not_executed_snapshot_basis_identity);
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::IntentDenial,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::IntentDenial,
                inspection.denial_identity(),
            ),
            query_identity: causal_observation_query_evidence_identity(
                "intent_denial",
                inspection.denial_identity(),
            ),
            basis_posture: CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
                &snapshot_basis_identity,
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "intent_denial_stage",
                inspection.denial_identity(),
                inspection.stage(),
            ),
            observation_target: target_handle(
                "intent_denial_intent",
                inspection.denial_identity(),
                inspection.intent_name(),
            ),
            outcome: CausalObservationOutcome::Denied,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_identity(),
                ),
            )],
        })
    }

    pub fn from_branch_intent_receipt_inspection(
        inspection: &WorthQueryBranchIntentReceiptInspection,
        inspection_basis: crate::basis_lifecycle::ScopedInspectionBasis,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::BranchIntentReceipt,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::BranchIntentReceipt,
                inspection.receipt_identity(),
            ),
            query_identity: causal_observation_query_evidence_identity(
                "branch_intent_receipt",
                inspection.receipt_identity(),
            ),
            basis_posture: CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
                inspection.basis_identity(),
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "branch_intent_admission",
                inspection.receipt_identity(),
                inspection.admission_digest(),
            ),
            observation_target: target_handle(
                "branch_intent_intent",
                inspection.receipt_identity(),
                inspection.intent_name(),
            ),
            outcome: CausalObservationOutcome::BranchPreview,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    causal_evidence_reference_identity_digest(
                        CausalEvidenceFamily::QueryInspection,
                        inspection.inspection_identity(),
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgePreview,
                    causal_evidence_reference_identity_digest(
                        CausalEvidenceFamily::BridgePreview,
                        inspection.admission_identity(),
                    ),
                ),
            ],
        })
    }

    pub fn from_preview_outcome_inspection(
        inspection: &WorthQueryPreviewOutcomeInspection,
        inspection_basis: crate::basis_lifecycle::ScopedInspectionBasis,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::PreviewOutcome,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::PreviewOutcome,
                inspection.closeout_identity(),
            ),
            query_identity: causal_observation_query_evidence_identity(
                "preview_outcome",
                &inspection.session_label().identity_digest(),
            ),
            basis_posture: CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::AuthorityLane(inspection.target_lane()),
                inspection.basis_identity(),
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "preview_outcome_residue",
                inspection.closeout_identity(),
                inspection.residue_digest(),
            ),
            observation_target: target_handle(
                "preview_outcome_session",
                inspection.closeout_identity(),
                inspection.session_label().display(),
            ),
            outcome: CausalObservationOutcome::BranchPreview,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    causal_evidence_reference_identity_digest(
                        CausalEvidenceFamily::QueryInspection,
                        inspection.inspection_identity(),
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgePreview,
                    causal_evidence_reference_identity_digest(
                        CausalEvidenceFamily::BridgePreview,
                        inspection.closeout_identity(),
                    ),
                ),
            ],
        })
    }

    pub fn from_read_receipt(
        receipt: &WorthQueryReadReceipt,
        inspection_basis: crate::basis_lifecycle::ScopedInspectionBasis,
    ) -> Self {
        let snapshot_evidence_identity = receipt.snapshot_evidence_identity();
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::ReadReceipt,
            observation_receipt_identity: read_observation_receipt_identity(receipt),
            query_identity: read_observation_query_identity(receipt, &snapshot_evidence_identity),
            basis_posture: CausalObservationBasisPosture::ReadExecution(
                receipt.execution_engine().clone(),
            ),
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::ReadExecution(receipt.execution_engine().clone()),
                &snapshot_evidence_identity,
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "read_receipt_graph",
                &snapshot_evidence_identity,
                receipt.read_graph_digest(),
            ),
            observation_target: target_handle(
                "read_receipt_result",
                &snapshot_evidence_identity,
                receipt.result_digest(),
            ),
            outcome: CausalObservationOutcome::Replayed,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                read_observation_result_reference_digest(receipt, &snapshot_evidence_identity),
            )],
        })
    }

    pub(crate) fn certification_historical_replay_fixture(label: &str) -> Self {
        let inspection_basis = crate::basis_lifecycle::basis_lifecycle()
            .historical_snapshot(format!("fixture-inspection:{label}"), true)
            .inspect()
            .expect("certification historical inspection basis should admit");
        let fixture_authority = fixture_authority_identity(label);
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::Fixture,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::Fixture,
                &fixture_component_identity(label, "observation_receipt", "replayed"),
            ),
            query_identity: causal_observation_query_evidence_identity(
                "fixture_replayed",
                &fixture_component_identity(label, "query", "historical_replay"),
            ),
            basis_posture: CausalObservationBasisPosture::HistoricalReplayCertification,
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::HistoricalReplayCertification,
                &fixture_component_identity(label, "basis", "historical_replay_certification"),
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "fixture_result_shape",
                &fixture_authority,
                format!("fixture-result-shape:{label}"),
            ),
            observation_target: target_handle(
                "fixture_target",
                &fixture_authority,
                format!("fixture-target:{label}"),
            ),
            outcome: CausalObservationOutcome::Replayed,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_identity_digest(
                    CausalEvidenceFamily::QueryInspection,
                    &fixture_component_identity(label, "evidence_reference", "query_inspection"),
                ),
            )],
        })
    }

    #[cfg(test)]
    pub(in crate::runtime) fn fixture(
        outcome: CausalObservationOutcome,
        evidence_identities: Vec<CausalObservationEvidenceIdentity>,
    ) -> Self {
        let inspection_basis = fixture_inspection_basis(outcome);
        let fixture_authority = fixture_authority_identity(outcome.as_str());
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::Fixture,
            observation_receipt_identity: causal_observation_receipt_evidence_identity(
                QueryObservationReceiptFamily::Fixture,
                &fixture_component_identity(outcome.as_str(), "observation_receipt", "fixture"),
            ),
            query_identity: causal_observation_query_evidence_identity(
                "fixture",
                &fixture_component_identity(outcome.as_str(), "query", "fixture"),
            ),
            basis_posture: CausalObservationBasisPosture::Fixture,
            basis_identity: causal_observation_basis_evidence_identity(
                &CausalObservationBasisPosture::Fixture,
                &fixture_component_identity(outcome.as_str(), "basis", "fixture"),
            ),
            inspection_basis,
            result_shape_context: result_shape_handle(
                "fixture_result_shape",
                &fixture_authority,
                "fixture-result-shape",
            ),
            observation_target: target_handle(
                "fixture_target",
                &fixture_authority,
                format!("fixture-target-{}", outcome.as_str()),
            ),
            outcome,
            evidence_identities,
        })
    }
}

#[cfg(test)]
pub(in crate::runtime) fn fixture_inspection_basis(
    outcome: CausalObservationOutcome,
) -> crate::basis_lifecycle::ScopedInspectionBasis {
    let lifecycle = crate::basis_lifecycle::basis_lifecycle();
    match outcome {
        CausalObservationOutcome::BranchPreview => lifecycle.preview("fixture-preview").inspect(),
        CausalObservationOutcome::Replayed => lifecycle
            .historical_snapshot("fixture-history", true)
            .inspect(),
        _ => lifecycle.current_head().inspect(),
    }
    .expect("fixture inspection basis should admit")
}

fn not_executed_snapshot_basis_identity() -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::CausalObservationBasis)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "not_executed_snapshot_basis_v1",
        )
        .field_shape(
            WorthQueryEvidenceTag::new("execution_state"),
            "not-executed",
        )
        .seal()
}

fn fixture_authority_identity(label: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::CausalQueryObservationReceipt)
        .field_shape(WorthQueryEvidenceTag::new("role"), "fixture_authority")
        .field_value(WorthQueryEvidenceTag::new("label"), label)
        .seal()
}

fn fixture_component_identity(
    label: &str,
    component: &'static str,
    descriptor: &'static str,
) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::CausalQueryObservationReceipt)
        .field_shape(WorthQueryEvidenceTag::new("role"), "fixture_component")
        .field_shape(WorthQueryEvidenceTag::new("component"), component)
        .field_value(WorthQueryEvidenceTag::new("label"), label)
        .field_value(WorthQueryEvidenceTag::new("descriptor"), descriptor)
        .seal()
}

fn result_shape_handle(
    role: &'static str,
    authority_identity: &WorthQueryEvidenceIdentity,
    descriptor: impl AsRef<str>,
) -> CausalResultShapeContextHandle {
    let identity =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::CausalResultShapeContext)
            .field_shape(WorthQueryEvidenceTag::new("role"), role)
            .field_evidence_identity(WorthQueryEvidenceTag::new("authority"), authority_identity)
            .field_value(
                WorthQueryEvidenceTag::new("descriptor"),
                descriptor.as_ref(),
            )
            .seal();
    CausalResultShapeContextHandle::from_evidence_identity(&identity)
}

fn target_handle(
    role: &'static str,
    authority_identity: &WorthQueryEvidenceIdentity,
    descriptor: impl AsRef<str>,
) -> CausalObservationTargetHandle {
    let identity =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::CausalObservationTarget)
            .field_shape(WorthQueryEvidenceTag::new("role"), role)
            .field_evidence_identity(WorthQueryEvidenceTag::new("authority"), authority_identity)
            .field_value(
                WorthQueryEvidenceTag::new("descriptor"),
                descriptor.as_ref(),
            )
            .seal();
    CausalObservationTargetHandle::from_evidence_identity(&identity)
}
