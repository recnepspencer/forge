use crate::graph_read_access_declarations::{
    current_worth_graph_read_access_declaration_catalog_closeout,
    WorthGraphReadAccessDeclarationPhaseTwoCloseout,
    WorthGraphReadAccessDeclarationPhaseTwoErrorKind,
    WorthGraphReadTouchedAuthorityLoweringErrorKind, WorthGraphReadTouchedAuthoritySourceFamily,
};
use crate::graph_read_access_inventory::{
    current_worth_graph_read_access_milestone_six_closeout_for_tests,
    future_receipt_scope_milestone_seven_seed_for_tests,
    mismatched_touched_authority_milestone_seven_seed_for_tests,
    operating_world_milestone_seven_seeds_for_tests,
    same_family_multiple_callers_milestone_seven_seed_for_tests,
    topology_and_spatial_milestone_seven_seed_for_tests, WorthGraphReadAccessScopeExpectation,
    WorthGraphReadAccessScopeFamily,
};

#[test]
fn topology_and_spatial_inputs_lower_through_same_catalog() {
    let phase_one =
        super::phase_one_closeout_from_seed(&topology_and_spatial_milestone_seven_seed_for_tests());
    let phase_two = super::phase_two_closeout(&phase_one);

    let records = phase_two.declaration_catalog().records();
    assert_eq!(records.len(), 2);
    assert_eq!(phase_two.lowering_summary().lowered_record_count(), 2);
    assert_eq!(phase_two.lowering_summary().topology_lowered_count(), 1);
    assert_eq!(phase_two.lowering_summary().spatial_lowered_count(), 1);
    assert!(records.iter().all(|record| {
        record
            .key()
            .lowered_authority()
            .claims_read_declaration_authority()
            && !record.key().query_touch_descriptor_digest().is_empty()
    }));

    let topology_authority = lowered_authority_for_source_family(
        &phase_two,
        WorthGraphReadTouchedAuthoritySourceFamily::TopologyClosure,
    );
    let spatial_authority = lowered_authority_for_source_family(
        &phase_two,
        WorthGraphReadTouchedAuthoritySourceFamily::SpatialContinuation,
    );

    assert_eq!(
        topology_authority.query_touch_collection_label(),
        "worth_topology_loop_cycle_neighborhood"
    );
    assert_ne!(
        topology_authority.query_touch_collection_label(),
        spatial_authority.query_touch_collection_label()
    );
    assert_eq!(
        spatial_authority.query_touch_collection_label(),
        "worth_spatial_planar_boolean_continuation_index"
    );
    assert_read_verbs_include_graph_touch_authority(
        topology_authority.query_touch_read_verb_digest(),
    );
    assert_read_verbs_include_graph_touch_authority(
        spatial_authority.query_touch_read_verb_digest(),
    );
}

#[test]
fn selected_obligation_proof_cannot_stand_in_for_read_declaration() {
    let phase_one = super::phase_one_closeout_from_seed(
        &same_family_multiple_callers_milestone_seven_seed_for_tests(),
    );
    let phase_two = super::phase_two_closeout(&phase_one);

    for record in phase_two.declaration_catalog().records() {
        let lowered = record.key().lowered_authority();
        assert_eq!(
            lowered.source_family(),
            WorthGraphReadTouchedAuthoritySourceFamily::TopologyClosure
        );
        assert!(lowered.claims_read_declaration_authority());
        assert!(!lowered.claims_selected_obligation_is_declaration_authority());
        assert!(!lowered.claims_execution_authority());
    }
}

#[test]
fn mismatched_touched_authority_digest_cannot_lower_declaration() {
    let phase_one = super::phase_one_closeout_from_seed(
        &mismatched_touched_authority_milestone_seven_seed_for_tests(),
    );

    let error = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect_err("candidate authority labels must not override scope authority");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessDeclarationPhaseTwoErrorKind::TouchedAuthorityLoweringFailed
    );
    assert_eq!(
        error.touched_authority_lowering_error_kind(),
        Some(WorthGraphReadTouchedAuthorityLoweringErrorKind::TouchedAuthorityDigestMismatch)
    );
}

