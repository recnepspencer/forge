use crate::{
    PageRecordCounterSnapshot, PhysicalAlgorithmReviewEvidence, PhysicalComplexityStatus,
    PhysicalForegroundBoundednessOutcome, PhysicalFreeSpaceSearchPolicy,
    PhysicalHeaderDecodeCounterSnapshot, PhysicalLocalityClass,
    PhysicalOperationComplexityContract, PhysicalOperationCounterSnapshot, PhysicalOperationKind,
    PhysicalReferenceValidationCounterSnapshot,
};

#[test]
fn every_required_physical_format_operation_has_verified_complexity_contract() {
    let operations = PhysicalOperationKind::required_physical_operations();

    assert_eq!(operations.len(), 8);
    for operation in operations {
        let contract = PhysicalOperationComplexityContract::required_complexity_contract(operation);
        assert_eq!(contract.operation(), operation);
        assert_eq!(contract.status(), PhysicalComplexityStatus::Declared);
        assert_eq!(contract.requirements().len(), 4);
        assert!(!contract.asymptotic_bound().is_empty());
        assert!(contract.is_physical_format_declared());
    }
}

#[test]
fn complexity_contracts_name_the_declared_locality_classes() {
    assert_eq!(
        PhysicalOperationComplexityContract::required_complexity_contract(
            PhysicalOperationKind::LocateByReference
        )
        .locality(),
        PhysicalLocalityClass::PageLocal
    );
    assert_eq!(
        PhysicalOperationComplexityContract::required_complexity_contract(
            PhysicalOperationKind::AppendRecordPlacement
        )
        .locality(),
        PhysicalLocalityClass::FreeSpaceClass
    );
    assert_eq!(
        PhysicalOperationComplexityContract::required_complexity_contract(
            PhysicalOperationKind::OfflineVerifierWalk
        )
        .locality(),
        PhysicalLocalityClass::ManifestDeclaredTraversal
    );
}

#[test]
fn normalized_counters_preserve_existing_operation_counter_values() {
    let locate_counters = PageRecordCounterSnapshot::for_locate_attempt()
        .with_slot_lookup()
        .with_frame_decode()
        .with_record_payload_view();
    let locate = PhysicalOperationCounterSnapshot::from_page_record_locate(locate_counters);
    assert_eq!(locate.operation(), PhysicalOperationKind::LocateByReference);
    assert_eq!(locate.observed("physical.slot_lookup"), Some(1));
    assert_eq!(locate.observed("physical.page_local_scan"), Some(0));

    let header = PhysicalOperationCounterSnapshot::from_header_decode(
        PhysicalHeaderDecodeCounterSnapshot::for_page_header_attempt(),
    );
    assert_eq!(header.observed("physical.header_decode_attempt"), Some(1));

    let reference = PhysicalOperationCounterSnapshot::from_reference_validation(
        PhysicalReferenceValidationCounterSnapshot::for_page_slot_attempt().with_generation_check(),
    );
    assert_eq!(reference.observed("physical.generation_check"), Some(1));
}

#[test]
fn fragmented_free_space_is_bounded_or_deferred_with_pressure_evidence() {
    let policy = PhysicalFreeSpaceSearchPolicy::foreground_bounded(2, 4);

    let bounded = policy.evaluate(2, 4);
    assert!(bounded.is_admitted());
    assert!(!bounded.pressure().exceeds_policy());

    let deferred = policy.evaluate(2, 5);
    assert_eq!(
        deferred.outcome(),
        PhysicalForegroundBoundednessOutcome::DeferredForMaintenance
    );
    assert!(deferred.pressure().exceeds_policy());
}

#[test]
fn algorithm_reviews_are_tied_to_the_operation_and_locality() {
    let review = PhysicalAlgorithmReviewEvidence::bounded_by_admitted_reference();

    assert_eq!(review.operation(), PhysicalOperationKind::LocateByReference);
    assert_eq!(review.locality(), PhysicalLocalityClass::PageLocal);
}
