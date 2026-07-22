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

pub struct PhysicalRecordScanSession<'runtime> {
    reader: PhysicalRecordReader<'runtime>,
    cursor: ManifestRangeCursor<'runtime>,
    pending: Option<CurrentPhysicalRecordPlacement>,
    batch_limit: usize,
    complete: bool,
    total: RecordScanCounterSnapshot,
    _lifecycle: super::super::lifecycle::record_lifecycle::RecordScanSessionLease,
    _allocation: worth_store_buffer_pool::OperationAllocationGrant,
}

impl<'runtime> PhysicalRecordReader<'runtime> {
    pub fn scan(
        self,
        request: RecordScanRequest,
    ) -> Result<PhysicalRecordScanSession<'runtime>, RecordScanError> {
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
        let manifest = ManifestReader::with_loader(
            self.media,
            self.frame_load,
            self.format,
            self.access,
            self.current_root,
        );
        let mut cursor = ManifestRangeCursor::new(manifest);
        let first = readmit_cursor(&self, request.cursor)?;
        let positioned = cursor
            .seek(self.current_root.routing_root(), first)
            .map_err(|_| {
                let error = manifest_error(&cursor, RecordScanDenial::ManifestUnavailable);
                self.health.observe_scan_denial(error.denial);
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
                self.health.observe_scan_denial(error.denial);
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

impl PhysicalRecordScanSession<'_> {
    pub fn read_next_into<'scratch>(
        &mut self,
        scratch: &'scratch mut [u8],
    ) -> Result<RecordScanOutcome<'scratch>, RecordScanError> {
        if self.complete {
            return Ok(RecordScanOutcome::Completed(CompletedRecordScan {
                observation: self.total,
            }));
        }
        let mut records = Vec::new();
        records
            .try_reserve_exact(self.batch_limit)
            .map_err(|_| RecordScanError {
                denial: RecordScanDenial::BatchMetadataUnavailable,
                observation: self.total,
            })?;
        let mut used = 0_usize;
        while records.len() < self.batch_limit {
            let placement = match self.take_next_placement()? {
                Some(placement) => placement,
                None => {
                    self.complete = true;
                    break;
                }
            };
            let payload_limit = u32::try_from(placement.payload_bytes())
                .ok()
                .and_then(|bytes| RecordByteLimit::new(bytes.max(1)))
                .ok_or(RecordScanError {
                    denial: RecordScanDenial::RecordRead(
                        super::super::RecordReadDenial::ArtifactDamaged,
                    ),
                    observation: self.total,
                })?;
            let payload_bytes =
                usize::try_from(placement.payload_bytes()).map_err(|_| RecordScanError {
                    denial: RecordScanDenial::RecordRead(
                        super::super::RecordReadDenial::ArtifactDamaged,
                    ),
                    observation: self.total,
                })?;
            let record = PhysicalRecordId::from_persisted(placement.record());
            if placement.payload_bytes() > u64::from(self.reader.access.scratch_limit().get()) {
                self.total.records = self.total.records.saturating_add(1);
                records.push(ScannedPhysicalRecord::new(
                    record,
                    None,
                    placement.payload_bytes(),
                ));
                continue;
            }
            if payload_bytes > scratch.len().saturating_sub(used) {
                self.pending = Some(placement);
                if records.is_empty() {
                    return Err(RecordScanError {
                        denial: RecordScanDenial::CallerScratchTooSmall {
                            required: placement.payload_bytes(),
                        },
                        observation: self.total,
                    });
                }
                break;
            }
            let mut session = match self.reader.open_known_placement(
                record,
                placement,
                RecordReadLimits::new(payload_limit),
                RecordReadObservation::default(),
            ) {
                Ok(session) => session,
                Err(error) => {
                    self.observe_record_read(error.observation());
                    return Err(RecordScanError {
                        denial: RecordScanDenial::RecordRead(error.denial()),
                        observation: self.total,
                    });
                }
            };
            let start = used;
            while used < start + payload_bytes {
                let count = match session.read_next(&mut scratch[used..start + payload_bytes]) {
                    Ok(count) => count,
                    Err(failure) => {
                        self.observe_record_read(session.observation());
                        return Err(RecordScanError {
                            denial: RecordScanDenial::RecordStream(failure.kind()),
                            observation: self.total,
                        });
                    }
                };
                if count == 0 {
                    self.observe_record_read(session.observation());
                    self.reader.health.observe_stream_failure(
                        super::super::RecordStreamFailureKind::ArtifactDamaged,
                    );
                    return Err(RecordScanError {
                        denial: RecordScanDenial::RecordStream(
                            super::super::RecordStreamFailureKind::ArtifactDamaged,
                        ),
                        observation: self.total,
                    });
                }
                used += count;
            }
            let read = session.observation();
            self.observe_record_read(read);
            self.total.records += 1;
            records.push(ScannedPhysicalRecord::new(
                record,
                Some(start..used),
                placement.payload_bytes(),
            ));
        }
        if records.len() == self.batch_limit {
            self.pending = self.take_next_placement()?;
            self.complete = self.pending.is_none();
        }
        if records.is_empty() {
            self.complete = true;
            return Ok(RecordScanOutcome::Completed(CompletedRecordScan {
                observation: self.total,
            }));
        }
        let end_cursor = cursor_for(
            &self.reader,
            self.reader.current_root,
            records.last().unwrap().record_id(),
        );
        Ok(RecordScanOutcome::Batch(RecordScanBatch::new(
            &scratch[..used],
            records,
            end_cursor,
            self.complete,
            self.total,
        )))
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
        let blocks = after.blocks_read().saturating_sub(before.blocks_read());
        self.total.manifest_blocks = self.total.manifest_blocks.saturating_add(blocks);
        self.total.manifest_bytes = self
            .total
            .manifest_bytes
            .saturating_add(after.bytes_read().saturating_sub(before.bytes_read()));
        self.total.manifest_comparisons = self
            .total
            .manifest_comparisons
            .saturating_add(after.comparisons().saturating_sub(before.comparisons()));
        self.total.frames = self.total.frames.saturating_add(blocks);
        next.map_err(|_| {
            let error = RecordScanError {
                denial: RecordScanDenial::ManifestUnavailable,
                observation: self.total,
            };
            self.reader.health.observe_scan_denial(error.denial);
            error
        })
    }

