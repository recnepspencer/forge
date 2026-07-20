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
    push_recovery_row(
        rows,
        "recover_from_outcome",
        WorthQueryOrchestrationSurfaceVisibility::Ordinary,
        "ordinary recovery entry",
        "WorthQueryOrdinaryOutcome",
        "WorthQueryOrdinaryOutcome",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            WorthQueryOrchestrationBasisPosture::Mixed,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
}

fn push_declaration_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_declaration_entry_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryDeclarationEntryOrchestrationChecked",
        "WorthQueryDeclarationEntryOrchestrationChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_declaration_entry_proof",
        WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "WorthQueryDeclarationEntryOrchestrationChecked",
        "WorthQueryDeclarationEntryOrchestrationProof",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_declaration_route_plan_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryDeclarationRoutePlanChecked",
        "WorthQueryDeclarationRoutePlanChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_declaration_receipt_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryDeclarationReceiptChecked",
        "WorthQueryDeclarationReceiptChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::DeclarationEntry,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
}

fn push_continuation_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_prepared_continuation_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryPreparedContinuationChecked",
        "WorthQueryPreparedContinuationChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_prepared_continuation_proof",
        WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "WorthQueryPreparedContinuationChecked",
        "WorthQueryPreparedContinuationTranscript",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_continuation_execution_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryContinuationExecutionChecked",
        "WorthQueryContinuationExecutionChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_continuation_execution_proof",
        WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "WorthQueryContinuationExecutionChecked",
        "WorthQueryContinuationExecutionTranscript",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            WorthQueryOrchestrationBasisPosture::ReadmissionAware,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
}

fn push_signal_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_signal_compatibility_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQuerySignalCompatibilityOrchestrationChecked",
        "WorthQuerySignalCompatibilityOrchestrationChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            WorthQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_signal_compatibility_proof",
        WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "WorthQuerySignalCompatibilityOrchestrationChecked",
        "WorthQuerySignalCompatibilityOrchestrationTranscript",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            WorthQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ),
        certification_command,
    );
}

fn push_contribution_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_contribution_composed_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryContributionComposedOrchestrationChecked",
        "WorthQueryContributionComposedOrchestrationChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
            WorthQueryOrchestrationBasisPosture::DeclarationScopedContribution,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_contribution_composed_proof",
        WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "WorthQueryContributionComposedOrchestrationChecked",
        "WorthQueryContributionComposedOrchestrationTranscript",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
            WorthQueryOrchestrationBasisPosture::DeclarationScopedContribution,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ),
        certification_command,
    );
}

fn push_grouped_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_grouped_orchestration_checked",
        WorthQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "WorthQueryGroupedOrchestrationChecked",
        "WorthQueryGroupedOrchestrationChecked",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_grouped_orchestration_proof",
        WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "WorthQueryGroupedOrchestrationChecked",
        "WorthQueryGroupedOrchestrationTranscript",
        recovery_profile(
            WorthQueryOrchestrationAspectPosture::RequiredContract,
            WorthQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
            WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            WorthQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
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

fn push_recovery_row(
    rows: &mut Vec<RowSpec>,
    public_name: &'static str,
    visibility: WorthQueryOrchestrationSurfaceVisibility,
    doc_section: &'static str,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
    certification_command: &'static str,
) {
    rows.push(RowSpec {
        public_name,
        canonical_base_name: public_name,
        family: WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary,
        visibility,
        ordinary_outcome_supported: false,
        binding_projection: WorthQueryOrchestrationBindingProjection::None,
        checked_type_name,
        proof_type_name,
        transcript_family: WorthQueryOrchestrationTranscriptFamily::RecoveryBoundary,
        checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind::RecoveryBoundary,
        support_surface: WorthQueryOrchestrationSupportSurface::RecoveryBoundary,
        semantic_profile,
        doc_path: "crates/worth-query/docs/domain-capabilities/recovery-boundary.md",
        doc_section,
        certification_suite: "recovery_boundary",
        certification_command,
    });
}
