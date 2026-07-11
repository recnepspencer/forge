use crate::{
    PhysicalComplexityEvidenceDenial, PhysicalComplexityEvidenceReport,
    PhysicalComplexityProofBundle, PhysicalHostileScaleCondition,
    PhysicalHostileScaleFixtureReport, PhysicalHostileScaleFixtureSource,
    PhysicalScalePropertyEvidence, PhysicalSubstrateLane,
};
use forge_store_physical_format::{
    ManifestDiscoveryCounterSnapshot, OfflineVerifierCounterSnapshot, PageRecordCounterSnapshot,
    PhysicalAlgorithmReviewEvidence, PhysicalComplexityStatus,
    PhysicalForegroundBoundednessOutcome, PhysicalFreeSpaceSearchPolicy,
    PhysicalHeaderDecodeCounterSnapshot, PhysicalLocalityClass,
    PhysicalOperationComplexityContract, PhysicalOperationCounterSnapshot, PhysicalOperationKind,
    PhysicalReferenceValidationCounterSnapshot, PlatformPhysicalFacadeCounterSnapshot,
};

#[test]
fn every_required_complexity_contract_has_four_part_evidence() {
    for operation in PhysicalOperationKind::required_physical_operations() {
        let report = PhysicalComplexityEvidenceReport::verify(
            PhysicalOperationComplexityContract::required_complexity_contract(operation),
            proof_for_operation(operation),
        )
        .unwrap();

        assert_eq!(report.lane(), PhysicalSubstrateLane::ScaleLocality);
        assert_eq!(
            report.contract().status(),
            PhysicalComplexityStatus::Declared
        );
        assert_eq!(report.status(), PhysicalComplexityStatus::Verified);
        assert!(report.is_platform_grade_verified());
        assert_eq!(report.counters().operation(), operation);
        assert!(!report.performance_receipt().counter_rows().is_empty());
    }
}

#[test]
fn every_required_contract_uses_named_hostile_condition() {
    for operation in PhysicalOperationKind::required_physical_operations() {
        let fixture = fixture_for_operation(operation);
        assert_eq!(fixture.operation(), operation);
        assert_eq!(fixture.condition(), expected_condition(operation));
        assert_eq!(fixture.source(), expected_source(operation));
    }
}

#[test]
fn debt_complexity_contract_is_rejected_before_receipt_construction() {
    let denial = PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::debt_for_tests(PhysicalOperationKind::HeaderDecode),
        proof_for_operation(PhysicalOperationKind::HeaderDecode),
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalComplexityEvidenceDenial::DebtContractRejected(PhysicalOperationKind::HeaderDecode)
    );
}

#[test]
fn algorithm_review_operation_must_match_contract() {
    let fixture = fixture_for_operation(PhysicalOperationKind::HeaderDecode);
    let proof = PhysicalComplexityProofBundle::new(
        fixture.baseline_counters().clone(),
        PhysicalAlgorithmReviewEvidence::bounded_by_admitted_reference(),
        fixture.clone(),
        scale_property_for_fixture(fixture),
    );
    let denial = PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::required_complexity_contract(PhysicalOperationKind::HeaderDecode),
        proof,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalComplexityEvidenceDenial::OperationMismatch {
            expected: PhysicalOperationKind::HeaderDecode,
            actual: PhysicalOperationKind::LocateByReference,
        }
    );
}

#[test]
fn locate_counters_are_stable_under_real_unrelated_growth_fixture() {
    let fixture = PhysicalHostileScaleFixtureReport::locate_reference_unrelated_growth().unwrap();
    let proof = PhysicalComplexityProofBundle::new(
        fixture.baseline_counters().clone(),
        PhysicalAlgorithmReviewEvidence::bounded_by_admitted_reference(),
        fixture.clone(),
        PhysicalScalePropertyEvidence::CounterStableAcrossUnrelatedGrowth { fixture },
    );
    let report = PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::required_complexity_contract(PhysicalOperationKind::LocateByReference),
        proof,
    )
    .unwrap();

    assert_eq!(report.counters().observed("physical.slot_lookup"), Some(1));
    assert_eq!(
        report.counters().observed("physical.page_local_scan"),
        Some(0)
    );
}

#[test]
fn fragmented_free_space_fixture_is_bounded_or_deferred_with_pressure() {
    let boundedness = PhysicalFreeSpaceSearchPolicy::foreground_bounded(2, 4).evaluate(2, 9);
    let fixture = PhysicalHostileScaleFixtureReport::fragmented_free_space_for_append(
        boundedness,
        counters_for_operation(PhysicalOperationKind::AppendRecordPlacement),
    );
    let proof = PhysicalComplexityProofBundle::new(
        fixture.baseline_counters().clone(),
        PhysicalAlgorithmReviewEvidence::bounded_append_placement(),
        fixture.clone(),
        PhysicalScalePropertyEvidence::FragmentedFreeSpaceBoundedOrDeferred { fixture },
    );
    let report = PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::required_complexity_contract(
            PhysicalOperationKind::AppendRecordPlacement,
        ),
        proof,
    )
    .unwrap();

    assert_eq!(
        boundedness.outcome(),
        PhysicalForegroundBoundednessOutcome::DeferredForMaintenance
    );
    assert_eq!(
        report.contract().locality(),
        PhysicalLocalityClass::FreeSpaceClass
    );
}

