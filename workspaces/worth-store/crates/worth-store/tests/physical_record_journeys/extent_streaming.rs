#[path = "extent_streaming/allocation.rs"]
mod allocation;
#[path = "extent_streaming/read_damage.rs"]
mod read_damage;
#[path = "extent_streaming/roundtrip.rs"]
mod roundtrip;
#[test]
fn extent_allocation_peak_is_independent_of_logical_record_length() {
    allocation::prove();
}
