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
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::migrated_products::CoveredDerivedProductMigrationSweepCloseout;
use crate::derived_topology::invalidation_plan::operator_cutover::{
    DerivedInvalidationOperatorCutoverError, DerivedInvalidationOperatorCutoverReceipt,
};
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;
use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::query_domain::TopologyQueryDomain;
use crate::topology_operators::TopologyDeclaredMutationSequence;
use crate::topology_operators::TopologyDeclaredTouchedGraphBasisProof;

use super::{
    TopologyMutationApplicationError, TopologyPostWriteQueryArtifact,
    TopologyRetainedApplicationHandoff,
};

pub(crate) use accepted_mutation_projection::TopologyAcceptedMutationProjection;
pub(crate) use mutation_evidence::TopologyMutationApplicationEvidence;
#[cfg(test)]
pub(crate) use query_anchor::TopologyOperatorApplicationQueryAnchor;

#[derive(Debug, Clone)]
pub(crate) struct TopologyDeclaredMutationArtifact {
    post_write_query_artifact: TopologyPostWriteQueryArtifact,
    accepted_mutation_projection: TopologyAcceptedMutationProjection,
    declared_touched_basis: TopologyDeclaredTouchedGraphBasisProof,
    graph_obligation_orchestration:
        Option<ForgeQueryAuthoritativeMutationObligationDispatchProjection>,
    #[cfg(test)]
    query_anchor: TopologyOperatorApplicationQueryAnchor,
    mutation_evidence: TopologyMutationApplicationEvidence,
}

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
            declared_touched_basis: retained_handoff.declared_touched_basis_proof().clone(),
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

    pub(crate) fn declared_touched_basis(&self) -> &TopologyDeclaredTouchedGraphBasisProof {
        &self.declared_touched_basis
    }

    pub(crate) fn bind_derived_invalidation_cutover(
        &self,
        phase_six_closeout: &CoveredDerivedProductMigrationSweepCloseout,
        selected_plan: &DerivedInvalidationSelectedPlan,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
    ) -> Result<DerivedInvalidationOperatorCutoverReceipt, DerivedInvalidationOperatorCutoverError>
    {
        DerivedInvalidationOperatorCutoverReceipt::bind_operator_cutover(
            phase_six_closeout,
            selected_plan,
            execution_receipt,
            &self.declared_touched_basis,
            &self.mutation_evidence,
        )
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

const _: () = {
    let _ = std::mem::size_of::<TopologyDeclaredMutationArtifact>();
    let _:
        for<'a, 'b> fn(
            &'static str,
            &'a TopologyRetainedApplicationHandoff<
                crate::topology_operators::TopologyCreateTopologyEntityDeclaration,
            >,
            &'b TopologyDeclaredMutationSequence,
            TopologyPostWriteQueryArtifact,
        ) -> Result<TopologyDeclaredMutationArtifact, TopologyMutationApplicationError> =
        TopologyDeclaredMutationArtifact::from_receipt;
    let _ = TopologyDeclaredMutationArtifact::mutation_evidence;
    let _ = TopologyDeclaredMutationArtifact::declared_touched_basis;
    let _ = TopologyDeclaredMutationArtifact::bind_derived_invalidation_cutover;
    let _ = TopologyDeclaredMutationArtifact::graph_obligation_envelope_digest;
    let _ = TopologyDeclaredMutationArtifact::graph_obligation_orchestration;
    let _ = TopologyDeclaredMutationArtifact::materialized;
    let _ = TopologyDeclaredMutationArtifact::into_materialized;
    let _ = TopologyDeclaredMutationArtifact::accepted_mutation_projection;
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_invalidation_cutover_binding_requires_operator_artifact_proofs() {
        let _: fn(
            &TopologyDeclaredMutationArtifact,
            &CoveredDerivedProductMigrationSweepCloseout,
            &DerivedInvalidationSelectedPlan,
            &DerivedInvalidationExecutionReceipt,
        ) -> Result<
            DerivedInvalidationOperatorCutoverReceipt,
            DerivedInvalidationOperatorCutoverError,
        > = TopologyDeclaredMutationArtifact::bind_derived_invalidation_cutover;
    }
}
