use super::aspect::WorthQueryOrchestrationAspectPosture;
use super::authority::{
    WorthQueryOrchestrationBasisPosture, WorthQueryOrchestrationCollaborativeExtensionPosture,
    WorthQueryOrchestrationLowerAuthorityAttachment, WorthQueryOrchestrationPolicyTenantPosture,
};
use super::contribution::WorthQueryOrchestrationContributionCompatibility;
use super::current_support::RowSpec;
use super::family::{
    WorthQueryOrchestrationBindingProjection, WorthQueryOrchestrationCheckedTopologyKind,
    WorthQueryOrchestrationSupportSurface, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationSurfaceVisibility, WorthQueryOrchestrationTranscriptFamily,
};
use super::row::WorthQueryOrchestrationSemanticProfile;
use super::strategy::WorthQueryOrchestrationStrategyAttachment;

macro_rules! push_recovery_row {
    ($rows:expr;
        public_name: $public_name:expr,
        visibility: $visibility:expr,
        doc_section: $doc_section:expr,
        checked_type_name: $checked_type_name:expr,
        proof_type_name: $proof_type_name:expr,
        semantic_profile: $semantic_profile:expr,
        certification_command: $certification_command:expr,
    ) => {
        $rows.push(RowSpec {
            public_name: $public_name,
            canonical_base_name: $public_name,
            family: WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary,
            visibility: $visibility,
            ordinary_outcome_supported: false,
            binding_projection: WorthQueryOrchestrationBindingProjection::None,
            checked_type_name: $checked_type_name,
            proof_type_name: $proof_type_name,
            transcript_family: WorthQueryOrchestrationTranscriptFamily::RecoveryBoundary,
            checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind::RecoveryBoundary,
            support_surface: WorthQueryOrchestrationSupportSurface::RecoveryBoundary,
            semantic_profile: $semantic_profile,
            doc_path: "crates/worth-query/docs/domain-capabilities/typed-stops-and-remediation-guidance.md",
            doc_section: $doc_section,
            certification_suite: "recovery_boundary",
            certification_command: $certification_command,
        });
    };
}

pub(crate) fn worth_query_recovery_orchestration_rows() -> Vec<RowSpec> {
    let certification_command = "cargo test -p worth-query recovery_boundary -- --nocapture";
    let mut rows = Vec::new();
    push_ordinary_recovery_rows(&mut rows, certification_command);
    push_declaration_recovery_rows(&mut rows, certification_command);
    push_continuation_recovery_rows(&mut rows, certification_command);
    push_signal_recovery_rows(&mut rows, certification_command);
    push_contribution_recovery_rows(&mut rows, certification_command);
    push_grouped_recovery_rows(&mut rows, certification_command);
    rows
}

fn push_ordinary_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row!(rows;
        public_name: "recover_from_outcome",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Ordinary,
        doc_section: "ordinary recovery entry",
        checked_type_name: "WorthQueryOrdinaryOutcome",
        proof_type_name: "WorthQueryOrdinaryOutcome",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            WorthQueryOrchestrationBasisPosture::Mixed,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
}

fn push_declaration_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row!(rows;
        public_name: "recover_from_declaration_entry_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryDeclarationEntryOrchestrationChecked",
        proof_type_name: "WorthQueryDeclarationEntryOrchestrationChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_declaration_entry_proof",
        visibility: WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        doc_section: "proof recovery entry",
        checked_type_name: "WorthQueryDeclarationEntryOrchestrationChecked",
        proof_type_name: "WorthQueryDeclarationEntryOrchestrationProof",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_declaration_route_plan_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryDeclarationRoutePlanChecked",
        proof_type_name: "WorthQueryDeclarationRoutePlanChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_declaration_receipt_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryDeclarationReceiptChecked",
        proof_type_name: "WorthQueryDeclarationReceiptChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command: certification_command,
    );
}

fn push_continuation_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row!(rows;
        public_name: "recover_from_prepared_continuation_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryPreparedContinuationChecked",
        proof_type_name: "WorthQueryPreparedContinuationChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_prepared_continuation_proof",
        visibility: WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        doc_section: "proof recovery entry",
        checked_type_name: "WorthQueryPreparedContinuationChecked",
        proof_type_name: "WorthQueryPreparedContinuationTranscript",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_continuation_execution_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryContinuationExecutionChecked",
        proof_type_name: "WorthQueryContinuationExecutionChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_continuation_execution_proof",
        visibility: WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        doc_section: "proof recovery entry",
        checked_type_name: "WorthQueryContinuationExecutionChecked",
        proof_type_name: "WorthQueryContinuationExecutionTranscript",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
}

fn push_signal_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row!(rows;
        public_name: "recover_from_signal_compatibility_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQuerySignalCompatibilityOrchestrationChecked",
        proof_type_name: "WorthQuerySignalCompatibilityOrchestrationChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            WorthQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_signal_compatibility_proof",
        visibility: WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        doc_section: "proof recovery entry",
        checked_type_name: "WorthQuerySignalCompatibilityOrchestrationChecked",
        proof_type_name: "WorthQuerySignalCompatibilityOrchestrationTranscript",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            WorthQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ),
        certification_command: certification_command,
    );
}

fn push_contribution_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row!(rows;
        public_name: "recover_from_contribution_composed_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryContributionComposedOrchestrationChecked",
        proof_type_name: "WorthQueryContributionComposedOrchestrationChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
            WorthQueryOrchestrationBasisPosture::DeclarationScopedContribution,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_contribution_composed_proof",
        visibility: WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        doc_section: "proof recovery entry",
        checked_type_name: "WorthQueryContributionComposedOrchestrationChecked",
        proof_type_name: "WorthQueryContributionComposedOrchestrationTranscript",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
            WorthQueryOrchestrationBasisPosture::DeclarationScopedContribution,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ),
        certification_command: certification_command,
    );
}

fn push_grouped_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row!(rows;
        public_name: "recover_from_grouped_orchestration_checked",
        visibility: WorthQueryOrchestrationSurfaceVisibility::Checked,
        doc_section: "checked recovery entry",
        checked_type_name: "WorthQueryGroupedOrchestrationChecked",
        proof_type_name: "WorthQueryGroupedOrchestrationChecked",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
    push_recovery_row!(rows;
        public_name: "recover_from_grouped_orchestration_proof",
        visibility: WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        doc_section: "proof recovery entry",
        checked_type_name: "WorthQueryGroupedOrchestrationChecked",
        proof_type_name: "WorthQueryGroupedOrchestrationTranscript",
        semantic_profile: recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command: certification_command,
    );
}

fn recovery_profile(
    aspect_posture: WorthQueryOrchestrationAspectPosture,
    basis_posture: WorthQueryOrchestrationBasisPosture,
    lower_authority_attachment: WorthQueryOrchestrationLowerAuthorityAttachment,
    strategy_attachment: WorthQueryOrchestrationStrategyAttachment,
) -> WorthQueryOrchestrationSemanticProfile {
    WorthQueryOrchestrationSemanticProfile::new(
        aspect_posture,
        basis_posture,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        lower_authority_attachment,
        strategy_attachment,
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    )
}
