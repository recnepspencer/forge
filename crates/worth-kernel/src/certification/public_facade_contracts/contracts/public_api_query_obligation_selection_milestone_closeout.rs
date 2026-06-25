use std::collections::BTreeSet;
use std::sync::OnceLock;

use worth_kernel::query_obligation_selection::selection_substrate::{
    deny_broad_collection_query_obligation_selector_authority,
    deny_copied_query_obligation_selection_parts,
    deny_in_memory_query_obligation_selection_authority,
    deny_lifecycle_only_query_obligation_selector_authority,
    deny_local_query_obligation_selector_authority,
    deny_local_support_row_query_obligation_authority,
    deny_raw_descriptor_query_obligation_selection_authority,
    deny_topology_spatial_substitution_query_obligation_authority, QueryObligationSelectionError,
    QueryObligationSelectionErrorKind,
};
use worth_kernel::workload_composition::{
    QueryGraphObligationSelectionAuthorityKind, WorkloadCatalog,
    WorthQueryObligationSelectionMilestoneFiveCloseout, WorthQuerySelectorPrecisionPosture,
    WorthWorkload,
};
use worth_spatial::facade::workload_vocabulary::lower_spatial_touch_authority_to_query_descriptor;

use super::public_api_query_obligation_selection_real_spatial_authority_support::real_spatial_authority_case;
use super::public_api_query_obligation_selection_support::primitive_construction_birth_cases;

#[test]
fn milestone_five_query_obligation_selection_closeout_is_closed() {
    let closeout = certified_milestone_five_closeout();

    assert!(closeout.is_closed());
    assert!(closeout.selected_obligation_count() > 0);
    assert!(closeout.execution_row_count() > 0);
    assert!(closeout.selected_registration_count() > 0);
    assert_eq!(
        closeout.selected_obligation_count(),
        closeout.selected_registration_count()
    );
    assert_eq!(
        closeout.selected_registration_count(),
        closeout.selected_registration_digests().len()
    );
    assert!(closeout.topology_selected_count() > 0);
    assert!(closeout.spatial_selected_count() > 0);
    assert!(!closeout.graph_read_access_planning_claimed());
    assert_eq!(closeout.open_finding_count(), 0);
    assert_eq!(closeout.topology_lane_count(), 1);
    assert_eq!(closeout.spatial_lane_count(), 1);
    assert_eq!(closeout.capped_broad_selector_residue_count(), 1);
    assert_eq!(closeout.uncapped_broad_selector_residue_count(), 0);
    assert_eq!(closeout.owned_query_gap_count(), 1);
    assert_eq!(closeout.incomplete_query_gap_count(), 0);
    assert_eq!(closeout.graph_read_access_planning_claimed_count(), 0);
    assert_no_empty_digest(closeout.authority_digests());
    assert_no_empty_digest(closeout.touch_descriptor_digests());
    assert_no_empty_digest(closeout.selected_registration_digests());
    assert_unique_digests(closeout.selected_registration_digests());
    assert_eq!(closeout.broad_selector_residue_count(), 1);
    assert_eq!(closeout.query_selector_gap_count(), 1);
}

#[test]
fn milestone_five_rejects_each_old_selector_authority_path() {
    assert_denial_kind(
        deny_local_query_obligation_selector_authority("local selector table"),
        QueryObligationSelectionErrorKind::LocalSelectorAuthorityDenied,
    );
    assert_denial_kind(
        deny_broad_collection_query_obligation_selector_authority("broad collection selector"),
        QueryObligationSelectionErrorKind::BroadCollectionSelectorAuthorityDenied,
    );
    assert_denial_kind(
        deny_lifecycle_only_query_obligation_selector_authority("lifecycle-only shortcut"),
        QueryObligationSelectionErrorKind::LifecycleOnlySelectorAuthorityDenied,
    );
    assert_denial_kind(
        deny_local_support_row_query_obligation_authority("local support row"),
        QueryObligationSelectionErrorKind::LocalSupportRowAuthorityDenied,
    );
    assert_denial_kind(
        deny_in_memory_query_obligation_selection_authority("in-memory proof"),
        QueryObligationSelectionErrorKind::InMemorySelectionAuthorityDenied,
    );
    assert_denial_kind(
        deny_copied_query_obligation_selection_parts("copied count"),
        QueryObligationSelectionErrorKind::CopiedSelectionPartsDenied,
    );
    assert_denial_kind(
        deny_raw_descriptor_query_obligation_selection_authority("raw descriptor"),
        QueryObligationSelectionErrorKind::RawDescriptorAuthorityDenied,
    );
    assert_denial_kind(
        deny_topology_spatial_substitution_query_obligation_authority(
            "topology/spatial substitution",
        ),
        QueryObligationSelectionErrorKind::TopologySpatialSubstitutionAuthorityDenied,
    );
}

