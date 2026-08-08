//! Unit evidence for R8.2 pre-image retention.

use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectValue, CanonicalFieldPath, FieldKey, InternedString,
};
use worth_query_installation::facade::{InstalledCorrectionMechanism, InstalledPreImageDemand};
use worth_relational::facade::identity::{EntityId, KindId, PartitionId};
use worth_relational::facade::transactions::{planned_aspect_field_locator, RecordRef};

use super::aftermath_schema_fixture as fixture;
use super::undo_preimage::{
    demanded_field_slot, retain_preimage_from_test_footprint, WorthQueryObservedPreImageCandidate,
    WorthQueryPreImageRetentionDenial,
};
use crate::domain_computation::primary_graph::WorthQueryTouchedRecordIdentity;

fn install_inverse(field: &str) -> InstalledPreImageDemand {
    let aftermath = match field {
        "frozen" => fixture::freeze_account(),
        "note" => fixture::freeze_note(),
        other => panic!("unexpected pre-image field {other}"),
    };
    let Some(InstalledCorrectionMechanism::RecordedInverse(inverse)) = aftermath.mechanism() else {
        panic!("expected inverse");
    };
    inverse.preimage_demand().clone()
}

fn observed(field: &str, value: AspectValue, slot: u64) -> WorthQueryObservedPreImageCandidate {
    WorthQueryObservedPreImageCandidate::from_observed_field(
        locator("estate", field),
        value,
        entity(slot),
        KindId(7),
    )
}

