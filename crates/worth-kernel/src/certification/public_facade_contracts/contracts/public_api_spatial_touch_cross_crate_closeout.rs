use std::marker::PhantomData;

use worth_spatial::facade::query_adoption::{
    current_spatial_query_consumer_kit_adoption_status,
    spatial_query_graph_obligation_residue_manifest,
};
use worth_spatial::facade::workload_vocabulary::{
    deny_copied_receipt_fields_as_spatial_query_lowering_authority,
    deny_manual_evidence_row_as_spatial_touch_authority,
    deny_raw_row_as_spatial_query_lowering_authority,
    deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority,
    deny_topology_touched_basis_as_spatial_query_lowering_authority,
    spatial_evidence_surface_deletion_ledger, SpatialEvidenceQueryLoweringDenialKind,
    SpatialEvidenceSubstitutionDenial, SpatialEvidenceSurfaceAuthorityCategory,
    SpatialEvidenceSurfaceCloseoutPosture, SpatialEvidenceSurfaceDeletionAction,
    SpatialEvidenceTopologySubstitutionSurface, WorkloadEvidenceBacking, WorkloadEvidenceRow,
    WorkloadEvidenceStage,
};

const SPATIAL_CARGO_TOML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/Cargo.toml"
));
const KERNEL_SPATIAL_TOUCH_AUTHORITY_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/workload_composition/worth_workload/spatial_touch_authority.rs"
));
const TOPO_FACADE_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-topo/src/facade.rs"
));
const SPATIAL_QUERY_ADOPTION_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/query_adoption.rs"
));
const SPATIAL_WORKLOAD_VOCABULARY_FACADE_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/facade/workload_vocabulary/mod.rs"
));
const SPATIAL_QUERY_CONSUMER_KIT_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/query_adoption/consumer_kit.rs"
));
const SPATIAL_QUERY_SUPPORT_PROJECTION_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/query_adoption/support_projection.rs"
));
const SPATIAL_TOUCH_REJECTED_INPUT_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission/input.rs"
));
const SPATIAL_QUERY_LOWERING_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../worth-spatial/src/workload_platform/evidence_ledger/spatial_touch_admission/query_lowering.rs"
));
const KERNEL_BOOLEAN_STAGE_REQUIREMENTS_RS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/src/workload_composition/worth_workload/boolean_stage_requirements.rs"
));
const TOUCHED_GRAPH_ROADMAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/worth/touched-graph-roadmap.md"
));
const MILESTONE_7_ROADMAP: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../_docs/worth/milestone-7-roadmap.md"
));

#[test]
fn phase10_residue_rejection_keeps_deleted_surfaces_from_reentering_authority() {
    let manual_row = WorkloadEvidenceRow::new(WorkloadEvidenceStage::BooleanSplit, "manual");
    let manual_denial = deny_manual_evidence_row_as_spatial_touch_authority(&manual_row)
        .expect_err("manual raw rows cannot satisfy spatial touch authority");
    assert!(matches!(
        manual_denial,
        SpatialEvidenceSubstitutionDenial::ManualEvidenceRow {
            backing: WorkloadEvidenceBacking::Manual,
            ..
        }
    ));

    let topology_denial =
        deny_topology_declared_touched_graph_basis_proof_as_spatial_touch_authority(PhantomData);
    assert!(matches!(
        topology_denial,
        SpatialEvidenceSubstitutionDenial::TopologyAuthorityCannotSatisfySpatialEvidence {
            surface:
                SpatialEvidenceTopologySubstitutionSurface::TopologyDeclaredTouchedGraphBasisProof
        }
    ));

    let raw_row_denial = deny_raw_row_as_spatial_query_lowering_authority("WorkloadEvidenceRow");
    assert_eq!(
        raw_row_denial.kind(),
        SpatialEvidenceQueryLoweringDenialKind::RawRowSubstitution
    );
    let copied_receipt_denial =
        deny_copied_receipt_fields_as_spatial_query_lowering_authority("CopiedReceiptFields");
    assert_eq!(
        copied_receipt_denial.kind(),
        SpatialEvidenceQueryLoweringDenialKind::CopiedReceiptSubstitution
    );
    let topology_query_denial = deny_topology_touched_basis_as_spatial_query_lowering_authority(
        "TopologyTouchedGraphBasis",
    );
    assert_eq!(
        topology_query_denial.kind(),
        SpatialEvidenceQueryLoweringDenialKind::TopologyTouchedBasisSubstitution
    );

    let ledger = spatial_evidence_surface_deletion_ledger();
    assert!(ledger.iter().any(|row| row.surface_name()
        == "geometry_only_evidence_admission_from_boolean_evidence_receipt"
        && row.deletion_action() == SpatialEvidenceSurfaceDeletionAction::Deleted
        && row.closeout_posture() == SpatialEvidenceSurfaceCloseoutPosture::Deleted));
    assert!(ledger.iter().any(|row| row.authority_category()
        == SpatialEvidenceSurfaceAuthorityCategory::TopologySubstitutionBoundary
        && !row.production_reachable()
        && row.deletion_action() == SpatialEvidenceSurfaceDeletionAction::CertificationOnly));
    assert!(ledger
        .iter()
        .all(|row| !row.violates_replaced_production_bypass()));

    let residue =
        spatial_query_graph_obligation_residue_manifest().expect("Query adoption residue manifest");
    assert_eq!(residue.rows().len(), 2);
    assert_eq!(
        current_spatial_query_consumer_kit_adoption_status()
            .expect("Query adoption status")
            .residue_row_count(),
        residue.rows().len()
    );

    assert_reintroduced_broad_stage_scan_is_absent_from_ordinary_authority_sources();
    assert_local_query_support_is_capped_to_consumer_kit_residue(&residue);
    assert_type_name_guard_is_absent_from_ordinary_public_facade();
}

