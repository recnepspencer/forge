use super::super::inventory_lane;
use super::super::phase_six_closeout::WorthGraphReadAccessPhaseSixCounters;
use super::super::{
    WorthGraphReadAccessMilestoneSevenSeed, WorthGraphReadDeclarationCandidate,
    WorthGraphReadDeletionLedgerItem, WorthGraphReadReadFamilyTarget,
    WorthGraphReadRequirementVocabulary,
};

pub(crate) fn same_family_multiple_callers_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    milestone_seven_seed_from_candidates(same_family_multiple_callers_candidates_for_tests())
}

pub(crate) fn same_family_multiple_callers_reversed_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    let mut candidates = same_family_multiple_callers_candidates_for_tests();
    candidates.reverse();
    milestone_seven_seed_from_candidates(candidates)
}

pub(crate) fn conflicting_requirement_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    milestone_seven_seed_from_candidates(vec![
        declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        ),
        declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain/conflicting",
            "TopologyReadGraphAccessConflictProbe",
            "authority-a",
            WorthGraphReadRequirementVocabulary::predicate_filtered_relation(),
        ),
    ])
}

pub(crate) fn semantic_authority_pair_milestone_seven_seeds_for_tests() -> (
    WorthGraphReadAccessMilestoneSevenSeed,
    WorthGraphReadAccessMilestoneSevenSeed,
) {
    (
        milestone_seven_seed_from_candidates(vec![declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
        milestone_seven_seed_from_candidates(vec![declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-b",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
    )
}

pub(crate) fn same_semantics_different_provenance_milestone_seven_seeds_for_tests() -> (
    WorthGraphReadAccessMilestoneSevenSeed,
    WorthGraphReadAccessMilestoneSevenSeed,
) {
    (
        milestone_seven_seed_from_candidates(vec![declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
        milestone_seven_seed_from_candidates(vec![declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain/provenance_probe",
            "TopologyReadGraphAccessProvenanceProbe",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
    )
}

pub(crate) fn topology_and_spatial_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    milestone_seven_seed_from_candidates(vec![
        declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-topology",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        ),
        spatial_declaration_candidate_for_test_row(
            "crates/worth-spatial/src/workload_platform/planar_boolean_events",
            "PlanarBooleanFragmentContinuationIndex",
            "authority-spatial",
            WorthGraphReadRequirementVocabulary::predicate_filtered_relation(),
        ),
    ])
}

pub(crate) fn topology_spatial_and_broad_boolean_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    milestone_seven_seed_from_candidates(vec![
        declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-topology",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        ),
        spatial_declaration_candidate_for_test_row(
            "crates/worth-spatial/src/workload_platform/planar_boolean_events",
            "PlanarBooleanFragmentContinuationIndex",
            "authority-spatial",
            WorthGraphReadRequirementVocabulary::predicate_filtered_relation(),
        ),
        broad_boolean_declaration_candidate_for_test_row(
            "crates/worth-spatial/src/workload_platform/planar_boolean_loop_reconstruction",
            "PlanarBooleanBroadPredicateRead",
            "authority-spatial",
            WorthGraphReadRequirementVocabulary::predicate_filtered_relation(),
        ),
    ])
}

pub(crate) fn mismatched_touched_authority_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    let row = inventory_lane::declaration_candidate_row_with_scope_for_tests(
        "crates/worth-topo/src/projection/read_views/domain/mismatch",
        "TopologyReadGraphAccessMismatchProbe",
        "authority-scope",
    );
    milestone_seven_seed_from_candidates(vec![
        WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
            .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
            .touched_authority_input("authority-candidate")
            .requirement_vocabulary(WorthGraphReadRequirementVocabulary::relation_frontier())
            .milestone_seven_lowering_target("Milestone 7 mismatched authority rejection seed")
            .build()
            .expect("mismatched authority candidate should build before Phase 3 lowering"),
    ])
}

pub(crate) fn future_receipt_scope_milestone_seven_seed_for_tests(
) -> WorthGraphReadAccessMilestoneSevenSeed {
    let row = inventory_lane::future_receipt_declaration_candidate_row_for_tests(
        "crates/worth-topo/src/projection/read_views/domain/future_receipt",
        "TopologyReadGraphAccessFutureReceiptProbe",
        "authority-a",
    );
    milestone_seven_seed_from_candidates(vec![
        WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
            .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
            .touched_authority_input("authority-a")
            .requirement_vocabulary(WorthGraphReadRequirementVocabulary::relation_frontier())
            .milestone_seven_lowering_target("Milestone 7 future receipt rejection seed")
            .build()
            .expect("future receipt candidate should build before Phase 3 lowering"),
    ])
}

pub(crate) fn operating_world_milestone_seven_seeds_for_tests() -> (
    WorthGraphReadAccessMilestoneSevenSeed,
    WorthGraphReadAccessMilestoneSevenSeed,
    WorthGraphReadAccessMilestoneSevenSeed,
) {
    (
        milestone_seven_seed_from_candidates(vec![declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
        milestone_seven_seed_from_candidates(vec![preview_declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain/preview",
            "TopologyReadGraphAccessPreviewProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
        milestone_seven_seed_from_candidates(vec![branch_declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain/branch",
            "TopologyReadGraphAccessBranchProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        )]),
    )
}

fn same_family_multiple_callers_candidates_for_tests() -> Vec<WorthGraphReadDeclarationCandidate> {
    vec![
        declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain",
            "TopologyReadGraphAccessProof",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        ),
        declaration_candidate_for_test_row(
            "crates/worth-topo/src/projection/read_views/domain/certification",
            "TopologyReadGraphAccessCertification",
            "authority-a",
            WorthGraphReadRequirementVocabulary::relation_frontier(),
        ),
    ]
}

fn milestone_seven_seed_from_candidates(
    candidates: Vec<WorthGraphReadDeclarationCandidate>,
) -> WorthGraphReadAccessMilestoneSevenSeed {
    WorthGraphReadAccessMilestoneSevenSeed::new(
        candidates.clone(),
        Vec::new(),
        vec![standard_deletion_item_for_tests()],
        WorthGraphReadAccessPhaseSixCounters::new(candidates.len(), 0, 1, 0, 0),
    )
}

fn standard_deletion_item_for_tests() -> WorthGraphReadDeletionLedgerItem {
    let row = inventory_lane::deletion_target_row()
        .build()
        .expect("standard deletion target row should build");
    WorthGraphReadDeletionLedgerItem::for_inventory_row(&row)
        .deletion_trigger("synthetic Milestone 7 seed carries standard old graph-read deletion")
        .blocker("synthetic full-chain seeds must preserve deletion proof")
        .build()
        .expect("standard deletion item should build")
}

fn declaration_candidate_for_test_row(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
    requirement_vocabulary: WorthGraphReadRequirementVocabulary,
) -> WorthGraphReadDeclarationCandidate {
    let row = inventory_lane::declaration_candidate_row_with_scope_for_tests(
        source_path,
        current_caller,
        authority_digest,
    );
    WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
        .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
        .touched_authority_input(authority_digest)
        .requirement_vocabulary(requirement_vocabulary)
        .milestone_seven_lowering_target("Milestone 7 shared topology graph-read declaration seed")
        .build()
        .expect("test inventory row should lower to declaration candidate")
}

fn spatial_declaration_candidate_for_test_row(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
    requirement_vocabulary: WorthGraphReadRequirementVocabulary,
) -> WorthGraphReadDeclarationCandidate {
    let row = inventory_lane::spatial_declaration_candidate_row_for_tests(
        source_path,
        current_caller,
        authority_digest,
    );
    WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
        .read_family_target(WorthGraphReadReadFamilyTarget::SpatialPlanarBooleanContinuationIndex)
        .touched_authority_input(authority_digest)
        .requirement_vocabulary(requirement_vocabulary)
        .milestone_seven_lowering_target("Milestone 7 shared spatial graph-read declaration seed")
        .build()
        .expect("test spatial inventory row should lower to declaration candidate")
}

fn broad_boolean_declaration_candidate_for_test_row(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
    requirement_vocabulary: WorthGraphReadRequirementVocabulary,
) -> WorthGraphReadDeclarationCandidate {
    let row = inventory_lane::spatial_declaration_candidate_row_for_tests(
        source_path,
        current_caller,
        authority_digest,
    );
    WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
        .read_family_target(WorthGraphReadReadFamilyTarget::BroadBooleanPredicateGraphRead)
        .touched_authority_input(authority_digest)
        .requirement_vocabulary(requirement_vocabulary)
        .milestone_seven_lowering_target("Milestone 7 broad boolean graph-read declaration seed")
        .build()
        .expect("test broad boolean inventory row should lower to declaration candidate")
}

fn preview_declaration_candidate_for_test_row(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
    requirement_vocabulary: WorthGraphReadRequirementVocabulary,
) -> WorthGraphReadDeclarationCandidate {
    let row = inventory_lane::preview_declaration_candidate_row_for_tests(
        source_path,
        current_caller,
        authority_digest,
    );
    WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
        .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
        .touched_authority_input(authority_digest)
        .requirement_vocabulary(requirement_vocabulary)
        .milestone_seven_lowering_target("Milestone 7 preview graph-read declaration seed")
        .build()
        .expect("test preview inventory row should lower to declaration candidate")
}

fn branch_declaration_candidate_for_test_row(
    source_path: &str,
    current_caller: &str,
    authority_digest: &str,
    requirement_vocabulary: WorthGraphReadRequirementVocabulary,
) -> WorthGraphReadDeclarationCandidate {
    let row = inventory_lane::branch_declaration_candidate_row_for_tests(
        source_path,
        current_caller,
        authority_digest,
    );
    WorthGraphReadDeclarationCandidate::for_inventory_row(&row)
        .read_family_target(WorthGraphReadReadFamilyTarget::TopologyLoopCycleNeighborhood)
        .touched_authority_input(authority_digest)
        .requirement_vocabulary(requirement_vocabulary)
        .milestone_seven_lowering_target("Milestone 7 branch graph-read declaration seed")
        .build()
        .expect("test branch inventory row should lower to declaration candidate")
}
