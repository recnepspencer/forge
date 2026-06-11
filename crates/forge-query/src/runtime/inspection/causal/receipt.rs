use serde_json::Value;

use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryBranchIntentReceiptInspection, ForgeQueryIntentDenialInspection,
    ForgeQueryIntentReceiptInspection, ForgeQueryPreviewOutcomeInspection, ForgeQueryReadReceipt,
    ForgeQueryWriteReceiptInspection,
};

use super::inventory::CausalEvidenceFamily;
use super::observation_identity::{
    CausalEvidenceReferenceDigest, CausalObservationBasisIdentity, CausalObservationQueryIdentity,
    CausalObservationReceiptIdentity, CausalObservationTargetHandle,
    CausalResultShapeContextHandle,
};
use super::receipt_types::{
    CausalObservationEvidenceIdentity, CausalObservationOutcome, ObservationReceiptParts,
    QueryObservationReceipt, QueryObservationReceiptFamily,
};

impl QueryObservationReceipt {
    pub fn from_write_receipt_inspection(inspection: &ForgeQueryWriteReceiptInspection) -> Self {
        let mut evidence_identities = vec![
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_digest(),
                ),
            ),
            CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::RelationalAuthority,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::RelationalAuthority,
                    inspection.commit_identity(),
                ),
            ),
        ];
        if let Some(causality) = inspection.causality_evidence() {
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryMutationCausality,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::QueryMutationCausality,
                    causality.causality_digest(),
                ),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeRoute,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::BridgeRoute,
                    causality.route_digest(),
                ),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::BridgeEvaluation,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::BridgeEvaluation,
                    causality.evaluation_surface_digest(),
                ),
            ));
        }
        if let Some(provenance) = inspection.provenance_evidence() {
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryMutationProvenance,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::QueryMutationProvenance,
                    provenance.feedback_provenance_digest(),
                ),
            ));
            evidence_identities.push(CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::Provenance,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::Provenance,
                    provenance.execution_record_digest(),
                ),
            ));
        }
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::WriteReceipt,
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::WriteReceipt,
                inspection.inspection_digest(),
            ),
            query_identity: write_observation_query_identity(inspection),
            basis_posture: inspection.basis_lane().as_str().to_string(),
            basis_identity: causal_observation_basis_identity(
                inspection.basis_lane().as_str(),
                inspection.snapshot_token(),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                inspection.mutation_family().to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(
                inspection
                    .target_entity_identity()
                    .or_else(|| inspection.declared_entity_identity())
                    .unwrap_or("mutation-target-unspecified")
                    .to_string(),
            ),
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
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::IntentReceipt,
                inspection.receipt_digest(),
            ),
            query_identity: causal_observation_query_identity(
                "intent_receipt",
                inspection.canonical_input_digest(),
            ),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_identity: causal_observation_basis_identity(
                inspection.target_lane().as_str(),
                inspection.snapshot_token(),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                inspection.strategy_descriptor_digest().to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(
                inspection.intent_name().to_string(),
            ),
            outcome,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    causal_evidence_reference_digest(
                        CausalEvidenceFamily::QueryInspection,
                        inspection.inspection_digest(),
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::RelationalAuthority,
                    causal_evidence_reference_digest(
                        CausalEvidenceFamily::RelationalAuthority,
                        inspection.commit_identity(),
                    ),
                ),
            ],
        })
    }

    pub fn from_intent_denial_inspection(inspection: &ForgeQueryIntentDenialInspection) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::IntentDenial,
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::IntentDenial,
                inspection.denial_digest(),
            ),
            query_identity: causal_observation_query_identity(
                "intent_denial",
                inspection.canonical_input_digest(),
            ),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_identity: causal_observation_basis_identity(
                inspection.target_lane().as_str(),
                inspection.snapshot_token().unwrap_or("not-executed"),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                inspection.stage().to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(
                inspection.intent_name().to_string(),
            ),
            outcome: CausalObservationOutcome::Denied,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::QueryInspection,
                    inspection.inspection_digest(),
                ),
            )],
        })
    }

    pub fn from_branch_intent_receipt_inspection(
        inspection: &ForgeQueryBranchIntentReceiptInspection,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::BranchIntentReceipt,
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::BranchIntentReceipt,
                inspection.receipt_digest(),
            ),
            query_identity: causal_observation_query_identity(
                "branch_intent_receipt",
                inspection.canonical_input_digest(),
            ),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_identity: causal_observation_basis_identity(
                inspection.target_lane().as_str(),
                inspection.basis_digest(),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                inspection.admission_digest().to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(
                inspection.intent_name().to_string(),
            ),
            outcome: CausalObservationOutcome::BranchPreview,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    causal_evidence_reference_digest(
                        CausalEvidenceFamily::QueryInspection,
                        inspection.inspection_digest(),
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgePreview,
                    causal_evidence_reference_digest(
                        CausalEvidenceFamily::BridgePreview,
                        inspection.admission_digest(),
                    ),
                ),
            ],
        })
    }

    pub fn from_preview_outcome_inspection(
        inspection: &ForgeQueryPreviewOutcomeInspection,
    ) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::PreviewOutcome,
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::PreviewOutcome,
                inspection.closeout_digest(),
            ),
            query_identity: causal_observation_query_identity(
                "preview_outcome",
                inspection.session_label().identity_digest().as_str(),
            ),
            basis_posture: inspection.target_lane().as_str().to_string(),
            basis_identity: causal_observation_basis_identity(
                inspection.target_lane().as_str(),
                inspection.basis_digest(),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                inspection.residue_digest().to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(
                inspection.session_label().display().to_string(),
            ),
            outcome: CausalObservationOutcome::BranchPreview,
            evidence_identities: vec![
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::QueryInspection,
                    causal_evidence_reference_digest(
                        CausalEvidenceFamily::QueryInspection,
                        inspection.inspection_digest(),
                    ),
                ),
                CausalObservationEvidenceIdentity::new(
                    CausalEvidenceFamily::BridgePreview,
                    causal_evidence_reference_digest(
                        CausalEvidenceFamily::BridgePreview,
                        inspection.closeout_digest(),
                    ),
                ),
            ],
        })
    }

    pub fn from_read_receipt(receipt: &ForgeQueryReadReceipt) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::ReadReceipt,
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::ReadReceipt,
                receipt.result_digest(),
            ),
            query_identity: causal_observation_query_identity(
                "read_receipt",
                receipt.query_digest(),
            ),
            basis_posture: format!("{:?}", receipt.execution_engine()),
            basis_identity: causal_observation_basis_identity(
                "read_execution",
                receipt.basis_digest(),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                receipt.read_graph_digest().to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(
                receipt.result_digest().to_string(),
            ),
            outcome: CausalObservationOutcome::Replayed,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::QueryInspection,
                    receipt.result_digest(),
                ),
            )],
        })
    }

    pub(crate) fn certification_historical_replay_fixture(label: &str) -> Self {
        Self::from_parts(ObservationReceiptParts {
            family: QueryObservationReceiptFamily::Fixture,
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::Fixture,
                format!("fixture-observation-replayed:{label}"),
            ),
            query_identity: causal_observation_query_identity(
                "fixture_replayed",
                format!("fixture-query:{label}"),
            ),
            basis_posture: "historical_replay_certification".to_string(),
            basis_identity: causal_observation_basis_identity(
                "historical_replay_certification",
                format!("fixture-basis:{label}"),
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(format!(
                "fixture-result-shape:{label}"
            )),
            observation_target: CausalObservationTargetHandle::from_rendered(format!(
                "fixture-target:{label}"
            )),
            outcome: CausalObservationOutcome::Replayed,
            evidence_identities: vec![CausalObservationEvidenceIdentity::new(
                CausalEvidenceFamily::QueryInspection,
                causal_evidence_reference_digest(
                    CausalEvidenceFamily::QueryInspection,
                    format!("fixture-query-inspection:{label}"),
                ),
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
            observation_receipt_identity: causal_observation_receipt_identity(
                QueryObservationReceiptFamily::Fixture,
                format!("fixture-observation-{}", outcome.as_str()),
            ),
            query_identity: causal_observation_query_identity("fixture", "fixture-query"),
            basis_posture: "fixture-basis-posture".to_string(),
            basis_identity: causal_observation_basis_identity(
                "fixture-basis-posture",
                "fixture-basis",
            ),
            result_shape_context: CausalResultShapeContextHandle::from_rendered(
                "fixture-result-shape".to_string(),
            ),
            observation_target: CausalObservationTargetHandle::from_rendered(format!(
                "fixture-target-{}",
                outcome.as_str()
            )),
            outcome,
            evidence_identities,
        })
    }
}

fn causal_observation_receipt_identity(
    family: QueryObservationReceiptFamily,
    source_receipt: impl Into<String>,
) -> CausalObservationReceiptIdentity {
    let source_receipt = source_receipt.into();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("source_receipt"),
            &source_receipt,
        )
        .seal()
        .into()
}