    fn observe_record_read(&mut self, observation: RecordReadObservation) {
        self.total.payload_bytes = self
            .total
            .payload_bytes
            .saturating_add(observation.bytes_completed());
        self.total.frames = self.total.frames.saturating_add(
            observation
                .manifest_blocks()
                .saturating_add(observation.touched_pages())
                .saturating_add(observation.touched_extents()),
        );
        self.total.manifest_blocks = self
            .total
            .manifest_blocks
            .saturating_add(observation.manifest_blocks());
        self.total.manifest_bytes = self
            .total
            .manifest_bytes
            .saturating_add(observation.manifest_bytes());
        self.total.manifest_comparisons = self
            .total
            .manifest_comparisons
            .saturating_add(observation.manifest_comparisons());
        self.total.transfer_count = self
            .total
            .transfer_count
            .saturating_add(observation.transfer_count());
        self.total.peak_transfer_width = self
            .total
            .peak_transfer_width
            .max(observation.peak_transfer_width());
        self.total.explicit_copy_count = self
            .total
            .explicit_copy_count
            .saturating_add(observation.explicit_copy_count());
        self.total.copied_bytes = self
            .total
            .copied_bytes
            .saturating_add(observation.copied_bytes());
        self.total.peak_scratch_bytes = self
            .total
            .peak_scratch_bytes
            .max(observation.peak_scratch_bytes());
    }
}
