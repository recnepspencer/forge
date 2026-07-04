use schema::facade::platform::authority::touched_graph_conflict::{
    BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
};
use topology::facade::TopologyDerivedReuseDecisionPosture;
use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

use crate::workload_composition::planner_owned_routing::selected_route::WorthTouchedGraphConflictSelectedRoutePacket;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy {
    MinimalOperationalTruth,
    RichLocalization,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictRichDerivedDiagnosticLocalization {
    touched_closure_digest: String,
    touched_semantic_family_key: String,
    selected_plan_digest: String,
    touched_aspect_count: usize,
    touched_scope_count: usize,
    selected_row_family_identities: Vec<String>,
    triggered_bridge_scopes: Vec<String>,
    compiled_product_reuse_route_packet_identity: String,
    topology_reuse_posture: TopologyDerivedReuseDecisionPosture,
    spatial_reuse_posture: String,
    spatial_reuse_decision_identity_digest: Option<String>,
    spatial_rebuild_denial_identity_digest: Option<String>,
    batch_admission_denial_witness_identity: Option<String>,
    batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
    conflict_independence_denial_witness_identity: Option<String>,
    conflict_independence_denial_witness_kind: Option<ConflictIndependencePlannerRouteWitnessKind>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictDerivedDiagnosticProjection {
    artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    selected_route_identity_digest: String,
    decision_trace_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    topology_reuse_posture: TopologyDerivedReuseDecisionPosture,
    spatial_reuse_posture: EvidenceLookupReuseDecisionPosture,
    selected_witness_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
    spatial_reuse_decision_identity_digest: Option<String>,
    spatial_rebuild_denial_identity_digest: Option<String>,
    batch_admission_denial_witness_identity_digest: Option<String>,
    batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
    conflict_independence_denial_witness_identity_digest: Option<String>,
    conflict_independence_denial_witness_kind: Option<ConflictIndependencePlannerRouteWitnessKind>,
    rich_localization: Option<WorthTouchedGraphConflictRichDerivedDiagnosticLocalization>,
}

impl WorthTouchedGraphConflictDerivedDiagnosticProjection {
    pub(crate) fn from_selected_route_packet(
        packet: &WorthTouchedGraphConflictSelectedRoutePacket,
        artifact_policy: WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
        rich_localization: Option<WorthTouchedGraphConflictRichDerivedDiagnosticLocalization>,
    ) -> Self {
        Self {
            artifact_policy,
            selected_route_identity_digest: packet.selected_route_identity_digest().to_string(),
            decision_trace_identity_digest: packet.decision_trace_identity_digest().to_string(),
            selected_family_identity: packet.selected_family_identity().to_string(),
            selected_product_identity_digest: packet.selected_product_identity_digest().to_string(),
            topology_reuse_posture: packet.topology_reuse_posture(),
            spatial_reuse_posture: packet.spatial_reuse_posture(),
            selected_witness_identity_digest: packet
                .selected_witness_identity_digest()
                .map(str::to_string),
            rebuild_denial_identity_digest: packet
                .rebuild_denial_identity_digest()
                .map(str::to_string),
            spatial_reuse_decision_identity_digest: packet
                .spatial_reuse_decision_identity_digest()
                .map(str::to_string),
            spatial_rebuild_denial_identity_digest: packet
                .spatial_rebuild_denial_identity_digest()
                .map(str::to_string),
            batch_admission_denial_witness_identity_digest: packet
                .batch_admission_denial_witness_identity()
                .map(str::to_string),
            batch_admission_denial_witness_kind: packet.batch_admission_denial_witness_kind(),
            conflict_independence_denial_witness_identity_digest: packet
                .conflict_independence_denial_witness_identity()
                .map(str::to_string),
            conflict_independence_denial_witness_kind: packet
                .conflict_independence_denial_witness_kind(),
            rich_localization,
        }
    }

    pub const fn artifact_policy(
        &self,
    ) -> WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy {
        self.artifact_policy
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn decision_trace_identity_digest(&self) -> &str {
        &self.decision_trace_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub const fn topology_reuse_posture(&self) -> TopologyDerivedReuseDecisionPosture {
        self.topology_reuse_posture
    }

    pub const fn spatial_reuse_posture(&self) -> EvidenceLookupReuseDecisionPosture {
        self.spatial_reuse_posture
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.spatial_reuse_decision_identity_digest.as_deref()
    }

    pub fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.spatial_rebuild_denial_identity_digest.as_deref()
    }

    pub fn batch_admission_denial_witness_identity_digest(&self) -> Option<&str> {
        self.batch_admission_denial_witness_identity_digest
            .as_deref()
    }

    pub const fn batch_admission_denial_witness_kind(
        &self,
    ) -> Option<BatchAdmissionPlannerRouteWitnessKind> {
        self.batch_admission_denial_witness_kind
    }

    pub fn conflict_independence_denial_witness_identity_digest(&self) -> Option<&str> {
        self.conflict_independence_denial_witness_identity_digest
            .as_deref()
    }

    pub const fn conflict_independence_denial_witness_kind(
        &self,
    ) -> Option<ConflictIndependencePlannerRouteWitnessKind> {
        self.conflict_independence_denial_witness_kind
    }

    pub fn rich_localization(
        &self,
    ) -> Option<&WorthTouchedGraphConflictRichDerivedDiagnosticLocalization> {
        self.rich_localization.as_ref()
    }
}

impl WorthTouchedGraphConflictRichDerivedDiagnosticLocalization {
    pub(crate) fn new(
        touched_closure_digest: String,
        touched_semantic_family_key: String,
        selected_plan_digest: String,
        touched_aspect_count: usize,
        touched_scope_count: usize,
        selected_row_family_identities: Vec<String>,
        triggered_bridge_scopes: Vec<String>,
        compiled_product_reuse_route_packet_identity: String,
        topology_reuse_posture: TopologyDerivedReuseDecisionPosture,
        spatial_reuse_posture: String,
        spatial_reuse_decision_identity_digest: Option<String>,
        spatial_rebuild_denial_identity_digest: Option<String>,
        batch_admission_denial_witness_identity: Option<String>,
        batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
        conflict_independence_denial_witness_identity: Option<String>,
        conflict_independence_denial_witness_kind: Option<
            ConflictIndependencePlannerRouteWitnessKind,
        >,
    ) -> Self {
        Self {
            touched_closure_digest,
            touched_semantic_family_key,
            selected_plan_digest,
            touched_aspect_count,
            touched_scope_count,
            selected_row_family_identities,
            triggered_bridge_scopes,
            compiled_product_reuse_route_packet_identity,
            topology_reuse_posture,
            spatial_reuse_posture,
            spatial_reuse_decision_identity_digest,
            spatial_rebuild_denial_identity_digest,
            batch_admission_denial_witness_identity,
            batch_admission_denial_witness_kind,
            conflict_independence_denial_witness_identity,
            conflict_independence_denial_witness_kind,
        }
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn touched_semantic_family_key(&self) -> &str {
        &self.touched_semantic_family_key
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub const fn touched_scope_count(&self) -> usize {
        self.touched_scope_count
    }

    pub fn selected_row_family_identities(&self) -> &[String] {
        &self.selected_row_family_identities
    }

    pub fn triggered_bridge_scopes(&self) -> &[String] {
        &self.triggered_bridge_scopes
    }

    pub fn compiled_product_reuse_route_packet_identity(&self) -> Option<&str> {
        Some(&self.compiled_product_reuse_route_packet_identity)
    }

    pub fn topology_reuse_posture(&self) -> Option<TopologyDerivedReuseDecisionPosture> {
        Some(self.topology_reuse_posture)
    }

    pub fn spatial_reuse_posture(&self) -> Option<&str> {
        Some(&self.spatial_reuse_posture)
    }

    pub fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.spatial_reuse_decision_identity_digest.as_deref()
    }

    pub fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.spatial_rebuild_denial_identity_digest.as_deref()
    }

    pub fn batch_admission_denial_witness_identity(&self) -> Option<&str> {
        self.batch_admission_denial_witness_identity.as_deref()
    }

    pub fn batch_admission_denial_witness_kind(
        &self,
    ) -> Option<BatchAdmissionPlannerRouteWitnessKind> {
        self.batch_admission_denial_witness_kind
    }

    pub fn conflict_independence_denial_witness_identity(&self) -> Option<&str> {
        self.conflict_independence_denial_witness_identity
            .as_deref()
    }

    pub fn conflict_independence_denial_witness_kind(
        &self,
    ) -> Option<ConflictIndependencePlannerRouteWitnessKind> {
        self.conflict_independence_denial_witness_kind
    }
}
