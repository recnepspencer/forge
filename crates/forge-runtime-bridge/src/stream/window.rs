use std::sync::Arc;

use crate::identity::{BridgeIdentity, StreamWindowIdentityTag};
use crate::routing::canonicalization::digest_string;

use super::declaration::{
    checkpoint_publication_mode_label, coalescing_family_label, StreamCheckpointPublicationMode,
    StreamCoalescingFamily, StreamDiagnosticsPolicyClass,
};
use super::counters::StreamProtocolCounters;
use super::lowered::LoweredConsumedChangeSet;
use super::member::CanonicalStreamMember;
use super::position::CanonicalStreamPosition;
use super::protocol::{AdmittedConsumerContract, ConsumerContractIdentity};

pub type StreamWindowIdentity = BridgeIdentity<StreamWindowIdentityTag>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlannedChangeStreamWindow {
    stream_window_identity: StreamWindowIdentity,
    consumer_contract_identity: ConsumerContractIdentity,
    first_stream_position: CanonicalStreamPosition,
    last_stream_position: CanonicalStreamPosition,
    members: Arc<[CanonicalStreamMember]>,
    positions: Arc<[CanonicalStreamPosition]>,
    member_set_digest: Arc<str>,
    coalescing_family: StreamCoalescingFamily,
    checkpoint_publication_mode: StreamCheckpointPublicationMode,
    diagnostics_policy_class: StreamDiagnosticsPolicyClass,
    lowered_change_set: Option<LoweredConsumedChangeSet>,
    counters: StreamProtocolCounters,
    digest: Arc<str>,
}

impl PlannedChangeStreamWindow {
    pub(crate) fn new(
        contract: &AdmittedConsumerContract,
        members: Vec<CanonicalStreamMember>,
        positions: Vec<CanonicalStreamPosition>,
    ) -> Self {
        let member_set_digest = digest_string(
            "stream-member-set",
            &positions
                .iter()
                .map(|position| position.canonical_stream_member_identity())
                .collect::<Vec<_>>()
                .join("|"),
        );
        let first_stream_position = positions.first().expect("stream window").clone();
        let last_stream_position = positions.last().expect("stream window").clone();
        let basis = format!(
            "planned-change-stream-window|contract={}|first-position={}|last-position={}|member-set-digest={}|coalescing-family={}|checkpoint-mode={}",
            contract.consumer_contract_identity().as_str(),
            first_stream_position.stream_position_identity(),
            last_stream_position.stream_position_identity(),
            member_set_digest.as_ref(),
            coalescing_family_label(contract.admitted_coalescing_family()),
            checkpoint_publication_mode_label(contract.admitted_checkpoint_mode()),
        );
        let digest = digest_string("planned-change-stream-window", &basis);
        let counters = StreamProtocolCounters::for_planned_window(
            members.len(),
            contract.admitted_coalescing_family() != StreamCoalescingFamily::None,
        );
        Self {
            stream_window_identity: StreamWindowIdentity::new(digest.clone()),
            consumer_contract_identity: contract.consumer_contract_identity().clone(),
            first_stream_position,
            last_stream_position,
            members: Arc::from(members),
            positions: Arc::from(positions),
            member_set_digest,
            coalescing_family: contract.admitted_coalescing_family(),
            checkpoint_publication_mode: contract.admitted_checkpoint_mode(),
            diagnostics_policy_class: contract.diagnostics_policy_class(),
            lowered_change_set: None,
            counters,
            digest,
        }
    }

    pub(crate) fn with_lowered_change_set(
        mut self,
        lowered_change_set: LoweredConsumedChangeSet,
    ) -> Self {
        self.lowered_change_set = Some(lowered_change_set);
        self
    }

    pub fn stream_window_identity(&self) -> &StreamWindowIdentity {
        &self.stream_window_identity
    }

    pub fn consumer_contract_identity(&self) -> &ConsumerContractIdentity {
        &self.consumer_contract_identity
    }

    pub fn first_stream_position(&self) -> &CanonicalStreamPosition {
        &self.first_stream_position
    }

    pub fn last_stream_position(&self) -> &CanonicalStreamPosition {
        &self.last_stream_position
    }

    pub fn members(&self) -> &[CanonicalStreamMember] {
        &self.members
    }

    pub fn positions(&self) -> &[CanonicalStreamPosition] {
        &self.positions
    }

    pub fn member_set_digest(&self) -> &str {
        self.member_set_digest.as_ref()
    }

    pub fn coalescing_family(&self) -> StreamCoalescingFamily {
        self.coalescing_family
    }

    pub fn checkpoint_publication_mode(&self) -> StreamCheckpointPublicationMode {
        self.checkpoint_publication_mode
    }

    pub fn diagnostics_policy_class(&self) -> StreamDiagnosticsPolicyClass {
        self.diagnostics_policy_class
    }

    pub fn lowered_change_set(&self) -> Option<&LoweredConsumedChangeSet> {
        self.lowered_change_set.as_ref()
    }

    pub fn counters(&self) -> &StreamProtocolCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
