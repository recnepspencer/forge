use super::generation_identity::WorthUiPreparedGenerationIdentityInput;
use super::lowering_authority::WorthUiPreparedApplicationLoweringInput;
use super::{
    WorthUiHostSessionPlan, WorthUiPreparedApplicationArtifact,
    WorthUiPreparedApplicationArtifactPosture, WorthUiPreparedApplicationGenerationIdentity,
    WorthUiPreparedApplicationLoweringAuthority, WorthUiPreparedDeclarationSourceIdentity,
    WorthUiPreparedVisualTraceSource,
};
use crate::declaration::{UiDeclarationArtifact, UiDeclarationAuthoredEvidenceIndex};
use crate::facade::lifecycle::{build_graph_evidence_indexes, WorthUiFacadeLifecycleBootstrap};
use crate::facade::registry::snapshot::CapabilitySnapshot;
use crate::graph::{UiGraphAspectEvidenceIndexes, UiGraphNodeEvidenceIndex, UiGraphSnapshot};
use std::rc::Rc;

mod graph_successor;
pub(crate) use graph_successor::{
    WorthUiPreparedApplicationGraphSuccessor, WorthUiPreparedApplicationGraphSuccessorDenial,
};

pub(crate) struct WorthUiPreparedApplicationAuthorityInput {
    pub(crate) capability_snapshot: Rc<CapabilitySnapshot>,
    pub(crate) canonical_artifact: WorthUiPreparedApplicationArtifact,
    pub(crate) declaration_source_identity: WorthUiPreparedDeclarationSourceIdentity,
    pub(crate) semantic_handoff: crate::runtime::WorthUiSemanticHandoffEvidence,
    pub(crate) declaration_artifacts: Vec<UiDeclarationArtifact>,
    pub(crate) graph_snapshot: UiGraphSnapshot,
    pub(crate) lifecycle: WorthUiFacadeLifecycleBootstrap,
    pub(crate) query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    pub(crate) host_session_plan: WorthUiHostSessionPlan,
    pub(crate) visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
    pub(crate) runtime_instance_basis_admissions:
        Box<[crate::graph::UiRuntimeInstanceBasisAdmission]>,
    pub(crate) measurement_inspection_evidence:
        Box<[crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle]>,
}

struct WorthUiPreparedApplicationAuthorities {
    generation_identity: WorthUiPreparedApplicationGenerationIdentity,
    lowering_authority: WorthUiPreparedApplicationLoweringAuthority,
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
    semantic_handoff: crate::runtime::WorthUiSemanticHandoffEvidence,
    declaration_artifacts: Rc<[UiDeclarationArtifact]>,
    graph_snapshot: UiGraphSnapshot,
    lifecycle: WorthUiFacadeLifecycleBootstrap,
    authored_evidence_index: Rc<UiDeclarationAuthoredEvidenceIndex>,
    graph_node_evidence_index: Rc<UiGraphNodeEvidenceIndex>,
    visual_trace_source: WorthUiPreparedVisualTraceSource,
    graph_aspect_evidence_indexes: UiGraphAspectEvidenceIndexes,
    query_binding_plan: worth_ui_query_binding::WorthUiQueryBindingPlan,
    host_session_plan: WorthUiHostSessionPlan,
    visual_inspection_policy: worth_ui_inspection::UiVisualInspectionPolicy,
    runtime_instance_basis_admissions: Box<[crate::graph::UiRuntimeInstanceBasisAdmission]>,
    measurement_inspection_evidence:
        Box<[crate::facade::inspection_bridge::UiMeasurementInspectionEvidenceBundle]>,
}

