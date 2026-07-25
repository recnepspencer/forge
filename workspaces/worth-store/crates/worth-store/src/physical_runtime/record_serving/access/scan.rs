use worth_store_physical_format::CurrentPhysicalRecordPlacement;

use super::super::{
    access::manifest_routing::{ManifestRangeCursor, ManifestReader},
    access::scan_observation::{manifest_error, manifest_snapshot, scan_error},
    access::scan_readmission::{cursor_for, readmit_cursor, ExternalRecordScanCursor},
    CompletedRecordScan, PhysicalRecordId, PhysicalRecordReader, RecordByteLimit, RecordCountLimit,
    RecordReadLimits, RecordReadObservation, RecordScanCounterSnapshot, RecordScanError,
};

#[path = "scan/batch.rs"]
mod batch;
#[path = "scan/batch_collection.rs"]
mod batch_collection;
pub use batch::{RecordScanBatch, RecordScanOutcome, ScannedPhysicalRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordScanRequest {
    cursor: Option<ExternalRecordScanCursor>,
    batch_limit: Option<RecordCountLimit>,
}

impl RecordScanRequest {
    pub const fn from_start() -> Self {
        Self {
            cursor: None,
            batch_limit: None,
        }
    }
    pub const fn resume(cursor: ExternalRecordScanCursor) -> Self {
        Self {
            cursor: Some(cursor),
            batch_limit: None,
        }
    }
    pub const fn with_batch_limit(mut self, limit: RecordCountLimit) -> Self {
        self.batch_limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordScanDenial {
    ServingRequiresInspection,
    BatchLimitExceeded,
    BatchMetadataUnavailable,
    ForeignStore,
    StaleRoot,
    FormatMismatch,
    RoutingTreeMismatch,
    CursorPositionNotFound,
    ManifestUnavailable,
    CallerScratchTooSmall { required: u64 },
    RecordRead(super::super::RecordReadDenial),
    RecordStream(super::super::RecordStreamFailureKind),
}

pub struct PhysicalRecordScanSession {
    reader: PhysicalRecordReader,
    cursor: ManifestRangeCursor<'static>,
    pending: Option<CurrentPhysicalRecordPlacement>,
    batch_limit: usize,
    complete: bool,
    total: RecordScanCounterSnapshot,
    _lifecycle: super::super::lifecycle::record_lifecycle::RecordScanSessionLease,
    _allocation: worth_store_buffer_pool::OperationAllocationGrant,
}

impl PhysicalRecordReader {
    pub fn scan(
        mut self,
        request: RecordScanRequest,
    ) -> Result<PhysicalRecordScanSession, RecordScanError> {
        let runtime = self
            .runtime
            .upgrade()
            .ok_or_else(|| scan_error(RecordScanDenial::ServingRequiresInspection))?;
        runtime
            .health
            .permit()
            .map_err(|_| scan_error(RecordScanDenial::ServingRequiresInspection))?;
        let requested = request
            .batch_limit
            .unwrap_or(self.access.scan_limit())
            .get();
        if requested > self.access.scan_limit().get() {
            return Err(scan_error(RecordScanDenial::BatchLimitExceeded));
        }
        let operation_bytes = u64::from(self.format.declaration().page_size().bytes())
            .saturating_add(
                u64::from(requested)
                    .saturating_mul(std::mem::size_of::<ScannedPhysicalRecord>() as u64),
            );
        let allocation = self
            .frame_ports
            .begin_operation(
                worth_store_buffer_pool::OperationAllocationScope::ForegroundRead,
                operation_bytes,
            )
            .map_err(|reason| {
                scan_error(RecordScanDenial::RecordRead(
                    super::super::RecordReadDenial::ResidencyUnavailable(reason),
                ))
            })?;
        self.source = self.source.for_scan();
        let manifest = ManifestReader::serving(
            self.frame_ports.clone(),
            self.source.clone(),
            self.format,
            self.access,
            self.current_root.clone(),
        );
        let mut cursor = ManifestRangeCursor::new(manifest);
        let first = readmit_cursor(&self, request.cursor)?;
        let positioned = cursor
            .seek(self.current_root.routing_root(), first)
            .map_err(|_| {
                let error = manifest_error(&cursor, RecordScanDenial::ManifestUnavailable);
                runtime.health.observe_scan_denial(error.denial);
                error
            })?;
        if first.is_some() && !positioned {
            return Err(manifest_error(
                &cursor,
                RecordScanDenial::CursorPositionNotFound,
            ));
        }
        if let Some(expected) = first {
            let found = cursor.next().map_err(|_| {
                let error = manifest_error(&cursor, RecordScanDenial::ManifestUnavailable);
                runtime.health.observe_scan_denial(error.denial);
                error
            })?;
            if found.map(|placement| placement.record()) != Some(expected) {
                return Err(manifest_error(
                    &cursor,
                    RecordScanDenial::CursorPositionNotFound,
                ));
            }
        }
        let total = manifest_snapshot(cursor.counters());
        let lifecycle = self.lifecycle.scan_session();
        Ok(PhysicalRecordScanSession {
            reader: self,
            cursor,
            pending: None,
            batch_limit: requested as usize,
            complete: !positioned && first.is_none(),
            total,
            _lifecycle: lifecycle,
            _allocation: allocation,
        })
    }
}

impl PhysicalRecordScanSession {
    pub fn read_next_into<'scratch>(
        &mut self,
        scratch: &'scratch mut [u8],
    ) -> Result<RecordScanOutcome<'scratch>, RecordScanError> {
        let runtime = self.reader.runtime.upgrade().ok_or(RecordScanError {
            denial: RecordScanDenial::ServingRequiresInspection,
            observation: self.total,
        })?;
        runtime.health.permit().map_err(|_| RecordScanError {
            denial: RecordScanDenial::ServingRequiresInspection,
            observation: self.total,
        })?;
        if self.complete {
            return Ok(RecordScanOutcome::Completed(CompletedRecordScan {
                observation: self.total,
            }));
        }
        self.collect_next_batch(scratch, &runtime)
    }

    fn take_next_placement(
        &mut self,
    ) -> Result<Option<CurrentPhysicalRecordPlacement>, RecordScanError> {
        if self.pending.is_some() {
            return Ok(self.pending.take());
        }
        let before = self.cursor.counters();
        let next = self.cursor.next();
        let after = self.cursor.counters();
        self.total.observe_manifest_delta(before, after);
        next.map_err(|_| {
            let error = RecordScanError {
                denial: RecordScanDenial::ManifestUnavailable,
                observation: self.total,
            };
            if let Some(runtime) = self.reader.runtime.upgrade() {
                runtime.health.observe_scan_denial(error.denial);
            }
            error
        })
    }

    fn observe_record_read(&mut self, observation: RecordReadObservation) {
        self.total.observe_record_read(observation);
    }
}
