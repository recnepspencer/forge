use schema::facade::platform::authority::touched_graph_conflict_internal::{
    admit_conflict_evidence_participant_identity_from_digest,
    admit_conflict_spatial_touch_authority_locality_identity_from_digest,
    admit_conflict_topology_touched_closure_locality_identity_from_digest,
    admit_conflict_validator_participant_identity_from_digest,
};

#[test]
fn raw_strings_cannot_mint_internal_participant_or_locality_identities() {
    let copied = "copied-authority".to_string();

    assert!(
        admit_conflict_evidence_participant_identity_from_digest(&copied).is_err(),
        "copied evidence text must not mint shared participant authority"
    );
    assert!(
        admit_conflict_validator_participant_identity_from_digest(&copied).is_err(),
        "copied validator text must not mint shared participant authority"
    );
    assert!(
        admit_conflict_topology_touched_closure_locality_identity_from_digest(&copied).is_err(),
        "copied topology text must not mint shared locality authority"
    );
    assert!(
        admit_conflict_spatial_touch_authority_locality_identity_from_digest(&copied).is_err(),
        "copied spatial text must not mint shared locality authority"
    );
}

#[test]
fn internal_conflict_authority_trait_seam_is_removed() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/touched_graph_conflict/internal_source_traits_removed.rs");
}