#[test]
fn milestone_six_starts_from_selected_query_obligations() {
    let seed = certified_milestone_five_closeout()
        .clone()
        .into_graph_read_inventory_seed();

    assert!(seed.requires_graph_read_access_planning());
    assert!(!seed.graph_read_access_planning_claimed());
    assert!(seed.selected_obligation_count() > 0);
    assert!(seed.selected_registration_count() > 0);
    assert!(seed.execution_row_count() > 0);
    assert_eq!(
        seed.selected_obligation_count(),
        seed.selected_registration_count()
    );
    assert_eq!(
        seed.selected_registration_count(),
        seed.selected_registration_digests().len()
    );
    assert_no_empty_digest(seed.authority_digests());
    assert_no_empty_digest(seed.touch_descriptor_digests());
    assert_no_empty_digest(seed.selected_registration_digests());
    assert_unique_digests(seed.selected_registration_digests());
    assert_no_empty_digest(seed.residue_manifest_digests());
    assert_no_empty_digest(seed.execution_proof_digests());
    assert_no_empty_digest(seed.adoption_manifest_digests());
    assert_no_empty_digest(seed.selector_precision_report_digests());
}

fn certified_milestone_five_closeout() -> &'static WorthQueryObligationSelectionMilestoneFiveCloseout
{
    static CLOSEOUT: OnceLock<WorthQueryObligationSelectionMilestoneFiveCloseout> = OnceLock::new();
    CLOSEOUT.get_or_init(|| {
        let mut selected_closeouts = Vec::with_capacity(2);
        let workload = public_selection_workload();
        selected_closeouts.push(real_topology_selected_closeout(&workload));
        selected_closeouts.push(real_spatial_selected_closeout());
        WorthQueryObligationSelectionMilestoneFiveCloseout::from_selected_closeouts(
            selected_closeouts,
        )
        .expect("Milestone 5 must close from real selected Query obligations")
    })
}

fn real_topology_selected_closeout(
    workload: &WorthWorkload,
) -> worth_kernel::workload_composition::WorthQuerySelectedGraphObligationCloseout {
    let case = primitive_construction_birth_cases()
        .into_iter()
        .next()
        .expect("primitive construction support should provide a topology case");
    let touched_basis = case.declared_touched_basis("phase8-milestone-closeout");
    let selected = workload
        .select_query_graph_obligations(&touched_basis)
        .expect("topology touched basis should select Query obligations");
    let closeout = selected.closeout();

    assert_eq!(
        closeout.authority_kind(),
        QueryGraphObligationSelectionAuthorityKind::TopologyTouchedBasis
    );
    assert_eq!(
        closeout.selector_precision_report().posture(),
        WorthQuerySelectorPrecisionPosture::TouchedDescriptorBounded
    );
    assert!(closeout.local_ceremony_is_clean());
    assert!(!closeout.graph_read_access_planning_claimed());

    closeout
}

fn real_spatial_selected_closeout(
) -> worth_kernel::workload_composition::WorthQuerySelectedGraphObligationCloseout {
    let authority_case = real_spatial_authority_case("phase8-milestone-spatial-closeout");
    let descriptor = lower_spatial_touch_authority_to_query_descriptor(
        authority_case.authority(),
        authority_case.lookup(),
    )
    .expect("real spatial authority must lower to Query descriptor");
    let selected = authority_case
        .workload()
        .select_query_graph_obligations(&descriptor)
        .expect("real spatial descriptor should select Query obligations");
    let closeout = selected.closeout();

    assert_eq!(
        closeout.authority_kind(),
        QueryGraphObligationSelectionAuthorityKind::SpatialQueryDescriptor
    );
    assert_eq!(
        closeout.selector_precision_report().posture(),
        WorthQuerySelectorPrecisionPosture::QueryExpressivenessGap
    );
    assert_eq!(
        closeout.spatial_query_gap_rows(),
        descriptor.gap_rows().len()
    );
    assert!(closeout.local_ceremony_is_clean());
    assert!(!closeout.graph_read_access_planning_claimed());

    closeout
}

fn public_selection_workload() -> WorthWorkload {
    WorkloadCatalog::cube()
        .with_retained_replay_artifacts()
        .build()
        .expect("catalog cube workload should build")
        .into_workload()
}

fn assert_denial_kind(
    error: QueryObligationSelectionError,
    expected: QueryObligationSelectionErrorKind,
) {
    assert_eq!(error.kind(), expected);
    assert!(!error.detail().is_empty());
}

fn assert_no_empty_digest(digests: &[String]) {
    assert!(!digests.is_empty());
    assert!(digests.iter().all(|digest| !digest.is_empty()));
}

fn assert_unique_digests(digests: &[String]) {
    let unique = digests.iter().collect::<BTreeSet<_>>();
    assert_eq!(unique.len(), digests.len());
}
