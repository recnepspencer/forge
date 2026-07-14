use super::UiAllocationFrameEpoch;

/// Closed source classification used to define deterministic cross-source order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationFrameSourceLane {
    HostMeasurement,
    QueryProjection,
    Interaction,
    DurableState,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiAllocationFrameSourceIdentity {
    Numeric(u64),
    Query(worth_ui_query_binding::WorthUiQueryAllocationSourceIdentity),
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationFrameSourceGeneration(u64);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationFrameSourceLeaseIdentity {
    slot: u16,
    generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationFrameIngressIdentity(u64);

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationFrameIngressKey {
    source_lane: UiAllocationFrameSourceLane,
    source_identity: UiAllocationFrameSourceIdentity,
    source_generation: UiAllocationFrameSourceGeneration,
    ingress_identity: UiAllocationFrameIngressIdentity,
    source_order: UiAdmittedAllocationSourceOrder,
}

/// Copy-only admission proof retained by evidence, retry, and replacement bookkeeping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationFrameIngressDescriptor {
    key: UiAllocationFrameIngressKey,
    source_fact_posture: super::gateway::UiAllocationFrameSourceFactPosture,
}

/// Move-only authority for one admitted producer generation.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct UiAllocationFrameSourceLease {
    lease_identity: UiAllocationFrameSourceLeaseIdentity,
    source_lane: UiAllocationFrameSourceLane,
    source_identity: UiAllocationFrameSourceIdentity,
    source_generation: UiAllocationFrameSourceGeneration,
}

impl UiAllocationFrameIngressIdentity {
    pub fn as_u64(self) -> u64 {
        self.0
    }

