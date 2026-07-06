use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionLedgerAssemblyDenialKind;

use super::support::{
    canonical_graph, foreign_lineage_bundle, missing_signature_bundle,
    synthetic_identity_row_bundle,
};

#[test]
fn ledger_assembly_rejects_missing_prior_proof_products() {
    let denial = missing_signature_bundle(&canonical_graph())
        .mint_overlap_region_ledger()
        .expect_err("missing signature proof should deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::MissingPriorProofProductDenied,
    );
}

#[test]
fn ledger_assembly_rejects_foreign_prior_lineage() {
    let denial = foreign_lineage_bundle(&canonical_graph())
        .mint_overlap_region_ledger()
        .expect_err("foreign lineage should deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::InputIdentityMismatchDenied,
    );
}

#[test]
fn ledger_assembly_rejects_synthetic_overlap_rows() {
    let denial = synthetic_identity_row_bundle(&canonical_graph())
        .mint_overlap_region_ledger()
        .expect_err("synthetic overlap rows should deny ledger assembly");

    assert_eq!(
        denial.kind(),
        PlanarBooleanOverlapRegionLedgerAssemblyDenialKind::SyntheticOverlapRowDenied,
    );
}
