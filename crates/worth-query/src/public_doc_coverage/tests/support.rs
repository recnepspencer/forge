use crate::public_doc_coverage::{
    WorthQueryPublicDocCoverageInventory, WorthQueryPublicDocCoverageRow,
    WorthQueryPublicDocReference, WorthQueryPublicGoldenTranscript,
    WorthQueryPublicGoldenTranscriptKind, WorthQueryPublicJourneyKind,
};

pub(super) fn current_row(public_name: &str) -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageInventory::current()
        .row_for_public_name(public_name)
        .unwrap_or_else(|| panic!("expected coverage row {public_name}"))
        .clone()
}

pub(super) fn inventory_without_public_name(
    public_name: &str,
) -> WorthQueryPublicDocCoverageInventory {
    let current = WorthQueryPublicDocCoverageInventory::current();
    let rows = current
        .rows()
        .iter()
        .filter(|row| row.public_name() != public_name)
        .cloned()
        .collect::<Vec<_>>();
    WorthQueryPublicDocCoverageInventory::new(current.source_inventory_digest().to_string(), rows)
}

pub(super) fn inventory_with_replaced_row(
    replacement: WorthQueryPublicDocCoverageRow,
) -> WorthQueryPublicDocCoverageInventory {
    let current = WorthQueryPublicDocCoverageInventory::current();
    let rows = current
        .rows()
        .iter()
        .map(|row| {
            if row.public_name() == replacement.public_name() {
                replacement.clone()
            } else {
                row.clone()
            }
        })
        .collect::<Vec<_>>();
    WorthQueryPublicDocCoverageInventory::new(current.source_inventory_digest().to_string(), rows)
}

pub(super) fn inventory_with_extra_row(
    extra: WorthQueryPublicDocCoverageRow,
) -> WorthQueryPublicDocCoverageInventory {
    let current = WorthQueryPublicDocCoverageInventory::current();
    let mut rows = current.rows().to_vec();
    rows.push(extra);
    WorthQueryPublicDocCoverageInventory::new(current.source_inventory_digest().to_string(), rows)
}

pub(super) fn inventory_without_golden_label(label: &str) -> WorthQueryPublicDocCoverageInventory {
    let current = WorthQueryPublicDocCoverageInventory::current();
    let rows = current
        .rows()
        .iter()
        .filter(|row| {
            row.golden_transcript()
                .is_none_or(|golden| golden.label() != label)
        })
        .cloned()
        .collect::<Vec<_>>();
    WorthQueryPublicDocCoverageInventory::new(current.source_inventory_digest().to_string(), rows)
}

pub(super) fn row_with_doc_reference(
    row: &WorthQueryPublicDocCoverageRow,
    path: &'static str,
    section: &'static str,
) -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.orchestration_family(),
        row.visibility(),
        row.surface_row_digest().to_string(),
        WorthQueryPublicDocReference::new(path, section),
        row.readme_discovery_label(),
        row.golden_transcript(),
        row.journey(),
    )
}

pub(super) fn row_without_golden(
    row: &WorthQueryPublicDocCoverageRow,
) -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.orchestration_family(),
        row.visibility(),
        row.surface_row_digest().to_string(),
        row.doc_reference(),
        row.readme_discovery_label(),
        None,
        row.journey(),
    )
}

pub(super) fn row_without_readme(
    row: &WorthQueryPublicDocCoverageRow,
) -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.orchestration_family(),
        row.visibility(),
        row.surface_row_digest().to_string(),
        row.doc_reference(),
        "",
        row.golden_transcript(),
        row.journey(),
    )
}

pub(super) fn row_without_journey(
    row: &WorthQueryPublicDocCoverageRow,
) -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.orchestration_family(),
        row.visibility(),
        row.surface_row_digest().to_string(),
        row.doc_reference(),
        row.readme_discovery_label(),
        row.golden_transcript(),
        None,
    )
}

pub(super) fn row_with_golden(
    row: &WorthQueryPublicDocCoverageRow,
    golden: WorthQueryPublicGoldenTranscript,
) -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageRow::new(
        row.public_name(),
        row.canonical_base_name(),
        row.orchestration_family(),
        row.visibility(),
        row.surface_row_digest().to_string(),
        row.doc_reference(),
        row.readme_discovery_label(),
        Some(golden),
        row.journey(),
    )
}

pub(super) fn orphan_row() -> WorthQueryPublicDocCoverageRow {
    WorthQueryPublicDocCoverageRow::new(
        "fake_surface",
        "fake_surface",
        current_row("orchestrate_signal_compatibility").orchestration_family(),
        current_row("orchestrate_signal_compatibility").visibility(),
        "fake-surface-digest".to_string(),
        WorthQueryPublicDocReference::new(
            "crates/worth-query/docs/domain-capabilities/orchestration-inventory.md",
            "surface contract",
        ),
        "Orchestration Inventory",
        Some(WorthQueryPublicGoldenTranscript::new(
            "public_doc_coverage_surface_readout",
            "tests/ui/domain_handle/golden/public_doc_coverage_surface_readout_compiles.rs",
            "public doc coverage readout",
            WorthQueryPublicGoldenTranscriptKind::CoverageBoundaryReadout,
            None,
        )),
        Some(WorthQueryPublicJourneyKind::PlatformEntry),
    )
}
