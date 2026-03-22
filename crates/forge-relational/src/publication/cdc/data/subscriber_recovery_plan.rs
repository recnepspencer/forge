use crate::publication::cdc::data::{
    NormalizedContinuationProof, SubscriberCheckpoint, SubscriberContinuationSummary,
    SubscriberRecoveryDecision, SubscriberResumeRequest,
};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::CanonicalCommitEnvelope;
use crate::schema::data::{
    DescriptorSemanticsVersion, SchemaBoundaryFingerprint, SchemaContinuationClassification,
};
use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use serde_json::json;
use super::{SubscriberContractDeclaration, SubscriberStreamFailureClass};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriberRecoveryPlan {
    pub(crate) request: SubscriberResumeRequest,
    pub(crate) decision: SubscriberRecoveryDecision,
    pub(crate) latest_available_checkpoint: Option<SubscriberCheckpoint>,
    pub(crate) start_after_position: Option<PatchStreamPosition>,
    pub(crate) selected_envelopes: Vec<CanonicalCommitEnvelope>,
    pub(crate) continuation_assessment: SubscriberContinuationAssessment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriberContinuationAssessment {
    pub(crate) crossed_boundaries: Vec<SchemaBoundaryFingerprint>,
    pub(crate) continuation_outcome: SchemaContinuationClassification,
    pub(crate) contract_upgrade_applied: bool,
    pub(crate) normalized_continuation_proof: NormalizedContinuationProof,
    pub(crate) continuation_summary: SubscriberContinuationSummary,
    pub(crate) boundary_assessments: Vec<SubscriberBoundaryAssessment>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriberBoundaryAssessment {
    pub(crate) boundary_fingerprint: SchemaBoundaryFingerprint,
    pub(crate) descriptor_continuation: SchemaContinuationClassification,
    pub(crate) subscriber_outcome: SchemaContinuationClassification,
    pub(crate) changed_strata: Vec<crate::schema::data::SchemaStratum>,
    pub(crate) contract_consumes_boundary: bool,
}

impl SubscriberContinuationAssessment {
    pub(crate) fn new(
        crossed_boundaries: Vec<SchemaBoundaryFingerprint>,
        continuation_outcome: SchemaContinuationClassification,
        contract_upgrade_applied: bool,
        normalized_continuation_proof: NormalizedContinuationProof,
        continuation_summary: SubscriberContinuationSummary,
        boundary_assessments: Vec<SubscriberBoundaryAssessment>,
    ) -> Self {
        Self {
            crossed_boundaries,
            continuation_outcome,
            contract_upgrade_applied,
            normalized_continuation_proof,
            continuation_summary,
            boundary_assessments,
        }
    }

    pub(crate) fn unchanged(
        contract_id: String,
        descriptor_semantics_version: DescriptorSemanticsVersion,
    ) -> Self {
        Self::new(
            Vec::new(),
            SchemaContinuationClassification::ContinueUnchanged,
            false,
            crate::publication::cdc::data::NormalizedContinuationProof::new(
                Vec::new(),
                descriptor_semantics_version,
            ),
            crate::publication::cdc::data::SubscriberContinuationSummary::unchanged(
                contract_id,
                descriptor_semantics_version,
            ),
            Vec::new(),
        )
    }

    pub(crate) fn to_summary_artifact(
        &self,
        contract_id: &str,
    ) -> RelationalDiagnosticArtifact {
        let mut entries = vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::SubscriberContractEvaluated,
            message: "subscriber continuation assessment completed".to_string(),
            fields: json!({
                "subscriber_contract_id": contract_id,
                "continuation_outcome": format!("{:?}", self.continuation_outcome),
                "crossed_boundary_count": self.crossed_boundaries.len(),
                "normalized_boundary_count": self
                    .normalized_continuation_proof
                    .normalized_boundary_count(),
                "contract_upgrade_applied": self.contract_upgrade_applied,
            }),
        }];
        if self.contract_upgrade_applied {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::SubscriberContractUpgradeDecision,
                message: "subscriber continuation applied declared contract upgrade support"
                    .to_string(),
                fields: json!({
                    "subscriber_contract_id": contract_id,
                    "continuation_outcome": format!("{:?}", self.continuation_outcome),
                    "normalized_boundary_count": self
                        .normalized_continuation_proof
                        .normalized_boundary_count(),
                }),
            });
        }
        if self.continuation_outcome
            == SchemaContinuationClassification::RequireRenegotiation
        {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::SubscriberRenegotiationDecision,
                message: "subscriber continuation requires explicit renegotiation".to_string(),
                fields: json!({
                    "subscriber_contract_id": contract_id,
                    "normalized_boundary_count": self
                        .normalized_continuation_proof
                        .normalized_boundary_count(),
                }),
            });
        }
        entries.extend(self.boundary_assessments.iter().map(|assessment| {
            assessment.to_diagnostic_entry()
        }));
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Replay,
            kind: DiagnosticsArtifactKind::MinimalSummary,
            determinism: DeterminismExpectation::Required,
            entries,
        }
    }

    pub(crate) fn to_rejection_artifact(
        &self,
        class: SubscriberStreamFailureClass,
        detail: &str,
        subscriber_contract: &SubscriberContractDeclaration,
        normalized_boundary_count_at_failure: usize,
    ) -> RelationalDiagnosticArtifact {
        let mut entries = vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::SubscriberContractEvaluated,
            message: "subscriber continuation assessment rejected".to_string(),
            fields: json!({
                "failure_class": format!("{:?}", class),
                "detail": detail,
                "subscriber_contract_id": subscriber_contract.contract_id,
                "accepted_continuation_classes": subscriber_contract
                    .accepted_continuation_classes
                    .iter()
                    .map(|classification| format!("{:?}", classification))
                    .collect::<Vec<_>>(),
                "accepted_upgrade_classes": subscriber_contract
                    .accepted_upgrade_classes
                    .iter()
                    .map(|classification| format!("{:?}", classification))
                    .collect::<Vec<_>>(),
                "consumable_strata": subscriber_contract
                    .consumable_strata
                    .iter()
                    .map(|stratum| format!("{:?}", stratum))
                    .collect::<Vec<_>>(),
                "normalized_boundary_count_at_failure": normalized_boundary_count_at_failure,
            }),
        }];
        if class == SubscriberStreamFailureClass::ContractUpgradeUnsupported {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::SubscriberContractUpgradeDecision,
                message: "subscriber continuation rejected because contract upgrade support was not declared"
                    .to_string(),
                fields: json!({
                    "subscriber_contract_id": subscriber_contract.contract_id,
                    "normalized_boundary_count_at_failure": normalized_boundary_count_at_failure,
                }),
            });
        }
        if class == SubscriberStreamFailureClass::RenegotiationRequired {
            entries.push(RelationalDiagnosticsEntry {
                code: DiagnosticCode::SubscriberRenegotiationDecision,
                message: "subscriber continuation rejected because renegotiation is required"
                    .to_string(),
                fields: json!({
                    "subscriber_contract_id": subscriber_contract.contract_id,
                    "normalized_boundary_count_at_failure": normalized_boundary_count_at_failure,
                }),
            });
        }
        entries.extend(self.boundary_assessments.iter().map(|assessment| {
            assessment.to_rejection_diagnostic_entry(subscriber_contract)
        }));
        RelationalDiagnosticArtifact {
            scope: DiagnosticsScope::Replay,
            kind: DiagnosticsArtifactKind::Failure,
            determinism: DeterminismExpectation::Required,
            entries,
        }
    }
}

