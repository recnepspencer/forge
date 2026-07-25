use super::{
    WorthUiHostSessionPlan, WorthUiPreparedApplicationArtifact,
    WorthUiPreparedApplicationArtifactPosture, WorthUiPreparedApplicationGenerationIdentity,
    WorthUiPreparedApplicationLoweringAuthority, WorthUiPreparedDeclarationSourceIdentity,
};
use crate::declaration::{UiDeclarationArtifact, UiDeclarationAuthoredEvidenceIndex};
use crate::facade::lifecycle::{build_graph_evidence_indexes, WorthUiFacadeLifecycleBootstrap};
use crate::facade::registry::snapshot::CapabilitySnapshot;
use crate::graph::{UiGraphAspectEvidenceIndexes, UiGraphNodeEvidenceIndex, UiGraphSnapshot};
use std::rc::Rc;

pub(crate) struct WorthUiPreparedApplicationAuthorityInput {
    pub(crate) capability_snapshot: Rc<CapabilitySnapshot>,
    pub(crate) canonical_artifact: WorthUiPreparedApplicationArtifact,
    pub(crate) declaration_source_identity: WorthUiPreparedDeclarationSourceIdentity,
    pub(crate) declaration_artifacts: Vec<UiDeclarationArtifact>,
    pub(crate) graph_snapshot: UiGraphSnapshot,
    pub(crate) lifecycle: WorthUiFacadeLifecycleBootstrap,
    pub(crate) query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    pub(crate) host_session_plan: WorthUiHostSessionPlan,
    pub(crate) runtime_instance_basis_admissions:
        Box<[crate::graph::UiRuntimeInstanceBasisAdmission]>,
    pub(crate) measurement_inspection_evidence:
        Box<[crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle]>,
}

/// The single move-only owner of all prepared truth for one application
/// generation. Public access is projection-only; constituent authority cannot
/// be extracted or independently launched.
pub struct WorthUiPreparedApplicationAuthority {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    lowering_authority: WorthUiPreparedApplicationLoweringAuthority,
    capability_snapshot: Rc<CapabilitySnapshot>,
    canonical_artifact: WorthUiPreparedApplicationArtifact,
    declaration_source_identity: WorthUiPreparedDeclarationSourceIdentity,
    declaration_artifacts: Vec<UiDeclarationArtifact>,
    graph_snapshot: UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
    authored_evidence_index: UiDeclarationAuthoredEvidenceIndex,
    graph_node_evidence_index: UiGraphNodeEvidenceIndex,
    graph_aspect_evidence_indexes: UiGraphAspectEvidenceIndexes,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    host_session_plan: WorthUiHostSessionPlan,
    runtime_instance_basis_admissions: Box<[crate::graph::UiRuntimeInstanceBasisAdmission]>,
    measurement_inspection_evidence:
        Box<[crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle]>,
}

pub(crate) struct WorthUiPreparedApplicationGraphSuccessor {
    predecessor_authority: crate::graph::UiGraphAuthorityIdentity,
    graph_snapshot: UiGraphSnapshot,
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    lowering_authority: WorthUiPreparedApplicationLoweringAuthority,
}

