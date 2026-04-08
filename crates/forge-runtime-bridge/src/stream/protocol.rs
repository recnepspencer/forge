use std::sync::Arc;

use crate::error::{BridgeStreamError, BridgeStreamErrorKind};
use crate::identity::{BridgeIdentity, ConsumerContractIdentityTag, StreamProtocolIdentityTag};
use crate::routing::canonicalization::digest_string;

use super::declaration::{
    checkpoint_publication_mode_label, coalescing_family_label,
    delivery_intent_label, replay_mode_label, resume_mode_label, ChangeStreamDeclaration,
    StreamCheckpointPublicationMode, StreamCoalescingFamily, StreamConsumerShape,
    StreamDeliveryIntent, StreamReplayMode,
};

pub type StreamProtocolIdentity = BridgeIdentity<StreamProtocolIdentityTag>;
pub type ConsumerContractIdentity = BridgeIdentity<ConsumerContractIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedStreamProtocol {
    stream_protocol_identity: StreamProtocolIdentity,
    declaration: ChangeStreamDeclaration,
    digest: Arc<str>,
}

impl ValidatedStreamProtocol {
    pub(crate) fn from_declaration(
        declaration: ChangeStreamDeclaration,
    ) -> Result<Self, BridgeStreamError> {
        let basis = format!(
            "validated-stream-protocol|declaration={}|consumer-shape={}|resume-mode={}|checkpoint-mode={}|coalescing-intent={}|protocol-semantics-version={}",
            declaration.declaration_identity().as_str(),
            super::declaration::consumer_shape_label(declaration.consumer_shape()),
            resume_mode_label(declaration.resume_mode()),
            checkpoint_publication_mode_label(declaration.checkpoint_publication_mode()),
            super::declaration::coalescing_intent_label(declaration.coalescing_intent()),
            declaration.protocol_semantics_version(),
        );
        let digest = digest_string("validated-stream-protocol", &basis);
        Ok(Self {
            stream_protocol_identity: StreamProtocolIdentity::new(digest.clone()),
            declaration,
            digest,
        })
    }

    pub fn stream_protocol_identity(&self) -> &StreamProtocolIdentity {
        &self.stream_protocol_identity
    }

