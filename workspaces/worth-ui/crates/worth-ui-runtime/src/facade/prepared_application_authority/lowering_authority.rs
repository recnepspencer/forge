use std::fmt;
use std::rc::Rc;

use super::{WorthUiHostSessionPlan, WorthUiPreparedApplicationGenerationIdentity};
use crate::facade::registry::CapabilitySnapshot;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiExecutionLaneSupport,
    WorthUiReplacementCandidateBasis,
};

struct WorthUiPreparedApplicationLoweringFacts {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    source_candidate_basis: Option<WorthUiReplacementCandidateBasis>,
    source_artifact_authority: Option<Rc<crate::source::WorthUiArtifact>>,
    graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
    capability_snapshot: Rc<CapabilitySnapshot>,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    host_session_plan: WorthUiHostSessionPlan,
    execution_lane_support: WorthUiExecutionLaneSupport,
}

/// Retained exact authority for every prepared constituent that can affect
/// execution-plan lowering. Cloning this value shares one sealed authority; it
/// does not recreate authority from comparison-safe identities or digests.
#[derive(Clone)]
pub(crate) struct WorthUiPreparedApplicationLoweringAuthority {
    facts: Rc<WorthUiPreparedApplicationLoweringFacts>,
}

impl WorthUiPreparedApplicationLoweringAuthority {
    pub(super) fn seal(
        generation_identity: WorthUiPreparedApplicationGenerationIdentity,
        source_candidate_basis: Option<WorthUiReplacementCandidateBasis>,
        source_artifact_authority: Option<Rc<crate::source::WorthUiArtifact>>,
        graph_authority_identity: crate::graph::UiGraphAuthorityIdentity,
        capability_snapshot: Rc<CapabilitySnapshot>,
        query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
        host_session_plan: WorthUiHostSessionPlan,
    ) -> Self {
        let execution_lane_support = WorthUiExecutionLaneSupport::for_prepared_application(
            host_session_plan.host_kind(),
            !query_binding_plan.is_query_free(),
        );
        Self {
            facts: Rc::new(WorthUiPreparedApplicationLoweringFacts {
                generation_identity,
                source_candidate_basis,
                source_artifact_authority,
                graph_authority_identity,
                capability_snapshot,
                query_binding_plan,
                host_session_plan,
                execution_lane_support,
            }),
        }
    }

    pub(crate) fn shares_authority_with(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.facts, &other.facts)
    }

    pub(crate) fn admits_candidate(&self, admitted: &WorthUiAdmittedReplacementCandidate) -> bool {
        self.facts.source_candidate_basis == Some(admitted.candidate().basis())
            && self
                .facts
                .source_artifact_authority
                .as_ref()
                .is_some_and(|artifact| {
                    admitted
                        .artifact_bundle()
                        .shares_artifact_authority_with(artifact)
                })
            && self.facts.capability_snapshot.digest().as_u64()
                == admitted.candidate().lowering_basis().snapshot_digest()
    }

    pub(crate) fn admits_launch_artifact(
        &self,
        artifact: &crate::source::WorthUiArtifact,
        artifact_digest: crate::source::WorthUiArtifactDigest,
    ) -> bool {
        self.facts
            .source_candidate_basis
            .is_some_and(|basis| basis.artifact_digest() == artifact_digest)
            && self
                .facts
                .source_artifact_authority
                .as_ref()
                .is_some_and(|admitted| std::ptr::eq(admitted.as_ref(), artifact))
    }

    pub(crate) fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.facts.generation_identity
    }

    pub(crate) fn graph_authority_identity(&self) -> crate::graph::UiGraphAuthorityIdentity {
        self.facts.graph_authority_identity
    }

    pub(crate) fn query_binding_plan(&self) -> &worth_ui_query_binding::WorthUiQueryBindingPlan {
        &self.facts.query_binding_plan
    }

    pub(crate) fn execution_lane_support(&self) -> &WorthUiExecutionLaneSupport {
        &self.facts.execution_lane_support
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn synthetic_successor_for_certification(
        &self,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Self {
        Self::seal(
            self.facts.generation_identity.clone(),
            Some(admitted.candidate().basis()),
            Some(admitted.artifact_bundle().artifact_authority()),
            self.facts.graph_authority_identity,
            Rc::clone(&self.facts.capability_snapshot),
            self.facts.query_binding_plan.clone(),
            self.facts.host_session_plan.clone(),
        )
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn synthetic_launch_for_certification(
        &self,
        artifact: Rc<crate::source::WorthUiArtifact>,
        artifact_digest: crate::source::WorthUiArtifactDigest,
    ) -> Self {
        Self::seal(
            self.facts.generation_identity.clone(),
            Some(WorthUiReplacementCandidateBasis::new(artifact_digest, 0, 0)),
            Some(artifact),
            self.facts.graph_authority_identity,
            Rc::clone(&self.facts.capability_snapshot),
            self.facts.query_binding_plan.clone(),
            self.facts.host_session_plan.clone(),
        )
    }
}

impl fmt::Debug for WorthUiPreparedApplicationLoweringAuthority {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("WorthUiPreparedApplicationLoweringAuthority")
            .field("generation_identity", &self.facts.generation_identity)
            .field(
                "graph_authority_identity",
                &self.facts.graph_authority_identity,
            )
            .field(
                "capability_snapshot_digest",
                &self.facts.capability_snapshot.digest(),
            )
            .field("query_binding_plan", &self.facts.query_binding_plan)
            .field("host_session_plan", &self.facts.host_session_plan)
            .finish_non_exhaustive()
    }
}

impl PartialEq for WorthUiPreparedApplicationLoweringAuthority {
    fn eq(&self, other: &Self) -> bool {
        self.shares_authority_with(other)
    }
}

impl Eq for WorthUiPreparedApplicationLoweringAuthority {}
