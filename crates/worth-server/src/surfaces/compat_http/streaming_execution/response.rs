use worth_proof::TransitionOutcome;

use crate::{
    WorthServerCompatibilityExecutionInput, WorthServerCompatibilityExecutionOutcome,
    WorthServerCompatibilityFacade, WorthServerCompatibilityRead,
    WorthServerOperatorEvidenceFacade, WorthServerQueryHandoffDenial,
    WorthServerQueryHandoffDenialCode,
};

use super::{
    cancellation::{WorthServerStreamCancellationKind, WorthServerStreamCancellationReceipt},
    chunk::WorthServerStreamingChunk,
    cursor::{estimate_payload_bytes, materialize_payload_bytes, WorthServerStreamCursor},
    export::{WorthServerBackgroundExportRequest, WorthServerCompatibilityExport},
    performance::{WorthServerStreamingMetricSnapshot, WorthServerStreamingPerformanceReceipt},
    selection::WorthServerStreamSelection,
};

#[derive(Debug)]
pub enum WorthServerStreamingResponse {
    Stream(WorthServerCompatibilityStream),
    Buffered(WorthServerCompatibilityExport),
    BackgroundExport(WorthServerBackgroundExportRequest),
}

#[derive(Debug)]
pub struct WorthServerCompatibilityStream {
    read: WorthServerCompatibilityRead,
    operator_evidence: WorthServerOperatorEvidenceFacade,
    selection: WorthServerStreamSelection,
    estimated_payload_bytes: Option<usize>,
    cursor: WorthServerStreamCursor,
    emitted_chunks: usize,
    emitted_bytes: usize,
    emitted_payload: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerStreamFinishError {
    StreamNotFullyConsumed,
}

impl WorthServerCompatibilityFacade {
    pub fn stream(
        &self,
        input: WorthServerCompatibilityExecutionInput,
        selection: WorthServerStreamSelection,
    ) -> WorthServerCompatibilityExecutionOutcome<WorthServerStreamingResponse> {
        if let Err(denial) = validate_streaming_request(input.prepared_request()) {
            return TransitionOutcome::Denied(denial);
        }
        let head_only = input.prepared_request().request_contract().method() == "HEAD";
        match self.read(input) {
            TransitionOutcome::Success(read) => {
                TransitionOutcome::Success(lower_streaming_response(
                    read,
                    self.operator_evidence.clone(),
                    selection,
                    head_only,
                ))
            }
            TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
            TransitionOutcome::Deferred(value) => TransitionOutcome::Deferred(value),
            TransitionOutcome::Stale(value) => TransitionOutcome::Stale(value),
            TransitionOutcome::RebindRequired(value) => TransitionOutcome::RebindRequired(value),
            TransitionOutcome::Failed(value) => TransitionOutcome::Failed(value),
        }
    }
}

impl WorthServerCompatibilityStream {
    pub fn read(&self) -> &WorthServerCompatibilityRead {
        &self.read
    }

