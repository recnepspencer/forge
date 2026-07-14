use crate::{
    PhysicalHostileScaleFixtureReport, PhysicalScalePropertyEvidence, PhysicalSubstrateLane,
};
use worth_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceContractName, FoundationalPerformanceCounterName,
    FoundationalPerformanceCounterRow, FoundationalPerformanceCounterSpec,
    FoundationalPerformanceEvidenceStrength, FoundationalPerformanceExecutionTemperature,
    FoundationalPerformanceFallbackDebtPosture, FoundationalPerformanceFreshnessRetentionPosture,
    FoundationalPerformanceWorkClass,
};
use worth_store_physical_format::{
    PhysicalAlgorithmReviewEvidence, PhysicalComplexityStatus, PhysicalLocalityClass,
    PhysicalOperationComplexityContract, PhysicalOperationCounterSnapshot, PhysicalOperationKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalComplexityProofBundle {
    counters: PhysicalOperationCounterSnapshot,
    algorithm_review: PhysicalAlgorithmReviewEvidence,
    hostile_fixture: PhysicalHostileScaleFixtureReport,
    scale_property: PhysicalScalePropertyEvidence,
}

impl PhysicalComplexityProofBundle {
    pub const fn new(
        counters: PhysicalOperationCounterSnapshot,
        algorithm_review: PhysicalAlgorithmReviewEvidence,
        hostile_fixture: PhysicalHostileScaleFixtureReport,
        scale_property: PhysicalScalePropertyEvidence,
    ) -> Self {
        Self {
            counters,
            algorithm_review,
            hostile_fixture,
            scale_property,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalComplexityEvidenceReport {
    contract: PhysicalOperationComplexityContract,
    lane: PhysicalSubstrateLane,
    counters: PhysicalOperationCounterSnapshot,
    performance_receipt:
        FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl PhysicalComplexityEvidenceReport {
    pub fn verify(
        contract: PhysicalOperationComplexityContract,
        proof: PhysicalComplexityProofBundle,
    ) -> Result<Self, PhysicalComplexityEvidenceDenial> {
        require_verified_contract(contract)?;
        require_operation_match(contract.operation(), proof.counters.operation())?;
        require_operation_match(contract.operation(), proof.algorithm_review.operation())?;
        require_operation_match(contract.operation(), proof.hostile_fixture.operation())?;
        require_operation_match(contract.operation(), proof.scale_property.operation())?;
        if proof.algorithm_review.locality() != contract.locality() {
            return Err(PhysicalComplexityEvidenceDenial::LocalityMismatch {
                expected: contract.locality(),
                actual: proof.algorithm_review.locality(),
            });
        }
        if !proof.hostile_fixture.proves_unrelated_growth() {
            return Err(PhysicalComplexityEvidenceDenial::MissingHostileFixture);
        }
        if proof.counters != *proof.hostile_fixture.baseline_counters() {
            return Err(PhysicalComplexityEvidenceDenial::FixtureCounterMismatch);
        }
        if proof.scale_property.fixture() != &proof.hostile_fixture {
            return Err(PhysicalComplexityEvidenceDenial::DetachedScaleProperty);
        }
        if !proof.scale_property.is_satisfied() {
            return Err(PhysicalComplexityEvidenceDenial::ScalePropertyNotProven);
        }
        let performance_receipt = counter_backed_receipt(contract, &proof.counters)?;
        Ok(Self {
            contract,
            lane: PhysicalSubstrateLane::ScaleLocality,
            counters: proof.counters,
            performance_receipt,
        })
    }

    pub const fn contract(&self) -> PhysicalOperationComplexityContract {
        self.contract
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn counters(&self) -> &PhysicalOperationCounterSnapshot {
        &self.counters
    }

    pub const fn performance_receipt(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.performance_receipt
    }

    pub fn is_platform_grade_verified(&self) -> bool {
        self.contract.status() == PhysicalComplexityStatus::Declared
            && self.status() == PhysicalComplexityStatus::Verified
    }

    pub const fn status(&self) -> PhysicalComplexityStatus {
        PhysicalComplexityStatus::Verified
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalComplexityEvidenceDenial {
    DebtContractRejected(PhysicalOperationKind),
    OperationMismatch {
        expected: PhysicalOperationKind,
        actual: PhysicalOperationKind,
    },
    LocalityMismatch {
        expected: PhysicalLocalityClass,
        actual: PhysicalLocalityClass,
    },
    MissingHostileFixture,
    FixtureCounterMismatch,
    DetachedScaleProperty,
    ScalePropertyNotProven,
    PerformanceBundleDenied(FoundationalPerformanceBundleConstructionDenial),
    PerformanceReceiptDenied(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

fn require_verified_contract(
    contract: PhysicalOperationComplexityContract,
) -> Result<(), PhysicalComplexityEvidenceDenial> {
    if contract.status() == PhysicalComplexityStatus::Debt {
        return Err(PhysicalComplexityEvidenceDenial::DebtContractRejected(
            contract.operation(),
        ));
    }
    Ok(())
}

fn require_operation_match(
    expected: PhysicalOperationKind,
    actual: PhysicalOperationKind,
) -> Result<(), PhysicalComplexityEvidenceDenial> {
    if expected != actual {
        return Err(PhysicalComplexityEvidenceDenial::OperationMismatch { expected, actual });
    }
    Ok(())
}

fn counter_backed_receipt(
    contract: PhysicalOperationComplexityContract,
    counters: &PhysicalOperationCounterSnapshot,
) -> Result<
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
    PhysicalComplexityEvidenceDenial,
> {
    let mut bundle_builder =
        performance_api::lower_lane::basis::performance_bundle(authoritative_claim(contract))
            .attach_contract_name(contract_name(contract.operation()));
    for counter in counters.rows() {
        bundle_builder =
            bundle_builder.attach_counter_spec(counter_spec(counter.name(), counter.count()));
    }
    let bundle = bundle_builder
        .finish()
        .map_err(PhysicalComplexityEvidenceDenial::PerformanceBundleDenied)?;
    let mut receipt_builder =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for counter in counters.rows() {
        receipt_builder =
            receipt_builder.attach_counter_row(counter_row(counter.name(), counter.count()));
    }
    receipt_builder
        .finish()
        .map_err(PhysicalComplexityEvidenceDenial::PerformanceReceiptDenied)
}

fn authoritative_claim(
    contract: PhysicalOperationComplexityContract,
) -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(breadth_locality(contract.locality()))
        .access_pattern(access_pattern(contract.operation()))
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::ValidationPlanning)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("static phase 12 performance claim is valid")
}

fn breadth_locality(
    locality: PhysicalLocalityClass,
) -> FoundationalPerformanceBreadthLocalityPosture {
    match locality {
        PhysicalLocalityClass::Constant
        | PhysicalLocalityClass::PageLocal
        | PhysicalLocalityClass::SegmentLocal
        | PhysicalLocalityClass::ExtentLocal
        | PhysicalLocalityClass::FreeSpaceClass => {
            FoundationalPerformanceBreadthLocalityPosture::PointLocal
        }
        PhysicalLocalityClass::RootManifest | PhysicalLocalityClass::ManifestDeclaredTraversal => {
            FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch
        }
        PhysicalLocalityClass::FullScan => {
            FoundationalPerformanceBreadthLocalityPosture::SnapshotBound
        }
    }
}

fn access_pattern(operation: PhysicalOperationKind) -> FoundationalPerformanceAccessPatternPosture {
    match operation {
        PhysicalOperationKind::LocateByReference
        | PhysicalOperationKind::ManifestLookup
        | PhysicalOperationKind::PhysicalReferenceValidation
        | PhysicalOperationKind::HeaderDecode => {
            FoundationalPerformanceAccessPatternPosture::PointLookup
        }
        PhysicalOperationKind::AppendRecordPlacement => {
            FoundationalPerformanceAccessPatternPosture::AppendHeavy
        }
        _ => FoundationalPerformanceAccessPatternPosture::TraversalLocal,
    }
}

fn contract_name(operation: PhysicalOperationKind) -> FoundationalPerformanceContractName {
    FoundationalPerformanceContractName::new(operation.contract_name())
        .expect("static contract name is valid")
}

fn counter_spec(name: &'static str, expected: u64) -> FoundationalPerformanceCounterSpec {
    FoundationalPerformanceCounterSpec::new(
        counter_name(name),
        FoundationalPerformanceWorkClass::ValidationPlanning,
        expected,
    )
}

fn counter_row(name: &'static str, observed: u64) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(counter_name(name), observed)
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).expect("static counter name is valid")
}