#[test]
fn future_execution_scope_cannot_stand_in_for_read_declaration() {
    let phase_one =
        super::phase_one_closeout_from_seed(&future_receipt_scope_milestone_seven_seed_for_tests());

    let error = current_worth_graph_read_access_declaration_catalog_closeout(&phase_one)
        .expect_err("future execution receipt scope must not lower as declaration authority");

    assert_eq!(
        error.kind(),
        WorthGraphReadAccessDeclarationPhaseTwoErrorKind::TouchedAuthorityLoweringFailed
    );
    assert_eq!(
        error.touched_authority_lowering_error_kind(),
        Some(WorthGraphReadTouchedAuthorityLoweringErrorKind::UnsupportedOperatingWorldScope)
    );
}

#[test]
fn branch_preview_and_authoritative_worlds_do_not_collapse() {
    let (authoritative_seed, preview_seed, branch_seed) =
        operating_world_milestone_seven_seeds_for_tests();
    let authoritative =
        super::phase_two_closeout(&super::phase_one_closeout_from_seed(&authoritative_seed));
    let preview = super::phase_two_closeout(&super::phase_one_closeout_from_seed(&preview_seed));
    let branch = super::phase_two_closeout(&super::phase_one_closeout_from_seed(&branch_seed));

    assert_ne!(
        super::only_catalog_record(&authoritative)
            .key()
            .operating_world_digest(),
        super::only_catalog_record(&preview)
            .key()
            .operating_world_digest()
    );
    assert_ne!(
        super::only_catalog_record(&authoritative)
            .key()
            .operating_world_digest(),
        super::only_catalog_record(&branch)
            .key()
            .operating_world_digest()
    );
    assert_ne!(
        super::only_catalog_record(&preview)
            .key()
            .operating_world_digest(),
        super::only_catalog_record(&branch)
            .key()
            .operating_world_digest()
    );
    assert_ne!(
        super::only_catalog_record(&authoritative).declaration_identity_digest(),
        super::only_catalog_record(&preview).declaration_identity_digest()
    );
    assert_ne!(
        super::only_catalog_record(&authoritative).declaration_identity_digest(),
        super::only_catalog_record(&branch).declaration_identity_digest()
    );
    assert_ne!(
        super::only_catalog_record(&preview).declaration_identity_digest(),
        super::only_catalog_record(&branch).declaration_identity_digest()
    );
}

#[test]
fn production_seed_exposes_real_topology_declarations_and_spatial_capability_gaps() {
    let milestone_six = current_worth_graph_read_access_milestone_six_closeout_for_tests();
    let seed = milestone_six.milestone_seven_seed();
    let phase_one = super::phase_one_closeout_from_seed(&seed);

    assert!(phase_one.declaration_candidates().iter().any(|candidate| {
        let scope = candidate.inventory_row_context().scope_binding();
        scope.scope_family() == WorthGraphReadAccessScopeFamily::TopologyReadLedger
            && scope.scope_expectation()
                == WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput
    }));
    assert!(seed.capability_gaps().iter().any(|gap| {
        let scope = gap.inventory_row_context().scope_binding();
        scope.scope_family() == WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation
            && scope.scope_expectation()
                == WorthGraphReadAccessScopeExpectation::QueryAccessRequirementCandidateInput
    }));
    assert!(!phase_one.declaration_candidates().iter().any(|candidate| {
        let scope = candidate.inventory_row_context().scope_binding();
        scope.scope_family() == WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation
            && scope.scope_expectation()
                == WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput
    }));
}

fn lowered_authority_for_source_family(
    phase_two: &WorthGraphReadAccessDeclarationPhaseTwoCloseout,
    source_family: WorthGraphReadTouchedAuthoritySourceFamily,
) -> &crate::graph_read_access_declarations::WorthGraphReadLoweredTouchedAuthority {
    let matches = phase_two
        .declaration_catalog()
        .records()
        .iter()
        .filter(|record| record.key().lowered_authority().source_family() == source_family)
        .collect::<Vec<_>>();
    assert_eq!(matches.len(), 1);
    matches[0].key().lowered_authority()
}

fn assert_read_verbs_include_graph_touch_authority(read_verb_digest: &str) {
    for expected in [
        "observes-collection",
        "observes-relation-kind",
        "observes-aspect",
        "exposes-derived-topology",
    ] {
        assert!(
            read_verb_digest.contains(expected),
            "missing {expected} in {read_verb_digest}"
        );
    }
}
