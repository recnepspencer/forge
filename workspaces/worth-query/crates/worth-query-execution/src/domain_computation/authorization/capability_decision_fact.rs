//! Retained capability decision authority and its one-way commit split.

use std::sync::Arc;

use worth_query_declaration::facade::application_capability::ApplicationCapabilityValidityTimeline;
use worth_relational::facade::authorization::RelationalAuthorizationObservationCounters;

use super::{
    WorthQueryAuthorizationDecisionFact, WorthQueryAuthorizationTimeSample,
    WorthQueryPrincipalCurrentnessDependency,
};
use super::retained_capability_request::WorthQueryRetainedCapabilityRequest;

pub(in crate::domain_computation) struct WorthQueryRetainedCapabilityAuthorization {
    principal: WorthQueryPrincipalCurrentnessDependency,
    decision: WorthQueryAuthorizationDecisionFact,
    capability_authority_identity: Arc<str>,
    request: WorthQueryRetainedCapabilityRequest,
    sample: WorthQueryAuthorizationTimeSample,
}

impl WorthQueryRetainedCapabilityAuthorization {
    pub(super) fn new(
        principal: WorthQueryPrincipalCurrentnessDependency,
        decision: WorthQueryAuthorizationDecisionFact,
        capability_authority_identity: Arc<str>,
        request: WorthQueryRetainedCapabilityRequest,
        sample: WorthQueryAuthorizationTimeSample,
    ) -> Self {
        Self {
            principal,
            decision,
            capability_authority_identity,
            request,
            sample,
        }
    }

    pub(super) fn principal(&self) -> &WorthQueryPrincipalCurrentnessDependency {
        &self.principal
    }

    pub(super) fn decision(&self) -> &WorthQueryAuthorizationDecisionFact {
        &self.decision
    }

    pub(super) fn request(&self) -> &WorthQueryRetainedCapabilityRequest {
        &self.request
    }

    pub(super) const fn exact_fact_count(&self) -> usize {
        2
    }

    pub(super) fn relational_counters(&self) -> RelationalAuthorizationObservationCounters {
        self.decision.relational.counters()
    }

    pub(super) fn signal_dependency_count(&self) -> usize {
        let counters = self.decision.bridge.counters();
        counters.entities_depended_on
            + counters.relations_depended_on
            + counters.adjacency_lists_depended_on
            + counters.fields_depended_on
    }

    pub(super) fn capability_authority_identity(&self) -> &str {
        &self.capability_authority_identity
    }

    pub(super) const fn timeline(&self) -> ApplicationCapabilityValidityTimeline {
        self.sample.timeline()
    }

    pub(super) const fn sampled_value(&self) -> &worth_foundational::facade::AspectValue {
        self.sample.value()
    }

    pub(super) fn replace_current_decision(
        &mut self,
        capability_authority_identity: &str,
        sample: WorthQueryAuthorizationTimeSample,
        decision: WorthQueryAuthorizationDecisionFact,
    ) -> Result<(), ()> {
        if self.capability_authority_identity.as_ref() != capability_authority_identity
            || self.sample.timeline() != sample.timeline()
        {
            return Err(());
        }
        self.sample = sample;
        self.decision = decision;
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
            capability_authority_identity: self.capability_authority_identity,
            request: self.request,
        };
        (self.principal, self.decision, commit)
    }
}

pub(in crate::domain_computation) struct WorthQueryCapabilityCommitBasis {
    principal: WorthQueryPrincipalCurrentnessDependency,
    capability_authority_identity: Arc<str>,
    request: WorthQueryRetainedCapabilityRequest,
}

impl WorthQueryCapabilityCommitBasis {
    pub(super) fn principal(&self) -> &WorthQueryPrincipalCurrentnessDependency {
        &self.principal
    }

    pub(super) fn capability_authority_identity(&self) -> &str {
        &self.capability_authority_identity
    }

    pub(super) fn request(&self) -> &WorthQueryRetainedCapabilityRequest {
        &self.request
    }
}