impl WorthUiPreparedApplicationGraphSuccessor {
    pub(crate) fn graph_snapshot(&self) -> &UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub(crate) fn lowering_authority(&self) -> WorthUiPreparedApplicationLoweringAuthority {
        self.lowering_authority.clone()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiPreparedApplicationGraphSuccessorDenial {
    StaleGraphPredecessor,
}

impl WorthUiPreparedApplicationAuthority {
    pub(crate) fn seal(input: WorthUiPreparedApplicationAuthorityInput) -> Self {
        let WorthUiPreparedApplicationAuthorityInput {
            capability_snapshot,
            canonical_artifact,
            declaration_source_identity,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
            query_binding_plan,
            host_session_plan,
            runtime_instance_basis_admissions,
            measurement_inspection_evidence,
        } = input;
        let authored_evidence_index =
            UiDeclarationAuthoredEvidenceIndex::rebuild(&declaration_artifacts, &graph_snapshot);
        let graph_evidence =
            build_graph_evidence_indexes(&declaration_artifacts, &graph_snapshot, &lifecycle);
        let generation_identity = WorthUiPreparedApplicationGenerationIdentity::derive(
            capability_snapshot.digest(),
            canonical_artifact.identity(),
            declaration_source_identity.clone(),
            graph_snapshot.authority_digest(),
            &query_binding_plan,
            &host_session_plan,
        );
        let source_artifact_authority = canonical_artifact
            .runtime_artifact_authority()
            .map(|(artifact, _)| artifact);
        let lowering_authority = WorthUiPreparedApplicationLoweringAuthority::seal(
            generation_identity.clone(),
            match &canonical_artifact {
                WorthUiPreparedApplicationArtifact::SourceBacked { basis, .. } => Some(*basis),
                WorthUiPreparedApplicationArtifact::DeclarationAuthored(_) => None,
            },
            source_artifact_authority,
            graph_snapshot.authority_identity(),
            Rc::clone(&capability_snapshot),
            query_binding_plan.clone(),
            host_session_plan.clone(),
        );
        Self {
            generation_identity,
            lowering_authority,
            capability_snapshot,
            canonical_artifact,
            declaration_source_identity,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
            authored_evidence_index,
            graph_node_evidence_index: graph_evidence.node,
            graph_aspect_evidence_indexes: graph_evidence.aspect,
            query_binding_plan,
            host_session_plan,
            runtime_instance_basis_admissions,
            measurement_inspection_evidence,
        }
    }

    pub fn generation_identity(&self) -> &WorthUiPreparedApplicationGenerationIdentity {
        &self.generation_identity
    }

    pub fn declaration_source_identity(&self) -> &WorthUiPreparedDeclarationSourceIdentity {
        &self.declaration_source_identity
    }

    pub fn application_artifact_posture(&self) -> WorthUiPreparedApplicationArtifactPosture {
        self.canonical_artifact.posture()
    }

    pub fn capabilities(&self) -> &CapabilitySnapshot {
        self.capability_snapshot.as_ref()
    }

    pub fn declaration_artifacts(&self) -> &[UiDeclarationArtifact] {
        &self.declaration_artifacts
    }

    pub fn host_session_plan(&self) -> &WorthUiHostSessionPlan {
        &self.host_session_plan
    }

    pub(crate) fn graph_snapshot(&self) -> &UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub(crate) fn advance_graph_snapshot(
        &mut self,
        committed: crate::graph::UiGraphMutationCommitResult,
    ) {
        self.graph_snapshot = committed.into_committed_snapshot();
        self.rebuild_derived_indexes();
        self.generation_identity = WorthUiPreparedApplicationGenerationIdentity::derive(
            self.capability_snapshot.digest(),
            self.canonical_artifact.identity(),
            self.declaration_source_identity.clone(),
            self.graph_snapshot.authority_digest(),
            &self.query_binding_plan,
            &self.host_session_plan,
        );
        self.lowering_authority = WorthUiPreparedApplicationLoweringAuthority::seal(
            self.generation_identity.clone(),
            self.source_backed_candidate_basis(),
            self.canonical_artifact
                .runtime_artifact_authority()
                .map(|(artifact, _)| artifact),
            self.graph_snapshot.authority_identity(),
            Rc::clone(&self.capability_snapshot),
            self.query_binding_plan.clone(),
            self.host_session_plan.clone(),
        );
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
        let generation_identity = WorthUiPreparedApplicationGenerationIdentity::derive(
            self.capability_snapshot.digest(),
            self.canonical_artifact.identity(),
            self.declaration_source_identity.clone(),
            graph_snapshot.authority_digest(),
            &self.query_binding_plan,
            &self.host_session_plan,
        );
        let lowering_authority = WorthUiPreparedApplicationLoweringAuthority::seal(
            generation_identity.clone(),
            self.source_backed_candidate_basis(),
            self.canonical_artifact
                .runtime_artifact_authority()
                .map(|(artifact, _)| artifact),
            graph_snapshot.authority_identity(),
            Rc::clone(&self.capability_snapshot),
            self.query_binding_plan.clone(),
            self.host_session_plan.clone(),
        );
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

    pub(crate) fn lifecycle(&self) -> &WorthUiFacadeLifecycleBootstrap {
        &self.lifecycle
    }

    pub(crate) fn authored_evidence_index(&self) -> &UiDeclarationAuthoredEvidenceIndex {
        &self.authored_evidence_index
    }

    pub(crate) fn graph_node_evidence_index(&self) -> &UiGraphNodeEvidenceIndex {
        &self.graph_node_evidence_index
    }

    pub(crate) fn graph_aspect_evidence_indexes(&self) -> &UiGraphAspectEvidenceIndexes {
        &self.graph_aspect_evidence_indexes
    }

    pub(crate) fn query_binding_plan(&self) -> &worth_ui_query_binding::WorthUiQueryBindingPlan {
        &self.query_binding_plan
    }

    pub(crate) fn lowering_authority(&self) -> WorthUiPreparedApplicationLoweringAuthority {
        self.lowering_authority.clone()
    }

    pub(crate) fn capability_authority(&self) -> Rc<CapabilitySnapshot> {
        Rc::clone(&self.capability_snapshot)
    }

    pub(crate) fn source_backed_candidate_basis(
        &self,
    ) -> Option<crate::runtime::WorthUiReplacementCandidateBasis> {
        match &self.canonical_artifact {
            WorthUiPreparedApplicationArtifact::SourceBacked { basis, .. } => Some(*basis),
            WorthUiPreparedApplicationArtifact::DeclarationAuthored(_) => None,
        }
    }

    pub(crate) fn runtime_instance_basis_admissions(
        &self,
    ) -> &[crate::graph::UiRuntimeInstanceBasisAdmission] {
        &self.runtime_instance_basis_admissions
    }

    pub(crate) fn measurement_inspection_evidence(
        &self,
    ) -> &[crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle] {
        &self.measurement_inspection_evidence
    }

    pub(crate) fn admit_launch(
        &self,
        diagnostic_policy: crate::runtime::WorthUiRuntimeDiagnosticPolicy,
    ) -> Result<super::WorthUiPreparedLaunchAdmission, crate::runtime::WorthUiRuntimeLaunchDenial>
    {
        let (artifact, artifact_digest) =
            self.canonical_artifact.runtime_artifact_authority().ok_or(
                crate::runtime::WorthUiRuntimeLaunchDenial::PreparedApplicationHasNoRuntimeArtifact,
            )?;
        let initial_allocation_commit = self.initial_allocation_commit(artifact_digest)?;
        Ok(super::WorthUiPreparedLaunchAdmission {
            lowering_authority: self.lowering_authority(),
            initial_allocation_commit,
            artifact,
            artifact_digest,
            snapshot_digest: self.capability_snapshot.digest(),
            diagnostic_policy,
            query_binding: self.query_binding_plan.prepare_downstream_state(),
            host_session_plan: self.host_session_plan.clone(),
        })
    }

    pub(crate) fn initial_allocation_commit(
        &self,
        artifact_digest: crate::source::WorthUiArtifactDigest,
    ) -> Result<
        crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommit,
        crate::runtime::WorthUiRuntimeLaunchDenial,
    > {
        let projection =
            crate::runtime::planning::allocation_planning::WorthUiAllocationPlanningProjection::seal(
                crate::runtime::WorthUiRuntimeFrameEpoch::initial(),
                artifact_digest.raw(),
                self.graph_snapshot.authority_identity(),
            );
        crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommit::commit(
            &self.graph_snapshot,
            projection,
        )
        .map_err(|denial| match denial {
            crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommitDenial::CandidateGraphAuthorityMismatch => {
                crate::runtime::WorthUiRuntimeLaunchDenial::InitialAllocationGraphAuthorityMismatch
            }
            crate::runtime::planning::allocation_planning::WorthUiInitialAllocationCommitDenial::ActiveAllocationObligations { node_count } => {
                crate::runtime::WorthUiRuntimeLaunchDenial::InitialAllocationObligationsUnsettled { node_count }
            }
        })
    }

    pub(crate) fn rebuild_derived_indexes(&mut self) {
        self.authored_evidence_index = UiDeclarationAuthoredEvidenceIndex::rebuild(
            &self.declaration_artifacts,
            &self.graph_snapshot,
        );
        let graph_evidence = build_graph_evidence_indexes(
            &self.declaration_artifacts,
            &self.graph_snapshot,
            &self.lifecycle,
        );
        self.graph_node_evidence_index = graph_evidence.node;
        self.graph_aspect_evidence_indexes = graph_evidence.aspect;
    }
}
