use forge_proof::TransitionOutcome;

use crate::{
    ForgeServerCompatibilityExecutionInput, ForgeServerCompatibilityExecutionOutcome,
    ForgeServerCompatibilityFacade, ForgeServerCompatibilityRead, ForgeServerQueryHandoffDenial,
    ForgeServerQueryHandoffDenialCode,
};

use super::{
    cancellation::{ForgeServerStreamCancellationKind, ForgeServerStreamCancellationReceipt},
    chunk::ForgeServerStreamingChunk,
    cursor::{estimate_payload_bytes, materialize_payload_bytes, ForgeServerStreamCursor},
    export::{ForgeServerBackgroundExportRequest, ForgeServerCompatibilityExport},
    performance::{ForgeServerStreamingMetricSnapshot, ForgeServerStreamingPerformanceReceipt},
    selection::ForgeServerStreamSelection,
};

#[derive(Debug)]
pub enum ForgeServerStreamingResponse {
    Stream(ForgeServerCompatibilityStream),
    Buffered(ForgeServerCompatibilityExport),
    BackgroundExport(ForgeServerBackgroundExportRequest),
}

#[derive(Debug)]
pub struct ForgeServerCompatibilityStream {
    read: ForgeServerCompatibilityRead,
    selection: ForgeServerStreamSelection,
    estimated_payload_bytes: Option<usize>,
    cursor: ForgeServerStreamCursor,
    emitted_chunks: usize,
    emitted_bytes: usize,
    emitted_payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerStreamFinishError {
    StreamNotFullyConsumed,
}

impl ForgeServerCompatibilityFacade {
    pub fn stream(
        &self,
        input: ForgeServerCompatibilityExecutionInput,
        selection: ForgeServerStreamSelection,
    ) -> ForgeServerCompatibilityExecutionOutcome<ForgeServerStreamingResponse> {
        if let Err(denial) = validate_streaming_request(input.prepared_request()) {
            return TransitionOutcome::Denied(denial);
        }
        let head_only = input.prepared_request().request_contract().method() == "HEAD";
        match self.read(input) {
            TransitionOutcome::Success(read) => {
                TransitionOutcome::Success(lower_streaming_response(read, selection, head_only))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
        }
    }
}

impl ForgeServerCompatibilityStream {
    pub fn read(&self) -> &ForgeServerCompatibilityRead {
        &self.read
    }

    pub fn selection(&self) -> &ForgeServerStreamSelection {
        &self.selection
    }

    pub fn estimated_payload_bytes(&self) -> Option<usize> {
        self.estimated_payload_bytes
    }

    pub fn emitted_chunks(&self) -> usize {
        self.emitted_chunks
    }

    pub fn emitted_bytes(&self) -> usize {
        self.emitted_bytes
    }

    pub fn next_chunk(&mut self) -> Result<Option<ForgeServerStreamingChunk>, serde_json::Error> {
        let Some(bytes) = self
            .cursor
            .next_chunk(&self.read, self.selection.chunk_bytes())?
        else {
            return Ok(None);
        };
        self.emitted_chunks += 1;
        self.emitted_bytes += bytes.len();
        self.emitted_payload.extend_from_slice(&bytes);
        let terminal = self.cursor.is_done();
        Ok(Some(ForgeServerStreamingChunk::new(
            self.emitted_chunks,
            bytes,
            terminal,
        )))
    }

    pub fn abort_due_to_disconnect(self) -> ForgeServerStreamCancellationReceipt {
        self.cancellation_receipt(
            ForgeServerStreamCancellationKind::ClientDisconnect,
            "client disconnected before stream transport completed",
        )
    }

    pub fn abort_due_to_backpressure(self) -> ForgeServerStreamCancellationReceipt {
        self.cancellation_receipt(
            ForgeServerStreamCancellationKind::DownstreamBackpressure,
            "downstream backpressure aborted compatibility streaming delivery",
        )
    }

    pub fn cancel_by_caller(self) -> ForgeServerStreamCancellationReceipt {
        self.cancellation_receipt(
            ForgeServerStreamCancellationKind::CallerCancelled,
            "caller cancelled compatibility streaming delivery",
        )
    }

    pub fn finish(self) -> Result<ForgeServerCompatibilityExport, ForgeServerStreamFinishError> {
        if !self.cursor.is_done() {
            return Err(ForgeServerStreamFinishError::StreamNotFullyConsumed);
        }
        let payload_bytes = self.emitted_payload;
        let payload_len = payload_bytes.len();
        let performance_receipt =
            ForgeServerStreamingPerformanceReceipt::build(ForgeServerStreamingMetricSnapshot {
                chunks_emitted: self.emitted_chunks as u64,
                bytes_emitted: payload_len as u64,
                full_buffer_materializations: 0,
                first_chunk_without_full_buffer: u64::from(payload_len > 0),
                backpressure_events: 0,
                disconnects: 0,
                cancellations: 0,
                background_export_fallbacks: 0,
            })
            .expect("stream completion counters should materialize");
        Ok(ForgeServerCompatibilityExport::new(
            self.read,
            payload_bytes,
            self.estimated_payload_bytes.unwrap_or(payload_len),
            self.selection,
            performance_receipt,
        ))
    }

