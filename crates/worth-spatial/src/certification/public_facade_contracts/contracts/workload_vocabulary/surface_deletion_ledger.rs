use std::collections::BTreeSet;
use std::marker::PhantomData;

use topology::facade::{TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedGraphBasis};
use worth_spatial::certification::workload_evidence::{
    certification_manual_stage_row_with_counters, certification_only_admitted_stage_row,
};
use worth_spatial::facade::query_adoption::spatial_query_adoption_inventory;
use worth_spatial::facade::workload_vocabulary::{
    deny_manual_evidence_row_as_spatial_touch_authority,
    deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority,
    deny_topology_laundering_as_spatial_touch_authority,
    deny_topology_touched_graph_basis_as_spatial_touch_authority,
    spatial_evidence_surface_deletion_ledger, SpatialEvidenceSubstitutionDenial,
    SpatialEvidenceSurfaceAuthorityCategory, SpatialEvidenceSurfaceCloseoutPosture,
    SpatialEvidenceSurfaceDeletionAction, SpatialEvidenceSurfaceDeletionLedgerRow,
    SpatialEvidenceSurfaceOwner, SpatialEvidenceTopologySubstitutionSurface,
    WorkloadEvidenceBacking, WorkloadEvidenceStage, WorkloadEvidenceStageCounters,
};

#[test]
fn inventory_parity_covers_public_facade_ledger_query_and_kernel_surfaces() {
    let rows = spatial_evidence_surface_deletion_ledger();
    assert_eq!(
        names_for_category(
            &rows,
            SpatialEvidenceSurfaceAuthorityCategory::PublicFacadeExport,
        ),
        facade_workload_vocabulary_exports_from_source(),
        "public facade export inventory must match the real workload_vocabulary facade"
    );
    assert_public_facade_rows_record_symbol_paths(&rows);
    assert_all_present(
        names_for_category(
            &rows,
            SpatialEvidenceSurfaceAuthorityCategory::LedgerConstructor,
        ),
        expected_ledger_constructors(),
    );
    assert_all_present(
        names_for_category(
            &rows,
            SpatialEvidenceSurfaceAuthorityCategory::BooleanReceiptImplementation,
        ),
        expected_boolean_receipts(),
    );
    assert_all_present(
        names_for_category(
            &rows,
            SpatialEvidenceSurfaceAuthorityCategory::KernelWorkloadEvidenceConsumption,
        ),
        expected_kernel_consumption_paths(),
    );
    assert_migrated_downstream_split_stage_index_input_is_deleted(&rows);

    for query_row in spatial_query_adoption_inventory() {
        assert!(
            rows.iter().any(|row| {
                row.source_path() == query_row.source_set()
                    && row.exported_facade_path() == query_row.exported_facade_path()
                    && row.current_caller() == query_row.current_caller()
                    && row.deletion_action() == query_row.deletion_action()
                    && row.owner() == query_row.owner()
                    && row.cap() == query_row.cap()
                    && row.removal_trigger() == query_row.removal_trigger()
            }),
            "query adoption row missing from deletion ledger: {}",
            query_row.source_set()
        );
    }
}

#[test]
fn deletion_pressure_rows_have_caps_and_no_replaced_production_bypass_survives() {
    let rows = spatial_evidence_surface_deletion_ledger();
    for row in &rows {
        assert!(
            row.has_deletion_or_cap_plan(),
            "row lacks deletion or cap plan: {}",
            row.surface_name()
        );
        assert!(
            !row.violates_replaced_production_bypass(),
            "production bypass remains reachable after replacement exists: {}",
            row.surface_name()
        );
    }

    let hostile_replaced_bypass = SpatialEvidenceSurfaceDeletionLedgerRow::new(
        "hostile-production-bypass-after-replacement",
        "crates/worth-spatial/src/workload_platform/evidence_ledger/row.rs",
        "worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow",
        SpatialEvidenceSurfaceAuthorityCategory::LedgerConstructor,
        "hostile closeout sentinel",
        SpatialEvidenceSurfaceDeletionAction::CollapseToSpatialTouchAuthority,
        SpatialEvidenceSurfaceOwner::WorthSpatial,
        "Sentinel row proves closeout fails when replacement residue stays production-reachable.",
        "Sentinel is not a production ledger row.",
        true,
        true,
    );

    assert_eq!(
        hostile_replaced_bypass.closeout_posture(),
        SpatialEvidenceSurfaceCloseoutPosture::ProductionReachableAfterReplacement
    );
    assert!(hostile_replaced_bypass.violates_replaced_production_bypass());
}

