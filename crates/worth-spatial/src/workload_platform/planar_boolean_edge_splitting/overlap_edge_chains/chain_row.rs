use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::boundary_role::PlanarBooleanOverlapChainPosture;
use super::chain_member::PlanarBooleanOverlapEdgeChainMember;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanOverlapEdgeChain {
    chain_identity: String,
    interval_event_identity: String,
    interval_event_kind: PlanarBooleanIntervalEventKind,
    posture: PlanarBooleanOverlapChainPosture,
    source_interval_identities: Vec<String>,
    normalized_interval_identities: Vec<String>,
    source_senses: Vec<PlanarBooleanSourceIntervalSense>,
    event_group_identities: Vec<String>,
    members: Vec<PlanarBooleanOverlapEdgeChainMember>,
}

impl PlanarBooleanOverlapEdgeChain {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        chain_identity: String,
        interval_event_identity: String,
        interval_event_kind: PlanarBooleanIntervalEventKind,
        posture: PlanarBooleanOverlapChainPosture,
        source_interval_identities: Vec<String>,
        normalized_interval_identities: Vec<String>,
        source_senses: Vec<PlanarBooleanSourceIntervalSense>,
        event_group_identities: Vec<String>,
        members: Vec<PlanarBooleanOverlapEdgeChainMember>,
    ) -> Self {
        Self {
            chain_identity,
            interval_event_identity,
            interval_event_kind,
            posture,
            source_interval_identities,
            normalized_interval_identities,
            source_senses,
            event_group_identities,
            members,
        }
    }

    pub fn chain_identity(&self) -> &str {
        &self.chain_identity
    }
    pub fn interval_event_identity(&self) -> &str {
        &self.interval_event_identity
    }
    pub fn interval_event_kind(&self) -> PlanarBooleanIntervalEventKind {
        self.interval_event_kind
    }
    pub fn posture(&self) -> PlanarBooleanOverlapChainPosture {
        self.posture
    }
    pub fn source_interval_identities(&self) -> &[String] {
        &self.source_interval_identities
    }
    pub fn normalized_interval_identities(&self) -> &[String] {
        &self.normalized_interval_identities
    }
    pub fn source_senses(&self) -> &[PlanarBooleanSourceIntervalSense] {
        &self.source_senses
    }
    pub fn event_group_identities(&self) -> &[String] {
        &self.event_group_identities
    }
    pub fn members(&self) -> &[PlanarBooleanOverlapEdgeChainMember] {
        &self.members
    }
}
