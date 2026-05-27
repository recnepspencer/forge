use super::certification::ForgeQueryOrchestrationSurfaceCertificationReference;
use super::docs::ForgeQueryOrchestrationSurfaceDocReference;
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
use super::row::ForgeQueryOrchestrationSurfaceRow;
use super::transcript::ForgeQueryOrchestrationProofContract;

pub(crate) fn forge_query_family_helper_orchestration_rows(
) -> Vec<ForgeQueryOrchestrationSurfaceRow> {
    let doc_path = "crates/forge-query/docs/domain-capabilities/family-helpers.md";
    let certification_command = "cargo test -p forge-query family_helpers -- --nocapture";

    let mut rows = Vec::new();
    push_four_lane_rows(
        &mut rows,
        doc_path,
        certification_command,
        "prepare_preview_for_active_face_selection",
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
    );
    push_four_lane_rows(
        &mut rows,
        doc_path,
        certification_command,
        "prepare_runtime_route_for_active_face_selection",
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
    );
    push_four_lane_rows(
        &mut rows,
        doc_path,
        certification_command,
        "prepare_current_truth_view_for_active_face_selection",
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
    );
    push_four_lane_rows(
        &mut rows,
        doc_path,
        certification_command,
        "prepare_historical_truth_view_for_active_face_selection",
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
    );
    push_four_lane_rows(
        &mut rows,
        doc_path,
        certification_command,
        "orchestrate_material_attachment_for_active_face_selection",
        ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::ContributionComposedOrchestration,
        "ForgeQueryContributionComposedOrchestrationChecked",
        "ForgeQueryContributionComposedOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::ContributionComposedOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::ContributionComposed,
        ForgeQueryOrchestrationBindingProjection::SharedContributionBinding,
    );
    push_four_lane_rows(
        &mut rows,
        doc_path,
        certification_command,
        "orchestrate_local_neighborhood_for_active_face_selection",
        ForgeQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::GroupedNeighborhoodOrchestration,
        "ForgeQueryGroupedOrchestrationChecked",
        "ForgeQueryGroupedOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
        ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
        ForgeQueryOrchestrationBindingProjection::SharedGroupedBinding,
    );
    rows
}

fn leak(text: String) -> &'static str {
    text.leak()
}

fn push_four_lane_rows(
    rows: &mut Vec<ForgeQueryOrchestrationSurfaceRow>,
    doc_path: &'static str,
    certification_command: &'static str,
    base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    support_surface: ForgeQueryOrchestrationSupportSurface,
    checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
) {
    for (public_name, visibility, doc_section) in [
        (
            base_name,
            ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
            "family helper ordinary lane",
        ),
        (
            leak(format!("{base_name}_outcome")),
            ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome,
            "family helper ordinary outcome lane",
        ),
        (
            leak(format!("{base_name}_checked")),
            ForgeQueryOrchestrationSurfaceVisibility::Checked,
            "family helper checked lane",
        ),
        (
            leak(format!("{base_name}_proof")),
            ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
            "family helper proof lane",
        ),
    ] {
        rows.push(ForgeQueryOrchestrationSurfaceRow::new(
            public_name,
            base_name,
            family,
            visibility,
            true,
            binding_projection,
            ForgeQueryOrchestrationProofContract::new(
                checked_type_name,
                proof_type_name,
                transcript_family,
                checked_topology_kind,
                support_surface,
            ),
            ForgeQueryOrchestrationSurfaceDocReference::new(doc_path, doc_section),
            ForgeQueryOrchestrationSurfaceCertificationReference::new(
                "family_helpers",
                certification_command,
            ),
        ));
    }
}
