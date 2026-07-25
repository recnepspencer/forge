#[path = "extent_streaming/allocation.rs"]
mod allocation;
#[path = "extent_streaming/identity.rs"]
mod identity;
#[path = "extent_streaming/read_damage.rs"]
mod read_damage;
#[path = "extent_streaming/residue.rs"]
mod residue;
#[path = "extent_streaming/roundtrip.rs"]
mod roundtrip;
#[path = "extent_streaming/source_failure.rs"]
mod source_failure;

#[test]
fn extent_allocation_peak_is_independent_of_logical_record_length() {
    allocation::prove();
}

#[test]
fn abandoned_candidate_identity_is_never_reused_by_a_later_publication() {
    identity::prove_abandoned_candidate_non_reuse();
}
