use crate::{
    PhysicalComplexityEvidenceReport, PhysicalComplexityProofBundle, PhysicalScalePropertyEvidence,
    PhysicalSubstrateCertificationDenial,
};
use worth_store_physical_format::{
    ManifestDiscoveryCounterSnapshot, OfflineVerifierCounterSnapshot, PageRecordCounterSnapshot,
    PhysicalAlgorithmReviewEvidence, PhysicalForegroundBoundednessReport,
    PhysicalFreeSpaceSearchPolicy, PhysicalHeaderDecodeCounterSnapshot,
    PhysicalOperationComplexityContract, PhysicalOperationCounterSnapshot, PhysicalOperationKind,
    PhysicalReferenceValidationCounterSnapshot, PlatformPhysicalFacadeCounterSnapshot,
};

pub(crate) fn complexity_reports(
) -> Result<Vec<PhysicalComplexityEvidenceReport>, PhysicalSubstrateCertificationDenial> {
    PhysicalOperationKind::s1_required()
        .into_iter()
        .map(complexity_report)
        .collect()
}

fn complexity_report(
    operation: PhysicalOperationKind,
) -> Result<PhysicalComplexityEvidenceReport, PhysicalSubstrateCertificationDenial> {
    let counters = operation_counters(operation);
    let fixture = hostile_fixture(operation, counters.clone())?;
    let scale_property = scale_property(operation, fixture.clone());
    PhysicalComplexityEvidenceReport::verify(
        PhysicalOperationComplexityContract::s1_required(operation),
        PhysicalComplexityProofBundle::new(
            counters,
            algorithm_review(operation),
            fixture,
            scale_property,
        ),
    )
    .map_err(|_| PhysicalSubstrateCertificationDenial::ComplexityEvidenceRejected)
}

fn scale_property(
    operation: PhysicalOperationKind,
    fixture: crate::PhysicalHostileScaleFixtureReport,
) -> PhysicalScalePropertyEvidence {
    if operation == PhysicalOperationKind::AppendRecordPlacement {
        PhysicalScalePropertyEvidence::FragmentedFreeSpaceBoundedOrDeferred { fixture }
    } else {
        PhysicalScalePropertyEvidence::CounterStableAcrossUnrelatedGrowth { fixture }
    }
}

fn operation_counters(operation: PhysicalOperationKind) -> PhysicalOperationCounterSnapshot {
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
            PhysicalOperationCounterSnapshot::from_page_record_locate(page_locate_counters())
        }
        PhysicalOperationKind::ManifestLookup => {
            PhysicalOperationCounterSnapshot::from_manifest_lookup(manifest_counters())
        }
        PhysicalOperationKind::RootManifestOpen => {
            PhysicalOperationCounterSnapshot::from_root_open(
                PlatformPhysicalFacadeCounterSnapshot::empty()
                    .with_open()
                    .with_reopen()
                    .with_root_publication(),
            )
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            PhysicalOperationCounterSnapshot::from_page_record_append(
                PageRecordCounterSnapshot::for_append(1).with_page_write(),
            )
        }
        PhysicalOperationKind::ManifestTraversal => {
            PhysicalOperationCounterSnapshot::from_manifest_traversal(manifest_counters())
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            PhysicalOperationCounterSnapshot::from_offline_verifier_walk(offline_counters())
        }
    }
}

fn hostile_fixture(
    operation: PhysicalOperationKind,
    counters: PhysicalOperationCounterSnapshot,
) -> Result<crate::PhysicalHostileScaleFixtureReport, PhysicalSubstrateCertificationDenial> {
    Ok(match operation {
        PhysicalOperationKind::HeaderDecode => {
            crate::PhysicalHostileScaleFixtureReport::header_decode_fixed_fields()
        }
        PhysicalOperationKind::PhysicalReferenceValidation => {
            crate::PhysicalHostileScaleFixtureReport::reference_validation_fixed_fields()
        }
        PhysicalOperationKind::LocateByReference => {
            crate::PhysicalHostileScaleFixtureReport::locate_reference_unrelated_growth()
                .map_err(|_| PhysicalSubstrateCertificationDenial::HostileScaleFixtureRejected)?
        }
        PhysicalOperationKind::ManifestLookup => {
            crate::PhysicalHostileScaleFixtureReport::manifest_index_lookup_growth(counters)
        }
        PhysicalOperationKind::RootManifestOpen => {
            crate::PhysicalHostileScaleFixtureReport::root_open_root_entries(counters)
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            crate::PhysicalHostileScaleFixtureReport::fragmented_free_space_for_append(
                fragmented_free_space(),
                counters,
            )
        }
        PhysicalOperationKind::ManifestTraversal => {
            crate::PhysicalHostileScaleFixtureReport::manifest_traversal_declared_growth(counters)
        }
        PhysicalOperationKind::OfflineVerifierWalk => {
            crate::PhysicalHostileScaleFixtureReport::offline_verifier_declared_walk(counters)
        }
    })
}

fn algorithm_review(operation: PhysicalOperationKind) -> PhysicalAlgorithmReviewEvidence {
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

fn page_locate_counters() -> PageRecordCounterSnapshot {
    PageRecordCounterSnapshot::for_locate_attempt()
        .with_slot_lookup()
        .with_frame_decode()
        .with_record_payload_view()
}

fn manifest_counters() -> ManifestDiscoveryCounterSnapshot {
    ManifestDiscoveryCounterSnapshot::for_reopen()
        .with_root_entries(4)
        .with_segment_manifest(1)
        .with_extent_manifest(1)
        .with_allocation_entries(4)
        .with_free_space_entries(1)
        .with_manifest_index_probe()
}

fn offline_counters() -> OfflineVerifierCounterSnapshot {
    OfflineVerifierCounterSnapshot::empty()
        .with_root_candidates_inspected(1)
        .with_manifest_rows_decoded(4)
        .with_header_decode()
        .with_slot_directory_entries(1)
        .with_extent_membership_check()
        .with_free_space_entry_checked()
        .with_parity_ready_references(4)
}

fn fragmented_free_space() -> PhysicalForegroundBoundednessReport {
    PhysicalFreeSpaceSearchPolicy::foreground_bounded(2, 2).evaluate(3, 3)
}