#[test]
fn phase10_dependency_direction_preserves_spatial_facade_ownership() {
    assert!(SPATIAL_CARGO_TOML.contains("forge-query.workspace = true"));
    assert!(SPATIAL_CARGO_TOML.contains("schema = { package = \"worth-schema\""));

    assert!(
        KERNEL_SPATIAL_TOUCH_AUTHORITY_RS.contains("worth_spatial::facade::workload_vocabulary")
    );
    assert!(KERNEL_SPATIAL_TOUCH_AUTHORITY_RS
        .contains("SpatialGeometryEvidenceTouchRequest::from_boolean_receipt"));
    assert!(!KERNEL_SPATIAL_TOUCH_AUTHORITY_RS.contains("worth_spatial::workload_platform"));
    assert!(!KERNEL_SPATIAL_TOUCH_AUTHORITY_RS.contains("WorkloadEvidenceRow::new"));
    assert!(!TOPO_FACADE_RS.contains("worth_spatial::"));
    assert!(SPATIAL_QUERY_ADOPTION_RS.contains("spatial_query_graph_obligation_adoption_proof"));
    assert!(SPATIAL_QUERY_CONSUMER_KIT_RS.contains("forge_query::facade::runtime"));
    assert!(SPATIAL_QUERY_CONSUMER_KIT_RS.contains("prove_adoption_with_execution"));
}

#[test]
fn phase10_roadmap_closeout_claims_only_milestone_four_surface() {
    let milestone_four = roadmap_section(
        TOUCHED_GRAPH_ROADMAP,
        "## Milestone 4: Spatial Geometry Evidence Touch Authority",
        "## Milestone 5:",
    );
    assert!(milestone_four.contains("sealed BooleanEvidenceReceipt-backed spatial touch authority"));
    assert!(milestone_four.contains("Query descriptor/adoption proof from spatial evidence"));
    assert!(milestone_four.contains("residue count: 1"));
    assert!(milestone_four.contains("worth_spatial::facade::workload_vocabulary"));
    assert!(milestone_four.contains("worth_spatial::facade::query_adoption"));
    assert!(milestone_four.contains("current_spatial_query_consumer_kit_adoption_status"));
    assert!(milestone_four.contains("does not close Milestone 5"));
    assert!(milestone_four.contains("does not close Milestones 6 through 8"));
    assert!(!milestone_four.contains("Query obligation selection is complete"));
    assert!(!milestone_four.contains("graph-read access is complete"));

    let touched_gate = roadmap_section(
        MILESTONE_7_ROADMAP,
        "## Worth Touched Graph Authority Gate",
        "## ",
    );
    assert!(
        touched_gate.contains("Milestone 4 is closed only for spatial evidence touch authority")
    );
    assert!(touched_gate.contains("Milestones 5 through 8 remain open"));
}