    fn cancellation_receipt(
        self,
        kind: ForgeServerStreamCancellationKind,
        detail: &'static str,
    ) -> ForgeServerStreamCancellationReceipt {
        let performance_receipt =
            ForgeServerStreamingPerformanceReceipt::build(ForgeServerStreamingMetricSnapshot {
                chunks_emitted: self.emitted_chunks as u64,
                bytes_emitted: self.emitted_bytes as u64,
                full_buffer_materializations: 0,
                first_chunk_without_full_buffer: u64::from(self.emitted_chunks > 0),
                backpressure_events: u64::from(matches!(
                    kind,
                    ForgeServerStreamCancellationKind::DownstreamBackpressure
                )),
                disconnects: u64::from(matches!(
                    kind,
                    ForgeServerStreamCancellationKind::ClientDisconnect
                )),
                cancellations: u64::from(matches!(
                    kind,
                    ForgeServerStreamCancellationKind::CallerCancelled
                )),
                background_export_fallbacks: 0,
            })
            .expect("stream cancellation counters should materialize");
        ForgeServerStreamCancellationReceipt::new(
            kind,
            self.emitted_chunks,
            self.emitted_bytes,
            true,
            detail,
            performance_receipt,
        )
    }
}

fn validate_streaming_request(
    prepared_request: &crate::ForgeServerCompatibilityPreparedRequest,
) -> Result<(), ForgeServerQueryHandoffDenial> {
    if prepared_request.request_contract().route_family()
        != crate::ForgeServerCompatHttpRouteFamily::Streaming
    {
        return Err(ForgeServerQueryHandoffDenial::new(
            ForgeServerQueryHandoffDenialCode::CompatibilityStreamingRequestInvalid,
            prepared_request
                .admission()
                .request_context()
                .diagnostics_profile(),
            "compatibility streaming execution requires the streaming route family",
        ));
    }
    Ok(())
}

fn lower_streaming_response(
    read: ForgeServerCompatibilityRead,
    selection: ForgeServerStreamSelection,
    head_only: bool,
) -> ForgeServerStreamingResponse {
    if let Some(threshold) = selection.background_export_threshold_bytes() {
        let estimated_payload_bytes = estimate_payload_bytes(&read)
            .expect("compatibility read rows should serialize into streaming payload bytes");
        if estimated_payload_bytes > threshold {
            let performance_receipt =
                ForgeServerStreamingPerformanceReceipt::build(ForgeServerStreamingMetricSnapshot {
                    background_export_fallbacks: 1,
                    ..ForgeServerStreamingMetricSnapshot::default()
                })
                .expect("background export counters should materialize");
            return ForgeServerStreamingResponse::BackgroundExport(
                ForgeServerBackgroundExportRequest::new(
                    read,
                    estimated_payload_bytes,
                    selection,
                    format!(
                        "estimated payload `{estimated_payload_bytes}` exceeded synchronous threshold `{threshold}`"
                    ),
                    performance_receipt,
                ),
            );
        }
    }
    if selection.is_buffered() || head_only {
        let estimated_payload_bytes = estimate_payload_bytes(&read)
            .expect("compatibility read rows should serialize into streaming payload bytes");
        let payload_bytes = if head_only {
            Vec::new()
        } else {
            materialize_payload_bytes(&read)
                .expect("compatibility read rows should serialize into a buffered payload")
        };
        let performance_receipt =
            ForgeServerStreamingPerformanceReceipt::build(ForgeServerStreamingMetricSnapshot {
                chunks_emitted: u64::from(!head_only),
                bytes_emitted: payload_bytes.len() as u64,
                full_buffer_materializations: u64::from(!head_only),
                ..ForgeServerStreamingMetricSnapshot::default()
            })
            .expect("buffered export counters should materialize");
        return ForgeServerStreamingResponse::Buffered(ForgeServerCompatibilityExport::new(
            read,
            payload_bytes,
            estimated_payload_bytes,
            selection,
            performance_receipt,
        ));
    }
    let cursor = ForgeServerStreamCursor::from_read(&read);
    ForgeServerStreamingResponse::Stream(ForgeServerCompatibilityStream {
        read,
        selection,
        estimated_payload_bytes: None,
        cursor,
        emitted_chunks: 0,
        emitted_bytes: 0,
        emitted_payload: Vec::new(),
    })
}
