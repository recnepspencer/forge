use std::rc::Rc;

use super::super::generation_identity::WorthUiPreparedGenerationIdentityInput;
use super::super::lowering_authority::WorthUiPreparedApplicationLoweringInput;
use super::super::{
    WorthUiPreparedApplicationGenerationIdentity, WorthUiPreparedApplicationLoweringAuthority,
};
use super::WorthUiPreparedApplicationAuthority;

pub(crate) struct WorthUiPreparedApplicationGraphSuccessor {
    predecessor_authority: crate::graph::UiGraphAuthorityIdentity,
    graph_snapshot: crate::graph::UiGraphSnapshot,
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    lowering_authority: WorthUiPreparedApplicationLoweringAuthority,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPreparedApplicationGraphSuccessorDenial {
    StaleGraphPredecessor,
}

impl WorthUiPreparedApplicationGraphSuccessor {
    pub(crate) fn graph_snapshot(&self) -> &crate::graph::UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub(crate) fn lowering_authority(&self) -> WorthUiPreparedApplicationLoweringAuthority {
        self.lowering_authority.clone()
    }
}

impl WorthUiPreparedApplicationAuthority {
    pub(crate) fn advance_graph_snapshot(
        &mut self,
        committed: crate::graph::UiGraphMutationCommitResult,
    ) {
        self.graph_snapshot = committed.into_committed_snapshot();
        self.generation_identity = self.derive_generation_identity(&self.graph_snapshot);
        self.lowering_authority =
            self.seal_lowering_authority(&self.graph_snapshot, self.generation_identity.clone());
        self.rebuild_derived_indexes();
    }

    pub(crate) fn prepare_graph_successor(
        &self,
        committed: crate::graph::UiGraphMutationCommitResult,
    ) -> Result<
        WorthUiPreparedApplicationGraphSuccessor,
        WorthUiPreparedApplicationGraphSuccessorDenial,
    > {
        let predecessor_authority = self.graph_snapshot.authority_identity();
        if committed.predecessor_authority() != Some(predecessor_authority) {
            return Err(WorthUiPreparedApplicationGraphSuccessorDenial::StaleGraphPredecessor);
        }
        let graph_snapshot = committed.into_committed_snapshot();
        let generation_identity = self.derive_generation_identity(&graph_snapshot);
        let lowering_authority =
            self.seal_lowering_authority(&graph_snapshot, generation_identity.clone());
        Ok(WorthUiPreparedApplicationGraphSuccessor {
            predecessor_authority,
            graph_snapshot,
            generation_identity,
            lowering_authority,
        })
    }

    pub(crate) fn commit_graph_successor(
        &mut self,
        successor: WorthUiPreparedApplicationGraphSuccessor,
    ) -> Result<
        WorthUiPreparedApplicationLoweringAuthority,
        WorthUiPreparedApplicationGraphSuccessorDenial,
    > {
        if self.graph_snapshot.authority_identity() != successor.predecessor_authority {
            return Err(WorthUiPreparedApplicationGraphSuccessorDenial::StaleGraphPredecessor);
        }
        self.graph_snapshot = successor.graph_snapshot;
        self.generation_identity = successor.generation_identity;
        self.lowering_authority = successor.lowering_authority;
        self.rebuild_derived_indexes();
        Ok(self.lowering_authority.clone())
    }

    fn derive_generation_identity(
        &self,
        graph_snapshot: &crate::graph::UiGraphSnapshot,
    ) -> WorthUiPreparedApplicationGenerationIdentity {
        WorthUiPreparedApplicationGenerationIdentity::derive(
            WorthUiPreparedGenerationIdentityInput {
                capability_snapshot: self.capability_snapshot.digest(),
                canonical_artifact: self.canonical_artifact.identity(),
                declaration_source: self.declaration_source_identity.clone(),
                semantic_package: self.semantic_handoff.identity().clone(),
                graph_authority_digest: graph_snapshot.authority_digest(),
                query_binding_plan: &self.query_binding_plan,
                host_session_plan: &self.host_session_plan,
                visual_inspection_policy: self.visual_inspection_policy,
            },
        )
    }

    fn seal_lowering_authority(
        &self,
        graph_snapshot: &crate::graph::UiGraphSnapshot,
        generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    ) -> WorthUiPreparedApplicationLoweringAuthority {
        WorthUiPreparedApplicationLoweringAuthority::seal(WorthUiPreparedApplicationLoweringInput {
            generation_identity,
            source_candidate_basis: self.source_backed_candidate_basis(),
            source_artifact_authority: self.canonical_artifact.runtime_artifact_authority().0,
            graph_authority_identity: graph_snapshot.authority_identity(),
            capability_snapshot: Rc::clone(&self.capability_snapshot),
            query_binding_plan: self.query_binding_plan.clone(),
            host_session_plan: self.host_session_plan.clone(),
        })
    }
}