#[test]
fn unstable_scale_property_is_rejected() {
    let baseline = counters_for_operation(PhysicalOperationKind::ManifestLookup);
    let grown = PhysicalOperationCounterSnapshot::from_manifest_lookup(
        ManifestDiscoveryCounterSnapshot::for_reopen()
            .with_segment_manifest(9)
            .with_manifest_index_probe(),
    );
    let fixture =
        PhysicalHostileScaleFixtureReport::manifest_index_lookup_counter_drift(baseline, grown);
    let proof = PhysicalComplexityProofBundle::new(
        fixture.baseline_counters().clone(),
        PhysicalAlgorithmReviewEvidence::manifest_lookup(),
        fixture.clone(),
        PhysicalScalePropertyEvidence::CounterStableAcrossUnrelatedGrowth { fixture },
    );
    let denial = PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::required_complexity_contract(PhysicalOperationKind::ManifestLookup),
        proof,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalComplexityEvidenceDenial::ScalePropertyNotProven
    );
}

#[test]
fn detached_scale_property_fixture_is_rejected() {
    let hostile_fixture = PhysicalHostileScaleFixtureReport::manifest_index_lookup_growth(
        counters_for_operation(PhysicalOperationKind::ManifestLookup),
    );
    let detached_fixture = PhysicalHostileScaleFixtureReport::manifest_index_lookup_growth(
        PhysicalOperationCounterSnapshot::from_manifest_lookup(
            ManifestDiscoveryCounterSnapshot::for_reopen()
                .with_segment_manifest(9)
                .with_manifest_index_probe(),
        ),
    );
    let proof = PhysicalComplexityProofBundle::new(
        hostile_fixture.baseline_counters().clone(),
        PhysicalAlgorithmReviewEvidence::manifest_lookup(),
        hostile_fixture,
        PhysicalScalePropertyEvidence::CounterStableAcrossUnrelatedGrowth {
            fixture: detached_fixture,
        },
    );
    let denial = PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::required_complexity_contract(PhysicalOperationKind::ManifestLookup),
        proof,
    )
    .unwrap_err();

    assert_eq!(
        denial,
        PhysicalComplexityEvidenceDenial::DetachedScaleProperty
    );
}

fn proof_for_operation(operation: PhysicalOperationKind) -> PhysicalComplexityProofBundle {
    let fixture = fixture_for_operation(operation);
    PhysicalComplexityProofBundle::new(
        fixture.baseline_counters().clone(),
        review_for_operation(operation),
        fixture.clone(),
        scale_property_for_fixture(fixture),
    )
}

fn fixture_for_operation(operation: PhysicalOperationKind) -> PhysicalHostileScaleFixtureReport {
    if operation == PhysicalOperationKind::LocateByReference {
        return PhysicalHostileScaleFixtureReport::locate_reference_unrelated_growth().unwrap();
    }
    if operation == PhysicalOperationKind::HeaderDecode {
        return PhysicalHostileScaleFixtureReport::header_decode_fixed_fields();
    }
    if operation == PhysicalOperationKind::PhysicalReferenceValidation {
        return PhysicalHostileScaleFixtureReport::reference_validation_fixed_fields();
    }
    if operation == PhysicalOperationKind::ManifestLookup {
        return PhysicalHostileScaleFixtureReport::manifest_index_lookup_growth(
            counters_for_operation(operation),
        );
    }
    if operation == PhysicalOperationKind::RootManifestOpen {
        return PhysicalHostileScaleFixtureReport::root_open_root_entries(counters_for_operation(
            operation,
        ));
    }
    if operation == PhysicalOperationKind::AppendRecordPlacement {
        return PhysicalHostileScaleFixtureReport::fragmented_free_space_for_append(
            PhysicalFreeSpaceSearchPolicy::foreground_bounded(2, 4).evaluate(2, 4),
            counters_for_operation(operation),
        );
    }
    if operation == PhysicalOperationKind::ManifestTraversal {
        return PhysicalHostileScaleFixtureReport::manifest_traversal_declared_growth(
            counters_for_operation(operation),
        );
    }
    PhysicalHostileScaleFixtureReport::offline_verifier_declared_walk(counters_for_operation(
        operation,
    ))
}

