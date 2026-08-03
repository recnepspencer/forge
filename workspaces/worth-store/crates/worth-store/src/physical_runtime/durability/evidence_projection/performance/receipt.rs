use worth_foundational::performance_api::lower_lane::{basis, receipts};
use worth_foundational::{
    FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceClaimConstructionDenial, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};
use worth_store_aspect_native::StorePerformanceReceiptEvidence;

use super::{
    PhysicalDurabilityPerformanceClaim, PhysicalDurabilityPerformanceContract,
    PhysicalDurabilityPerformanceSummary,
};

pub type StorePhysicalDurabilityPerformanceReceiptEvidence =
    StorePerformanceReceiptEvidence<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalDurabilityPerformanceEvidenceDenial {
    ClaimMismatch,
    CounterMismatch,
    Claim(FoundationalPerformanceClaimConstructionDenial),
    Bundle(FoundationalPerformanceBundleConstructionDenial),
    Receipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

pub fn lower_physical_durability_performance_receipt(
    contract: PhysicalDurabilityPerformanceContract,
    summary: PhysicalDurabilityPerformanceSummary,
) -> Result<
    StorePhysicalDurabilityPerformanceReceiptEvidence,
    PhysicalDurabilityPerformanceEvidenceDenial,
> {
    let observed = summary.observed(contract.claim());
    if observed.claim() != contract.claim() {
        return Err(PhysicalDurabilityPerformanceEvidenceDenial::ClaimMismatch);
    }
    let expected_rows = contract.rows();
    let observed_rows = observed.rows();
    if expected_rows != observed_rows {
        return Err(PhysicalDurabilityPerformanceEvidenceDenial::CounterMismatch);
    }
    let posture = claim_posture(contract.claim());
    let claim = worth_foundational::performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(posture.breadth)
        .access_pattern(posture.pattern)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(posture.work)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .map_err(PhysicalDurabilityPerformanceEvidenceDenial::Claim)?;
    let mut bundle = basis::performance_bundle(claim);
    for (name, value) in &expected_rows {
        bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            counter_name(name),
            posture.work,
            *value,
        ));
    }
    let bundle = bundle
        .finish()
        .map_err(PhysicalDurabilityPerformanceEvidenceDenial::Bundle)?;
    let mut receipt = receipts::counter_backed_performance_receipt(bundle);
    for (name, value) in observed_rows {
        receipt = receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
            counter_name(name),
            value,
        ));
    }
    let receipt = receipt
        .finish()
        .map_err(PhysicalDurabilityPerformanceEvidenceDenial::Receipt)?;
    Ok(StorePerformanceReceiptEvidence::new(
        receipt,
        summary.physical_witness(),
    ))
}

struct ClaimPosture {
    breadth: FoundationalPerformanceBreadthLocalityPosture,
    pattern: FoundationalPerformanceAccessPatternPosture,
    work: FoundationalPerformanceWorkClass,
}

const fn claim_posture(claim: PhysicalDurabilityPerformanceClaim) -> ClaimPosture {
    match claim {
        PhysicalDurabilityPerformanceClaim::GroupCommitAmplification => ClaimPosture {
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::AppendHeavy,
            work: FoundationalPerformanceWorkClass::AuthoritativeMutation,
        },
        PhysicalDurabilityPerformanceClaim::CheckpointBoundedness => ClaimPosture {
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::ScanHeavy,
            work: FoundationalPerformanceWorkClass::PublicationDelivery,
        },
        PhysicalDurabilityPerformanceClaim::PageBasisBoundedness => ClaimPosture {
            breadth: FoundationalPerformanceBreadthLocalityPosture::PointLocal,
            pattern: FoundationalPerformanceAccessPatternPosture::AppendHeavy,
            work: FoundationalPerformanceWorkClass::AuthoritativeMutation,
        },
        PhysicalDurabilityPerformanceClaim::IdempotencyRetention => ClaimPosture {
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::PointLookup,
            work: FoundationalPerformanceWorkClass::AuthoritativeObservation,
        },
        PhysicalDurabilityPerformanceClaim::TerminalCloseout => ClaimPosture {
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::ScanHeavy,
            work: FoundationalPerformanceWorkClass::AuthoritativeObservation,
        },
    }
}

fn counter_name(name: &str) -> basis::FoundationalPerformanceCounterName {
    basis::FoundationalPerformanceCounterName::new(name)
        .expect("Store durability counter names are static and valid")
}
