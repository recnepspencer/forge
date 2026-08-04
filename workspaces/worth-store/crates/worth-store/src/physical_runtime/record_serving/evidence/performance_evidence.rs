use super::super::{CompletedRecordScan, RecordReadObservation, RecordScanCounterSnapshot};
use worth_foundational::performance_api::lower_lane::{basis, receipts};
use worth_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
    FoundationalCounterBackedPerformanceReceiptConstructionDenial,
    FoundationalPerformanceAccessPatternPosture, FoundationalPerformanceBoundary,
    FoundationalPerformanceBreadthLocalityPosture, FoundationalPerformanceBundleConstructionDenial,
    FoundationalPerformanceClaimConstructionDenial, FoundationalPerformanceCounterRow,
    FoundationalPerformanceCounterSpec, FoundationalPerformanceEvidenceStrength,
    FoundationalPerformanceExecutionTemperature, FoundationalPerformanceFallbackDebtPosture,
    FoundationalPerformanceFreshnessRetentionPosture, FoundationalPerformanceWorkClass,
};

const COUNTER_NAMES: [&str; 20] = [
    "store.record.operation.append",
    "store.record.operation.locate",
    "store.record.operation.scan",
    "store.record.records",
    "store.record.payload.bytes",
    "store.record.manifest.blocks",
    "store.record.manifest.bytes",
    "store.record.manifest.comparisons",
    "store.record.allocation.segments",
    "store.record.allocation.extents",
    "store.record.identity.minted",
    "store.record.transfer.count",
    "store.record.transfer.peak_width_bytes",
    "store.record.copy.count",
    "store.record.copy.bytes",
    "store.record.scratch.peak_bytes",
    "store.record.barrier.file",
    "store.record.barrier.directory",
    "store.record.catalog.replacements",
    "store.record.frames.traversed",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordAccessSummary {
    values: [u64; COUNTER_NAMES.len()],
    breadth: FoundationalPerformanceBreadthLocalityPosture,
    pattern: FoundationalPerformanceAccessPatternPosture,
    work: FoundationalPerformanceWorkClass,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalRecordPerformanceContract {
    expected_values: [u64; COUNTER_NAMES.len()],
    breadth: FoundationalPerformanceBreadthLocalityPosture,
    pattern: FoundationalPerformanceAccessPatternPosture,
    work: FoundationalPerformanceWorkClass,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordManifestPerformanceExpectation {
    pub blocks: u64,
    pub bytes: u64,
    pub comparisons: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordTransferPerformanceExpectation {
    pub transfers: u64,
    pub peak_transfer_bytes: u64,
    pub explicit_copies: u64,
    pub copied_bytes: u64,
    pub peak_scratch_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordAppendPerformanceExpectation {
    pub records: u64,
    pub payload_bytes: u64,
    pub manifest: RecordManifestPerformanceExpectation,
    pub allocated_segments: u64,
    pub allocated_extents: u64,
    pub transfer: RecordTransferPerformanceExpectation,
    pub file_barriers: u64,
    pub directory_barriers: u64,
    pub catalog_replacements: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordLocatePerformanceExpectation {
    pub payload_bytes: u64,
    pub manifest: RecordManifestPerformanceExpectation,
    pub transfer: RecordTransferPerformanceExpectation,
    pub frames_traversed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordScanPerformanceExpectation {
    pub records: u64,
    pub payload_bytes: u64,
    pub manifest: RecordManifestPerformanceExpectation,
    pub transfer: RecordTransferPerformanceExpectation,
    pub frames_traversed: u64,
}

impl PhysicalRecordPerformanceContract {
    pub const fn append(expected: RecordAppendPerformanceExpectation) -> Self {
        Self {
            expected_values: append_contract_values(expected),
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::AppendHeavy,
            work: FoundationalPerformanceWorkClass::AuthoritativeMutation,
        }
    }

    pub const fn locate(expected: RecordLocatePerformanceExpectation) -> Self {
        Self {
            expected_values: locate_contract_values(expected),
            breadth: FoundationalPerformanceBreadthLocalityPosture::PointLocal,
            pattern: FoundationalPerformanceAccessPatternPosture::PointLookup,
            work: FoundationalPerformanceWorkClass::ValidationPlanning,
        }
    }

    pub const fn scan(expected: RecordScanPerformanceExpectation) -> Self {
        Self {
            expected_values: scan_contract_values(expected),
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::ScanHeavy,
            work: FoundationalPerformanceWorkClass::ValidationPlanning,
        }
    }
}

const fn append_contract_values(value: RecordAppendPerformanceExpectation) -> [u64; 20] {
    [
        1,
        0,
        0,
        value.records,
        value.payload_bytes,
        value.manifest.blocks,
        value.manifest.bytes,
        value.manifest.comparisons,
        value.allocated_segments,
        value.allocated_extents,
        value.records,
        value.transfer.transfers,
        value.transfer.peak_transfer_bytes,
        value.transfer.explicit_copies,
        value.transfer.copied_bytes,
        value.transfer.peak_scratch_bytes,
        value.file_barriers,
        value.directory_barriers,
        value.catalog_replacements,
        0,
    ]
}

const fn locate_contract_values(value: RecordLocatePerformanceExpectation) -> [u64; 20] {
    [
        0,
        1,
        0,
        1,
        value.payload_bytes,
        value.manifest.blocks,
        value.manifest.bytes,
        value.manifest.comparisons,
        0,
        0,
        0,
        value.transfer.transfers,
        value.transfer.peak_transfer_bytes,
        value.transfer.explicit_copies,
        value.transfer.copied_bytes,
        value.transfer.peak_scratch_bytes,
        0,
        0,
        0,
        value.frames_traversed,
    ]
}

const fn scan_contract_values(value: RecordScanPerformanceExpectation) -> [u64; 20] {
    [
        0,
        0,
        1,
        value.records,
        value.payload_bytes,
        value.manifest.blocks,
        value.manifest.bytes,
        value.manifest.comparisons,
        0,
        0,
        0,
        value.transfer.transfers,
        value.transfer.peak_transfer_bytes,
        value.transfer.explicit_copies,
        value.transfer.copied_bytes,
        value.transfer.peak_scratch_bytes,
        0,
        0,
        0,
        value.frames_traversed,
    ]
}

impl PhysicalRecordAccessSummary {
    pub fn from_completed_read(
        observation: RecordReadObservation,
    ) -> Result<Self, RecordPerformanceEvidenceDenial> {
        if observation.bytes_completed() != observation.bytes_requested() {
            return Err(RecordPerformanceEvidenceDenial::IncompleteOperation);
        }
        Ok(Self {
            values: read_values(observation),
            breadth: FoundationalPerformanceBreadthLocalityPosture::PointLocal,
            pattern: FoundationalPerformanceAccessPatternPosture::PointLookup,
            work: FoundationalPerformanceWorkClass::ValidationPlanning,
        })
    }

    pub fn from_completed_scan(scan: CompletedRecordScan) -> Self {
        Self {
            values: scan_values(scan.observation()),
            breadth: FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            pattern: FoundationalPerformanceAccessPatternPosture::ScanHeavy,
            work: FoundationalPerformanceWorkClass::ValidationPlanning,
        }
    }
}

pub type StoreRecordPerformanceReceipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPerformanceEvidenceDenial {
    IncompleteOperation,
    ContractOperationMismatch,
    Claim(FoundationalPerformanceClaimConstructionDenial),
    Bundle(FoundationalPerformanceBundleConstructionDenial),
    Receipt(FoundationalCounterBackedPerformanceReceiptConstructionDenial),
}

pub fn lower_record_operation_performance_receipt(
    contract: PhysicalRecordPerformanceContract,
    summary: PhysicalRecordAccessSummary,
) -> Result<StoreRecordPerformanceReceipt, RecordPerformanceEvidenceDenial> {
    if (contract.breadth, contract.pattern, contract.work)
        != (summary.breadth, summary.pattern, summary.work)
    {
        return Err(RecordPerformanceEvidenceDenial::ContractOperationMismatch);
    }
    let claim = worth_foundational::performance()
        .claim()
        .authoritative_execution()
        .boundary(FoundationalPerformanceBoundary::AuthoritativeExecution)
        .evidence_strength(FoundationalPerformanceEvidenceStrength::CounterBackedExecutionReceipt)
        .breadth_locality(contract.breadth)
        .access_pattern(contract.pattern)
        .execution_temperature(FoundationalPerformanceExecutionTemperature::HotPath)
        .freshness_retention(FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent)
        .fallback_debt(FoundationalPerformanceFallbackDebtPosture::Verified)
        .include_work(contract.work)
        .exclude_work(FoundationalPerformanceWorkClass::SupportReportAssembly)
        .finish()
        .map_err(RecordPerformanceEvidenceDenial::Claim)?;
    let mut bundle = basis::performance_bundle(claim);
    for (name, value) in COUNTER_NAMES.into_iter().zip(contract.expected_values) {
        bundle = bundle.attach_counter_spec(FoundationalPerformanceCounterSpec::new(
            counter_name(name),
            contract.work,
            value,
        ));
    }
    let bundle = bundle
        .finish()
        .map_err(RecordPerformanceEvidenceDenial::Bundle)?;
    let mut receipt = receipts::counter_backed_performance_receipt(bundle);
    for (name, value) in COUNTER_NAMES.into_iter().zip(summary.values) {
        receipt = receipt.attach_counter_row(FoundationalPerformanceCounterRow::new(
            counter_name(name),
            value,
        ));
    }
    receipt
        .finish()
        .map_err(RecordPerformanceEvidenceDenial::Receipt)
}

fn read_values(value: RecordReadObservation) -> [u64; COUNTER_NAMES.len()] {
    [
        0,
        1,
        0,
        1,
        value.payload_bytes(),
        value.manifest_blocks(),
        value.manifest_bytes(),
        value.manifest_comparisons(),
        0,
        0,
        0,
        value.transfer_count(),
        value.peak_transfer_width(),
        value.explicit_copy_count(),
        value.copied_bytes(),
        value.peak_scratch_bytes(),
        0,
        0,
        0,
        value
            .manifest_blocks()
            .saturating_add(value.touched_pages())
            .saturating_add(value.touched_extents()),
    ]
}

fn scan_values(value: RecordScanCounterSnapshot) -> [u64; COUNTER_NAMES.len()] {
    [
        0,
        0,
        1,
        value.records(),
        value.payload_bytes(),
        value.manifest_blocks(),
        value.manifest_bytes(),
        value.manifest_comparisons(),
        0,
        0,
        0,
        value.transfer_count(),
        value.peak_transfer_width(),
        value.explicit_copy_count(),
        value.copied_bytes(),
        value.peak_scratch_bytes(),
        0,
        0,
        0,
        value.frames_traversed(),
    ]
}

fn counter_name(name: &str) -> basis::FoundationalPerformanceCounterName {
    basis::FoundationalPerformanceCounterName::new(name)
        .expect("Store record counter names are static and valid")
}
