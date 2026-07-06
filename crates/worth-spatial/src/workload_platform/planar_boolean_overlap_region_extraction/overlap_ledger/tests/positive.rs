use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionDecisionKind;

use super::support::{
    canonical_graph, decision_log, direct_bundle, ledger_bundle, replayed_inputs,
};

#[test]
fn overlap_region_ledger_is_replay_stable() {
    let (canonical, replayed) = replayed_inputs();

    assert_eq!(
        canonical.mint_overlap_region_ledger(),
        replayed.mint_overlap_region_ledger(),
    );
}

#[test]
fn phase_fourteen_bundle_is_the_ordinary_lowering_surface() {
    let identity_lineage = super::support::identity_bundle(&canonical_graph());
    let direct = direct_bundle(&identity_lineage);
    let ordinary = identity_lineage
        .mint_overlap_region_ledger()
        .expect("ordinary phase-fourteen seam should succeed");

    assert_eq!(ordinary, direct);
}

#[test]
fn decision_log_records_all_required_overlap_decision_families() {
    let bundle = ledger_bundle(&canonical_graph());
    let kinds = decision_log(&bundle)
        .rows()
        .iter()
        .map(|row| row.kind())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Request));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Participation));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Adjacency));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Arrangement));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Contact));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Area));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Winding));
    assert!(kinds.contains(&PlanarBooleanOverlapRegionDecisionKind::Identity));
}

#[test]
fn ledger_receipt_is_the_exclusive_downstream_overlap_boundary() {
    let identity_lineage = super::support::identity_bundle(&canonical_graph());
    let bundle = identity_lineage
        .mint_overlap_region_ledger()
        .expect("phase-fourteen seam should mint receipt from identity-lineage proof");
    let receipt = bundle.receipt();

    assert!(!receipt.receipt_identity().is_empty());
    assert_eq!(
        receipt.request_identity(),
        bundle.decision_log().request_identity(),
    );
    assert_eq!(
        receipt.decision_log_identity(),
        bundle.decision_log().decision_log_identity(),
    );
    assert_eq!(receipt.ledger_identity(), bundle.ledger().ledger_identity());
    assert_eq!(
        receipt.overlap_region_identity_map_identity(),
        identity_lineage
            .overlap_region_identity_map()
            .map_identity(),
    );
    assert_eq!(
        receipt.persistent_name_propagation_map_identity(),
        identity_lineage
            .persistent_name_propagation_map()
            .map_identity(),
    );
    assert_eq!(
        receipt.subshape_signature_map_identity(),
        identity_lineage.subshape_signature_map().map_identity(),
    );
}
