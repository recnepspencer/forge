use super::catalog::current_derived_invalidation_authority_inventory;
use super::classification::{
    DerivedInvalidationAuthorityDisposition, DerivedInvalidationAuthorityOwner,
    DerivedInvalidationOldAuthorityKind, DerivedInvalidationProductCategory,
    DerivedInvalidationReplacementPhase,
};
use super::closeout::DerivedInvalidationAuthorityInventoryCloseout;
use super::error::DerivedInvalidationAuthorityInventoryErrorKind;
use super::ordinary_admission::DerivedInvalidationOrdinaryProofAdmission;
use super::report::DerivedInvalidationAuthorityInventoryReport;
use super::row::DerivedInvalidationAuthorityInventoryRow;
use super::source_scan::DerivedInvalidationSourceCorpus;

#[test]
fn current_inventory_closes_with_phase_two_seed() {
    let inventory = current_derived_invalidation_authority_inventory();

    let closeout = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .expect("current derived invalidation authority inventory should close");

    assert_eq!(closeout.source_scan().uncovered_pattern_count(), 0);
    assert!(closeout.source_scan().scanned_source_count() >= 13);
    assert!(closeout.source_scan().observed_pattern_count() >= 13);
    assert!(!closeout.phase_two_seed().inventory_digest().is_empty());
    assert!(!closeout.phase_two_seed().seed_digest().is_empty());
    assert_eq!(closeout.inventory().counters().row_count(), 18);
    assert_eq!(closeout.inventory().counters().migrate_count(), 0);
    assert_eq!(closeout.inventory().counters().delete_count(), 8);
    assert_eq!(
        closeout
            .inventory()
            .counters()
            .certification_bootstrap_residue_count(),
        10
    );
    assert_eq!(closeout.inventory().counters().true_query_gap_count(), 0);
    assert_eq!(closeout.inventory().counters().capped_residue_count(), 10);
    assert_eq!(closeout.inventory().required_ordinary_categories().len(), 8);

    assert_required_row(
        closeout.inventory(),
        "crates/worth-topo/src/derived_topology/materialized_graph/mod.rs",
        "TopologyMaterializer::materialize_query_input",
        DerivedInvalidationProductCategory::MaterializedGraph,
        DerivedInvalidationOldAuthorityKind::QueryInputMaterialization,
        DerivedInvalidationAuthorityOwner::WorthTopoDerivedTopology,
        DerivedInvalidationReplacementPhase::PhaseSixProductMigrationSweep,
        DerivedInvalidationAuthorityDisposition::Delete,
    );
    assert_required_row(
        closeout.inventory(),
        "crates/worth-topo/src/projection/runtime_boundary/read_stage.rs",
        "deleted_projection_read_stage_ordinary_expansion",
        DerivedInvalidationProductCategory::ProjectionReadStage,
        DerivedInvalidationOldAuthorityKind::ProjectionReadStage,
        DerivedInvalidationAuthorityOwner::WorthTopoProjectionRuntimeBoundary,
        DerivedInvalidationReplacementPhase::PhaseSixProductMigrationSweep,
        DerivedInvalidationAuthorityDisposition::Delete,
    );
    assert_required_row(
        closeout.inventory(),
        "crates/worth-topo/src/certification/topology_operator_closeout/derived_fallout/derived_work_breadth.rs",
        "derived_work_breadth acceptance over declared_derived_region_count and fallback_count",
        DerivedInvalidationProductCategory::CertificationBootstrap,
        DerivedInvalidationOldAuthorityKind::OperatorDerivedBreadthCloseout,
        DerivedInvalidationAuthorityOwner::WorthTopoCertification,
        DerivedInvalidationReplacementPhase::CertificationBootstrapResidue,
        DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue,
    );
    assert_required_row(
        closeout.inventory(),
        "crates/worth-topo/src/derived_topology/traversal_views/wire_compatibility.rs",
        "interpret_wires",
        DerivedInvalidationProductCategory::WireViews,
        DerivedInvalidationOldAuthorityKind::TraversalInterpretation,
        DerivedInvalidationAuthorityOwner::WorthTopoDerivedTopology,
        DerivedInvalidationReplacementPhase::PhaseEightDeletionFirewall,
        DerivedInvalidationAuthorityDisposition::Delete,
    );
}

#[test]
fn closeout_rejects_missing_covered_product_category() {
    let mut rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    rows.retain(|row| row.product_category() != DerivedInvalidationProductCategory::WireViews);
    let inventory = DerivedInvalidationAuthorityInventoryReport::new(rows);

    let error = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .expect_err("missing covered product category must fail");

    assert_eq!(
        error.kind(),
        &DerivedInvalidationAuthorityInventoryErrorKind::MissingCoveredProductCategory {
            category: "wire_views"
        }
    );
}

#[test]
fn ordinary_whole_view_rebuild_cannot_close_as_residue() {
    let mut rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    rows.push(DerivedInvalidationAuthorityInventoryRow::new(
        "crates/worth-topo/src/derived_topology/materialized_graph/types.rs",
        "MaterializationFallbackClass::WholeViewRebuild",
        DerivedInvalidationProductCategory::MaterializedGraph,
        DerivedInvalidationOldAuthorityKind::WholeViewMaterialization,
        DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue,
        DerivedInvalidationAuthorityOwner::WorthTopoDerivedTopology,
        "ordinary product still needs migration",
        "covered product migrates in Phase 6",
        DerivedInvalidationReplacementPhase::CertificationBootstrapResidue,
        true,
        true,
        Some(1),
    ));
    let inventory = DerivedInvalidationAuthorityInventoryReport::new(rows);

    let error = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .expect_err("ordinary residue must fail before source scan");

    assert!(matches!(
        error.kind(),
        DerivedInvalidationAuthorityInventoryErrorKind::InvalidOrdinaryDisposition { .. }
    ));
}