    pub fn selection(&self) -> &WorthServerStreamSelection {
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

    pub fn next_chunk(&mut self) -> Result<Option<WorthServerStreamingChunk>, serde_json::Error> {
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
        Ok(Some(WorthServerStreamingChunk::new(
            self.emitted_chunks,
            bytes,
            terminal,
        )))
    }

    pub fn abort_due_to_disconnect(self) -> WorthServerStreamCancellationReceipt {
        self.cancellation_receipt(
            WorthServerStreamCancellationKind::ClientDisconnect,
            "client disconnected before stream transport completed",
        )
    }

    pub fn abort_due_to_backpressure(self) -> WorthServerStreamCancellationReceipt {
        self.cancellation_receipt(
            WorthServerStreamCancellationKind::DownstreamBackpressure,
            "downstream backpressure aborted compatibility streaming delivery",
        )
    }

    pub fn cancel_by_caller(self) -> WorthServerStreamCancellationReceipt {
        self.cancellation_receipt(
            WorthServerStreamCancellationKind::CallerCancelled,
            "caller cancelled compatibility streaming delivery",
        )
    }

    pub fn finish(self) -> Result<WorthServerCompatibilityExport, WorthServerStreamFinishError> {
        if !self.cursor.is_done() {
            return Err(WorthServerStreamFinishError::StreamNotFullyConsumed);
        }
        let payload_bytes = self.emitted_payload;
        let payload_len = payload_bytes.len();
        let performance_receipt =
            WorthServerStreamingPerformanceReceipt::build(WorthServerStreamingMetricSnapshot {
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
        let file_envelope = crate::surfaces::compat_http::project_binary_egress_envelope(
            &self.read,
            Some("application/json".to_string()),
            payload_len as u64,
            false,
            crate::WorthServerFileTransferDisposition::SelectedEgress,
        );
        let certification_bundle =
            crate::surfaces::compat_http::build_streaming_export_certification_bundle(
                &self.operator_evidence,
                self.read.support_posture(),
                &file_envelope,
                self.read.response_envelope(),
                &performance_receipt,
            );
        Ok(WorthServerCompatibilityExport::new(
            self.read,
            payload_bytes,
            self.estimated_payload_bytes.unwrap_or(payload_len),
            self.selection,
            performance_receipt,
            certification_bundle,
        ))
    }

    fn cancellation_receipt(
        self,
        kind: WorthServerStreamCancellationKind,
        detail: &'static str,
    ) -> WorthServerStreamCancellationReceipt {
        let performance_receipt =
            WorthServerStreamingPerformanceReceipt::build(WorthServerStreamingMetricSnapshot {
                chunks_emitted: self.emitted_chunks as u64,
                bytes_emitted: self.emitted_bytes as u64,
                full_buffer_materializations: 0,
                first_chunk_without_full_buffer: u64::from(self.emitted_chunks > 0),
                backpressure_events: u64::from(matches!(
                    kind,
                    WorthServerStreamCancellationKind::DownstreamBackpressure
                )),
                disconnects: u64::from(matches!(
                    kind,
                    WorthServerStreamCancellationKind::ClientDisconnect
                )),
                cancellations: u64::from(matches!(
                    kind,
                    WorthServerStreamCancellationKind::CallerCancelled
                )),
                background_export_fallbacks: 0,
            })
            .expect("stream cancellation counters should materialize");
        WorthServerStreamCancellationReceipt::new(
            super::WorthServerStreamCancellationReceiptParts {
                kind,
                chunks_emitted: self.emitted_chunks,
                bytes_emitted: self.emitted_bytes,
                canonical_result_completed: true,
                detail: detail.to_string(),
                tenant_id: self
                    .read
                    .direct_context()
                    .workspace_target()
                    .tenant_id()
                    .to_string(),
                workspace_digest: self.read.direct_context().workspace_digest().to_string(),
                branch_digest: self.read.direct_context().branch_digest().to_string(),
                transfer_provenance: self.read.file_envelope().transfer_provenance().clone(),
                performance_receipt,
            },
        )
    }
}

fn validate_streaming_request(
    prepared_request: &crate::WorthServerCompatibilityPreparedRequest,
) -> Result<(), WorthServerQueryHandoffDenial> {
    if prepared_request.request_contract().route_family()
        != crate::WorthServerCompatHttpRouteFamily::Streaming
    {
        return Err(WorthServerQueryHandoffDenial::new(
            WorthServerQueryHandoffDenialCode::CompatibilityStreamingRequestInvalid,
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
    read: WorthServerCompatibilityRead,
    operator_evidence: WorthServerOperatorEvidenceFacade,
    selection: WorthServerStreamSelection,
    head_only: bool,
) -> WorthServerStreamingResponse {
    if let Some(threshold) = selection.background_export_threshold_bytes() {
        let estimated_payload_bytes = estimate_payload_bytes(&read)
            .expect("compatibility read rows should serialize into streaming payload bytes");
        if estimated_payload_bytes > threshold {
            let performance_receipt =
                WorthServerStreamingPerformanceReceipt::build(WorthServerStreamingMetricSnapshot {
                    background_export_fallbacks: 1,
                    ..WorthServerStreamingMetricSnapshot::default()
                })
                .expect("background export counters should materialize");
            let file_envelope = crate::surfaces::compat_http::project_binary_egress_envelope(
                &read,
                Some("application/json".to_string()),
                0,
                false,
                crate::WorthServerFileTransferDisposition::MetadataOnlyObservation,
            );
            let certification_bundle =
                crate::surfaces::compat_http::build_background_export_certification_bundle(
                    &operator_evidence,
                    read.support_posture(),
                    &file_envelope,
                    read.response_envelope(),
                    &performance_receipt,
                );
            return WorthServerStreamingResponse::BackgroundExport(
                WorthServerBackgroundExportRequest::new(
                    read,
                    estimated_payload_bytes,
                    selection,
                    format!(
                        "estimated payload `{estimated_payload_bytes}` exceeded synchronous threshold `{threshold}`"
                    ),
                    performance_receipt,
                    certification_bundle,
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
            WorthServerStreamingPerformanceReceipt::build(WorthServerStreamingMetricSnapshot {
                chunks_emitted: u64::from(!head_only),
                bytes_emitted: payload_bytes.len() as u64,
                full_buffer_materializations: u64::from(!head_only),
                ..WorthServerStreamingMetricSnapshot::default()
            })
            .expect("buffered export counters should materialize");
        let file_envelope = crate::surfaces::compat_http::project_binary_egress_envelope(
            &read,
            Some("application/json".to_string()),
            payload_bytes.len() as u64,
            false,
            crate::WorthServerFileTransferDisposition::SelectedEgress,
        );
        let certification_bundle =
            crate::surfaces::compat_http::build_buffered_export_certification_bundle(
                &operator_evidence,
                read.support_posture(),
                &file_envelope,
                read.response_envelope(),
                &performance_receipt,
            );
        return WorthServerStreamingResponse::Buffered(WorthServerCompatibilityExport::new(
            read,
            payload_bytes,
            estimated_payload_bytes,
            selection,
            performance_receipt,
            certification_bundle,
        ));
    }
    let cursor = WorthServerStreamCursor::from_read(&read);
    WorthServerStreamingResponse::Stream(WorthServerCompatibilityStream {
        read,
        operator_evidence,
        selection,
        estimated_payload_bytes: None,
        cursor,
        emitted_chunks: 0,
        emitted_bytes: 0,
        emitted_payload: Vec::new(),
    })
}