    #[cfg(test)]
    pub(crate) fn for_test(value: u64) -> Self {
        Self(value)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn from_gateway(value: u64) -> Self {
        Self(value)
    }
}

impl UiAllocationFrameSourceIdentity {
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::Numeric(value) => Some(*value),
            Self::Query(_) => None,
        }
    }

    pub fn as_opaque(&self) -> Option<&str> {
        match self {
            Self::Query(value) => Some(value.as_str()),
            Self::Numeric(_) => None,
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test(value: u64) -> Self {
        Self::Numeric(value)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn from_query(
        value: worth_ui_query_binding::WorthUiQueryAllocationSourceIdentity,
    ) -> Self {
        Self::Query(value)
    }
}

impl From<u64> for UiAllocationFrameSourceIdentity {
    fn from(value: u64) -> Self {
        Self::Numeric(value)
    }
}

impl UiAllocationFrameSourceGeneration {
    pub fn as_u64(self) -> u64 {
        self.0
    }

    #[cfg(test)]
    pub(crate) fn for_test(value: u64) -> Self {
        Self(value)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn from_gateway(value: u64) -> Self {
        Self(value)
    }
}

impl UiAllocationFrameSourceLeaseIdentity {
    pub(in crate::runtime::allocation_frame_dispatch) fn from_registry(
        slot: u16,
        generation: u64,
    ) -> Self {
        Self { slot, generation }
    }

    pub fn slot(self) -> u16 {
        self.slot
    }

    pub fn generation(self) -> u64 {
        self.generation
    }
}

impl UiAllocationFrameIngressKey {
    pub fn source_lane(&self) -> UiAllocationFrameSourceLane {
        self.source_lane
    }

    pub fn source_identity(&self) -> UiAllocationFrameSourceIdentity {
        self.source_identity.clone()
    }

    pub fn source_generation(&self) -> UiAllocationFrameSourceGeneration {
        self.source_generation
    }

    pub fn ingress_identity(&self) -> UiAllocationFrameIngressIdentity {
        self.ingress_identity
    }

    pub fn source_order(&self) -> UiAdmittedAllocationSourceOrder {
        self.source_order
    }
}

impl UiAllocationFrameSourceLease {
    pub(in crate::runtime::allocation_frame_dispatch) fn from_registry(
        lease_identity: UiAllocationFrameSourceLeaseIdentity,
        source_lane: UiAllocationFrameSourceLane,
        source_identity: UiAllocationFrameSourceIdentity,
        source_generation: UiAllocationFrameSourceGeneration,
    ) -> Self {
        Self {
            lease_identity,
            source_lane,
            source_identity,
            source_generation,
        }
    }

    #[cfg(test)]
    pub(super) fn admit_ingress(
        &self,
        authority: &super::dispatcher::UiAllocationFrameDispatcherTestAuthority,
        identity: UiAllocationFrameIngressIdentity,
        source_order: UiAdmittedAllocationSourceOrder,
    ) -> UiAdmittedAllocationStreamIngress {
        UiAdmittedAllocationStreamIngress::from_support(authority, self, identity, source_order)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn admit_gateway_ingress(
        &self,
        source_generation: UiAllocationFrameSourceGeneration,
        identity: UiAllocationFrameIngressIdentity,
        source_order: UiAdmittedAllocationSourceOrder,
        source_fact: super::gateway::UiAllocationFrameSourceFact,
    ) -> UiAdmittedAllocationStreamIngress {
        UiAdmittedAllocationStreamIngress {
            source_lease: self.lease_identity,
            source_lane: self.source_lane,
            source_identity: self.source_identity.clone(),
            source_generation,
            identity,
            source_order,
            source_fact,
        }
    }

    pub fn lease_identity(&self) -> UiAllocationFrameSourceLeaseIdentity {
        self.lease_identity
    }

    pub fn source_lane(&self) -> UiAllocationFrameSourceLane {
        self.source_lane
    }

    pub fn source_identity(&self) -> UiAllocationFrameSourceIdentity {
        self.source_identity.clone()
    }

    pub fn source_generation(&self) -> UiAllocationFrameSourceGeneration {
        self.source_generation
    }
}

/// Opaque source ordering already admitted by the source authority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAllocationFrameIngressSequence {
    epoch: UiAllocationFrameEpoch,
    canonical_ordinal: u16,
}

impl UiAllocationFrameIngressSequence {
    pub(super) fn assign(
        _authority: &super::dispatcher::UiAllocationFrameSealAuthority,
        epoch: UiAllocationFrameEpoch,
        canonical_ordinal: u16,
    ) -> Self {
        Self {
            epoch,
            canonical_ordinal,
        }
    }

    pub fn epoch(self) -> UiAllocationFrameEpoch {
        self.epoch
    }

    pub fn canonical_ordinal(self) -> u16 {
        self.canonical_ordinal
    }
}

/// Canonical order already admitted by the source authority.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiAdmittedAllocationSourceOrder(u64);

impl UiAdmittedAllocationSourceOrder {
    pub fn as_u64(self) -> u64 {
        self.0
    }

    #[cfg(test)]
    pub(crate) fn for_test(value: u64) -> Self {
        Self(value)
    }

    pub(in crate::runtime::allocation_frame_dispatch) fn from_gateway(value: u64) -> Self {
        Self(value)
    }
}

/// Source truth after admission, before policy or allocation work.
#[derive(Debug, PartialEq)]
pub struct UiAdmittedAllocationStreamIngress {
    source_lease: UiAllocationFrameSourceLeaseIdentity,
    source_lane: UiAllocationFrameSourceLane,
    source_identity: UiAllocationFrameSourceIdentity,
    source_generation: UiAllocationFrameSourceGeneration,
    identity: UiAllocationFrameIngressIdentity,
    source_order: UiAdmittedAllocationSourceOrder,
    source_fact: super::gateway::UiAllocationFrameSourceFact,
}

impl UiAdmittedAllocationStreamIngress {
    #[cfg(test)]
    fn from_support(
        _authority: &super::dispatcher::UiAllocationFrameDispatcherTestAuthority,
        lease: &UiAllocationFrameSourceLease,
        identity: UiAllocationFrameIngressIdentity,
        source_order: UiAdmittedAllocationSourceOrder,
    ) -> Self {
        Self {
            source_lease: lease.lease_identity,
            source_lane: lease.source_lane,
            source_identity: lease.source_identity.clone(),
            source_generation: lease.source_generation,
            identity,
            source_order,
            source_fact: super::gateway::UiAllocationFrameSourceFact::Interaction(
                crate::runtime::WorthUiAdmittedTransientInteraction::for_dispatcher_test(
                    crate::graph::UiGraphNodeIdentity::new(identity.as_u64()),
                    lease.source_generation.as_u64(),
                    source_order.as_u64(),
                ),
            ),
        }
    }

    pub fn source_lane(&self) -> UiAllocationFrameSourceLane {
        self.source_lane
    }

    pub fn source_lease(&self) -> UiAllocationFrameSourceLeaseIdentity {
        self.source_lease
    }

    pub fn source_identity(&self) -> UiAllocationFrameSourceIdentity {
        self.source_identity.clone()
    }

    pub fn source_generation(&self) -> UiAllocationFrameSourceGeneration {
        self.source_generation
    }

    pub fn identity(&self) -> UiAllocationFrameIngressIdentity {
        self.identity
    }

    pub fn source_order(&self) -> UiAdmittedAllocationSourceOrder {
        self.source_order
    }

    pub fn source_fact(&self) -> &super::gateway::UiAllocationFrameSourceFact {
        &self.source_fact
    }

    pub fn key(&self) -> UiAllocationFrameIngressKey {
        UiAllocationFrameIngressKey {
            source_lane: self.source_lane,
            source_identity: self.source_identity.clone(),
            source_generation: self.source_generation,
            ingress_identity: self.identity,
            source_order: self.source_order,
        }
    }

    pub fn descriptor(&self) -> UiAllocationFrameIngressDescriptor {
        UiAllocationFrameIngressDescriptor {
            key: self.key(),
            source_fact_posture: self.source_fact.posture(),
        }
    }

    pub(crate) fn into_source_fact(self) -> super::gateway::UiAllocationFrameSourceFact {
        self.source_fact
    }
}

#[cfg(test)]
impl Clone for UiAdmittedAllocationStreamIngress {
    fn clone(&self) -> Self {
        Self {
            source_lease: self.source_lease,
            source_lane: self.source_lane,
            source_identity: self.source_identity.clone(),
            source_generation: self.source_generation,
            identity: self.identity,
            source_order: self.source_order,
            source_fact: self.source_fact.clone(),
        }
    }
}

impl UiAllocationFrameIngressDescriptor {
    pub fn key(&self) -> UiAllocationFrameIngressKey {
        self.key.clone()
    }

    pub fn source_fact_posture(&self) -> super::gateway::UiAllocationFrameSourceFactPosture {
        self.source_fact_posture
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationFrameSubmissionDenial {
    ReplacementPaused,
    Shutdown,
    EpochExhausted,
    ConflictingIdentity,
    ConflictingSourceOrder,
    RetryWindowExpired,
    SourceDomainCapacityExhausted,
    SourceLeaseExpired,
    ConflictingSourceLease,
}