impl WorthUiPreparedApplicationAuthority {
    pub(crate) fn seal(input: WorthUiPreparedApplicationAuthorityInput) -> Self {
        let authorities = derive_prepared_application_authorities(&input);
        let WorthUiPreparedApplicationAuthorityInput {
            capability_snapshot,
            canonical_artifact,
            declaration_source_identity,
            semantic_handoff,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
            query_binding_plan,
            host_session_plan,
            visual_inspection_policy,
            runtime_instance_basis_admissions,
            measurement_inspection_evidence,
        } = input;
        let declaration_artifacts: Rc<[UiDeclarationArtifact]> = declaration_artifacts.into();
        let authored_evidence_index = Rc::new(UiDeclarationAuthoredEvidenceIndex::rebuild(
            declaration_artifacts.as_ref(),
            &graph_snapshot,
        ));
        let graph_evidence = build_graph_evidence_indexes(
            declaration_artifacts.as_ref(),
            &graph_snapshot,
            &lifecycle,
        );
        let graph_node_evidence_index = Rc::new(graph_evidence.node);
        let visual_trace_source = WorthUiPreparedVisualTraceSource::new(
            authorities.generation_identity.clone(),
            Rc::clone(&declaration_artifacts),
            Rc::clone(&authored_evidence_index),
            Rc::clone(&graph_node_evidence_index),
        );
        Self {
            generation_identity: authorities.generation_identity,
            lowering_authority: authorities.lowering_authority,
            capability_snapshot,
            canonical_artifact,
            declaration_source_identity,
            semantic_handoff,
            declaration_artifacts,
            graph_snapshot,
            lifecycle,
            authored_evidence_index,
            graph_node_evidence_index,
            visual_trace_source,
            graph_aspect_evidence_indexes: graph_evidence.aspect,
            query_binding_plan,
            host_session_plan,
            visual_inspection_policy,
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

    pub fn semantic_handoff(&self) -> &crate::runtime::WorthUiSemanticHandoffEvidence {
        &self.semantic_handoff
    }

    pub fn application_artifact_posture(&self) -> WorthUiPreparedApplicationArtifactPosture {
        self.canonical_artifact.posture()
    }

    pub fn capabilities(&self) -> &CapabilitySnapshot {
        self.capability_snapshot.as_ref()
    }

    pub fn declaration_artifacts(&self) -> &[UiDeclarationArtifact] {
        self.declaration_artifacts.as_ref()
    }

    pub fn host_session_plan(&self) -> &WorthUiHostSessionPlan {
        &self.host_session_plan
    }

    pub const fn visual_inspection_policy(&self) -> worth_ui_inspection::UiVisualInspectionPolicy {
        self.visual_inspection_policy
    }

    pub(crate) fn graph_snapshot(&self) -> &UiGraphSnapshot {
        &self.graph_snapshot
    }

    pub(crate) fn lifecycle(&self) -> &WorthUiFacadeLifecycleBootstrap {
        &self.lifecycle
    }

    pub(crate) fn authored_evidence_index(&self) -> &UiDeclarationAuthoredEvidenceIndex {
        self.authored_evidence_index.as_ref()
    }

    pub(crate) fn graph_node_evidence_index(&self) -> &UiGraphNodeEvidenceIndex {
        self.graph_node_evidence_index.as_ref()
    }

    pub(crate) fn visual_trace_source(&self) -> WorthUiPreparedVisualTraceSource {
        self.visual_trace_source.clone()
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
    ) -> crate::runtime::WorthUiReplacementCandidateBasis {
        self.canonical_artifact.candidate_basis()
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
        let (artifact, artifact_digest) = self.canonical_artifact.runtime_artifact_authority();
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
        self.authored_evidence_index = Rc::new(UiDeclarationAuthoredEvidenceIndex::rebuild(
            self.declaration_artifacts.as_ref(),
            &self.graph_snapshot,
        ));
        let graph_evidence = build_graph_evidence_indexes(
            self.declaration_artifacts.as_ref(),
            &self.graph_snapshot,
            &self.lifecycle,
        );
        self.graph_node_evidence_index = Rc::new(graph_evidence.node);
        self.graph_aspect_evidence_indexes = graph_evidence.aspect;
        self.visual_trace_source = WorthUiPreparedVisualTraceSource::new(
            self.generation_identity.clone(),
            Rc::clone(&self.declaration_artifacts),
            Rc::clone(&self.authored_evidence_index),
            Rc::clone(&self.graph_node_evidence_index),
        );
    }
}

fn derive_prepared_application_authorities(
    input: &WorthUiPreparedApplicationAuthorityInput,
) -> WorthUiPreparedApplicationAuthorities {
    let generation_identity = WorthUiPreparedApplicationGenerationIdentity::derive(
        WorthUiPreparedGenerationIdentityInput {
            capability_snapshot: input.capability_snapshot.digest(),
            canonical_artifact: input.canonical_artifact.identity(),
            declaration_source: input.declaration_source_identity.clone(),
            semantic_package: input.semantic_handoff.identity().clone(),
            graph_authority_digest: input.graph_snapshot.authority_digest(),
            query_binding_plan: &input.query_binding_plan,
            host_session_plan: &input.host_session_plan,
            visual_inspection_policy: input.visual_inspection_policy,
        },
    );
    let lowering_authority = WorthUiPreparedApplicationLoweringAuthority::seal(
        WorthUiPreparedApplicationLoweringInput {
            generation_identity: generation_identity.clone(),
            source_candidate_basis: input.canonical_artifact.candidate_basis(),
            source_artifact_authority: input.canonical_artifact.runtime_artifact_authority().0,
            graph_authority_identity: input.graph_snapshot.authority_identity(),
            capability_snapshot: Rc::clone(&input.capability_snapshot),
            query_binding_plan: input.query_binding_plan.clone(),
            host_session_plan: input.host_session_plan.clone(),
        },
    );
    WorthUiPreparedApplicationAuthorities {
        generation_identity,
        lowering_authority,
    }
}