#[test]
fn manual_workload_evidence_row_cannot_satisfy_spatial_touch_authority() {
    let matching_counters = WorkloadEvidenceStageCounters::boolean_split();
    let evidence_identity = "planar-boolean-split:matching-stage-identity-and-counters";
    let admitted_counter_row = certification_only_admitted_stage_row(
        WorkloadEvidenceStage::BooleanSplit,
        evidence_identity,
        matching_counters,
    );
    let manual_row_with_matching_evidence = certification_manual_stage_row_with_counters(
        admitted_counter_row.stage(),
        admitted_counter_row.evidence_identity(),
        admitted_counter_row.counters(),
    );

    assert_eq!(
        manual_row_with_matching_evidence.stage(),
        admitted_counter_row.stage()
    );
    assert_eq!(
        manual_row_with_matching_evidence.evidence_identity(),
        admitted_counter_row.evidence_identity()
    );
    assert_eq!(
        manual_row_with_matching_evidence.counters(),
        admitted_counter_row.counters()
    );

    let denial =
        deny_manual_evidence_row_as_spatial_touch_authority(&manual_row_with_matching_evidence)
            .expect_err("manual evidence rows cannot substitute for spatial touch authority");

    assert_eq!(
        denial,
        SpatialEvidenceSubstitutionDenial::ManualEvidenceRow {
            stage: WorkloadEvidenceStage::BooleanSplit,
            backing: WorkloadEvidenceBacking::Manual,
        }
    );
}

#[test]
fn topology_touched_graph_basis_cannot_launder_spatial_evidence_authority() {
    assert_eq!(
        deny_topology_touched_graph_basis_as_spatial_touch_authority(
            PhantomData::<TopologyTouchedGraphBasis>
        ),
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface: SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis,
        }
    );
    assert_eq!(
        deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority(
            PhantomData::<TopologyDeclaredTouchedGraphBasisProof>,
        ),
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface:
                SpatialEvidenceTopologySubstitutionSurface::TopologyDeclaredTouchedGraphBasisProof,
        }
    );
    assert_eq!(
        deny_topology_laundering_as_spatial_touch_authority(
            SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis
        ),
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface: SpatialEvidenceTopologySubstitutionSurface::TopologyTouchedGraphBasis,
        }
    );
}

#[test]
fn facade_export_parser_covers_block_and_single_line_public_use_shapes() {
    let parsed_exports = facade_exports_from_source(
        r#"
        pub use crate::workload_platform::evidence_ledger::{BlockOne, BlockTwo};
        pub use crate::workload_platform::evidence_ledger::SingleLineExport;
        pub use crate::workload_platform::evidence_ledger::{
            BlockThree,
            BlockFour as PublicBlockFour,
        };
        "#,
    );

    assert!(parsed_exports.contains("BlockOne"));
    assert!(parsed_exports.contains("BlockTwo"));
    assert!(parsed_exports.contains("SingleLineExport"));
    assert!(parsed_exports.contains("BlockThree"));
    assert!(parsed_exports.contains("PublicBlockFour"));
}

fn names_for_category(
    rows: &[worth_spatial::facade::workload_vocabulary::SpatialEvidenceSurfaceDeletionLedgerRow],
    category: SpatialEvidenceSurfaceAuthorityCategory,
) -> BTreeSet<&'static str> {
    rows.iter()
        .filter(|row| row.authority_category() == category)
        .map(|row| row.surface_name())
        .collect()
}

fn assert_all_present(actual: BTreeSet<&'static str>, expected: &[&'static str]) {
    for name in expected {
        assert!(actual.contains(name), "missing inventory row for {name}");
    }
}

fn facade_workload_vocabulary_exports_from_source() -> BTreeSet<&'static str> {
    let source = include_str!("../../../../facade/workload_vocabulary/mod.rs");
    facade_exports_from_source(source)
}