fn roadmap_section<'a>(document: &'a str, start_marker: &str, next_marker: &str) -> &'a str {
    let start = document
        .find(start_marker)
        .unwrap_or_else(|| panic!("{start_marker} missing"));
    let after_start = &document[start + start_marker.len()..];
    let end = after_start.find(next_marker).unwrap_or(after_start.len());
    &after_start[..end]
}

fn assert_reintroduced_broad_stage_scan_is_absent_from_ordinary_authority_sources() {
    let ordinary_authority_sources = [
        (
            "kernel spatial touch authority",
            KERNEL_SPATIAL_TOUCH_AUTHORITY_RS,
        ),
        (
            "kernel boolean stage requirements",
            KERNEL_BOOLEAN_STAGE_REQUIREMENTS_RS,
        ),
        ("spatial query lowering", SPATIAL_QUERY_LOWERING_RS),
    ];

    for (label, source) in ordinary_authority_sources {
        assert!(
            !source.contains("broad_stage_scan"),
            "{label} reintroduced a broad-stage scan path"
        );
        assert!(
            !source.contains("broad stage scan"),
            "{label} reintroduced broad-stage scan prose without certification"
        );
        assert!(
            !source.contains("broad_ledger_scan_count += 1"),
            "{label} reintroduced broad ledger scan execution"
        );
        assert!(
            !source.contains("WorkloadEvidenceStage::BooleanSplit")
                || source.contains("SpatialGeometryEvidenceTouchRequest::from_boolean_receipt"),
            "{label} is classifying spatial authority by stage instead of receipt-backed proof"
        );
    }
}

fn assert_local_query_support_is_capped_to_consumer_kit_residue(
    residue: &forge_query::facade::consumer_kit::ForgeQueryGraphObligationResidueManifest,
) {
    let support_projection_residue = residue
        .rows()
        .iter()
        .find(|row| row.class() == "worth-spatial-runtime-facade-support-projection")
        .expect("support projection residue row remains the only local Query support residue");

    assert_eq!(support_projection_residue.current_count(), 1);
    assert_eq!(support_projection_residue.must_not_exceed_count(), 1);
    assert_eq!(support_projection_residue.owner(), "worth-spatial");
    assert!(support_projection_residue
        .removal_trigger()
        .contains("Milestone 6.5 consumes graph-obligation adoption status directly"));
    assert!(support_projection_residue
        .blocker()
        .contains("public facade still exposes current_spatial_workload_support_pin_rows"));
    assert_eq!(support_projection_residue.decision(), "capped-residue");
    assert!(SPATIAL_QUERY_CONSUMER_KIT_RS.contains("support_projection.rs"));
    assert!(SPATIAL_QUERY_SUPPORT_PROJECTION_RS.contains("ForgeQuerySupportPinReport"));
    assert!(!SPATIAL_QUERY_SUPPORT_PROJECTION_RS.contains("pub fn spatial_touch_authority"));
    assert!(!SPATIAL_QUERY_SUPPORT_PROJECTION_RS.contains("SpatialGeometryEvidenceTouchAuthority"));
}

fn assert_type_name_guard_is_absent_from_ordinary_public_facade() {
    assert!(!SPATIAL_WORKLOAD_VOCABULARY_FACADE_RS
        .contains("SpatialGeometryEvidenceTouchRejectedInput"));
    assert!(!SPATIAL_WORKLOAD_VOCABULARY_FACADE_RS
        .contains("SpatialGeometryEvidenceTouchRejectedInputKind"));
    assert!(!SPATIAL_WORKLOAD_VOCABULARY_FACADE_RS.contains("type_name"));
    assert!(SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("use std::any::type_name;"));
    assert!(SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("SpatialGeometryEvidenceTouchRejectedInput"));
    assert!(SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("pub(crate) struct"));
    assert!(SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("pub(crate) fn deny(self)"));
    assert!(!SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("pub fn type_name"));
    assert!(!SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("SpatialGeometryEvidenceTouchAuthority"));
    assert!(!SPATIAL_TOUCH_REJECTED_INPUT_RS.contains("SpatialGeometryEvidenceTouchRequest"));
    assert!(!KERNEL_SPATIAL_TOUCH_AUTHORITY_RS.contains("std::any::type_name"));
    assert!(!KERNEL_SPATIAL_TOUCH_AUTHORITY_RS.contains("type_name::<"));
}