fn counters_for_operation(operation: PhysicalOperationKind) -> PhysicalOperationCounterSnapshot {
    match operation {
        PhysicalOperationKind::HeaderDecode => {
            PhysicalOperationCounterSnapshot::from_header_decode(
                PhysicalHeaderDecodeCounterSnapshot::for_page_header_attempt(),
            )
        }
        PhysicalOperationKind::PhysicalReferenceValidation => {
            PhysicalOperationCounterSnapshot::from_reference_validation(
                PhysicalReferenceValidationCounterSnapshot::for_page_slot_attempt()
                    .with_generation_check(),
            )
        }
        PhysicalOperationKind::LocateByReference => {
            PhysicalOperationCounterSnapshot::from_page_record_locate(
                PageRecordCounterSnapshot::for_locate_attempt()
                    .with_slot_lookup()
                    .with_frame_decode()
                    .with_record_payload_view(),
            )
        }
        PhysicalOperationKind::ManifestLookup => {
            PhysicalOperationCounterSnapshot::from_manifest_lookup(
                ManifestDiscoveryCounterSnapshot::for_reopen()
                    .with_segment_manifest(3)
                    .with_manifest_index_probe(),
            )
        }
        PhysicalOperationKind::RootManifestOpen => {
            PhysicalOperationCounterSnapshot::from_root_open(
                PlatformPhysicalFacadeCounterSnapshot::empty().with_open(),
            )
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            PhysicalOperationCounterSnapshot::from_page_record_append(
                PageRecordCounterSnapshot::for_append(1).with_page_write(),
            )
        }
        PhysicalOperationKind::ManifestTraversal => {
            PhysicalOperationCounterSnapshot::from_manifest_traversal(
                ManifestDiscoveryCounterSnapshot::for_reopen()
                    .with_root_entries(2)
                    .with_segment_manifest(3)
                    .with_extent_manifest(5),
            )
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            PhysicalOperationCounterSnapshot::from_offline_verifier_walk(
                OfflineVerifierCounterSnapshot::empty()
                    .with_root_candidates_inspected(1)
                    .with_manifest_rows_decoded(4)
                    .with_header_decode()
                    .with_slot_directory_entries(1)
                    .with_extent_membership_check()
                    .with_free_space_entry_checked(),
            )
        }
    }
}

fn review_for_operation(operation: PhysicalOperationKind) -> PhysicalAlgorithmReviewEvidence {
    match operation {
        PhysicalOperationKind::HeaderDecode => {
            PhysicalAlgorithmReviewEvidence::constant_header_decode()
        }
        PhysicalOperationKind::PhysicalReferenceValidation => {
            PhysicalAlgorithmReviewEvidence::constant_reference_validation()
        }
        PhysicalOperationKind::LocateByReference => {
            PhysicalAlgorithmReviewEvidence::bounded_by_admitted_reference()
        }
        PhysicalOperationKind::ManifestLookup => PhysicalAlgorithmReviewEvidence::manifest_lookup(),
        PhysicalOperationKind::RootManifestOpen => {
            PhysicalAlgorithmReviewEvidence::root_manifest_open()
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            PhysicalAlgorithmReviewEvidence::bounded_append_placement()
        }
        PhysicalOperationKind::ManifestTraversal => {
            PhysicalAlgorithmReviewEvidence::manifest_traversal()
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            PhysicalAlgorithmReviewEvidence::offline_verifier_walk()
        }
    }
}

fn scale_property_for_fixture(
    fixture: PhysicalHostileScaleFixtureReport,
) -> PhysicalScalePropertyEvidence {
    if fixture.operation() == PhysicalOperationKind::AppendRecordPlacement {
        return PhysicalScalePropertyEvidence::FragmentedFreeSpaceBoundedOrDeferred { fixture };
    }
    PhysicalScalePropertyEvidence::CounterStableAcrossUnrelatedGrowth { fixture }
}

fn expected_condition(operation: PhysicalOperationKind) -> PhysicalHostileScaleCondition {
    match operation {
        PhysicalOperationKind::HeaderDecode => PhysicalHostileScaleCondition::FixedHeaderDecode,
        PhysicalOperationKind::PhysicalReferenceValidation => {
            PhysicalHostileScaleCondition::ReferenceValidationFixedFields
        }
        PhysicalOperationKind::LocateByReference => {
            PhysicalHostileScaleCondition::LocateUnrelatedGrowth
        }
        PhysicalOperationKind::ManifestLookup => {
            PhysicalHostileScaleCondition::ManifestIndexUnrelatedGrowth
        }
        PhysicalOperationKind::RootManifestOpen => {
            PhysicalHostileScaleCondition::RootOpenRootEntries
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            PhysicalHostileScaleCondition::FragmentedFreeSpacePressure
        }
        PhysicalOperationKind::ManifestTraversal => {
            PhysicalHostileScaleCondition::DeclaredManifestTraversal
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            PhysicalHostileScaleCondition::OfflineVerifierDeclaredWalk
        }
    }
}

fn expected_source(operation: PhysicalOperationKind) -> PhysicalHostileScaleFixtureSource {
    match operation {
        PhysicalOperationKind::HeaderDecode
        | PhysicalOperationKind::PhysicalReferenceValidation
        | PhysicalOperationKind::LocateByReference => {
            PhysicalHostileScaleFixtureSource::AuthorityExecution
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            PhysicalHostileScaleFixtureSource::PolicyEvaluation
        }
        PhysicalOperationKind::ManifestLookup
        | PhysicalOperationKind::RootManifestOpen
        | PhysicalOperationKind::ManifestTraversal
        | PhysicalOperationKind::OfflineVerifierWalk => {
            PhysicalHostileScaleFixtureSource::DeclaredCounterReceipt
        }
    }
}
