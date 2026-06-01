use super::aspect::ForgeQueryOrchestrationAspectPosture;
use super::authority::{
    ForgeQueryOrchestrationBasisPosture, ForgeQueryOrchestrationCollaborativeExtensionPosture,
    ForgeQueryOrchestrationLowerAuthorityAttachment, ForgeQueryOrchestrationPolicyTenantPosture,
};
use super::contribution::ForgeQueryOrchestrationContributionCompatibility;
use super::current_support::RowSpec;
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
use super::row::ForgeQueryOrchestrationSemanticProfile;
use super::strategy::ForgeQueryOrchestrationStrategyAttachment;

pub(crate) fn forge_query_recovery_orchestration_rows() -> Vec<RowSpec> {
    let certification_command = "cargo test -p forge-query recovery_boundary -- --nocapture";
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
        ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
        "ordinary recovery entry",
        "ForgeQueryOrdinaryOutcome",
        "ForgeQueryOrdinaryOutcome",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            ForgeQueryOrchestrationBasisPosture::Mixed,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
}

fn push_declaration_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_declaration_entry_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryDeclarationEntryOrchestrationChecked",
        "ForgeQueryDeclarationEntryOrchestrationChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RequiredContract,
            ForgeQueryOrchestrationBasisPosture::DeclarationEntry,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_declaration_entry_proof",
        ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "ForgeQueryDeclarationEntryOrchestrationChecked",
        "ForgeQueryDeclarationEntryOrchestrationProof",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RequiredContract,
            ForgeQueryOrchestrationBasisPosture::DeclarationEntry,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_declaration_route_plan_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryDeclarationRoutePlanChecked",
        "ForgeQueryDeclarationRoutePlanChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RequiredContract,
            ForgeQueryOrchestrationBasisPosture::DeclarationEntry,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_declaration_receipt_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryDeclarationReceiptChecked",
        "ForgeQueryDeclarationReceiptChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RequiredContract,
            ForgeQueryOrchestrationBasisPosture::DeclarationEntry,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ),
        certification_command,
    );
}

fn push_continuation_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_prepared_continuation_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryPreparedContinuationChecked",
        "ForgeQueryPreparedContinuationChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            ForgeQueryOrchestrationBasisPosture::ReadmissionAware,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_prepared_continuation_proof",
        ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "ForgeQueryPreparedContinuationChecked",
        "ForgeQueryPreparedContinuationTranscript",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            ForgeQueryOrchestrationBasisPosture::ReadmissionAware,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_continuation_execution_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryContinuationExecutionChecked",
        "ForgeQueryContinuationExecutionChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            ForgeQueryOrchestrationBasisPosture::ReadmissionAware,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_continuation_execution_proof",
        ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "ForgeQueryContinuationExecutionChecked",
        "ForgeQueryContinuationExecutionTranscript",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
            ForgeQueryOrchestrationBasisPosture::ReadmissionAware,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
}

fn push_signal_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_signal_compatibility_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            ForgeQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_signal_compatibility_proof",
        ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationTranscript",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
            ForgeQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ),
        certification_command,
    );
}

fn push_contribution_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_contribution_composed_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryContributionComposedOrchestrationChecked",
        "ForgeQueryContributionComposedOrchestrationChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
            ForgeQueryOrchestrationBasisPosture::DeclarationScopedContribution,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_contribution_composed_proof",
        ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "ForgeQueryContributionComposedOrchestrationChecked",
        "ForgeQueryContributionComposedOrchestrationTranscript",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
            ForgeQueryOrchestrationBasisPosture::DeclarationScopedContribution,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ),
        certification_command,
    );
}

fn push_grouped_recovery_rows(rows: &mut Vec<RowSpec>, certification_command: &'static str) {
    push_recovery_row(
        rows,
        "recover_from_grouped_orchestration_checked",
        ForgeQueryOrchestrationSurfaceVisibility::Checked,
        "checked recovery entry",
        "ForgeQueryGroupedOrchestrationChecked",
        "ForgeQueryGroupedOrchestrationChecked",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RequiredContract,
            ForgeQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
    push_recovery_row(
        rows,
        "recover_from_grouped_orchestration_proof",
        ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        "proof recovery entry",
        "ForgeQueryGroupedOrchestrationChecked",
        "ForgeQueryGroupedOrchestrationTranscript",
        recovery_profile(
            ForgeQueryOrchestrationAspectPosture::RequiredContract,
            ForgeQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
            ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(
            ),
            ForgeQueryOrchestrationStrategyAttachment::none(),
        ),
        certification_command,
    );
}

fn recovery_profile(
    aspect_posture: ForgeQueryOrchestrationAspectPosture,
    basis_posture: ForgeQueryOrchestrationBasisPosture,
    lower_authority_attachment: ForgeQueryOrchestrationLowerAuthorityAttachment,
    strategy_attachment: ForgeQueryOrchestrationStrategyAttachment,
) -> ForgeQueryOrchestrationSemanticProfile {
    ForgeQueryOrchestrationSemanticProfile::new(
        aspect_posture,
        basis_posture,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        lower_authority_attachment,
        strategy_attachment,
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    )
}

fn push_recovery_row(
    rows: &mut Vec<RowSpec>,
    public_name: &'static str,
    visibility: ForgeQueryOrchestrationSurfaceVisibility,
    doc_section: &'static str,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
    certification_command: &'static str,
) {
    rows.push(RowSpec {
        public_name,
        canonical_base_name: public_name,
        family: ForgeQueryOrchestrationSurfaceFamily::RecoveryBoundary,
        visibility,
        ordinary_outcome_supported: false,
        binding_projection: ForgeQueryOrchestrationBindingProjection::None,
        checked_type_name,
        proof_type_name,
        transcript_family: ForgeQueryOrchestrationTranscriptFamily::RecoveryBoundary,
        checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::RecoveryBoundary,
        support_surface: ForgeQueryOrchestrationSupportSurface::RecoveryBoundary,
        semantic_profile,
        doc_path: "crates/forge-query/docs/domain-capabilities/recovery-boundary.md",
        doc_section,
        certification_suite: "recovery_boundary",
        certification_command,
    });
}
