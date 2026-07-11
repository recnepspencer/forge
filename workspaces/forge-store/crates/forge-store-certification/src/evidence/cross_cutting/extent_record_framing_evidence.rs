use crate::PhysicalSubstrateLane;
use forge_foundational::{
    performance, performance_api, FoundationalAuthoritativePerformanceClaim,
    FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceCounterName, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};
use forge_store_physical_format::{
    ExtentRecordCounterSnapshot, ExtentRecordDenial, ExtentRecordDenialKind,
    ExtentRecordLocateReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalExtentRecordFramingEvidenceRow {
    ExtentBackedLargeRecord,
    ExtentLengthMismatchDenied,
    MissingExtentMembershipDenied,
    MovedSlotMisuseDenied,
    ExtentLocalCountersExact,
}

impl PhysicalExtentRecordFramingEvidenceRow {
    pub const fn s1_required() -> [Self; 5] {
        [
            Self::ExtentBackedLargeRecord,
            Self::ExtentLengthMismatchDenied,
            Self::MissingExtentMembershipDenied,
            Self::MovedSlotMisuseDenied,
            Self::ExtentLocalCountersExact,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        match self {
            Self::ExtentBackedLargeRecord => PhysicalSubstrateLane::HappyAuthority,
            Self::ExtentLocalCountersExact => PhysicalSubstrateLane::ScaleLocality,
            Self::ExtentLengthMismatchDenied
            | Self::MissingExtentMembershipDenied
            | Self::MovedSlotMisuseDenied => PhysicalSubstrateLane::HostileFormat,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalExtentRecordFramingEvidenceReport {
    row: PhysicalExtentRecordFramingEvidenceRow,
    lane: PhysicalSubstrateLane,
    counters: ExtentRecordCounterSnapshot,
    performance_receipt:
        FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl PhysicalExtentRecordFramingEvidenceReport {
    pub fn from_locate_report(
        row: PhysicalExtentRecordFramingEvidenceRow,
        report: ExtentRecordLocateReport<'_>,
    ) -> Result<Self, PhysicalExtentRecordFramingEvidenceDenial> {
        if !matches!(
            row,
            PhysicalExtentRecordFramingEvidenceRow::ExtentBackedLargeRecord
                | PhysicalExtentRecordFramingEvidenceRow::ExtentLocalCountersExact
        ) {
            return Err(
                PhysicalExtentRecordFramingEvidenceDenial::UnexpectedEvidenceRowForReport(row),
            );
        }
        Self::from_counters(row, report.counters())
    }

    pub fn from_extent_denial(
        row: PhysicalExtentRecordFramingEvidenceRow,
        denial: ExtentRecordDenial,
    ) -> Result<Self, PhysicalExtentRecordFramingEvidenceDenial> {
        let expected = expected_denial_kind(row)?;
        if denial.kind() != expected {
            return Err(
                PhysicalExtentRecordFramingEvidenceDenial::UnexpectedExtentRecordDenial {
                    expected,
                    actual: denial.kind(),
                },
            );
        }
        Self::from_counters(row, denial.counters())
    }

    pub fn from_counters(
        row: PhysicalExtentRecordFramingEvidenceRow,
        counters: ExtentRecordCounterSnapshot,
    ) -> Result<Self, PhysicalExtentRecordFramingEvidenceDenial> {
        require_exact_counter_snapshot(row, counters)?;
        let performance_receipt = counter_backed_receipt(counters)?;
        Ok(Self {
            row,
            lane: row.physical_substrate_lane(),
            counters,
            performance_receipt,
        })
    }

    pub const fn row(&self) -> PhysicalExtentRecordFramingEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn counters(&self) -> ExtentRecordCounterSnapshot {
        self.counters
    }

    pub const fn performance_receipt(
        &self,
    ) -> &FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>
    {
        &self.performance_receipt
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalExtentRecordFramingEvidenceDenial {
    CounterExpectationMismatch {
        expected: ExtentRecordCounterSnapshot,
        actual: ExtentRecordCounterSnapshot,
    },
    UnexpectedEvidenceRowForReport(PhysicalExtentRecordFramingEvidenceRow),
    UnexpectedEvidenceRowForDenial(PhysicalExtentRecordFramingEvidenceRow),
    UnexpectedExtentRecordDenial {
        expected: ExtentRecordDenialKind,
        actual: ExtentRecordDenialKind,
    },
    PerformanceBundleDenied(FoundationalPerformanceBundleConstructionDenial),
    PerformanceReceiptDenied(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

fn expected_denial_kind(
    row: PhysicalExtentRecordFramingEvidenceRow,
) -> Result<ExtentRecordDenialKind, PhysicalExtentRecordFramingEvidenceDenial> {
    match row {
        PhysicalExtentRecordFramingEvidenceRow::ExtentLengthMismatchDenied => {
            Ok(ExtentRecordDenialKind::ExtentLengthMismatch)
        }
        PhysicalExtentRecordFramingEvidenceRow::MissingExtentMembershipDenied => {
            Ok(ExtentRecordDenialKind::MissingExtentMembership)
        }
        PhysicalExtentRecordFramingEvidenceRow::MovedSlotMisuseDenied => {
            Ok(ExtentRecordDenialKind::MovedSlotMisuse)
        }
        _ => Err(PhysicalExtentRecordFramingEvidenceDenial::UnexpectedEvidenceRowForDenial(row)),
    }
}

fn require_exact_counter_snapshot(
    row: PhysicalExtentRecordFramingEvidenceRow,
    counters: ExtentRecordCounterSnapshot,
) -> Result<(), PhysicalExtentRecordFramingEvidenceDenial> {
    let expected = expected_counters(row);
    if counters != expected {
        return Err(
            PhysicalExtentRecordFramingEvidenceDenial::CounterExpectationMismatch {
                expected,
                actual: counters,
            },
        );
    }
    Ok(())
}

fn expected_counters(row: PhysicalExtentRecordFramingEvidenceRow) -> ExtentRecordCounterSnapshot {
    match row {
        PhysicalExtentRecordFramingEvidenceRow::ExtentBackedLargeRecord
        | PhysicalExtentRecordFramingEvidenceRow::ExtentLocalCountersExact => {
            successful_locate_counters()
        }
        PhysicalExtentRecordFramingEvidenceRow::ExtentLengthMismatchDenied => {
            ExtentRecordCounterSnapshot::for_locate_attempt()
                .with_membership_check()
                .with_length_check()
        }
        PhysicalExtentRecordFramingEvidenceRow::MissingExtentMembershipDenied => {
            ExtentRecordCounterSnapshot::for_locate_attempt().with_membership_check()
        }
        PhysicalExtentRecordFramingEvidenceRow::MovedSlotMisuseDenied => {
            ExtentRecordCounterSnapshot::for_locate_attempt().with_moved_slot_misuse_rejection()
        }
    }
}

fn successful_locate_counters() -> ExtentRecordCounterSnapshot {
    ExtentRecordCounterSnapshot::for_locate_attempt()
        .with_membership_check()
        .with_length_check()
        .with_header_decode()
        .with_payload_view()
}

fn counter_backed_receipt(
    counters: ExtentRecordCounterSnapshot,
) -> Result<
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
    PhysicalExtentRecordFramingEvidenceDenial,
> {
    let mut bundle_builder =
        performance_api::lower_lane::basis::performance_bundle(authoritative_claim())
            .attach_contract_name(contract_name());
    for counter in extent_performance_counters(counters) {
        bundle_builder = bundle_builder.attach_counter_spec(counter.spec());
    }
    let bundle = bundle_builder
        .finish()
        .map_err(PhysicalExtentRecordFramingEvidenceDenial::PerformanceBundleDenied)?;

    let mut receipt_builder =
        performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle);
    for counter in extent_performance_counters(counters) {
        receipt_builder = receipt_builder.attach_counter_row(counter.row());
    }
    receipt_builder
        .finish()
        .map_err(PhysicalExtentRecordFramingEvidenceDenial::PerformanceReceiptDenied)
}

fn authoritative_claim() -> FoundationalAuthoritativePerformanceClaim {
    performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(FoundationalPerformanceBreadthLocalityPosture::PointLocal)
        .access_pattern(FoundationalPerformanceAccessPatternPosture::PointLookup)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(FoundationalPerformanceWorkClass::AuthoritativeMutation)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .expect("static phase 7 performance claim is valid")
}

fn contract_name() -> forge_foundational::FoundationalPerformanceContractName {
    forge_foundational::FoundationalPerformanceContractName::new("physical.extent_record_locate")
        .expect("static contract name is valid")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExtentPerformanceCounter {
    name: &'static str,
    count: u32,
    work_class: FoundationalPerformanceWorkClass,
}

impl ExtentPerformanceCounter {
    const fn validation(name: &'static str, count: u32) -> Self {
        Self {
            name,
            count,
            work_class: FoundationalPerformanceWorkClass::ValidationPlanning,
        }
    }

    const fn mutation(name: &'static str, count: u32) -> Self {
        Self {
            name,
            count,
            work_class: FoundationalPerformanceWorkClass::AuthoritativeMutation,
        }
    }

    fn spec(self) -> FoundationalPerformanceCounterSpec {
        FoundationalPerformanceCounterSpec::new(
            counter_name(self.name),
            self.work_class,
            self.count as u64,
        )
    }

    fn row(self) -> FoundationalPerformanceCounterRow {
        FoundationalPerformanceCounterRow::new(counter_name(self.name), self.count as u64)
    }
}

fn extent_performance_counters(
    counters: ExtentRecordCounterSnapshot,
) -> [ExtentPerformanceCounter; 8] {
    [
        ExtentPerformanceCounter::validation("physical.extent_read", counters.extent_read_count()),
        ExtentPerformanceCounter::mutation("physical.extent_write", counters.extent_write_count()),
        ExtentPerformanceCounter::validation(
            "physical.extent_header_decode",
            counters.extent_header_decode_count(),
        ),
        ExtentPerformanceCounter::validation(
            "physical.extent_membership_check",
            counters.extent_membership_check_count(),
        ),
        ExtentPerformanceCounter::validation(
            "physical.extent_length_check",
            counters.extent_length_check_count(),
        ),
        ExtentPerformanceCounter::validation(
            "physical.extent_locate",
            counters.extent_locate_count(),
        ),
        ExtentPerformanceCounter::validation(
            "physical.extent_payload_view",
            counters.extent_payload_view_count(),
        ),
        ExtentPerformanceCounter::validation(
            "physical.moved_slot_misuse_rejection",
            counters.moved_slot_misuse_rejection_count(),
        ),
    ]
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).expect("static counter name is valid")
}
