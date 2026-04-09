//! Subscriber that applies audit-level filtering to finalization output.

use crate::engine::facade::AuditLevel;
use crate::engine::transaction::data::feature_event::KernelFeatureEvent;
use crate::engine::transaction::data::operation_outputs::{
    FinalizationOutput, OperationEnvelopeOutput,
};
use crate::engine::transaction::data::subscriber_data_id::KernelSubscriberDataId;
use crate::engine::transaction::logic::feature_event_runtime::FeatureEventRuntimeContext;
use forge_core::KernelError;
use forge_signal::facade::adapters::{EventSubscriber, SubscriberContext, SubscriberId};
use forge_signal::facade::runtime::CheckpointBarrier;
use forge_signal::facade::SignalError;

use super::{kernel_to_signal, stage_output_value};

pub(crate) struct AuditSubscriber {
    feature_kind: Option<&'static str>,
    audit_level: AuditLevel,
}

impl AuditSubscriber {
    pub(crate) fn new() -> Self {
        Self {
            feature_kind: None,
            audit_level: AuditLevel::None,
        }
    }
}

impl EventSubscriber for AuditSubscriber {
    type Event = KernelFeatureEvent;
    type DataId = KernelSubscriberDataId;
    type RuntimeContext = FeatureEventRuntimeContext;

    fn id(&self) -> SubscriberId {
        SubscriberId::new(30)
    }

    fn name(&self) -> &'static str {
        "AuditSubscriber"
    }

    fn requires(&self) -> &'static [Self::DataId] {
        &[KernelSubscriberDataId::Finalization]
    }

    fn provides(&self) -> &'static [Self::DataId] {
        &[KernelSubscriberDataId::OperationEnvelope]
    }

    fn on_begin(
        &mut self,
        _ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) {
        self.feature_kind = None;
        self.audit_level = AuditLevel::None;
    }

    fn on_event(&mut self, event: &Self::Event) {
        if let KernelFeatureEvent::OperationStarted {
            feature_kind,
            audit_level,
            ..
        } = event
        {
            self.feature_kind = Some(*feature_kind);
            self.audit_level = *audit_level;
        }
    }

    fn on_checkpoint(
        &mut self,
        barrier: CheckpointBarrier,
        ctx: &mut SubscriberContext<Self::DataId>,
        _runtime: &mut Self::RuntimeContext,
    ) -> Result<(), SignalError> {
        if barrier != CheckpointBarrier::PerOperation {
            return Ok(());
        }

        let finalized = ctx
            .staged::<FinalizationOutput>(KernelSubscriberDataId::Finalization)
            .ok_or_else(|| {
                kernel_to_signal(KernelError::InternalError {
                    message: "Finalization output missing in AuditSubscriber".to_string(),
                    context: None,
                })
            })?;

        let feature_kind = self.feature_kind.ok_or_else(|| {
            kernel_to_signal(KernelError::InternalError {
                message: "AuditSubscriber missing feature_kind from OperationStarted".to_string(),
                context: None,
            })
        })?;

        let mut decision_log = finalized.decision_log.clone();
        let mut extra_summaries = Vec::new();

        match self.audit_level {
            AuditLevel::None => {
                decision_log = forge_core::DecisionLog::new();
            }
            AuditLevel::Summary => {
                let summary = format!(
                    "summary: {} decisions, {} warnings | trace={:016x}, adjuncts={}",
                    decision_log.len(),
                    finalized.warnings.len(),
                    finalized.trace_fingerprint.trace_hash,
                    finalized.adjunct_count,
                );
                let span_name = format!("audit/{}", feature_kind);
                let span_id = decision_log.start_span(&span_name);
                decision_log.end_span(span_id, 0);
                extra_summaries.push(summary);
            }
            AuditLevel::Full => {
                let decision_details: Vec<String> = decision_log
                    .decisions()
                    .map(|d| {
                        format!(
                            "{:?}/tier={:?}/margin={:.2e}",
                            d.get_kind(),
                            d.get_tier(),
                            d.get_margin(),
                        )
                    })
                    .collect();
                let detail = format!(
                    "full: {} decisions, {} warnings | trace={:016x}, adjuncts={} | [{}]",
                    decision_log.len(),
                    finalized.warnings.len(),
                    finalized.trace_fingerprint.trace_hash,
                    finalized.adjunct_count,
                    decision_details.join(", "),
                );
                let span_name = format!("audit/{}", feature_kind);
                let span_id = decision_log.start_span(&span_name);
                decision_log.end_span(span_id, 0);
                extra_summaries.push(detail);
            }
        }

        let output = OperationEnvelopeOutput {
            decision_log,
            warnings: finalized.warnings.clone(),
            metrics: finalized.metrics.clone(),
            lineage_delta: finalized.lineage_delta.clone(),
            accumulated_error_budget: finalized.accumulated_error_budget,
            state_hash_before: finalized.state_hash_before,
            state_hash_after: finalized.state_hash_after,
            extra_summaries,
        };

        stage_output_value(
            ctx,
            KernelSubscriberDataId::OperationEnvelope,
            output,
            "operation_envelope",
        )
    }
}
