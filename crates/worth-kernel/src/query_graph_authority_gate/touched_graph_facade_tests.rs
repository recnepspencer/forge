use super::*;

#[test]
fn ordinary_public_facade_audit_derives_exports_from_source() {
    let synthetic_source = r#"
mod internal;
pub(crate) use internal::InternalOnly;
pub mod reference_integrity;
pub use error::TopologyValidationError;
pub use facade::{
    validate_interpreted_topology, validate_named_topology_truth, SyntheticNewFacadeExport,
};
"#;

    let exports = touched_graph_facade_audit::ordinary_public_facade_exports_from_source(
        "crates/worth-topo/src/validation/mod.rs",
        synthetic_source,
    );

    assert_exported_surface(&exports, "reference_integrity");
    assert_exported_surface(&exports, "TopologyValidationError");
    assert_exported_surface(&exports, "validate_interpreted_topology");
    assert_exported_surface(&exports, "validate_named_topology_truth");
    assert_exported_surface(&exports, "SyntheticNewFacadeExport");
    assert!(
        exports
            .iter()
            .all(|export| export.exported_surface() != "InternalOnly"),
        "pub(crate) surfaces must not be treated as ordinary public exports"
    );
}

#[test]
fn ordinary_public_facade_audit_derives_spatial_workload_authority_source() {
    let synthetic_source = r#"
pub use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceRow, WorkloadEvidenceStageIndexProduct,
};
"#;

    let exports = touched_graph_facade_audit::ordinary_public_facade_exports_from_source(
        "crates/worth-spatial/src/facade/workload_vocabulary/mod.rs",
        synthetic_source,
    );

    assert!(
        exports
            .iter()
            .any(|export| export.exported_surface() == "WorkloadEvidenceRow"
                && export.authority_source_path()
                    == "crates/worth-spatial/src/workload_platform/evidence_ledger"),
        "spatial facade audit must derive source-owned workload authority families"
    );
}

#[test]
fn ordinary_public_facade_audit_derives_topology_projection_authority_source() {
    let exports =
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports();

    assert!(
        exports
            .iter()
            .any(|export| export.exported_surface() == "topology_runtime"
                && export.authority_source_path()
                    == "crates/worth-topo/src/projection/runtime_boundary/query_runtime"),
        "topology facade audit must derive source-owned projection runtime exports"
    );
    assert!(
        exports.iter().any(|export| export.exported_surface() == "NamingAttachmentReport"
            && export.authority_source_path()
                == "crates/worth-topo/src/projection/runtime_boundary/declared_query_surfaces/truth_surfaces"),
        "topology facade audit must derive source-owned projection truth-surface exports"
    );
}

#[test]
fn touched_graph_deletion_rejects_spatial_facade_exported_authority_family() {
    let mut touched_deletion_ledger = current_worth_touched_graph_deletion_ledger();
    touched_deletion_ledger.push(WorthTouchedGraphAuthorityDeletionLedgerRow::new(
        "bad.spatial-public-collapse",
        "spatial.evidence-ledger",
        "crates/worth-spatial/src/workload_platform/evidence_ledger",
        "worth-spatial",
        WorthTouchedGraphAuthorityDisposition::Collapse,
        "WorkloadEvidenceRow",
        "collapse row falsely claims workload evidence facade is sealed",
        "Phase 7 touched evidence lookup keyed by graph digest, stage, and receipt digest",
        "Phase 7 no-raw-scan public contract test passes.",
        touched_graph_inventory::SEALED_FROM_ORDINARY_FACADE,
        "negative test",
    ));

    let violation = certify_with_touched_inputs(
        current_worth_touched_graph_authority_inventory(),
        touched_deletion_ledger,
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err("source-owned spatial facade exports must fail delete/collapse certification");

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::TouchedGraphDeletionStillExportedByFacade(
            "bad.spatial-public-collapse"
        )
    );
}

#[test]
fn touched_graph_deletion_rejects_topology_projection_facade_exported_authority_family() {
    let mut touched_deletion_ledger = current_worth_touched_graph_deletion_ledger();
    touched_deletion_ledger.push(WorthTouchedGraphAuthorityDeletionLedgerRow::new(
        "bad.topology-projection-public-collapse",
        "topology.projection.read-stage",
        "crates/worth-topo/src/projection",
        "worth-topo",
        WorthTouchedGraphAuthorityDisposition::Collapse,
        "topology_runtime",
        "collapse row falsely claims projection facade is sealed",
        "Phase 6 touched invalidation and dirty propagation product",
        "Phase 6 dirty propagation proof is available to derived consumers.",
        touched_graph_inventory::SEALED_FROM_ORDINARY_FACADE,
        "negative test",
    ));

    let violation = certify_with_touched_inputs(
        current_worth_touched_graph_authority_inventory(),
        touched_deletion_ledger,
        touched_graph_static_authority::current_worth_touched_graph_static_authority_entries(),
        touched_graph_facade_audit::current_worth_touched_graph_ordinary_public_facade_exports(),
    )
    .expect_err(
        "source-owned topology projection facade exports must fail delete/collapse certification",
    );

    assert_eq!(
        violation,
        WorthGraphAuthorityGateViolation::TouchedGraphDeletionStillExportedByFacade(
            "bad.topology-projection-public-collapse"
        )
    );
}

fn assert_exported_surface(
    exports: &[touched_graph_facade_audit::WorthTouchedGraphOrdinaryPublicFacadeExport],
    expected_surface: &str,
) {
    assert!(
        exports
            .iter()
            .any(|export| export.exported_surface() == expected_surface),
        "missing source-derived ordinary facade export {expected_surface}"
    );
}

fn certify_with_touched_inputs(
    touched_graph_inventory: Vec<WorthTouchedGraphAuthorityInventoryRow>,
    touched_graph_deletion_ledger: Vec<WorthTouchedGraphAuthorityDeletionLedgerRow>,
    static_authority_entries: Vec<
        touched_graph_static_authority::WorthTouchedGraphStaticAuthorityEntry,
    >,
    ordinary_public_facade_exports: Vec<
        touched_graph_facade_audit::WorthTouchedGraphOrdinaryPublicFacadeExport,
    >,
) -> Result<WorthGraphAuthorityGateReport, WorthGraphAuthorityGateViolation> {
    certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        current_worth_graph_authority_deletion_ledger(),
        touched_graph_inventory,
        touched_graph_deletion_ledger,
        static_authority_entries,
        ordinary_public_facade_exports,
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &current_worth_graph_authority_audited_source_paths(),
    )
}