#[test]
fn certification_residue_requires_cap_and_nonordinary_boundary() {
    let mut rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    rows.push(DerivedInvalidationAuthorityInventoryRow::new(
        "crates/worth-topo/src/certification/topology_operator_closeout/shared.rs",
        "derived_validation_report_from_materialized",
        DerivedInvalidationProductCategory::CertificationBootstrap,
        DerivedInvalidationOldAuthorityKind::DerivedValidationDiagnostic,
        DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue,
        DerivedInvalidationAuthorityOwner::WorthTopoCertification,
        "certification oracle",
        "ordinary invalidation receipt replaces oracle",
        DerivedInvalidationReplacementPhase::CertificationBootstrapResidue,
        false,
        true,
        None,
    ));
    let inventory = DerivedInvalidationAuthorityInventoryReport::new(rows);

    let error = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .expect_err("uncapped certification residue must fail");

    assert!(matches!(
        error.kind(),
        DerivedInvalidationAuthorityInventoryErrorKind::InvalidCertificationResidue { .. }
    ));
}

#[test]
fn source_corpus_scan_rejects_uncovered_dirty_authority_pattern() {
    let rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    let corpus = DerivedInvalidationSourceCorpus::from_inline_sources(vec![(
        "src/derived_topology/invalidation_plan/migrated_products/wire_views/legacy_interpretation.rs",
        "fn operator_path() { let dirty_products = true; }",
    )]);

    let scan = corpus
        .scan_against_inventory(&rows)
        .expect("inline corpus should scan");

    assert_eq!(scan.scanned_source_count(), 1);
    assert_eq!(scan.observed_pattern_count(), 1);
    assert_eq!(scan.uncovered_pattern_count(), 1);
    assert!(scan.uncovered_patterns()[0].contains("DirtyProducer"));
    assert!(scan.uncovered_patterns()[0].contains("dirty_products"));
}

#[test]
fn source_corpus_scan_treats_missing_configured_sources_as_deleted() {
    let rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    let corpus = DerivedInvalidationSourceCorpus::from_workspace_sources(vec![
        "src/derived_topology/definitely_missing_dirty_authority.rs",
    ]);

    let scan = corpus
        .scan_against_inventory(&rows)
        .expect("missing configured source means the old source was deleted");

    assert_eq!(scan.scanned_source_count(), 1);
    assert_eq!(scan.observed_pattern_count(), 0);
    assert_eq!(scan.uncovered_pattern_count(), 0);
}

#[test]
fn certification_residue_cannot_admit_as_ordinary_invalidation_proof() {
    let inventory = current_derived_invalidation_authority_inventory();
    let residue = inventory
        .rows()
        .iter()
        .find(|row| {
            row.disposition()
                == DerivedInvalidationAuthorityDisposition::CertificationBootstrapResidue
        })
        .expect("current inventory should include certification residue");

    let error = DerivedInvalidationOrdinaryProofAdmission::admit_inventory_row(residue)
        .expect_err("certification residue must not satisfy ordinary invalidation");

    assert!(matches!(
        error.kind(),
        DerivedInvalidationAuthorityInventoryErrorKind::CertificationResidueCannotSatisfyOrdinaryInvalidation { .. }
    ));
}

fn assert_required_row(
    inventory: &DerivedInvalidationAuthorityInventoryReport,
    source_path: &str,
    surface: &str,
    category: DerivedInvalidationProductCategory,
    authority_kind: DerivedInvalidationOldAuthorityKind,
    owner: DerivedInvalidationAuthorityOwner,
    replacement_phase: DerivedInvalidationReplacementPhase,
    disposition: DerivedInvalidationAuthorityDisposition,
) {
    let row = inventory
        .rows()
        .iter()
        .find(|row| row.source_path() == source_path && row.surface() == surface)
        .expect("required derived invalidation inventory row should exist");

    assert_eq!(row.product_category(), category);
    assert_eq!(row.authority_kind(), authority_kind);
    assert_eq!(row.owner(), owner);
    assert_eq!(row.disposition(), disposition);
    assert_eq!(row.replacement_phase(), replacement_phase);
    assert!(!row.blocker().trim().is_empty());
    assert!(!row.removal_trigger().trim().is_empty());
}

#[test]
fn ordinary_deleted_rows_can_close_inventory_before_final_source_firewall() {
    let mut rows = current_derived_invalidation_authority_inventory()
        .rows()
        .to_vec();
    rows.push(DerivedInvalidationAuthorityInventoryRow::new(
        "crates/worth-topo/src/derived_topology/materialized_graph/deleted_ordinary_path.rs",
        "deleted_whole_view_materializer",
        DerivedInvalidationProductCategory::MaterializedGraph,
        DerivedInvalidationOldAuthorityKind::WholeViewMaterialization,
        DerivedInvalidationAuthorityDisposition::Delete,
        DerivedInvalidationAuthorityOwner::WorthTopoDerivedTopology,
        "old ordinary path has been hard-deleted",
        "final source firewall confirms no lingering implementation",
        DerivedInvalidationReplacementPhase::PhaseEightDeletionFirewall,
        true,
        false,
        None,
    ));
    let inventory = DerivedInvalidationAuthorityInventoryReport::new(rows);

    let closeout = DerivedInvalidationAuthorityInventoryCloseout::close(inventory)
        .expect("ordinary deleted rows are valid inventory classifications");

    assert!(closeout
        .inventory()
        .rows()
        .iter()
        .any(|row| row.disposition() == DerivedInvalidationAuthorityDisposition::Delete));
}
