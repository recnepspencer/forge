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
    PageRecordCounterSnapshot, PageRecordDenial, PageRecordDenialKind, RecordLocateReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalPageRecordFramingEvidenceRow {
    SlotDirectoryLocateBounded,
    MovedSlotBoundedOrDenied,
    ReopenLocateStableFramedRecord,
    SlotLookupCountersExact,
}

impl PhysicalPageRecordFramingEvidenceRow {
    pub const fn s1_required() -> [Self; 4] {
        [
            Self::SlotDirectoryLocateBounded,
            Self::MovedSlotBoundedOrDenied,
            Self::ReopenLocateStableFramedRecord,
            Self::SlotLookupCountersExact,
        ]
    }

    pub const fn physical_substrate_lane(self) -> PhysicalSubstrateLane {
        match self {
            Self::SlotDirectoryLocateBounded | Self::MovedSlotBoundedOrDenied => {
                PhysicalSubstrateLane::HostileFormat
            }
            Self::ReopenLocateStableFramedRecord => PhysicalSubstrateLane::HappyAuthority,
            Self::SlotLookupCountersExact => PhysicalSubstrateLane::ScaleLocality,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalPageRecordFramingEvidenceReport {
    row: PhysicalPageRecordFramingEvidenceRow,
    lane: PhysicalSubstrateLane,
    counters: PageRecordCounterSnapshot,
    performance_receipt:
        FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
}

impl PhysicalPageRecordFramingEvidenceReport {
    pub fn from_locate_report(
        row: PhysicalPageRecordFramingEvidenceRow,
        report: RecordLocateReport<'_>,
    ) -> Result<Self, PhysicalPageRecordFramingEvidenceDenial> {
        Self::from_counters(row, report.counters())
    }

    pub fn from_counters(
        row: PhysicalPageRecordFramingEvidenceRow,
        counters: PageRecordCounterSnapshot,
    ) -> Result<Self, PhysicalPageRecordFramingEvidenceDenial> {
        require_exact_counter_snapshot(row, counters)?;
        let performance_receipt = counter_backed_receipt(counters)?;
        Ok(Self {
            row,
            lane: row.physical_substrate_lane(),
            counters,
            performance_receipt,
        })
    }

    pub fn from_page_record_denial(
        row: PhysicalPageRecordFramingEvidenceRow,
        denial: PageRecordDenial,
    ) -> Result<Self, PhysicalPageRecordFramingEvidenceDenial> {
        if row != PhysicalPageRecordFramingEvidenceRow::MovedSlotBoundedOrDenied
            || denial.kind() != PageRecordDenialKind::MovedSlotWithoutAdmittedReference
        {
            return Err(
                PhysicalPageRecordFramingEvidenceDenial::UnexpectedPageRecordDenial {
                    expected: PageRecordDenialKind::MovedSlotWithoutAdmittedReference,
                    actual: denial.kind(),
                },
            );
        }
        Self::from_counters(row, denial.counters())
    }

    pub const fn row(&self) -> PhysicalPageRecordFramingEvidenceRow {
        self.row
    }

    pub const fn lane(&self) -> PhysicalSubstrateLane {
        self.lane
    }

    pub const fn counters(&self) -> PageRecordCounterSnapshot {
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
pub enum PhysicalPageRecordFramingEvidenceDenial {
    MissingSlotLookupCounter,
    CounterExpectationMismatch {
        expected: PageRecordCounterSnapshot,
        actual: PageRecordCounterSnapshot,
    },
    UnexpectedPageRecordDenial {
        expected: PageRecordDenialKind,
        actual: PageRecordDenialKind,
    },
    PerformanceBundleDenied(FoundationalPerformanceBundleConstructionDenial),
    PerformanceReceiptDenied(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

fn require_exact_counter_snapshot(
    row: PhysicalPageRecordFramingEvidenceRow,
    counters: PageRecordCounterSnapshot,
) -> Result<(), PhysicalPageRecordFramingEvidenceDenial> {
    if counters.slot_lookup_count() != 1 {
        return Err(PhysicalPageRecordFramingEvidenceDenial::MissingSlotLookupCounter);
    }
    let expected = expected_counters(row);
    if counters != expected {
        return Err(
            PhysicalPageRecordFramingEvidenceDenial::CounterExpectationMismatch {
                expected,
                actual: counters,
            },
        );
    }
    Ok(())
}

fn expected_counters(row: PhysicalPageRecordFramingEvidenceRow) -> PageRecordCounterSnapshot {
    match row {
        PhysicalPageRecordFramingEvidenceRow::MovedSlotBoundedOrDenied => {
            PageRecordCounterSnapshot::for_locate_attempt().with_slot_lookup()
        }
        PhysicalPageRecordFramingEvidenceRow::SlotDirectoryLocateBounded
        | PhysicalPageRecordFramingEvidenceRow::ReopenLocateStableFramedRecord
        | PhysicalPageRecordFramingEvidenceRow::SlotLookupCountersExact => {
            PageRecordCounterSnapshot::for_locate_attempt()
                .with_slot_lookup()
                .with_frame_decode()
                .with_record_payload_view()
        }
    }
}

fn counter_backed_receipt(
    counters: PageRecordCounterSnapshot,
) -> Result<
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>,
    PhysicalPageRecordFramingEvidenceDenial,
> {
    let bundle = performance_api::lower_lane::basis::performance_bundle(authoritative_claim())
        .attach_contract_name(contract_name())
        .attach_counter_spec(counter_spec(
            "physical.page_read",
            counters.page_read_count(),
        ))
        .attach_counter_spec(counter_spec(
            "physical.page_write",
            counters.page_write_count(),
        ))
        .attach_counter_spec(counter_spec(
            "physical.frame_decode",
            counters.frame_decode_count(),
        ))
        .attach_counter_spec(counter_spec(
            "physical.record_locate",
            counters.record_locate_count(),
        ))
        .attach_counter_spec(counter_spec(
            "physical.slot_lookup",
            counters.slot_lookup_count(),
        ))
        .attach_counter_spec(counter_spec(
            "physical.page_local_scan",
            counters.page_local_scan_count(),
        ))
        .finish()
        .map_err(PhysicalPageRecordFramingEvidenceDenial::PerformanceBundleDenied)?;
    performance_api::lower_lane::receipts::counter_backed_performance_receipt(bundle)
        .attach_counter_row(counter_row(
            "physical.page_read",
            counters.page_read_count(),
        ))
        .attach_counter_row(counter_row(
            "physical.page_write",
            counters.page_write_count(),
        ))
        .attach_counter_row(counter_row(
            "physical.frame_decode",
            counters.frame_decode_count(),
        ))
        .attach_counter_row(counter_row(
            "physical.record_locate",
            counters.record_locate_count(),
        ))
        .attach_counter_row(counter_row(
            "physical.slot_lookup",
            counters.slot_lookup_count(),
        ))
        .attach_counter_row(counter_row(
            "physical.page_local_scan",
            counters.page_local_scan_count(),
        ))
        .finish()
        .map_err(PhysicalPageRecordFramingEvidenceDenial::PerformanceReceiptDenied)
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
        .expect("static phase 6 performance claim is valid")
}

fn contract_name() -> forge_foundational::FoundationalPerformanceContractName {
    forge_foundational::FoundationalPerformanceContractName::new("physical.page_slot_locate")
        .expect("static contract name is valid")
}

fn counter_spec(name: &'static str, expected: u32) -> FoundationalPerformanceCounterSpec {
    FoundationalPerformanceCounterSpec::new(
        counter_name(name),
        work_class_for(name),
        expected as u64,
    )
}

fn counter_row(name: &'static str, observed: u32) -> FoundationalPerformanceCounterRow {
    FoundationalPerformanceCounterRow::new(counter_name(name), observed as u64)
}

fn counter_name(name: &'static str) -> FoundationalPerformanceCounterName {
    FoundationalPerformanceCounterName::new(name).expect("static counter name is valid")
}

fn work_class_for(name: &str) -> FoundationalPerformanceWorkClass {
    if name == "physical.page_write" {
        FoundationalPerformanceWorkClass::AuthoritativeMutation
    } else {
        FoundationalPerformanceWorkClass::ValidationPlanning
    }
}
