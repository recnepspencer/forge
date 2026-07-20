#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CausalInspectionRepresentativeKind {
    ChangedResult,
    SuppressedResult,
    QueryDeniedBeforeBridgeEnvelope,
    AdvisoryRedactedCausalEnvelope,
    PolicyRedacted,
    BranchPreview,
    HistoricalReplay,
    WorthStyleQueryOnlyConsumer,
    BridgeRouteAndSignalEvidenceBindSameObservation,
    ObservationAnchorBindsOneQueryReceipt,
    BridgeRecordsBindThroughExistingDiagnostics,
    SignalForensicAvailabilityAndReplayCursor,
    CausalRichnessDoesNotChangeQueryMeaning,
    CausalInspectionScaleHonesty,
    MissingBridgeRouteEvidenceDenied,
    MissingSignalInvalidationEvidenceDenied,
    MissingSignalEvaluationEvidenceDenied,
    RelationalAuthorityMismatchDenied,
    RedactionPolicyOverclaimDenied,
    UnsupportedExplanationFamilyDenied,
    DirectBridgeDiagnosticsDomainExplanationForbidden,
    DirectRelationalRuntimeDomainExplanationForbidden,
    DirectSignalGraphDomainExplanationForbidden,
    DurableCausalArchiveOverclaimForbidden,
    StoreBackedReplayReconstructionOverclaimForbidden,
}

impl CausalInspectionRepresentativeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ChangedResult => "changed_result",
            Self::SuppressedResult => "suppressed_result",
            Self::QueryDeniedBeforeBridgeEnvelope => "query_denied_before_bridge_envelope",
            Self::AdvisoryRedactedCausalEnvelope => "advisory_redacted_causal_envelope",
            Self::PolicyRedacted => "policy_redacted",
            Self::BranchPreview => "branch_preview",
            Self::HistoricalReplay => "historical_replay",
            Self::WorthStyleQueryOnlyConsumer => "worth_style_query_only_consumer",
            Self::BridgeRouteAndSignalEvidenceBindSameObservation => {
                "bridge_route_and_signal_evidence_bind_same_observation"
            }
            Self::ObservationAnchorBindsOneQueryReceipt => {
                "observation_anchor_binds_one_query_receipt"
            }
            Self::BridgeRecordsBindThroughExistingDiagnostics => {
                "bridge_records_bind_through_existing_diagnostics"
            }
            Self::SignalForensicAvailabilityAndReplayCursor => {
                "signal_forensic_availability_and_replay_cursor"
            }
            Self::CausalRichnessDoesNotChangeQueryMeaning => {
                "causal_richness_does_not_change_query_meaning"
            }
            Self::CausalInspectionScaleHonesty => "causal_inspection_scale_honesty",
            Self::MissingBridgeRouteEvidenceDenied => "missing_bridge_route_evidence_denied",
            Self::MissingSignalInvalidationEvidenceDenied => {
                "missing_signal_invalidation_evidence_denied"
            }
            Self::MissingSignalEvaluationEvidenceDenied => {
                "missing_signal_evaluation_evidence_denied"
            }
            Self::RelationalAuthorityMismatchDenied => "relational_authority_mismatch_denied",
            Self::RedactionPolicyOverclaimDenied => "redaction_policy_overclaim_denied",
            Self::UnsupportedExplanationFamilyDenied => "unsupported_explanation_family_denied",
            Self::DirectBridgeDiagnosticsDomainExplanationForbidden => {
                "direct_bridge_diagnostics_domain_explanation_forbidden"
            }
            Self::DirectRelationalRuntimeDomainExplanationForbidden => {
                "direct_relational_runtime_domain_explanation_forbidden"
            }
            Self::DirectSignalGraphDomainExplanationForbidden => {
                "direct_signal_graph_domain_explanation_forbidden"
            }
            Self::DurableCausalArchiveOverclaimForbidden => {
                "durable_causal_archive_overclaim_forbidden"
            }
            Self::StoreBackedReplayReconstructionOverclaimForbidden => {
                "store_backed_replay_reconstruction_overclaim_forbidden"
            }
        }
    }

    pub(super) fn required() -> &'static [Self] {
        &[
            Self::ChangedResult,
            Self::SuppressedResult,
            Self::QueryDeniedBeforeBridgeEnvelope,
            Self::AdvisoryRedactedCausalEnvelope,
            Self::PolicyRedacted,
            Self::BranchPreview,
            Self::HistoricalReplay,
            Self::WorthStyleQueryOnlyConsumer,
            Self::BridgeRouteAndSignalEvidenceBindSameObservation,
            Self::ObservationAnchorBindsOneQueryReceipt,
            Self::BridgeRecordsBindThroughExistingDiagnostics,
            Self::SignalForensicAvailabilityAndReplayCursor,
            Self::CausalRichnessDoesNotChangeQueryMeaning,
            Self::CausalInspectionScaleHonesty,
            Self::MissingBridgeRouteEvidenceDenied,
            Self::MissingSignalInvalidationEvidenceDenied,
            Self::MissingSignalEvaluationEvidenceDenied,
            Self::RelationalAuthorityMismatchDenied,
            Self::RedactionPolicyOverclaimDenied,
            Self::UnsupportedExplanationFamilyDenied,
            Self::DirectBridgeDiagnosticsDomainExplanationForbidden,
            Self::DirectRelationalRuntimeDomainExplanationForbidden,
            Self::DirectSignalGraphDomainExplanationForbidden,
            Self::DurableCausalArchiveOverclaimForbidden,
            Self::StoreBackedReplayReconstructionOverclaimForbidden,
        ]
    }
}