fn causal_observation_query_identity(
    family: &str,
    source_query: impl Into<String>,
) -> CausalObservationQueryIdentity {
    let source_query = source_query.into();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationQuery)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family)
        .field_identity(ForgeQueryEvidenceTag::new("source_query"), &source_query)
        .seal()
        .into()
}

fn causal_observation_basis_identity(
    posture: &str,
    basis_digest: impl Into<String>,
) -> CausalObservationBasisIdentity {
    let basis_digest = basis_digest.into();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationBasis)
        .field_shape(ForgeQueryEvidenceTag::new("posture"), posture)
        .field_identity(ForgeQueryEvidenceTag::new("source_basis"), &basis_digest)
        .seal()
        .into()
}

fn causal_evidence_reference_digest(
    family: CausalEvidenceFamily,
    source_reference: impl Into<String>,
) -> CausalEvidenceReferenceDigest {
    let source_reference = source_reference.into();
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalEvidenceReference)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("source_reference"),
            &source_reference,
        )
        .seal()
        .into()
}

fn write_observation_query_identity(
    inspection: &ForgeQueryWriteReceiptInspection,
) -> CausalObservationQueryIdentity {
    let mut encoder =
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::CausalObservationQuery)
            .field_shape(ForgeQueryEvidenceTag::new("family"), "write_receipt")
            .field_shape(
                ForgeQueryEvidenceTag::new("mutation_family"),
                inspection.mutation_family(),
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("basis_lane"),
                inspection.basis_lane().as_str(),
            )
            .field_identity(
                ForgeQueryEvidenceTag::new("snapshot_token"),
                inspection.snapshot_token(),
            );
    if !inspection.mutation_metadata().entries().is_empty() {
        encoder = encoder.field_value_sequence(
            ForgeQueryEvidenceTag::new("metadata_entries"),
            inspection
                .mutation_metadata()
                .entries()
                .iter()
                .flat_map(|(key, value)| [key.to_string(), stable_json_value(value)]),
        );
    }
    encoder.seal().into()
}

fn stable_json_value(value: &Value) -> String {
    serde_json::to_string(value).expect("mutation metadata values are valid JSON")
}
