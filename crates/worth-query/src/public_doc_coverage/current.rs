use super::docs::WorthQueryPublicDocReference;
use super::goldens::{golden_transcript_by_label, WorthQueryPublicGoldenTranscript};
use super::inventory::WorthQueryPublicDocCoverageInventory;
use super::journeys::WorthQueryPublicJourneyKind;
use super::row::WorthQueryPublicDocCoverageRow;
use crate::orchestration_inventory::{
    WorthQueryOrchestrationSurfaceFamily, WorthQueryOrchestrationSurfaceInventory,
};

pub(crate) fn worth_query_public_doc_coverage_inventory() -> WorthQueryPublicDocCoverageInventory {
    let surface_inventory = WorthQueryOrchestrationSurfaceInventory::current();
    let rows = surface_inventory
        .rows()
        .iter()
        .map(|surface| {
            let (doc_path, doc_section, readme_label, golden, journey) = coverage_for_surface(
                section_for_surface(surface.public_name(), surface.canonical_base_name()),
                surface.doc_reference().path(),
                surface.family(),
            );
            WorthQueryPublicDocCoverageRow::new(
                surface.public_name(),
                surface.canonical_base_name(),
                surface.family(),
                surface.visibility(),
                surface.row_digest().to_string(),
                WorthQueryPublicDocReference::new(doc_path, doc_section),
                readme_label,
                Some(golden),
                Some(journey),
            )
        })
        .collect::<Vec<_>>();
    WorthQueryPublicDocCoverageInventory::new(
        surface_inventory.inventory_digest().to_string(),
        rows,
    )
}

fn coverage_for_surface(
    section_key: &'static str,
    inventory_doc_path: &'static str,
    family: WorthQueryOrchestrationSurfaceFamily,
) -> (
    &'static str,
    &'static str,
    &'static str,
    WorthQueryPublicGoldenTranscript,
    WorthQueryPublicJourneyKind,
) {
    let helper_surface = inventory_doc_path.ends_with("family-helpers.md");
    match family {
        WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget
        | WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext
        | WorthQueryOrchestrationSurfaceFamily::ContinuationExecute => (
            inventory_doc_path,
            section_key,
            "Continuation Pipeline",
            golden("continuation_pipeline_surface_readout"),
            WorthQueryPublicJourneyKind::Continuation,
        ),
        WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
            if helper_surface =>
        {
            (
                inventory_doc_path,
                section_key,
                "Family Helpers",
                golden("family_helper_surface_readout"),
                WorthQueryPublicJourneyKind::HelperProjection,
            )
        }
        WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration => (
            inventory_doc_path,
            section_key,
            "Signal Compatibility Orchestration",
            golden("signal_compatibility_surface_readout"),
            WorthQueryPublicJourneyKind::SignalFacing,
        ),
        WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
            if helper_surface =>
        {
            (
                inventory_doc_path,
                section_key,
                "Family Helpers",
                golden("family_helper_surface_readout"),
                WorthQueryPublicJourneyKind::HelperProjection,
            )
        }
        WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration => (
            inventory_doc_path,
            section_key,
            "Contribution-Composed Orchestration",
            golden("contribution_composed_surface_readout"),
            WorthQueryPublicJourneyKind::ContributionComposed,
        ),
        WorthQueryOrchestrationSurfaceFamily::RecoveryBoundary => (
            inventory_doc_path,
            section_key,
            "Recovery Boundary",
            golden("recovery_boundary_surface_readout"),
            WorthQueryPublicJourneyKind::Recovery,
        ),
        WorthQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration => (
            inventory_doc_path,
            section_key,
            "Grouped Authoring",
            golden("grouped_authoring_surface_readout"),
            WorthQueryPublicJourneyKind::GroupedAuthoring,
        ),
        WorthQueryOrchestrationSurfaceFamily::DeclarationEntry
        | WorthQueryOrchestrationSurfaceFamily::RouteFromProgressed
        | WorthQueryOrchestrationSurfaceFamily::ReceiptFromProgressed
        | WorthQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed => (
            inventory_doc_path,
            section_key,
            "Declaration Entry Orchestration",
            golden("declaration_entry_orchestration_surface_readout"),
            WorthQueryPublicJourneyKind::PlatformEntry,
        ),
    }
}

fn section_for_surface(
    public_name: &'static str,
    canonical_base_name: &'static str,
) -> &'static str {
    if public_name.contains("_with_intent")
        || public_name.ends_with("_checked")
        || public_name.ends_with("_proof")
        || public_name.ends_with("_outcome")
    {
        canonical_base_name
    } else {
        public_name
    }
}

fn golden(label: &str) -> WorthQueryPublicGoldenTranscript {
    golden_transcript_by_label(label)
}
