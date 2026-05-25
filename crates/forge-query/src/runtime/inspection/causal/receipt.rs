use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryBranchIntentReceiptInspection, ForgeQueryIntentDenialInspection,
    ForgeQueryIntentReceiptInspection, ForgeQueryPreviewOutcomeInspection, ForgeQueryReadReceipt,
    ForgeQueryWriteReceiptInspection,
};

use super::inventory::CausalEvidenceFamily;
use super::receipt_types::{
    CausalObservationEvidenceIdentity, CausalObservationOutcome, ObservationReceiptParts,
    QueryObservationReceipt, QueryObservationReceiptFamily,
};

impl QueryObservationReceipt {
    pub fn from_write_receipt_inspection(inspection: &ForgeQueryWriteReceiptInspection) -> Self {
        let mut evidence_identities = vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                inspection.inspection_digest(),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::RelationalAuthority,
                inspection.commit_identity(),
            ),
        ];
        if let Some(causality) = inspection.causality_evidence() {
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryMutationCausality,
                causality.causality_digest(),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                causality.route_digest(),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeEvaluation,
                causality.evaluation_surface_digest(),
            ));
        }
        if let Some(provenance) = inspection.provenance_evidence() {
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryMutationProvenance,
                provenance.feedback_provenance_digest(),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::Provenance,
                provenance.execution_record_digest(),
            ));
        }
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::WriteReceipt,
            observation_receipt_digest: inspection.inspection_digest().to_string(),
            query_digest: hash_parts(&[
                "query_observation_write_metadata_v1".to_string(),
                format!("{:?}", inspection.mutation_metadata().entries()),
            ]),
            basis_posture: inspection.basis_lane().as_str().to_string(),
            basis_digest: inspection.snapshot_token().to_string(),
            result_shape_context_digest: inspection.mutation_family().to_string(),
            observation_target_digest: inspection
                .target_entity_identity()
                .or_else(|| inspection.declared_entity_identity())
                .unwrap_or("mutation-target-unspecified")
                .to_string(),
            outcome: CausalObservationOutcome::Changed,
            evidence_identities,
        })
    }

    pub fn from_intent_receipt_inspection(inspection: &ForgeQueryIntentReceiptInspection) -> Self {
        let outcome = if inspection.produced_mutation_digest().is_some() {
            CausalObservationOutcome::Changed
        } else {
            CausalObservationOutcome::Suppressed
        };
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::IntentReceipt,
            observation_receipt_digest: inspection.receipt_digest().to_string(),
            query_digest: inspection.canonical_input_digest().to_string(),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_digest: inspection.snapshot_token().to_string(),
            result_shape_context_digest: inspection.strategy_descriptor_digest().to_string(),
            observation_target_digest: inspection.intent_name().to_string(),
            outcome,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_digest(),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::RelationalAuthority,
                    inspection.commit_identity(),
                ),
            ],
        })
    }

    pub fn from_intent_denial_inspection(inspection: &ForgeQueryIntentDenialInspection) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::IntentDenial,
            observation_receipt_digest: inspection.denial_digest().to_string(),
            query_digest: inspection.canonical_input_digest().to_string(),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_digest: inspection
                .snapshot_token()
                .unwrap_or("not-executed")
                .to_string(),
            result_shape_context_digest: inspection.stage().to_string(),
            observation_target_digest: inspection.intent_name().to_string(),
            outcome: CausalObservationOutcome::Denied,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                inspection.inspection_digest(),
            )],
        })
    }

    pub fn from_branch_intent_receipt_inspection(
        inspection: &ForgeQueryBranchIntentReceiptInspection,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::BranchIntentReceipt,
            observation_receipt_digest: inspection.receipt_digest().to_string(),
            query_digest: inspection.canonical_input_digest().to_string(),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_digest: inspection.basis_digest().to_string(),
            result_shape_context_digest: inspection.admission_digest().to_string(),
            observation_target_digest: inspection.intent_name().to_string(),
            outcome: CausalObservationOutcome::BranchPreview,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_digest(),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgePreview,
                    inspection.admission_digest(),
                ),
            ],
        })
    }

    pub fn from_preview_outcome_inspection(
        inspection: &ForgeQueryPreviewOutcomeInspection,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::PreviewOutcome,
            observation_receipt_digest: inspection.closeout_digest().to_string(),
            query_digest: inspection.label().to_string(),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_digest: inspection.basis_digest().to_string(),
            result_shape_context_digest: inspection.residue_digest().to_string(),
            observation_target_digest: inspection.label().to_string(),
            outcome: CausalObservationOutcome::BranchPreview,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_digest(),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgePreview,
                    inspection.closeout_digest(),
                ),
            ],
        })
    }

    pub fn from_read_receipt(receipt: &ForgeQueryReadReceipt) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::ReadReceipt,
            observation_receipt_digest: receipt.result_digest().to_string(),
            query_digest: receipt.query_digest().to_string(),
            basis_posture: format!("{:?}", receipt.execution_engine()),
            basis_digest: receipt.basis_digest().to_string(),
            result_shape_context_digest: receipt.read_graph_digest().to_string(),
            observation_target_digest: receipt.result_digest().to_string(),
            outcome: CausalObservationOutcome::Replayed,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                receipt.result_digest(),
            )],
        })
    }

    pub(crate) fn certification_historical_replay_fixture(label: &str) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::Fixture,
            observation_receipt_digest: format!("fixture-observation-replayed:{label}"),
            query_digest: format!("fixture-query:{label}"),
            basis_posture: "historical_replay_certification".to_string(),
            basis_digest: format!("fixture-basis:{label}"),
            result_shape_context_digest: format!("fixture-result-shape:{label}"),
            observation_target_digest: format!("fixture-target:{label}"),
            outcome: CausalObservationOutcome::Replayed,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                format!("fixture-query-inspection:{label}"),
            )],
        })
    }

    #[cfg(test)]
    pub(in crate::runtime) fn fixture(
        outcome: CausalObservationOutcome,
        evidence_identities: Vec<CausalObservationEvidenceIdentity>,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::Fixture,
            observation_receipt_digest: format!("fixture-observation-{}", outcome.as_str()),
            query_digest: "fixture-query".to_string(),
            basis_posture: "fixture-basis-posture".to_string(),
            basis_digest: "fixture-basis".to_string(),
            result_shape_context_digest: "fixture-result-shape".to_string(),
            observation_target_digest: format!("fixture-target-{}", outcome.as_str()),
            outcome,
            evidence_identities,
        })
    }
}
