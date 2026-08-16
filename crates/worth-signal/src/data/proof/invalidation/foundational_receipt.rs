use crate::data::telemetry::{InvalidationPerformedCounter, SignalInvalidationRealizedCounters};

use super::SignalInvalidationExecutionReceipt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvalidationFoundationalReceiptDenial {
    Claim(worth_foundational::FoundationalPerformanceClaimConstructionDenial),
    Attachment(worth_foundational::FoundationalPerformanceAttachmentConstructionDenial),
    Bundle(worth_foundational::FoundationalPerformanceBundleConstructionDenial),
    CounterRows(worth_foundational::FoundationalCounterBackedPerformanceReceiptConstructionDenial),
    ExcludedRecoveryWork,
}

pub type FoundationalInvalidationPerformanceReceipt =
    worth_foundational::FoundationalCounterBackedPerformanceReceipt<
        worth_foundational::FoundationalAuthoritativePerformanceClaim,
    >;

/// Validate Signal's performed rows against one exact expected contract.
pub fn attach_foundational_invalidation_performance_receipt(
    performed: SignalInvalidationExecutionReceipt,
    expected: SignalInvalidationRealizedCounters,
) -> Result<FoundationalInvalidationPerformanceReceipt, InvalidationFoundationalReceiptDenial> {
    if performed
        .realized_counters()
        .value(InvalidationPerformedCounter::RecoveryReconstructionWork)
        != 0
        || expected.value(InvalidationPerformedCounter::RecoveryReconstructionWork) != 0
    {
        return Err(InvalidationFoundationalReceiptDenial::ExcludedRecoveryWork);
    }
    let bundle = build_foundational_bundle(expected)?;
    let mut receipt = worth_foundational::counter_backed_performance_receipt(bundle);
    for counter in hot_counter_rows() {
        receipt =
            receipt.attach_counter_row(worth_foundational::FoundationalPerformanceCounterRow::new(
                counter_name(counter)?,
                performed.realized_counters().value(counter),
            ));
    }
    receipt
        .finish()
        .map_err(InvalidationFoundationalReceiptDenial::CounterRows)
}

pub(super) fn build_foundational_bundle(
    expected: SignalInvalidationRealizedCounters,
) -> Result<
    worth_foundational::FoundationalPerformanceBundle<
        worth_foundational::FoundationalAuthoritativePerformanceClaim,
    >,
    InvalidationFoundationalReceiptDenial,
> {
    let performance = worth_foundational::performance();
    let claim = performance
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
            worth_foundational::FoundationalPerformanceAccessPatternPosture::TraversalLocal,
        )
        .execution_temperature(
            worth_foundational::FoundationalPerformanceExecutionTemperature::HotPath,
        )
        .freshness_retention(
            worth_foundational::FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
        )
        .fallback_debt(worth_foundational::FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(worth_foundational::FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .include_work(
            worth_foundational::FoundationalPerformanceWorkClass::AuthoritativeObservation,
        )
        .include_work(worth_foundational::FoundationalPerformanceWorkClass::ValidationPlanning)
        .include_work(worth_foundational::FoundationalPerformanceWorkClass::PublicationDelivery)
        .exclude_work(worth_foundational::FoundationalPerformanceWorkClass::ReplayReconstruction)
        .exclude_work(
            worth_foundational::FoundationalPerformanceWorkClass::SupportReportAssembly,
        )
        .exclude_work(worth_foundational::FoundationalPerformanceWorkClass::ForensicParity)
        .finish()
        .map_err(InvalidationFoundationalReceiptDenial::Claim)?;
    let contract = worth_foundational::FoundationalPerformanceContractName::new(
        "signal.invalidation.performed.v1",
    )
    .map_err(InvalidationFoundationalReceiptDenial::Attachment)?;
    let layout = performance.define_layout_intent(
        worth_foundational::FoundationalPerformanceLayoutIntent::SoA,
        worth_foundational::FoundationalPerformanceAccessPatternPosture::TraversalLocal,
        worth_foundational::FoundationalPerformanceAllocationPosture::BatchLocal,
    );
    let mut bundle = worth_foundational::performance_bundle(claim)
        .attach_contract_name(contract)
        .attach_layout_intent_claim(layout);
    for counter in hot_counter_rows() {
        let name = counter_name(counter)?;
        bundle = bundle.attach_counter_spec(
            worth_foundational::FoundationalPerformanceCounterSpec::new(
                name,
                counter_work_class(counter),
                expected.value(counter),
            ),
        );
    }
    bundle
        .finish()
        .map_err(InvalidationFoundationalReceiptDenial::Bundle)
}

fn hot_counter_rows() -> impl Iterator<Item = InvalidationPerformedCounter> {
    InvalidationPerformedCounter::ALL
        .into_iter()
        .filter(|counter| *counter != InvalidationPerformedCounter::RecoveryReconstructionWork)
}

const fn counter_work_class(
    counter: InvalidationPerformedCounter,
) -> worth_foundational::FoundationalPerformanceWorkClass {
    use worth_foundational::FoundationalPerformanceWorkClass as Work;
    match counter {
        InvalidationPerformedCounter::SourceOutputDeltasConsumed
        | InvalidationPerformedCounter::DirectSettlementsProduced
        | InvalidationPerformedCounter::ProducedDeltasEmitted
        | InvalidationPerformedCounter::PropagationStops => Work::PublicationDelivery,
        InvalidationPerformedCounter::DirectSubscriberEdgesExamined
        | InvalidationPerformedCounter::ReverseIndexBucketProbes
        | InvalidationPerformedCounter::ReverseIndexCandidatesReturned
        | InvalidationPerformedCounter::CandidatesRejectedByAspectContract
        | InvalidationPerformedCounter::CandidatesRejectedByScope
        | InvalidationPerformedCounter::CandidatesRejectedByComparator
        | InvalidationPerformedCounter::StaleWorkRejected
        | InvalidationPerformedCounter::TopologyRevisionRevalidations
        | InvalidationPerformedCounter::RejectedTopologyMutations => Work::ValidationPlanning,
        InvalidationPerformedCounter::WorkItemsAdmitted
        | InvalidationPerformedCounter::WorkItemsMerged
        | InvalidationPerformedCounter::ReadyItemsEnqueued
        | InvalidationPerformedCounter::ReadyItemsPopped
        | InvalidationPerformedCounter::NodesEvaluated
        | InvalidationPerformedCounter::NonSemanticNodeVisits => Work::AuthoritativeMutation,
        InvalidationPerformedCounter::MaximumReadyFrontierWidth
        | InvalidationPerformedCounter::RetainedReadyFrontierWidth
        | InvalidationPerformedCounter::BatchLocalAllocations
        | InvalidationPerformedCounter::PeakBatchMemoryItems => Work::AuthoritativeObservation,
        InvalidationPerformedCounter::RecoveryReconstructionWork => Work::ReplayReconstruction,
    }
}

pub(super) fn counter_name(
    counter: InvalidationPerformedCounter,
) -> Result<
    worth_foundational::FoundationalPerformanceCounterName,
    InvalidationFoundationalReceiptDenial,
> {
    worth_foundational::FoundationalPerformanceCounterName::new(counter.name())
        .map_err(InvalidationFoundationalReceiptDenial::Attachment)
}