    pub fn declaration(&self) -> &ChangeStreamDeclaration {
        &self.declaration
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedConsumerContract {
    consumer_contract_identity: ConsumerContractIdentity,
    stream_protocol_identity: StreamProtocolIdentity,
    consumer_shape: StreamConsumerShape,
    admitted_resume_mode: super::declaration::StreamResumeMode,
    admitted_checkpoint_mode: StreamCheckpointPublicationMode,
    admitted_coalescing_family: StreamCoalescingFamily,
    admitted_replay_mode: StreamReplayMode,
    admitted_delivery_intent: StreamDeliveryIntent,
    diagnostics_policy_class: super::declaration::StreamDiagnosticsPolicyClass,
    digest: Arc<str>,
}

impl AdmittedConsumerContract {
    pub(crate) fn resolve(
        protocol: &ValidatedStreamProtocol,
    ) -> Result<Self, BridgeStreamError> {
        let declaration = protocol.declaration();
        if declaration.resume_mode() == super::declaration::StreamResumeMode::FromCheckpointOnly
            && declaration.replay_mode() != StreamReplayMode::Enabled
        {
            return Err(BridgeStreamError::new(
                BridgeStreamErrorKind::UnsupportedResumeMode,
                "Checkpoint-based resume requires replay mode to be enabled so the bridge can retain canonical resume records.",
            ));
        }

        let admitted_resume_mode = match declaration.resume_mode() {
            super::declaration::StreamResumeMode::FromCheckpointOnly => {
                super::declaration::StreamResumeMode::FromCheckpointOnly
            }
            super::declaration::StreamResumeMode::FromStreamPosition => {
                return Err(BridgeStreamError::new(
                    BridgeStreamErrorKind::UnsupportedResumeMode,
                    "Milestone 6 only admits checkpoint-based resume. Stream-position resume is not implemented yet.",
                ));
            }
        };

        let admitted_delivery_intent = match (
            declaration.consumer_shape(),
            declaration.delivery_intent(),
        ) {
            (
                StreamConsumerShape::RoutingConsumer,
                StreamDeliveryIntent::RouteInvalidations,
            ) => StreamDeliveryIntent::RouteInvalidations,
            (
                StreamConsumerShape::ReplayAuditConsumer,
                StreamDeliveryIntent::ReplayAudit,
            ) => StreamDeliveryIntent::ReplayAudit,
            _ => {
                return Err(BridgeStreamError::new(
                    BridgeStreamErrorKind::UnsupportedConsumerShape,
                    "The declared delivery intent is incompatible with the selected stream consumer shape.",
                ));
            }
        };

        let admitted_coalescing_family = match declaration.coalescing_intent() {
            super::declaration::StreamCoalescingIntent::None => StreamCoalescingFamily::None,
            super::declaration::StreamCoalescingIntent::Prefer(
                StreamCoalescingFamily::RoutingWindowCoalescing,
            ) if declaration.consumer_shape() == StreamConsumerShape::RoutingConsumer => {
                StreamCoalescingFamily::RoutingWindowCoalescing
            }
            super::declaration::StreamCoalescingIntent::Prefer(
                StreamCoalescingFamily::ReplayAuditWindowCoalescing,
            ) if declaration.consumer_shape() == StreamConsumerShape::ReplayAuditConsumer => {
                StreamCoalescingFamily::ReplayAuditWindowCoalescing
            }
            super::declaration::StreamCoalescingIntent::Prefer(_) => {
                return Err(BridgeStreamError::new(
                    BridgeStreamErrorKind::IllegalCoalescingBoundary,
                    "The requested coalescing family is incompatible with the declared stream consumer shape.",
                ));
            }
        };

        let basis = format!(
            "admitted-consumer-contract|protocol={}|consumer-shape={}|resume-mode={}|checkpoint-mode={}|coalescing-family={}|replay-mode={}|delivery-intent={}|protocol-semantics-version={}",
            protocol.stream_protocol_identity().as_str(),
            super::declaration::consumer_shape_label(declaration.consumer_shape()),
            resume_mode_label(admitted_resume_mode),
            checkpoint_publication_mode_label(declaration.checkpoint_publication_mode()),
            coalescing_family_label(admitted_coalescing_family),
            replay_mode_label(declaration.replay_mode()),
            delivery_intent_label(admitted_delivery_intent),
            declaration.protocol_semantics_version(),
        );
        let digest = digest_string("admitted-consumer-contract", &basis);

        Ok(Self {
            consumer_contract_identity: ConsumerContractIdentity::new(digest.clone()),
            stream_protocol_identity: protocol.stream_protocol_identity().clone(),
            consumer_shape: declaration.consumer_shape(),
            admitted_resume_mode,
            admitted_checkpoint_mode: declaration.checkpoint_publication_mode(),
            admitted_coalescing_family,
            admitted_replay_mode: declaration.replay_mode(),
            admitted_delivery_intent,
            diagnostics_policy_class: declaration.diagnostics_policy_class(),
            digest,
        })
    }

    pub fn consumer_contract_identity(&self) -> &ConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn stream_protocol_identity(&self) -> &StreamProtocolIdentity {
        &self.stream_protocol_identity
    }

    pub fn consumer_shape(&self) -> StreamConsumerShape {
        self.consumer_shape
    }

    pub fn admitted_resume_mode(&self) -> super::declaration::StreamResumeMode {
        self.admitted_resume_mode
    }

    pub fn admitted_checkpoint_mode(&self) -> StreamCheckpointPublicationMode {
        self.admitted_checkpoint_mode
    }

    pub fn admitted_coalescing_family(&self) -> StreamCoalescingFamily {
        self.admitted_coalescing_family
    }

    pub fn admitted_replay_mode(&self) -> StreamReplayMode {
        self.admitted_replay_mode
    }

    pub fn admitted_delivery_intent(&self) -> StreamDeliveryIntent {
        self.admitted_delivery_intent
    }

    pub fn diagnostics_policy_class(&self) -> super::declaration::StreamDiagnosticsPolicyClass {
        self.diagnostics_policy_class
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
