use forge_query::facade::{
    ForgeQueryAuthoritativeMutationEvidenceCloseout,
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryRuntime,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimeError, ForgeQueryRuntimeSupportProfile,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::data::authority::{RawWorthTopologyIntent, WorthEntityReference, WorthTopologyMutation};
use crate::data::entities::WorthEntityKind;
use crate::data::relations::WorthRelationKind;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthQueryMutationAdmissionBlocker {
    ExistingIdentityBindingRequired,
    SymbolicCreateReferenceRequired,
    ProjectedNamingWritebackRequired,
    UnsupportedGeometryTruthMutation,
    UnsupportedDiagnosticsTruthMutation,
}

impl WorthQueryMutationAdmissionBlocker {
    pub fn message(&self) -> &'static str {
        match self {
            Self::ExistingIdentityBindingRequired => {
                "query-native lowering still needs an admitted authoritative identity binding for existing truth mutations"
            }
            Self::SymbolicCreateReferenceRequired => {
                "query-native batch authoring still needs admitted symbolic create references for same-batch topology graph construction"
            }
            Self::ProjectedNamingWritebackRequired => {
                "query-native lowering still needs admitted projected naming writeback to author persistent-name truth without a shadow Worth runtime"
            }
            Self::UnsupportedGeometryTruthMutation => {
                "geometry truth mutation is outside the current topology-only query rewrite lane"
            }
            Self::UnsupportedDiagnosticsTruthMutation => {
                "diagnostic truth mutation is outside the current derived-topology query rewrite lane"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthQueryMutationAdmissionReport {
    pub mutation_index: usize,
    pub blocker: WorthQueryMutationAdmissionBlocker,
    pub mutation_summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthQueryMutationAdmission {
    Admitted,
    Blocked(Vec<WorthQueryMutationAdmissionReport>),
}

impl WorthQueryMutationAdmission {
    pub fn is_admitted(&self) -> bool {
        matches!(self, Self::Admitted)
    }

    pub fn blockers(&self) -> &[WorthQueryMutationAdmissionReport] {
        match self {
            Self::Admitted => &[],
            Self::Blocked(blockers) => blockers,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthQueryMutationSupportContract {
    pub query_support_digest: String,
    pub query_closeout_digest: String,
    pub admitted_query_substrate_families: Vec<String>,
    pub blocked_until_invariant_complete_workflow: Vec<String>,
    pub blocked_until_explicit_lowering: Vec<String>,
    pub contract_digest: String,
}

pub fn worth_query_mutation_support_contract(
) -> Result<WorthQueryMutationSupportContract, ForgeQueryRuntimeError> {
    let query_support = public_authoritative_mutation_evidence_support();
    let query_closeout = public_authoritative_mutation_evidence_closeout(&query_support);

    assert!(
        query_support
            .symbolic_target_reference_families()
            .iter()
            .any(|family| family == "same_batch_declared_target"),
        "forge-query authoritative mutation evidence must admit same-batch symbolic target references before worth widens raw topology creation",
    );
    assert!(
        query_support
            .symbolic_aspect_reference_families()
            .iter()
            .any(|family| family == "same_batch_declared_entity_identity"),
        "forge-query authoritative mutation evidence must admit same-batch symbolic aspect entity-identity references before worth widens same-batch topology relation construction",
    );
    assert!(
        query_support
            .existing_truth_binding_families()
            .iter()
            .any(|family| family == "direct_entity_identity"),
        "forge-query authoritative mutation evidence must admit direct existing-truth bindings before worth widens existing-truth updates",
    );
    assert!(
        query_support
            .existing_truth_binding_families()
            .iter()
            .any(|family| family == "direct_relation_identity"),
        "forge-query authoritative mutation evidence must admit direct existing-truth relation bindings before worth widens imported topology relation removal",
    );

    let admitted_query_substrate_families = vec![
        "insert_topology_entity_with_projected_persistent_name".to_string(),
        "insert_topology_relation_with_existing_entity_bindings".to_string(),
        "insert_topology_relation_with_same_batch_symbolic_entity_identity_refs".to_string(),
        "verify_existing_topology_entity_kind".to_string(),
        "verify_existing_topology_relation_shape".to_string(),
        "update_existing_topology_relation_shape_identity_preserving".to_string(),
        "delete_existing_topology_entity".to_string(),
        "delete_existing_topology_relation".to_string(),
    ];
    let blocked_until_invariant_complete_workflow = vec![
        "topology_relation_create_workflows_beyond_face_inner_loop_require_invariant_complete_subgraphs".to_string(),
        "topology_shell_or_wire_membership_workflows_beyond_admitted_full_wire_rehome_connected_wire_split_single_face_two_face_shell_split_and_full_shell_face_set_rehome_require_invariant_complete_owner_rehome_or_shell_subgraphs"
            .to_string(),
        "topology_relation_loop_successor_workflows_beyond_admitted_half_edge_relocation_lanes_require_invariant_complete_topology_update_workflows"
            .to_string(),
    ];
    let blocked_until_explicit_lowering = vec![
        "raw_naming_truth_requires_projected_naming_writeback".to_string(),
        "geometry_truth_outside_topology_lane".to_string(),
        "diagnostics_truth_outside_topology_lane".to_string(),
    ];
    let contract_digest = hash_parts(
        &std::iter::once("worth_query_mutation_support_contract_v1".to_string())
            .chain(std::iter::once(format!(
                "query-support:{}",
                query_support.support_digest()
            )))
            .chain(std::iter::once(format!(
                "query-closeout:{}",
                query_closeout.closeout_digest()
            )))
            .chain(
                admitted_query_substrate_families
                    .iter()
                    .map(|family| format!("admitted:{family}")),
            )
            .chain(
                blocked_until_invariant_complete_workflow
                    .iter()
                    .map(|family| format!("workflow-blocked:{family}")),
            )
            .chain(
                blocked_until_explicit_lowering
                    .iter()
                    .map(|family| format!("blocked:{family}")),
            )
            .collect::<Vec<_>>(),
    );

    Ok(WorthQueryMutationSupportContract {
        query_support_digest: query_support.support_digest().to_string(),
        query_closeout_digest: query_closeout.closeout_digest().to_string(),
        admitted_query_substrate_families,
        blocked_until_invariant_complete_workflow,
        blocked_until_explicit_lowering,
        contract_digest,
    })
}

pub fn admit_worth_query_mutation_batch(
    intent: &RawWorthTopologyIntent,
) -> WorthQueryMutationAdmission {
    let mut blockers = Vec::new();
    for (mutation_index, mutation) in intent.mutations.iter().enumerate() {
        classify_mutation(mutation_index, mutation, &mut blockers);
    }
    if blockers.is_empty() {
        WorthQueryMutationAdmission::Admitted
    } else {
        WorthQueryMutationAdmission::Blocked(blockers)
    }
}

fn classify_mutation(
    mutation_index: usize,
    mutation: &WorthTopologyMutation,
    blockers: &mut Vec<WorthQueryMutationAdmissionReport>,
) {
    match mutation {
        WorthTopologyMutation::CreateEntity { kind, .. } => match kind {
            WorthEntityKind::Topology(_) => {}
            WorthEntityKind::Naming(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::ProjectedNamingWritebackRequired,
            ),
            WorthEntityKind::Geometry(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedGeometryTruthMutation,
            ),
            WorthEntityKind::Diagnostics(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedDiagnosticsTruthMutation,
            ),
        },
        WorthTopologyMutation::CreateRelation {
            kind,
            source,
            target,
            ..
        } => match kind {
            WorthRelationKind::Topology(_) => {
                let _same_batch_symbolic_reference =
                    matches!(source, WorthEntityReference::Created(_))
                        || matches!(target, WorthEntityReference::Created(_));
            }
            WorthRelationKind::Naming(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::ProjectedNamingWritebackRequired,
            ),
            WorthRelationKind::Geometry(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedGeometryTruthMutation,
            ),
            WorthRelationKind::Diagnostics(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedDiagnosticsTruthMutation,
            ),
        },
        WorthTopologyMutation::UpsertEntity { kind, .. } => match kind {
            WorthEntityKind::Topology(_) => {}
            WorthEntityKind::Naming(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::ProjectedNamingWritebackRequired,
            ),
            WorthEntityKind::Geometry(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedGeometryTruthMutation,
            ),
            WorthEntityKind::Diagnostics(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedDiagnosticsTruthMutation,
            ),
        },
        WorthTopologyMutation::UpsertRelation { kind, .. } => match kind {
            WorthRelationKind::Topology(_) => {}
            WorthRelationKind::Naming(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::ProjectedNamingWritebackRequired,
            ),
            WorthRelationKind::Geometry(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedGeometryTruthMutation,
            ),
            WorthRelationKind::Diagnostics(_) => push_blocker(
                blockers,
                mutation_index,
                mutation,
                WorthQueryMutationAdmissionBlocker::UnsupportedDiagnosticsTruthMutation,
            ),
        },
        WorthTopologyMutation::RemoveEntity { .. }
        | WorthTopologyMutation::RemoveRelation { .. } => {}
    }
}

fn push_blocker(
    blockers: &mut Vec<WorthQueryMutationAdmissionReport>,
    mutation_index: usize,
    mutation: &WorthTopologyMutation,
    blocker: WorthQueryMutationAdmissionBlocker,
) {
    if blockers
        .iter()
        .any(|row| row.mutation_index == mutation_index && row.blocker == blocker)
    {
        return;
    }
    blockers.push(WorthQueryMutationAdmissionReport {
        mutation_index,
        blocker,
        mutation_summary: format!("{mutation:?}"),
    });
}

fn hash_parts(parts: &[String]) -> String {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update([0x1f]);
    }
    format!("{:x}", hasher.finalize())
}

fn public_authoritative_mutation_evidence_support() -> ForgeQueryAuthoritativeMutationEvidenceSupport
{
    ForgeQueryRuntime::public_authoritative_mutation_evidence_support_for_posture(
        ForgeQueryRuntimeBackendPosture::Primary,
    )
}

fn public_authoritative_mutation_evidence_closeout(
    _query_support: &ForgeQueryAuthoritativeMutationEvidenceSupport,
) -> ForgeQueryAuthoritativeMutationEvidenceCloseout {
    let support_profile = ForgeQueryRuntimeSupportProfile::bridge_backed(
        "worth-query-mutation-support-contract-live",
        "worth-query-mutation-support-contract-preview",
        "worth-query-mutation-support-contract-inspect",
    );
    ForgeQueryRuntime::public_authoritative_mutation_evidence_closeout_for_support_profile(
        &support_profile,
    )
}
