use super::aspect::WorthQueryOrchestrationAspectPosture;
use super::authority::{
    WorthQueryOrchestrationBasisPosture, WorthQueryOrchestrationCollaborativeExtensionPosture,
    WorthQueryOrchestrationLowerAuthorityAttachment, WorthQueryOrchestrationPolicyTenantPosture,
};
use super::contribution::WorthQueryOrchestrationContributionCompatibility;
use super::current_support::{push_four_lane_rows, FourLaneSpec};
use super::family::{
    WorthQueryOrchestrationBindingProjection, WorthQueryOrchestrationCheckedTopologyKind,
    WorthQueryOrchestrationSupportSurface, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationTranscriptFamily,
};
use super::row::{WorthQueryOrchestrationSemanticProfile, WorthQueryOrchestrationSurfaceRow};
use super::strategy::WorthQueryOrchestrationStrategyAttachment;

pub(crate) fn worth_query_family_helper_orchestration_rows(
) -> Vec<WorthQueryOrchestrationSurfaceRow> {
    let doc_path = "crates/worth-query/docs/domain-capabilities/family-helpers.md";
    let certification_command = "cargo test -p worth-query family_helpers -- --nocapture";
    let mut specs = Vec::new();

    let preview_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        WorthQueryOrchestrationBasisPosture::PreviewSpecialized,
        WorthQueryOrchestrationPolicyTenantPosture::WorkflowPreviewAware,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let current_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        WorthQueryOrchestrationBasisPosture::CurrentTruthViewSpecialized,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let historical_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        WorthQueryOrchestrationBasisPosture::HistoricalTruthViewSpecialized,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let contribution_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
        WorthQueryOrchestrationBasisPosture::DeclarationScopedContribution,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        WorthQueryOrchestrationContributionCompatibility::declaration_scoped(vec![
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
        ]),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let grouped_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        WorthQueryOrchestrationBasisPosture::GroupedNeighborhoodDeclaration,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::none(),
        WorthQueryOrchestrationContributionCompatibility::grouped_neighborhood(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
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
                family: WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
                transcript_family:
                    WorthQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
                checked_type_name: "WorthQuerySignalCompatibilityOrchestrationChecked",
                proof_type_name: "WorthQuerySignalCompatibilityOrchestrationTranscript",
                support_surface:
                    WorthQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
                checked_topology_kind:
                    WorthQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
                binding_projection:
                    WorthQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
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
            family: WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration,
            transcript_family:
                WorthQueryOrchestrationTranscriptFamily::ContributionComposedOrchestration,
            checked_type_name: "WorthQueryContributionComposedOrchestrationChecked",
            proof_type_name: "WorthQueryContributionComposedOrchestrationTranscript",
            support_surface:
                WorthQueryOrchestrationSupportSurface::ContributionComposedOrchestration,
            checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind::ContributionComposed,
            binding_projection: WorthQueryOrchestrationBindingProjection::SharedContributionBinding,
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
            family: WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration,
            transcript_family:
                WorthQueryOrchestrationTranscriptFamily::GroupedNeighborhoodOrchestration,
            checked_type_name: "WorthQueryGroupedOrchestrationChecked",
            proof_type_name: "WorthQueryGroupedOrchestrationTranscript",
            support_surface: WorthQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
            checked_topology_kind:
                WorthQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
            binding_projection: WorthQueryOrchestrationBindingProjection::SharedGroupedBinding,
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
