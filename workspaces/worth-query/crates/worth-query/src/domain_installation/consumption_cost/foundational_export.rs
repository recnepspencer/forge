use worth_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt, FoundationalPerformanceAccessPatternPosture,
    FoundationalPerformanceBoundary, FoundationalPerformanceBreadthLocalityPosture,
    FoundationalPerformanceContractName, FoundationalPerformanceCounterName,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};

use super::WorthQueryConsumptionCostSnapshot;

pub type WorthQueryFoundationalConsumptionCostReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryConsumptionCostExportDenialKind {
    Claim,
    ContractName,
    CounterName,
    Bundle,
    Receipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumptionCostExportDenial {
    kind: WorthQueryConsumptionCostExportDenialKind,
}

impl WorthQueryConsumptionCostExportDenial {
    pub const fn kind(&self) -> WorthQueryConsumptionCostExportDenialKind {
        self.kind
    }
}

impl WorthQueryConsumptionCostSnapshot {
    pub fn materialize_foundational_receipt(
        &self,
    ) -> Result<WorthQueryFoundationalConsumptionCostReceipt, WorthQueryConsumptionCostExportDenial>
    {
        let claim = foundational_claim(self)?;
        let contract = FoundationalPerformanceContractName::new("query.consumption.settled")
            .map_err(|_| denied(WorthQueryConsumptionCostExportDenialKind::ContractName))?;
        let mut bundle = performance_api::lower_lane::basis::performance_bundle(claim)
            .attach_contract_name(contract);
        for row in self.rows() {
            bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
                foundational_name(row.name())?,
                row.work_class(),
                row.observed_count(),
            ));
        }
        let bundle = bundle
            .finish()
            .map_err(|_| denied(WorthQueryConsumptionCostExportDenialKind::Bundle))?;
        let mut receipt =
            performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
        for row in self.rows() {
            receipt = receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
                foundational_name(row.name())?,
                row.observed_count(),
            ));
        }
        receipt
            .finish()
            .map_err(|_| denied(WorthQueryConsumptionCostExportDenialKind::Receipt))
    }
}

fn foundational_claim(
    snapshot: &WorthQueryConsumptionCostSnapshot,
) -> Result<FoundationalAuthoritativePerformanceClaim, WorthQueryConsumptionCostExportDenial> {
    let mut claim = performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::DeltaBound)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::TraversalLocal)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .exclude_work(FoundationalPerformanceWorkClass::ReplayReconstruction)
        .exclude_work(FoundationalPerformanceWorkClass::ForensicParity);
    let mut included = std::collections::BTreeSet::new();
    for row in snapshot.rows() {
        if included.insert(row.work_class()) {
            claim = claim.include_work(row.work_class());
        }
    }
    claim
        .finish()
        .map_err(|_| denied(WorthQueryConsumptionCostExportDenialKind::Claim))
}

fn foundational_name(
    name: &str,
) -> Result<FoundationalPerformanceCounterName, WorthQueryConsumptionCostExportDenial> {
    FoundationalPerformanceCounterName::new(name)
        .map_err(|_| denied(WorthQueryConsumptionCostExportDenialKind::CounterName))
}

const fn denied(
    kind: WorthQueryConsumptionCostExportDenialKind,
) -> WorthQueryConsumptionCostExportDenial {
    WorthQueryConsumptionCostExportDenial { kind }
}
