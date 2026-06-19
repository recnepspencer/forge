mod accepted_mutation_projection;
mod mutation_evidence;
#[cfg(test)]
mod query_anchor;

use forge_query::facade::{
    ForgeQueryAuthoritativeMutationObligationDispatchProjection, ForgeQueryDeclarationInput,
};
#[cfg(test)]
use forge_query::facade::{ForgeQueryBatchWriteReceipt, ForgeQueryBatchWriteReceiptInspection};

#[cfg(test)]
use super::TopologyQueryMutationLaneExecutionShape;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::query_domain::TopologyQueryDomain;
use crate::topology_operators::TopologyDeclaredMutationSequence;

use super::{
    TopologyMutationApplicationError, TopologyPostWriteQueryArtifact,
    TopologyRetainedApplicationHandoff,
};

pub(crate) use accepted_mutation_projection::TopologyAcceptedMutationProjection;
pub(crate) use mutation_evidence::TopologyMutationApplicationEvidence;
#[cfg(test)]
pub(crate) use query_anchor::TopologyOperatorApplicationQueryAnchor;

#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct TopologyDeclaredMutationArtifact {
    post_write_query_artifact: TopologyPostWriteQueryArtifact,
    accepted_mutation_projection: TopologyAcceptedMutationProjection,
    graph_obligation_orchestration:
        Option<ForgeQueryAuthoritativeMutationObligationDispatchProjection>,
    #[cfg(test)]
    query_anchor: TopologyOperatorApplicationQueryAnchor,
    mutation_evidence: TopologyMutationApplicationEvidence,
}

#[cfg_attr(not(test), allow(dead_code))]
impl TopologyDeclaredMutationArtifact {
    pub(crate) fn from_receipt(
        semantic_family_key: &'static str,
        retained_handoff: &TopologyRetainedApplicationHandoff<
            impl ForgeQueryDeclarationInput<TopologyQueryDomain>,
        >,
        sequence: &TopologyDeclaredMutationSequence,
        post_write_query_artifact: TopologyPostWriteQueryArtifact,
    ) -> Result<Self, TopologyMutationApplicationError> {
        if retained_handoff.declaration_family_key() != semantic_family_key {
            return Err(
                TopologyMutationApplicationError::QueryAnchorFamilyMismatch {
                    semantic_family_key,
                    query_declaration_family_key: retained_handoff.declaration_family_key(),
                },
            );
        }

        let accepted_query_contribution_semantic_projection = retained_handoff
            .retain_accepted_query_contribution_semantic_projection(
                semantic_family_key,
                sequence,
            )?;
        let graph_obligation_orchestration =
            retained_handoff.graph_obligation_dispatch_projection();
        let mutation_evidence =
            TopologyMutationApplicationEvidence::from_inspection_and_graph_obligation_projection(
                post_write_query_artifact.inspection(),
                graph_obligation_orchestration.as_ref(),
            );

        Ok(Self {
            post_write_query_artifact,
            accepted_mutation_projection:
                TopologyAcceptedMutationProjection::from_sequence_and_semantic_projection(
                    semantic_family_key,
                    sequence,
                    &accepted_query_contribution_semantic_projection,
                ),
            graph_obligation_orchestration,
            #[cfg(test)]
            query_anchor: TopologyOperatorApplicationQueryAnchor::from_retained_handoff(
                retained_handoff,
            ),
            mutation_evidence,
        })
    }

    #[cfg(test)]
    pub(crate) fn query_anchor(&self) -> &TopologyOperatorApplicationQueryAnchor {
        &self.query_anchor
    }

    pub(crate) fn mutation_evidence(&self) -> TopologyMutationApplicationEvidence {
        self.mutation_evidence.clone()
    }

    pub(crate) fn graph_obligation_envelope_digest(&self) -> Option<&str> {
        self.graph_obligation_orchestration
            .as_ref()
            .and_then(|projection| projection.envelope_digest())
    }

    pub(crate) fn graph_obligation_orchestration(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatchProjection> {
        self.graph_obligation_orchestration.as_ref()
    }

    #[cfg(test)]
    pub(crate) fn graph_composition_obligation(
        &self,
    ) -> Option<ForgeQueryAuthoritativeMutationObligationDispatchProjection> {
        self.receipt()
            .obligation_dispatch()
            .map(|dispatch| dispatch.evidence_projection())
    }

    #[cfg(test)]
    pub(crate) fn receipt(&self) -> &ForgeQueryBatchWriteReceipt {
        self.post_write_query_artifact.receipt()
    }

    #[cfg(test)]
    pub(crate) fn inspection(&self) -> &ForgeQueryBatchWriteReceiptInspection {
        self.post_write_query_artifact.inspection()
    }

    pub(crate) fn materialized(&self) -> &MaterializedTopologyView {
        self.post_write_query_artifact.materialized()
    }

    #[cfg(test)]
    pub(crate) fn execution_shape(&self) -> TopologyQueryMutationLaneExecutionShape {
        self.post_write_query_artifact.execution_shape()
    }

    pub(crate) fn into_materialized(self) -> MaterializedTopologyView {
        self.post_write_query_artifact.into_materialized()
    }

    pub(crate) fn accepted_mutation_projection(&self) -> &TopologyAcceptedMutationProjection {
        &self.accepted_mutation_projection
    }

    #[cfg(test)]
    pub(crate) fn post_write_query_artifact_for_test(&self) -> TopologyPostWriteQueryArtifact {
        self.post_write_query_artifact.clone()
    }
}
