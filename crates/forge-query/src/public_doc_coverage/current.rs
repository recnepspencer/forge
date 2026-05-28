use super::docs::ForgeQueryPublicDocReference;
use super::goldens::{golden_transcript_by_label, ForgeQueryPublicGoldenTranscript};
use super::inventory::ForgeQueryPublicDocCoverageInventory;
use super::journeys::ForgeQueryPublicJourneyKind;
use super::row::ForgeQueryPublicDocCoverageRow;
use crate::orchestration_inventory::{
    ForgeQueryOrchestrationSurfaceFamily, ForgeQueryOrchestrationSurfaceInventory,
};

pub(crate) fn forge_query_public_doc_coverage_inventory() -> ForgeQueryPublicDocCoverageInventory {
    let surface_inventory = ForgeQueryOrchestrationSurfaceInventory::current();
    let rows = surface_inventory
        .rows()
        .iter()
        .map(|surface| {
            let (doc_path, doc_section, readme_label, golden, journey) = coverage_for_surface(
                section_for_surface(surface.public_name(), surface.canonical_base_name()),
                surface.doc_reference().path(),
                surface.family(),
            );
            ForgeQueryPublicDocCoverageRow::new(
                surface.public_name(),
                surface.canonical_base_name(),
                surface.family(),
                surface.visibility(),
                surface.row_digest().to_string(),
                ForgeQueryPublicDocReference::new(doc_path, doc_section),
                readme_label,
                Some(golden),
                Some(journey),
            )
        })
        .collect::<Vec<_>>();
    ForgeQueryPublicDocCoverageInventory::new(
        surface_inventory.inventory_digest().to_string(),
        rows,
    )
}

fn coverage_for_surface(
    section_key: &'static str,
    inventory_doc_path: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
) -> (
    &'static str,
    &'static str,
    &'static str,
    ForgeQueryPublicGoldenTranscript,
    ForgeQueryPublicJourneyKind,
) {
    let helper_surface = inventory_doc_path.ends_with("family-helpers.md");
    match family {
        ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget
        | ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareContext
        | ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute => (
            inventory_doc_path,
            section_key,
            "Continuation Pipeline",
            golden("continuation_pipeline_surface_readout"),
            ForgeQueryPublicJourneyKind::Continuation,
        ),
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration
            if helper_surface =>
        {
            (
                inventory_doc_path,
                section_key,
                "Family Helpers",
                golden("family_helper_surface_readout"),
                ForgeQueryPublicJourneyKind::HelperProjection,
            )
        }
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration => (
            inventory_doc_path,
            section_key,
            "Signal Compatibility Orchestration",
            golden("signal_compatibility_surface_readout"),
            ForgeQueryPublicJourneyKind::SignalFacing,
        ),
        ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration
            if helper_surface =>
        {
            (
                inventory_doc_path,
                section_key,
                "Family Helpers",
                golden("family_helper_surface_readout"),
                ForgeQueryPublicJourneyKind::HelperProjection,
            )
        }
        ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration => (
            inventory_doc_path,
            section_key,
            "Contribution-Composed Orchestration",
            golden("contribution_composed_surface_readout"),
            ForgeQueryPublicJourneyKind::ContributionComposed,
        ),
        ForgeQueryOrchestrationSurfaceFamily::RecoveryBoundary => (
            inventory_doc_path,
            section_key,
            "Recovery Boundary",
            golden("recovery_boundary_surface_readout"),
            ForgeQueryPublicJourneyKind::Recovery,
        ),
        ForgeQueryOrchestrationSurfaceFamily::GroupedNeighborhoodOrchestration => (
            inventory_doc_path,
            section_key,
            "Grouped Authoring",
            golden("grouped_authoring_surface_readout"),
            ForgeQueryPublicJourneyKind::GroupedAuthoring,
        ),
        ForgeQueryOrchestrationSurfaceFamily::DeclarationEntry
        | ForgeQueryOrchestrationSurfaceFamily::RouteFromProgressed
        | ForgeQueryOrchestrationSurfaceFamily::ReceiptFromProgressed
        | ForgeQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed => (
            inventory_doc_path,
            section_key,
            "Declaration Entry Orchestration",
            golden("declaration_entry_orchestration_surface_readout"),
            ForgeQueryPublicJourneyKind::PlatformEntry,
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

fn golden(label: &str) -> ForgeQueryPublicGoldenTranscript {
    golden_transcript_by_label(label)
}
