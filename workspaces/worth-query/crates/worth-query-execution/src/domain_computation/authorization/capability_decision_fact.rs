//! Retained capability decision authority and its one-way commit split.

use std::sync::Arc;

use worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline;

use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;
use super::{
    WorthQueryAuthorizationDecisionFact, WorthQueryAuthorizationTimeSample,
    WorthQueryPrincipalCurrentnessDependency,
};

pub(in crate::domain_computation) struct WorthQueryRetainedCapabilityAuthorization {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decision: WorthQueryAuthorizationDecisionFact,
    capability_authority_identity: Arc<str>,
    grant: worth_relational::facade::identity::EntityId,
    request: WorthQueryRetainedCapabilityRequest,
    sample: WorthQueryAuthorizationTimeSample,
}

impl WorthQueryRetainedCapabilityAuthorization {
    pub(in crate::domain_computation) fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decision: WorthQueryAuthorizationDecisionFact,
        capability_authority_identity: Arc<str>,
        grant: worth_relational::facade::identity::EntityId,
        request: WorthQueryRetainedCapabilityRequest,
        sample: WorthQueryAuthorizationTimeSample,
    ) -> Self {
        Self {
            principal,
            decision,
            capability_authority_identity,
            grant,
            request,
            sample,
        }
    }

    pub(super) fn principal(&self) -> &WorthQueryPrincipalCurrentnessDependency {
        &self.principal
    }

    pub(in crate::domain_computation) fn decision(&self) -> &WorthQueryAuthorizationDecisionFact {
        &self.decision
    }

    pub(in crate::domain_computation) fn belongs_to_branch(
        &self,
        branch_id: &worth_relational::facade::history::BranchId,
    ) -> bool {
        self.principal.branch_id() == branch_id && self.decision.branch_id() == branch_id
    }

    pub(in crate::domain_computation) fn belongs_to_session(
        &self,
        session_identity: &worth_foundational::facade::CanonicalDigestId,
    ) -> bool {
        self.principal.session_identity() == session_identity
            && self.decision.session_identity() == session_identity
    }

    pub(in crate::domain_computation) const fn capability_identity(&self) -> &[u8; 32] {
        &self.request.capability_identity
    }

    pub(super) fn request(&self) -> &WorthQueryRetainedCapabilityRequest {
        &self.request
    }

    pub(super) const fn grant(&self) -> worth_relational::facade::identity::EntityId {
        self.grant
    }

    pub(in crate::domain_computation) const fn exact_fact_count(&self) -> usize {
        2
    }

    pub(in crate::domain_computation) fn capability_authority_identity(&self) -> &str {
        &self.capability_authority_identity
    }

    pub(in crate::domain_computation) fn decision_identity(&self) -> [u8; 32] {
        *self.decision.relational.observation_identity().bytes()
    }

    pub(super) const fn timeline(&self) -> ApplicationCapabilityValidityTimeline {
        self.sample.timeline()
    }

    pub(in crate::domain_computation) const fn sampled_value(
        &self,
    ) -> &worth_foundational::facade::AspectValue {
        self.sample.value()
    }

    pub(super) fn replace_current_decision(
        &mut self,
        capability_authority_identity: &str,
        grant: worth_relational::facade::identity::EntityId,
        sample: WorthQueryAuthorizationTimeSample,
        decision: WorthQueryAuthorizationDecisionFact,
    ) -> Result<(), ()> {
        if self.capability_authority_identity.as_ref() != capability_authority_identity
            || self.grant != grant
            || self.sample.timeline() != sample.timeline()
        {
            return Err(());
        }
        self.sample = sample;
        self.decision = decision;
        Ok(())
    }

    pub(in crate::domain_computation) fn into_rebound_session(
        self,
        session_identity: worth_foundational::facade::CanonicalDigestId,
        branch_id: worth_relational::facade::history::BranchId,
        sample: WorthQueryAuthorizationTimeSample,
        decision: WorthQueryAuthorizationDecisionFact,
    ) -> Result<Self, ()> {
        if self.sample.timeline() != sample.timeline()
            || decision.session_identity() != &session_identity
            || decision.branch_id() != &branch_id
        {
            return Err(());
        }
        Ok(Self {
            principal: self.principal.rebind_session(session_identity, branch_id),
            decision,
            capability_authority_identity: self.capability_authority_identity,
            grant: self.grant,
            request: self.request,
            sample,
        })
    }

    pub(in crate::domain_computation) fn validate_currentness_in(
        &self,
        runtime: &worth_relational::facade::runtime::RelationalRuntime,
        snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
        bridge: &worth_runtime_bridge::facade::BridgeAuthorizationRuntime,
    ) -> Result<(), super::WorthQueryOperationAuthorizationDenialKind> {
        if !self.principal.remains_current_in(runtime, snapshot) {
            return Err(super::WorthQueryOperationAuthorizationDenialKind::StalePrincipal);
        }
        if !self.decision.remains_current_in(runtime, snapshot, bridge) {
            return Err(super::WorthQueryOperationAuthorizationDenialKind::StaleAuthorization);
        }
        Ok(())
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryPrincipalCurrentnessDependency,
        WorthQueryAuthorizationDecisionFact,
        WorthQueryCapabilityCommitBasis,
    ) {
        let commit = WorthQueryCapabilityCommitBasis {
            principal: self.principal.clone(),
            decision: self.decision.clone(),
            capability_authority_identity: self.capability_authority_identity,
            grant: self.grant,
            request: self.request,
        };
        (self.principal, self.decision, commit)
    }
}

pub(in crate::domain_computation) struct WorthQueryCapabilityCommitBasis {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decision: WorthQueryAuthorizationDecisionFact,
    capability_authority_identity: Arc<str>,
    grant: worth_relational::facade::identity::EntityId,
    request: WorthQueryRetainedCapabilityRequest,
}

impl WorthQueryCapabilityCommitBasis {
    pub(super) fn principal(&self) -> &WorthQueryPrincipalCurrentnessDependency {
        &self.principal
    }

    pub(super) fn decision(&self) -> &WorthQueryAuthorizationDecisionFact {
        &self.decision
    }

    pub(super) fn capability_authority_identity(&self) -> &str {
        &self.capability_authority_identity
    }

    pub(super) const fn grant(&self) -> worth_relational::facade::identity::EntityId {
        self.grant
    }

    pub(super) fn request(&self) -> &WorthQueryRetainedCapabilityRequest {
        &self.request
    }

    pub(super) fn belongs_to_session(
        &self,
        session_identity: &worth_foundational::facade::CanonicalDigestId,
    ) -> bool {
        self.principal.session_identity() == session_identity
            && self.decision.session_identity() == session_identity
    }

    pub(super) fn belongs_to_branch(
        &self,
        branch_id: &worth_relational::facade::history::BranchId,
    ) -> bool {
        self.principal.branch_id() == branch_id && self.decision.branch_id() == branch_id
    }
}
