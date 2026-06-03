use super::routing_digest;
use crate::facade::{
    CanonicalStreamReplayRecord, ConsumerCheckpointToken, StreamReplayAuditResult,
    StreamWindowDeliveryResult,
};
use crate::stream::BackpressureDecisionRecord;
use std::sync::Arc;

pub(super) struct StreamHarnessCertificationBundle {
    stream_digest: Arc<str>,
    window_digest: Arc<str>,
    checkpoint_digest: Arc<str>,
    consumer_contract_digest: Arc<str>,
    diagnostics_digest: Arc<str>,
    replay_digest: Arc<str>,
    routing_digest: Option<String>,
    failure_digest: Option<String>,
    pressure_report: BackpressureDecisionRecord,
    counters: crate::facade::StreamProtocolCounters,
}

impl StreamHarnessCertificationBundle {
    pub(super) fn routing(
        result: &StreamWindowDeliveryResult,
        checkpoint: &ConsumerCheckpointToken,
        replay_record: &CanonicalStreamReplayRecord,
        pressure_report: BackpressureDecisionRecord,
    ) -> Self {
        Self {
            stream_digest: Arc::from(result.summary().stream_digest()),
            window_digest: Arc::from(result.summary().window_digest()),
            checkpoint_digest: Arc::from(checkpoint.checkpoint_token_identity()),
            consumer_contract_digest: Arc::from(result.summary().consumer_contract_digest()),
            diagnostics_digest: Arc::from(result.summary().diagnostics_digest()),
            replay_digest: Arc::from(replay_record.digest()),
            routing_digest: Some(routing_digest(result)),
            failure_digest: None,
            pressure_report,
            counters: result.summary().counters().clone(),
        }
    }

    pub(super) fn replay_audit(
        result: &StreamReplayAuditResult,
        pressure_report: BackpressureDecisionRecord,
    ) -> Self {
        Self {
            stream_digest: Arc::from(result.summary().stream_digest()),
            window_digest: Arc::from(result.summary().window_digest()),
            checkpoint_digest: Arc::from(result.checkpoint().checkpoint_token_identity()),
            consumer_contract_digest: Arc::from(result.summary().consumer_contract_digest()),
            diagnostics_digest: Arc::from(result.summary().diagnostics_digest()),
            replay_digest: Arc::from(result.replay_record().digest()),
            routing_digest: None,
            failure_digest: None,
            pressure_report,
            counters: result.summary().counters().clone(),
        }
    }

    pub(super) fn stream_digest(&self) -> &str {
        self.stream_digest.as_ref()
    }

    pub(super) fn window_digest(&self) -> &str {
        self.window_digest.as_ref()
    }

    pub(super) fn checkpoint_digest(&self) -> &str {
        self.checkpoint_digest.as_ref()
    }

    pub(super) fn consumer_contract_digest(&self) -> &str {
        self.consumer_contract_digest.as_ref()
    }

    pub(super) fn diagnostics_digest(&self) -> &str {
        self.diagnostics_digest.as_ref()
    }

    pub(super) fn replay_digest(&self) -> &str {
        self.replay_digest.as_ref()
    }

    pub(super) fn routing_digest(&self) -> Option<&str> {
        self.routing_digest.as_deref()
    }

    pub(super) fn failure_digest(&self) -> Option<&str> {
        self.failure_digest.as_deref()
    }

    pub(super) fn pressure_report(&self) -> &BackpressureDecisionRecord {
        &self.pressure_report
    }

    pub(super) fn counters(&self) -> &crate::facade::StreamProtocolCounters {
        &self.counters
    }
}