impl SubscriberBoundaryAssessment {
    pub(crate) fn new(
        boundary_fingerprint: SchemaBoundaryFingerprint,
        descriptor_continuation: SchemaContinuationClassification,
        subscriber_outcome: SchemaContinuationClassification,
        changed_strata: Vec<crate::schema::data::SchemaStratum>,
        contract_consumes_boundary: bool,
    ) -> Self {
        Self {
            boundary_fingerprint,
            descriptor_continuation,
            subscriber_outcome,
            changed_strata,
            contract_consumes_boundary,
        }
    }

    pub(crate) fn to_diagnostic_entry(&self) -> RelationalDiagnosticsEntry {
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SubscriberBoundaryEvaluated,
            message: "subscriber boundary assessed against declared contract".to_string(),
            fields: json!({
                "boundary_fingerprint": self.boundary_fingerprint.0,
                "descriptor_continuation": format!("{:?}", self.descriptor_continuation),
                "subscriber_outcome": format!("{:?}", self.subscriber_outcome),
                "changed_strata": self
                    .changed_strata
                    .iter()
                    .map(|stratum| format!("{:?}", stratum))
                    .collect::<Vec<_>>(),
                "contract_consumes_boundary": self.contract_consumes_boundary,
            }),
        }
    }

    pub(crate) fn to_rejection_diagnostic_entry(
        &self,
        subscriber_contract: &SubscriberContractDeclaration,
    ) -> RelationalDiagnosticsEntry {
        RelationalDiagnosticsEntry {
            code: DiagnosticCode::SubscriberBoundaryEvaluated,
            message: "subscriber boundary rejected against declared contract".to_string(),
            fields: json!({
                "boundary_fingerprint": self.boundary_fingerprint.0,
                "descriptor_continuation": format!("{:?}", self.descriptor_continuation),
                "subscriber_outcome": format!("{:?}", self.subscriber_outcome),
                "changed_strata": self
                    .changed_strata
                    .iter()
                    .map(|stratum| format!("{:?}", stratum))
                    .collect::<Vec<_>>(),
                "contract_consumes_boundary": self.contract_consumes_boundary,
                "accepted_continuation": subscriber_contract
                    .accepted_continuation_classes
                    .contains(&self.subscriber_outcome),
                "accepted_upgrade": subscriber_contract
                    .accepted_upgrade_classes
                    .contains(&self.subscriber_outcome),
            }),
        }
    }
}

impl SubscriberRecoveryPlan {
    pub(crate) fn new(
        request: SubscriberResumeRequest,
        decision: SubscriberRecoveryDecision,
        latest_available_checkpoint: Option<SubscriberCheckpoint>,
        start_after_position: Option<PatchStreamPosition>,
        selected_envelopes: Vec<CanonicalCommitEnvelope>,
        continuation_assessment: SubscriberContinuationAssessment,
    ) -> Self {
        Self {
            request,
            decision,
            latest_available_checkpoint,
            start_after_position,
            selected_envelopes,
            continuation_assessment,
        }
    }
}