fn entity(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn mutating(
    field: &str,
    slots: impl IntoIterator<Item = u64>,
) -> Vec<(RecordRef, AspectFieldLocator)> {
    slots
        .into_iter()
        .map(|slot| (RecordRef::Entity(entity(slot)), locator("estate", field)))
        .collect()
}

fn locator(aspect: &str, field: &str) -> AspectFieldLocator {
    planned_aspect_field_locator(
        AspectKey::new(aspect).unwrap(),
        CanonicalFieldPath::single(FieldKey::new(field).unwrap()),
    )
}

#[test]
fn retain_preimage_slices_exact_demand_and_bounds_bytes() {
    let demand = install_inverse("frozen");
    let candidates = vec![
        observed("frozen", AspectValue::Bool(true), 1),
        observed("ignored", AspectValue::Bool(false), 2),
    ];
    let retained =
        retain_preimage_from_test_footprint(&demand, &candidates, &mutating("frozen", [1]))
            .expect("retain");
    assert_eq!(retained.fields().len(), 1);
    assert_eq!(
        retained.field("frozen").unwrap().value(),
        &AspectValue::Bool(true)
    );
    assert!(retained.field("ignored").is_none());
    assert_eq!(
        retained.target_record(),
        Some(&RecordRef::Entity(EntityId::new(PartitionId::main(), 1, 1)))
    );
    assert_eq!(retained.field("frozen").unwrap().entity_kind(), KindId(7));
    assert_eq!(
        retained.field("frozen").unwrap().locator(),
        &locator("estate", "frozen")
    );
    assert!(retained.total_encoded_bytes() <= demand.maximum_encoded_bytes());
}

#[test]
fn right_record_wrong_field_cannot_supply_prior_truth() {
    let demand = install_inverse("frozen");
    let denied = retain_preimage_from_test_footprint(
        &demand,
        &[observed("frozen", AspectValue::Bool(true), 1)],
        &mutating("note", [1]),
    )
    .expect_err("changing another field on the same record is insufficient");
    assert_eq!(
        denied,
        WorthQueryPreImageRetentionDenial::MissingDemandedField
    );
}

#[test]
fn same_field_name_on_another_aspect_cannot_supply_prior_truth() {
    let demand = install_inverse("frozen");
    let candidate = WorthQueryObservedPreImageCandidate::from_observed_field(
        locator("estate", "frozen"),
        AspectValue::Bool(true),
        entity(1),
        KindId(7),
    );
    let changed = vec![(
        RecordRef::Entity(entity(1)),
        locator("accounting", "frozen"),
    )];
    let denied = retain_preimage_from_test_footprint(&demand, &[candidate], &changed)
        .expect_err("a same-named field in another aspect is a different target");
    assert_eq!(
        denied,
        WorthQueryPreImageRetentionDenial::MissingDemandedField
    );
}

#[test]
fn retain_preimage_denies_missing_demanded_field() {
    let demand = install_inverse("frozen");
    let denied = retain_preimage_from_test_footprint(&demand, &[], &mutating("frozen", [1]))
        .expect_err("missing field");
    assert_eq!(
        denied,
        WorthQueryPreImageRetentionDenial::MissingDemandedField
    );
}

#[test]
fn retain_preimage_denies_when_bytes_exceed_bound() {
    let demand = install_inverse("note");
    let big = AspectValue::String(InternedString::Raw("too-large-for-bound".into()));
    let denied = retain_preimage_from_test_footprint(
        &demand,
        &[observed("note", big, 1)],
        &mutating("note", [1]),
    )
    .expect_err("bound exceeded");
    assert_eq!(denied, WorthQueryPreImageRetentionDenial::ExceedsByteBound);
}

/// Q8.26-C3: a demanded slot is named only by an exactly-one-segment path.
///
/// The projection reduced each observed path to `field_path().fields().first()`,
/// so a nested `Account.Status` observation answered a demand for `Status` and
/// bound a value from the wrong depth as the prior truth. A demand has no
/// vocabulary for a nested path, so a multi-segment path names no demanded slot
/// at all.
#[test]
fn only_a_single_segment_path_names_a_demanded_field_slot() {
    let exact = CanonicalFieldPath::single(FieldKey::new("Status").unwrap());
    assert_eq!(
        demanded_field_slot(&exact).map(FieldKey::as_str),
        Some("Status")
    );

    let nested = CanonicalFieldPath::new([
        FieldKey::new("Account").unwrap(),
        FieldKey::new("Status").unwrap(),
    ])
    .unwrap();
    assert!(
        demanded_field_slot(&nested).is_none(),
        "a nested path must not satisfy a demand through its first segment"
    );
}

/// Q8.26-C7: the demanded slot must be bound to a record this operation
/// mutates, not merely to a slot name somewhere in the read-set.
///
/// Selection was `find(|c| c.field_slot() == slot)`. A decision read-set that
/// observes the demanded slot on several records — the common shape when an
/// invariant is checked across siblings before one of them is written — silently
/// retained whichever the iteration reached first. The receipt then bound a
/// "prior truth" belonging to a record the commit never touched, and undo would
/// faithfully restore the wrong entity's value.
#[test]
fn retain_preimage_binds_the_mutated_record_not_the_first_matching_slot() {
    let demand = install_inverse("frozen");
    let candidates = vec![
        // Observed first, but this record is only read for an invariant check.
        observed("frozen", AspectValue::Bool(false), 1),
        // The record the operation actually writes.
        observed("frozen", AspectValue::Bool(true), 2),
    ];
    let retained =
        retain_preimage_from_test_footprint(&demand, &candidates, &mutating("frozen", [2]))
            .expect("the mutated record's observation satisfies the demand");
    assert_eq!(
        retained.field("frozen").unwrap().value(),
        &AspectValue::Bool(true),
        "retention must take the prior truth of the record being mutated"
    );
    assert_eq!(
        retained.target_record(),
        Some(&RecordRef::Entity(entity(2)))
    );
}

/// Q8.26-C7: an observation on a record the operation never mutates cannot
/// satisfy the demand at all.
#[test]
fn retain_preimage_denies_when_only_unmutated_records_observed_the_slot() {
    let demand = install_inverse("frozen");
    let denied = retain_preimage_from_test_footprint(
        &demand,
        &[observed("frozen", AspectValue::Bool(true), 1)],
        &mutating("frozen", [2]),
    )
    .expect_err("an unmutated record's value is not this commit's prior truth");
    assert_eq!(
        denied,
        WorthQueryPreImageRetentionDenial::MissingDemandedField
    );
}

/// Q8.26-C7: when the demanded slot is observed on more than one *mutated*
/// record, no single prior truth is named and retention must refuse rather than
/// pick by position.
#[test]
fn retain_preimage_denies_an_ambiguous_demanded_slot() {
    let demand = install_inverse("frozen");
    let candidates = vec![
        observed("frozen", AspectValue::Bool(false), 1),
        observed("frozen", AspectValue::Bool(true), 2),
    ];
    let mutated = mutating("frozen", [1, 2]);
    let denied = retain_preimage_from_test_footprint(&demand, &candidates, &mutated)
        .expect_err("two mutated records observed the demanded slot");
    assert_eq!(
        denied,
        WorthQueryPreImageRetentionDenial::AmbiguousDemandedField
    );
}

/// Q8.26-C2/C7: an operation that creates but never modifies an existing record
/// has no prior truth, so a recorded inverse over it cannot be honoured.
#[test]
fn retain_preimage_denies_when_the_operation_mutates_no_existing_record() {
    let demand = install_inverse("frozen");
    let denied = retain_preimage_from_test_footprint(
        &demand,
        &[observed("frozen", AspectValue::Bool(true), 1)],
        &mutating("frozen", []),
    )
    .expect_err("a create-only operation has no prior truth to retain");
    assert_eq!(denied, WorthQueryPreImageRetentionDenial::NoMutatedRecord);
}

#[test]
fn undo_admission_rejects_preimage_target_substitution() {
    let target = EntityId::new(PartitionId::main(), 1, 1);
    let substituted = EntityId::new(PartitionId::main(), 2, 1);
    let preimage = retain_preimage_from_test_footprint(
        &install_inverse("frozen"),
        &[WorthQueryObservedPreImageCandidate::from_observed_field(
            locator("estate", "frozen"),
            AspectValue::Bool(true),
            target,
            KindId(7),
        )],
        &mutating("frozen", [1]),
    )
    .expect("retain exact target");
    let touched = [WorthQueryTouchedRecordIdentity::axis_probe(
        RecordRef::Entity(substituted),
    )];
    let denial = super::undo_evidence::require_exact_preimage_target(&preimage, &touched)
        .expect_err("a different touched record cannot receive this preimage");
    assert_eq!(
        denial.kind(),
        super::WorthQueryUndoDenialKind::TouchedRecordsRequired
    );
}
