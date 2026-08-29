//! What an observed write actually copies, partition by partition.
//!
//! Owning the map spine is not owning the partitions behind it. A write that
//! finds a reader edition outstanding copies the spine once and then copies
//! each partition it mutates out of structural sharing, and that second copy is
//! Theta(the partition's slots) rather than Theta(pointers). These tests hold
//! the spine cost and the partition cost apart, and pair every positive witness
//! with the unobserved run that must charge nothing.

use crate::storage::substrate::PinClass;
use crate::tests::support::*;

#[test]
fn complexity_budget_unobserved_write_copies_no_partition() {
    let mut runtime = populated_partitions();
    let anchor = create_entity(&mut runtime, "anchor");

    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, anchor, "anchor-updated");
    let counters = runtime.performance_access().counters();

    // The negative twin. Nothing holds the substrate, so every mutation lands
    // in place and neither copy lane has anything to report.
    assert_eq!(counters.full_state_clones, 0);
    assert_eq!(counters.ordinary_partitions_copied_on_write, 0);
    assert_eq!(counters.ordinary_partition_slots_copied_on_write, 0);
}

#[test]
fn complexity_budget_observed_write_copies_only_the_partition_it_touches() {
    let mut runtime = populated_partitions();
    let anchor = create_entity(&mut runtime, "anchor");
    let resident_partitions = runtime.acquire_partition_edition().len();
    assert!(resident_partitions >= 4);

    let observer = runtime.acquire_partition_edition();
    runtime.performance_access().reset_counters();
    let _ = update_entity(&mut runtime, anchor, "anchor-updated");
    let counters = runtime.performance_access().counters();
    drop(observer);

    // The positive witness the spine counter cannot give: an observed write
    // copies partition states, and that cost is charged rather than absorbed.
    assert_eq!(counters.full_state_clones, 1);
    assert_eq!(counters.ordinary_partitions_copied_on_write, 1);
    assert!(
        counters.ordinary_partition_slots_copied_on_write > 0,
        "a copied partition carries the slots it holds: {counters:?}"
    );
    // And it copies only what it touches. Charging the spine alone would have
    // reported the same number here whether one partition moved or all of them
    // did.
    assert!(
        counters.ordinary_partitions_copied_on_write < resident_partitions,
        "a single-partition write must not copy the whole substrate: {counters:?}"
    );
}

#[test]
fn complexity_budget_observed_substrate_wide_pass_copies_every_partition_it_holds() {
    let runtime = populated_partitions();
    let resident_partitions = runtime.acquire_partition_edition().len();

    let observer = runtime.acquire_partition_edition();
    runtime.performance_access().reset_counters();
    runtime
        .storage_authority()
        .clear_named_pins(PinClass::Branch);
    let counters = runtime.performance_access().counters();
    drop(observer);

    // A pass that takes every partition mutably under an outstanding reader
    // copies every one of them. This is the case the spine counter would have
    // reported as a single event.
    assert_eq!(counters.full_state_clones, 1);
    assert_eq!(
        counters.ordinary_partitions_copied_on_write,
        resident_partitions
    );
    assert!(counters.ordinary_partition_slots_copied_on_write > 0);
}

#[test]
fn complexity_budget_unobserved_substrate_wide_pass_copies_nothing() {
    let runtime = populated_partitions();

    runtime.performance_access().reset_counters();
    runtime
        .storage_authority()
        .clear_named_pins(PinClass::Branch);
    let counters = runtime.performance_access().counters();

    assert_eq!(counters.full_state_clones, 0);
    assert_eq!(counters.ordinary_partitions_copied_on_write, 0);
    assert_eq!(counters.ordinary_partition_slots_copied_on_write, 0);
}

/// A runtime whose records live in several partitions, so a per-partition
/// claim can be told apart from a substrate-wide one.
fn populated_partitions() -> crate::facade::runtime::RelationalRuntime {
    let mut runtime = runtime_with_test_schema();
    for (ordinal, partition) in [PartitionId(7), PartitionId(11), PartitionId(23)]
        .into_iter()
        .enumerate()
    {
        let _ = create_entity_in_partition(&mut runtime, &format!("resident-{ordinal}"), partition);
    }
    runtime
}