fn facade_exports_from_source(source: &'static str) -> BTreeSet<&'static str> {
    let mut exports = BTreeSet::new();
    let mut inside_export_block = false;

    for line in source.lines() {
        let trimmed = line_before_comment(line).trim();
        if trimmed.is_empty() {
            continue;
        }
        if inside_export_block {
            if let Some(closing_items) = trimmed.strip_suffix("};") {
                insert_facade_export_items(&mut exports, closing_items);
                inside_export_block = false;
                continue;
            }
            insert_facade_export_items(&mut exports, trimmed);
            continue;
        }

        let Some(export_path) = trimmed.strip_prefix("pub use ") else {
            continue;
        };
        if export_path.ends_with("::{") {
            inside_export_block = true;
            continue;
        }

        if let Some(grouped_exports) = single_line_grouped_export_items(export_path) {
            insert_facade_export_items(&mut exports, grouped_exports);
            continue;
        }

        if let Some(single_export) = single_line_export_name(export_path) {
            insert_facade_export_items(&mut exports, single_export);
        }
    }

    assert!(
        !inside_export_block,
        "workload_vocabulary facade export block must be closed"
    );
    assert!(
        !exports.is_empty(),
        "real workload_vocabulary facade exports must be parsed"
    );
    exports
}

fn line_before_comment(line: &'static str) -> &'static str {
    line.split_once("//")
        .map_or(line, |(before_comment, _)| before_comment)
}

fn single_line_grouped_export_items(export_path: &'static str) -> Option<&'static str> {
    let (_, grouped_exports) = export_path.split_once("::{")?;
    grouped_exports.strip_suffix("};")
}

fn single_line_export_name(export_path: &'static str) -> Option<&'static str> {
    export_path
        .strip_suffix(';')
        .and_then(|path| path.rsplit_once("::"))
        .map(|(_, export_name)| export_name)
}

fn insert_facade_export_items(exports: &mut BTreeSet<&'static str>, raw_exports: &'static str) {
    for export in raw_exports
        .split(',')
        .map(normalize_facade_export_name)
        .filter(|item| !item.is_empty())
    {
        exports.insert(export);
    }
}

fn normalize_facade_export_name(export: &'static str) -> &'static str {
    export
        .trim()
        .trim_end_matches(';')
        .trim()
        .rsplit_once(" as ")
        .map_or_else(
            || export.trim().trim_end_matches(';').trim(),
            |(_, alias)| alias.trim(),
        )
}

fn assert_public_facade_rows_record_symbol_paths(rows: &[SpatialEvidenceSurfaceDeletionLedgerRow]) {
    for row in rows.iter().filter(|row| {
        row.authority_category() == SpatialEvidenceSurfaceAuthorityCategory::PublicFacadeExport
    }) {
        assert_eq!(
            row.exported_facade_path(),
            format!(
                "worth_spatial::facade::workload_vocabulary::{}",
                row.surface_name()
            ),
            "public facade row must record its concrete exported symbol path"
        );
    }
}

fn expected_ledger_constructors() -> &'static [&'static str] {
    &[
        "WorkloadEvidenceRow::new",
        "WorkloadEvidenceRow::from_boolean_evidence_receipt",
        "WorkloadEvidenceLedger::from_rows",
        "WorkloadEvidenceLedger::certify_complete",
        "CompleteWorkloadEvidenceLedger::require_boolean_receipt",
        "CompleteWorkloadEvidenceLedger::require_boolean_receipt_lookup",
        "CompleteWorkloadEvidenceLedger::link_required_stages",
        "CompleteWorkloadEvidenceLedger::with_boolean_evidence_receipt",
        "CompleteWorkloadEvidenceLedger::into_ledger",
        "WorkloadEvidenceStageIndexProduct::require_boolean_receipt",
        "WorkloadEvidenceStageIndexProduct::require_boolean_receipt_lookup",
        "WorkloadEvidenceStageIndexProduct::link_required_stages",
    ]
}

fn expected_boolean_receipts() -> &'static [&'static str] {
    &[
        "PlanarBooleanEventLedgerReceipt",
        "PlanarBooleanSegmentPairEnumerationReceipt",
        "PlanarBooleanSplitEdgeChainLedgerReceipt",
        "PlanarBooleanLoopReconstructionLedgerReceipt",
    ]
}

fn expected_kernel_consumption_paths() -> &'static [&'static str] {
    &[
        "WorthWorkload::require_boolean_declaration_entry",
        "WorthWorkload::require_boolean_route_plan",
        "WorthWorkload::require_boolean_operand_pair_construction",
        "WorthWorkload::require_boolean_blocker_provenance",
        "WorthWorkload::require_boolean_shared_plane_identity",
        "WorthWorkload::require_boolean_precision_agreement",
        "WorthWorkload::require_boolean_local_frame_selection",
        "WorthWorkload::require_boolean_operand_a_projection_consumption",
        "WorthWorkload::require_boolean_operand_b_projection_consumption",
        "WorthWorkload::require_boolean_reduced_operand_pair",
        "WorthWorkload::require_boolean_event_extraction_request",
        "WorthWorkload::require_boolean_segment_pair_enumeration",
        "WorthWorkload::require_boolean_event_ledger",
        "WorthWorkload::require_boolean_split",
        "WorthWorkload::require_boolean_split_lookup",
        "WorthWorkload::require_boolean_loop_reconstruction",
        "WorthWorkload::require_boolean_loop_reconstruction_lookup",
        "WorthWorkload::with_completed_boolean_split_ledger",
        "WorthWorkload::complete_boolean_split_handoff",
        "WorthWorkload::with_completed_boolean_loop_reconstruction",
        "PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(stage_index)",
    ]
}

fn assert_migrated_downstream_split_stage_index_input_is_deleted(
    rows: &[SpatialEvidenceSurfaceDeletionLedgerRow],
) {
    let row = rows
        .iter()
        .find(|row| {
            row.surface_name()
                == "PlanarBooleanDownstreamSplitConsumptionInput::from_split_ledger_receipt(stage_index)"
        })
        .expect("phase 6 migrated downstream split residue row must be recorded");
    assert_eq!(
        row.deletion_action(),
        SpatialEvidenceSurfaceDeletionAction::Deleted
    );
    assert!(!row.production_reachable());
    assert!(row.replacement_exists());
}
