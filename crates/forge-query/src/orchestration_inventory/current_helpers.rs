use super::aspect::ForgeQueryOrchestrationAspectPosture;
use super::authority::{
    ForgeQueryOrchestrationBasisPosture, ForgeQueryOrchestrationCollaborativeExtensionPosture,
    ForgeQueryOrchestrationLowerAuthorityAttachment, ForgeQueryOrchestrationPolicyTenantPosture,
};
use super::contribution::ForgeQueryOrchestrationContributionCompatibility;
use super::current_support::{push_four_lane_rows, FourLaneSpec};
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationTranscriptFamily,
};
use super::row::{ForgeQueryOrchestrationSemanticProfile, ForgeQueryOrchestrationSurfaceRow};
use super::strategy::ForgeQueryOrchestrationStrategyAttachment;

pub(crate) fn forge_query_family_helper_orchestration_rows(
) -> Vec<ForgeQueryOrchestrationSurfaceRow> {
    let doc_path = "crates/forge-query/docs/domain-capabilities/family-helpers.md";
    let certification_command = "cargo test -p forge-query family_helpers -- --nocapture";
    let mut specs = Vec::new();

    let preview_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        ForgeQueryOrchestrationBasisPosture::PreviewSpecialized,
        ForgeQueryOrchestrationPolicyTenantPosture::WorkflowPreviewAware,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let current_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        ForgeQueryOrchestrationBasisPosture::CurrentTruthViewSpecialized,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let historical_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        ForgeQueryOrchestrationBasisPosture::HistoricalTruthViewSpecialized,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let contribution_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
        ForgeQueryOrchestrationBasisPosture::DeclarationScopedContribution,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ForgeQueryOrchestrationContributionCompatibility::declaration_scoped(vec![
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::Admission,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
        ]),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let grouped_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        ForgeQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::none(),
        ForgeQueryOrchestrationContributionCompatibility::grouped_neighborhood(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );

    for (name, profile) in [
        ("prepare_preview_for_active_face_selection", preview_profile),
        (
            "prepare_runtime_route_for_active_face_selection",
            current_profile.clone(),
        ),
        (
            "prepare_current_truth_view_for_active_face_selection",
            current_profile,
        ),
        (
            "prepare_historical_truth_view_for_active_face_selection",
            historical_profile,
        ),
    ] {
        push_four_lane_rows(
            &mut specs,
            FourLaneSpec {
                base_name: name,
                family: ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
                transcript_family:
                    ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
                checked_type_name: "ForgeQuerySignalCompatibilityOrchestrationChecked",
                proof_type_name: "ForgeQuerySignalCompatibilityOrchestrationTranscript",
                support_surface:
                    ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
                checked_topology_kind:
                    ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
                binding_projection:
                    ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
                semantic_profile: profile,
                doc_path,
                ordinary_section: "family helper ordinary lane",
                outcome_section: "family helper ordinary outcome lane",
                checked_section: "family helper checked lane",
                proof_section: "family helper proof lane",
                certification_suite: "family_helpers",
                certification_command,
            },
        );
    }

    push_four_lane_rows(
        &mut specs,
        FourLaneSpec {
            base_name: "orchestrate_material_attachment_for_active_face_selection",
            family: ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration,
            transcript_family:
                ForgeQueryOrchestrationTranscriptFamily::ContributionComposedOrchestration,
            checked_type_name: "ForgeQueryContributionComposedOrchestrationChecked",
            proof_type_name: "ForgeQueryContributionComposedOrchestrationTranscript",
            support_surface:
                ForgeQueryOrchestrationSupportSurface::ContributionComposedOrchestration,
            checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::ContributionComposed,
            binding_projection: ForgeQueryOrchestrationBindingProjection::SharedContributionBinding,
            semantic_profile: contribution_profile,
            doc_path,
            ordinary_section: "family helper ordinary lane",
            outcome_section: "family helper ordinary outcome lane",
            checked_section: "family helper checked lane",
            proof_section: "family helper proof lane",
            certification_suite: "family_helpers",
            certification_command,
        },
    );
    push_four_lane_rows(
        &mut specs,
        FourLaneSpec {
            base_name: "orchestrate_local_neighborhood_for_active_face_selection",
            family: ForgeQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration,
            transcript_family:
                ForgeQueryOrchestrationTranscriptFamily::GroupedNeighborhoodOrchestration,
            checked_type_name: "ForgeQueryGroupedOrchestrationChecked",
            proof_type_name: "ForgeQueryGroupedOrchestrationTranscript",
            support_surface: ForgeQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
            checked_topology_kind:
                ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
            binding_projection: ForgeQueryOrchestrationBindingProjection::SharedGroupedBinding,
            semantic_profile: grouped_profile,
            doc_path,
            ordinary_section: "family helper ordinary lane",
            outcome_section: "family helper ordinary outcome lane",
            checked_section: "family helper checked lane",
            proof_section: "family helper proof lane",
            certification_suite: "family_helpers",
            certification_command,
        },
    );

    specs.into_iter().map(|spec| spec.into_row()).collect()
}
