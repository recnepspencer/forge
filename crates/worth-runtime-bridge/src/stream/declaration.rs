use std::sync::Arc;

use crate::identity::{BridgeIdentity, ChangeStreamDeclarationIdentityTag};
use crate::routing::canonicalization::digest_string;

pub type ChangeStreamDeclarationIdentity = BridgeIdentity<ChangeStreamDeclarationIdentityTag>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamConsumerShape {
    RoutingConsumer,
    ReplayAuditConsumer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamResumeMode {
    FromCheckpointOnly,
    FromStreamPosition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamCheckpointPublicationMode {
    PublishEveryWindow,
    PublishOnDemand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamCoalescingFamily {
    None,
    RoutingWindowCoalescing,
    ReplayAuditWindowCoalescing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamCoalescingIntent {
    None,
    Prefer(StreamCoalescingFamily),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamReplayMode {
    Disabled,
    Enabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamDeliveryIntent {
    RouteInvalidations,
    ReplayAudit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StreamDiagnosticsPolicyClass {
    Minimal,
    Standard,
    Exhaustive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeStreamDeclaration {
    declaration_identity: ChangeStreamDeclarationIdentity,
    consumer_shape: StreamConsumerShape,
    resume_mode: StreamResumeMode,
    checkpoint_publication_mode: StreamCheckpointPublicationMode,
    coalescing_intent: StreamCoalescingIntent,
    replay_mode: StreamReplayMode,
    delivery_intent: StreamDeliveryIntent,
    diagnostics_policy_class: StreamDiagnosticsPolicyClass,
    protocol_semantics_version: Arc<str>,
    digest: Arc<str>,
}

impl ChangeStreamDeclaration {
    pub fn new(
        consumer_shape: StreamConsumerShape,
        resume_mode: StreamResumeMode,
        checkpoint_publication_mode: StreamCheckpointPublicationMode,
        coalescing_intent: StreamCoalescingIntent,
        replay_mode: StreamReplayMode,
        delivery_intent: StreamDeliveryIntent,
        diagnostics_policy_class: StreamDiagnosticsPolicyClass,
    ) -> Self {
        let protocol_semantics_version: Arc<str> = Arc::from("worth-runtime-bridge.stream.v1");
        let basis = format!(
            "change-stream-declaration|consumer-shape={}|resume-mode={}|checkpoint-mode={}|coalescing-intent={}|replay-mode={}|delivery-intent={}|protocol-semantics-version={}",
            consumer_shape_label(consumer_shape),
            resume_mode_label(resume_mode),
            checkpoint_publication_mode_label(checkpoint_publication_mode),
            coalescing_intent_label(coalescing_intent),
            replay_mode_label(replay_mode),
            delivery_intent_label(delivery_intent),
            protocol_semantics_version.as_ref(),
        );
        let digest = digest_string("change-stream-declaration", &basis);
        Self {
            declaration_identity: ChangeStreamDeclarationIdentity::admit_bridge_owned(
                digest.clone(),
            ),
            consumer_shape,
            resume_mode,
            checkpoint_publication_mode,
            coalescing_intent,
            replay_mode,
            delivery_intent,
            diagnostics_policy_class,
            protocol_semantics_version,
            digest,
        }
    }

    pub fn declaration_identity(&self) -> &ChangeStreamDeclarationIdentity {
        &self.declaration_identity
    }

    pub fn consumer_shape(&self) -> StreamConsumerShape {
        self.consumer_shape
    }

    pub fn resume_mode(&self) -> StreamResumeMode {
        self.resume_mode
    }

    pub fn checkpoint_publication_mode(&self) -> StreamCheckpointPublicationMode {
        self.checkpoint_publication_mode
    }

    pub fn coalescing_intent(&self) -> StreamCoalescingIntent {
        self.coalescing_intent
    }

    pub fn replay_mode(&self) -> StreamReplayMode {
        self.replay_mode
    }

    pub fn delivery_intent(&self) -> StreamDeliveryIntent {
        self.delivery_intent
    }

    pub fn diagnostics_policy_class(&self) -> StreamDiagnosticsPolicyClass {
        self.diagnostics_policy_class
    }

    pub fn protocol_semantics_version(&self) -> &str {
        self.protocol_semantics_version.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

pub(crate) fn consumer_shape_label(value: StreamConsumerShape) -> &'static str {
    match value {
        StreamConsumerShape::RoutingConsumer => "routing-consumer",
        StreamConsumerShape::ReplayAuditConsumer => "replay-audit-consumer",
    }
}

pub(crate) fn resume_mode_label(value: StreamResumeMode) -> &'static str {
    match value {
        StreamResumeMode::FromCheckpointOnly => "from-checkpoint-only",
        StreamResumeMode::FromStreamPosition => "from-stream-position",
    }
}

pub(crate) fn checkpoint_publication_mode_label(
    value: StreamCheckpointPublicationMode,
) -> &'static str {
    match value {
        StreamCheckpointPublicationMode::PublishEveryWindow => "publish-every-window",
        StreamCheckpointPublicationMode::PublishOnDemand => "publish-on-demand",
    }
}

pub(crate) fn coalescing_family_label(value: StreamCoalescingFamily) -> &'static str {
    match value {
        StreamCoalescingFamily::None => "none",
        StreamCoalescingFamily::RoutingWindowCoalescing => "routing-window-coalescing",
        StreamCoalescingFamily::ReplayAuditWindowCoalescing => "replay-audit-window-coalescing",
    }
}

pub(crate) fn coalescing_intent_label(value: StreamCoalescingIntent) -> Arc<str> {
    match value {
        StreamCoalescingIntent::None => Arc::from("none"),
        StreamCoalescingIntent::Prefer(family) => {
            Arc::from(format!("prefer:{}", coalescing_family_label(family)))
        }
    }
}

pub(crate) fn replay_mode_label(value: StreamReplayMode) -> &'static str {
    match value {
        StreamReplayMode::Disabled => "disabled",
        StreamReplayMode::Enabled => "enabled",
    }
}

pub(crate) fn delivery_intent_label(value: StreamDeliveryIntent) -> &'static str {
    match value {
        StreamDeliveryIntent::RouteInvalidations => "route-invalidations",
        StreamDeliveryIntent::ReplayAudit => "replay-audit",
    }
}

pub(crate) fn diagnostics_policy_class_label(value: StreamDiagnosticsPolicyClass) -> &'static str {
    match value {
        StreamDiagnosticsPolicyClass::Minimal => "minimal",
        StreamDiagnosticsPolicyClass::Standard => "standard",
        StreamDiagnosticsPolicyClass::Exhaustive => "exhaustive",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declaration_identity_is_stable_for_identical_inputs() {
        let left = ChangeStreamDeclaration::new(
            StreamConsumerShape::RoutingConsumer,
            StreamResumeMode::FromCheckpointOnly,
            StreamCheckpointPublicationMode::PublishEveryWindow,
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
            StreamReplayMode::Enabled,
            StreamDeliveryIntent::RouteInvalidations,
            StreamDiagnosticsPolicyClass::Standard,
        );
        let right = ChangeStreamDeclaration::new(
            StreamConsumerShape::RoutingConsumer,
            StreamResumeMode::FromCheckpointOnly,
            StreamCheckpointPublicationMode::PublishEveryWindow,
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
            StreamReplayMode::Enabled,
            StreamDeliveryIntent::RouteInvalidations,
            StreamDiagnosticsPolicyClass::Standard,
        );

        assert_eq!(left, right);
        assert!(left
            .digest()
            .starts_with("change-stream-declaration:sha256:"));
    }

    #[test]
    fn declaration_identity_is_invariant_across_diagnostics_tiers() {
        let standard = ChangeStreamDeclaration::new(
            StreamConsumerShape::RoutingConsumer,
            StreamResumeMode::FromCheckpointOnly,
            StreamCheckpointPublicationMode::PublishEveryWindow,
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
            StreamReplayMode::Enabled,
            StreamDeliveryIntent::RouteInvalidations,
            StreamDiagnosticsPolicyClass::Standard,
        );
        let exhaustive = ChangeStreamDeclaration::new(
            StreamConsumerShape::RoutingConsumer,
            StreamResumeMode::FromCheckpointOnly,
            StreamCheckpointPublicationMode::PublishEveryWindow,
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
            StreamReplayMode::Enabled,
            StreamDeliveryIntent::RouteInvalidations,
            StreamDiagnosticsPolicyClass::Exhaustive,
        );

        assert_eq!(
            standard.declaration_identity(),
            exhaustive.declaration_identity()
        );
        assert_eq!(standard.digest(), exhaustive.digest());
        assert_ne!(
            standard.diagnostics_policy_class(),
            exhaustive.diagnostics_policy_class()
        );
    }
}
