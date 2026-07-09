use worth_foundational::performance_api::lower_lane::basis::{
    FoundationalPerformanceAttachmentConstructionDenial, FoundationalPerformanceBundle,
    FoundationalPerformanceBundleConstructionDenial, FoundationalPerformanceContractName,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterSpec,
};
use worth_foundational::performance_api::lower_lane::receipts::{
    counter_backed_performance_receipt, FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceCounterRow,
};
use worth_foundational::{
    performance, performance_bundle, FoundationalAuthoritativePerformanceClaim,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

use super::{LatchAcquisitionDenial, LatchWaitCounterSnapshot};

pub type LatchCounterPerformanceReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LatchCounterEvidenceDenial {
    Name(FoundationalPerformanceAttachmentConstructionDenial),
    Claim,
    Bundle(FoundationalPerformanceBundleConstructionDenial),
    Receipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

#[derive(Debug)]
pub struct LatchDeniedBeforeWaitEvidence {
    denial: LatchAcquisitionDenial,
    counters: LatchWaitCounterSnapshot,
    counter_receipt: LatchCounterPerformanceReceipt,
}

impl LatchDeniedBeforeWaitEvidence {
    pub fn new(
        denial: LatchAcquisitionDenial,
        counters: LatchWaitCounterSnapshot,
    ) -> Result<Self, LatchCounterEvidenceDenial> {
        let counter_receipt = latch_counter_backed_performance_receipt(counters)?;
        Ok(Self {
            denial,
            counters,
            counter_receipt,
        })
    }

    pub fn denial(&self) -> LatchAcquisitionDenial {
        self.denial
    }

    pub fn counters(&self) -> LatchWaitCounterSnapshot {
        self.counters
    }

    pub fn counter_receipt(&self) -> &LatchCounterPerformanceReceipt {
        &self.counter_receipt
    }
}

pub fn latch_counter_backed_performance_receipt(
    counters: LatchWaitCounterSnapshot,
) -> Result<LatchCounterPerformanceReceipt, LatchCounterEvidenceDenial> {
    let bundle = latch_counter_performance_bundle(counters)?;
    counter_backed_performance_receipt(bundle)
        .attach_counter_row(counter_row("s5.latch.attempts", counters.attempt_count())?)
        .attach_counter_row(counter_row("s5.latch.waits", counters.wait_count())?)
        .attach_counter_row(counter_row(
            "s5.latch.denied-upgrades",
            counters.denied_upgrade_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.latch.detected-cycles",
            counters.detected_cycle_count(),
        )?)
        .attach_counter_row(counter_row(
            "s5.latch.execution-time-discovery-denials",
            counters.execution_time_discovery_denial_count(),
        )?)
        .finish()
        .map_err(LatchCounterEvidenceDenial::Receipt)
}

fn latch_counter_performance_bundle(
    counters: LatchWaitCounterSnapshot,
) -> Result<
    FoundationalPerformanceBundle<FoundationalAuthoritativePerformanceClaim>,
    LatchCounterEvidenceDenial,
> {
    let claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .finish()
        .map_err(|_| LatchCounterEvidenceDenial::Claim)?;

    performance_bundle(claim)
        .attach_contract_name(contract_name()?)
        .attach_counter_spec(counter_spec("s5.latch.attempts", counters.attempt_count())?)
        .attach_counter_spec(counter_spec("s5.latch.waits", counters.wait_count())?)
        .attach_counter_spec(counter_spec(
            "s5.latch.denied-upgrades",
            counters.denied_upgrade_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.latch.detected-cycles",
            counters.detected_cycle_count(),
        )?)
        .attach_counter_spec(counter_spec(
            "s5.latch.execution-time-discovery-denials",
            counters.execution_time_discovery_denial_count(),
        )?)
        .finish()
        .map_err(LatchCounterEvidenceDenial::Bundle)
}

fn contract_name() -> Result<FoundationalPerformanceContractName, LatchCounterEvidenceDenial> {
    FoundationalPerformanceContractName::new("worth-store.s5.latch-counter-evidence")
        .map_err(LatchCounterEvidenceDenial::Name)
}

fn counter_spec(
    name: &'static str,
    expected_exact_count: u64,
) -> Result<FoundationalPerformanceCounterSpec, LatchCounterEvidenceDenial> {
    Ok(FoundationalPerformanceCounterSpec::new(
        counter_name(name)?,
        FoundationalPerformanceWorkClass::ValidationPlanning,
        expected_exact_count,
    ))
}

fn counter_row(
    name: &'static str,
    observed_count: u64,
) -> Result<FoundationalPerformanceCounterRow, LatchCounterEvidenceDenial> {
    Ok(FoundationalPerformanceCounterRow::new(
        counter_name(name)?,
        observed_count,
    ))
}

fn counter_name(
    name: &'static str,
) -> Result<FoundationalPerformanceCounterName, LatchCounterEvidenceDenial> {
    FoundationalPerformanceCounterName::new(name).map_err(LatchCounterEvidenceDenial::Name)
}
