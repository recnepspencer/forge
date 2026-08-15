use crate::data::telemetry::{InvalidationPerformedCounter, SignalInvalidationRealizedCounters};

use super::foundational_receipt::{build_foundational_bundle, counter_name};

#[test]
fn foundational_receipt_denies_missing_duplicate_and_unexpected_rows() {
    let expected = SignalInvalidationRealizedCounters::default();
    let missing = worth_foundational::counter_backed_performance_receipt(
        build_foundational_bundle(expected).unwrap(),
    )
    .finish();
    assert!(matches!(
        missing,
        Err(worth_foundational::FoundationalCounterBackedPerformanceReceiptConstructionDenial::MissingCounterRowForSpec)
    ));

    let duplicate_name = counter_name(InvalidationPerformedCounter::NodesEvaluated).unwrap();
    let duplicate = worth_foundational::counter_backed_performance_receipt(
        build_foundational_bundle(expected).unwrap(),
    )
    .attach_counter_row(worth_foundational::FoundationalPerformanceCounterRow::new(
        duplicate_name.clone(),
        0,
    ))
    .attach_counter_row(worth_foundational::FoundationalPerformanceCounterRow::new(
        duplicate_name,
        0,
    ))
    .finish();
    assert!(matches!(
        duplicate,
        Err(worth_foundational::FoundationalCounterBackedPerformanceReceiptConstructionDenial::DuplicateCounterRow)
    ));

    let mut unexpected = worth_foundational::counter_backed_performance_receipt(
        build_foundational_bundle(expected).unwrap(),
    );
    for counter in InvalidationPerformedCounter::ALL {
        unexpected = unexpected.attach_counter_row(
            worth_foundational::FoundationalPerformanceCounterRow::new(
                counter_name(counter).unwrap(),
                0,
            ),
        );
    }
    unexpected =
        unexpected.attach_counter_row(worth_foundational::FoundationalPerformanceCounterRow::new(
            worth_foundational::FoundationalPerformanceCounterName::new("unexpected_row").unwrap(),
            0,
        ));
    assert!(matches!(
        unexpected.finish(),
        Err(worth_foundational::FoundationalCounterBackedPerformanceReceiptConstructionDenial::UnexpectedCounterRow)
    ));
}

#[test]
fn foundational_hot_path_disclosure_rejects_replay_and_support_laundering() {
    for laundered in [
        worth_foundational::FoundationalPerformanceWorkClass::ReplayReconstruction,
        worth_foundational::FoundationalPerformanceWorkClass::SupportReportAssembly,
        worth_foundational::FoundationalPerformanceWorkClass::ForensicParity,
    ] {
        let claim = worth_foundational::performance()
            .claim()
            .authoritative_execution()
            .boundary(worth_foundational::FoundationalPerformanceBoundary::AuthoritativeExecution)
            .evidence_strength(
                worth_foundational::FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt,
            )
            .breadth_locality(
                worth_foundational::FoundationalPerformanceBreadthLocalityPosture::DeltaBound,
            )
            .access_pattern(
                worth_foundational::FoundationalPerformanceAccessPatternPosture::DensityAdaptive,
            )
            .execution_temperature(
                worth_foundational::FoundationalPerformanceExecutionTemperature::HotPath,
            )
            .freshness_retention(
                worth_foundational::FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
            )
            .fallback_debt(worth_foundational::FoundationalPerformanceFallbackDebtPosture::Verified)
            .include_work(
                worth_foundational::FoundationalPerformanceWorkClass::AuthoritativeMutation,
            )
            .include_work(
                worth_foundational::FoundationalPerformanceWorkClass::AuthoritativeObservation,
            )
            .include_work(
                worth_foundational::FoundationalPerformanceWorkClass::ValidationPlanning,
            )
            .include_work(
                worth_foundational::FoundationalPerformanceWorkClass::PublicationDelivery,
            )
            .include_work(laundered);
        let claim = [
            worth_foundational::FoundationalPerformanceWorkClass::ReplayReconstruction,
            worth_foundational::FoundationalPerformanceWorkClass::SupportReportAssembly,
            worth_foundational::FoundationalPerformanceWorkClass::ForensicParity,
        ]
        .into_iter()
        .filter(|work| *work != laundered)
        .fold(claim, |claim, work| claim.exclude_work(work))
        .finish();
        assert!(matches!(
            claim,
            Err(worth_foundational::FoundationalPerformanceClaimConstructionDenial::PrimitiveLegality(
                worth_foundational::FoundationalPerformancePrimitiveLegalityDenial::HotPathIncludedWorkMustExcludeReplaySupportAndForensics
            ))
        ));
    }
}
