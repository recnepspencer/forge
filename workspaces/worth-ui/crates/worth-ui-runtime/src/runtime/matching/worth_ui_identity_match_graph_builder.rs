use std::collections::BTreeMap;

use crate::runtime::active::WorthUiActiveArtifact;
use crate::runtime::{
    WorthUiAdmittedReplacementCandidate, WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial,
    WorthUiIdentityMatchEdge, WorthUiIdentityMatchGraph, WorthUiIdentityMatchNode,
    WorthUiIdentityMatchNodeSide, WorthUiIdentityMatchReport, WorthUiMovedNodeIdentity,
    WorthUiRepeatedTemplateIdentity, WorthUiRuntimeImpactNarrowing,
};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactIdentitySeed, WorthUiArtifactNode,
    WorthUiDurableStateEligibility,
};

#[derive(Clone, Debug, Default)]
pub struct WorthUiIdentityMatchGraphBuilder;

#[derive(Clone, Debug)]
struct IndexedIdentityNode {
    node: WorthUiIdentityMatchNode,
}

impl WorthUiIdentityMatchGraphBuilder {
    pub(crate) fn build(
        active_artifact: &WorthUiActiveArtifact,
        narrowing: &WorthUiRuntimeImpactNarrowing,
        admitted: &WorthUiAdmittedReplacementCandidate,
    ) -> Result<WorthUiIdentityMatchReport, WorthUiIdentityMatchDenial> {
        let mut counters = WorthUiIdentityMatchCounters::default();
        reject_mismatched_active_basis(active_artifact, narrowing, counters)?;
        reject_mismatched_candidate(narrowing, admitted, counters)?;
        reject_changed_admission_receipts(admitted, counters)?;

        let active_index = index_artifact_nodes(
            active_artifact.artifact(),
            WorthUiIdentityMatchNodeSide::Active,
            &mut counters,
        )?;
        let candidate_index = index_artifact_nodes(
            admitted.artifact_bundle().artifact(),
            WorthUiIdentityMatchNodeSide::Candidate,
            &mut counters,
        )?;
        let graph = build_match_graph(active_index, candidate_index, counters)?;

        Ok(WorthUiIdentityMatchReport::new(
            narrowing.active_artifact_digest(),
            narrowing.candidate_artifact_digest(),
            graph,
        ))
    }
}

fn reject_mismatched_active_basis(
    active_artifact: &WorthUiActiveArtifact,
    narrowing: &WorthUiRuntimeImpactNarrowing,
    counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    let active_artifact_digest = active_artifact.digest().raw();
    if narrowing.active_artifact_digest() == active_artifact_digest {
        Ok(())
    } else {
        Err(WorthUiIdentityMatchDenial::NarrowingActiveBasisMismatch {
            narrowing_active_artifact_digest: narrowing.active_artifact_digest(),
            active_artifact_digest,
            counters,
        })
    }
}

fn reject_mismatched_candidate(
    narrowing: &WorthUiRuntimeImpactNarrowing,
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    let admitted_candidate_artifact_digest = admitted.artifact_bundle().artifact_digest().raw();
    if narrowing.candidate_artifact_digest() == admitted_candidate_artifact_digest {
        Ok(())
    } else {
        Err(WorthUiIdentityMatchDenial::NarrowingCandidateMismatch {
            narrowing_candidate_artifact_digest: narrowing.candidate_artifact_digest(),
            admitted_candidate_artifact_digest,
            counters,
        })
    }
}

fn reject_changed_admission_receipts(
    admitted: &WorthUiAdmittedReplacementCandidate,
    counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    admitted
        .verify_receipts_unchanged()
        .map_err(|_| WorthUiIdentityMatchDenial::AdmissionReceiptChanged { counters })
}

fn index_artifact_nodes(
    artifact: &WorthUiArtifact,
    side: WorthUiIdentityMatchNodeSide,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<BTreeMap<String, IndexedIdentityNode>, WorthUiIdentityMatchDenial> {
    let mut index = BTreeMap::new();
    for module_id in artifact.module_ids() {
        let Some(module) = artifact.module(module_id) else {
            continue;
        };
        for node in module.nodes() {
            match side {
                WorthUiIdentityMatchNodeSide::Active => counters.record_active_node_indexed(),
                WorthUiIdentityMatchNodeSide::Candidate => counters.record_candidate_node_indexed(),
            }
            let identity_seed = node_identity_seed(node);
            counters.record_stable_seed_lookup();
            if !identity_seed.is_stable() {
                continue;
            }
            reject_position_only_repeated_template_identity(node, identity_seed, side, *counters)?;
            let match_node = WorthUiIdentityMatchNode::new(
                side,
                node.handle().clone(),
                identity_seed.basis().to_owned(),
                node.authored_provenance_digest(),
                identity_seed.is_stable(),
                durable_state_is_eligible(node_durable_state_eligibility(node)),
            );
            insert_indexed_identity_node(
                &mut index,
                side,
                identity_seed.basis(),
                identity_seed.basis().to_owned(),
                IndexedIdentityNode {
                    node: match_node.clone(),
                },
                counters,
            )?;
        }
    }
    Ok(index)
}

fn reject_position_only_repeated_template_identity(
    node: &WorthUiArtifactNode,
    identity_seed: &WorthUiArtifactIdentitySeed,
    side: WorthUiIdentityMatchNodeSide,
    mut counters: WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    if WorthUiRepeatedTemplateIdentity::is_position_only(identity_seed.basis()) {
        match side {
            WorthUiIdentityMatchNodeSide::Active => counters.record_duplicate_active_identity(),
            WorthUiIdentityMatchNodeSide::Candidate => {
                counters.record_duplicate_candidate_identity();
            }
        }
        Err(
            WorthUiIdentityMatchDenial::PositionOnlyRepeatedTemplateIdentity {
                identity_basis: identity_seed.basis().to_owned(),
                node_summary: format!(
                    "{:?}:{}:{}",
                    node.handle().kind(),
                    node.handle().module_id().as_str(),
                    node.handle().node_index()
                ),
                counters,
            },
        )
    } else {
        Ok(())
    }
}

fn duplicate_identity_denial<T>(
    side: WorthUiIdentityMatchNodeSide,
    identity_basis: String,
    first_node_summary: String,
    second_node_summary: String,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<T, WorthUiIdentityMatchDenial> {
    match side {
        WorthUiIdentityMatchNodeSide::Active => {
            counters.record_duplicate_active_identity();
            Err(WorthUiIdentityMatchDenial::DuplicateActiveIdentity {
                identity_basis,
                first_node_summary,
                second_node_summary,
                counters: *counters,
            })
        }
        WorthUiIdentityMatchNodeSide::Candidate => {
            counters.record_duplicate_candidate_identity();
            Err(WorthUiIdentityMatchDenial::DuplicateCandidateIdentity {
                identity_basis,
                first_node_summary,
                second_node_summary,
                counters: *counters,
            })
        }
    }
}

fn insert_indexed_identity_node(
    index: &mut BTreeMap<String, IndexedIdentityNode>,
    side: WorthUiIdentityMatchNodeSide,
    identity_basis: &str,
    owned_identity_basis: String,
    indexed_node: IndexedIdentityNode,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    if let Some(previous) = index.get(identity_basis) {
        reject_same_side_identity_kind_mismatch(
            side,
            identity_basis,
            previous,
            &indexed_node.node,
            counters,
        )?;
        return duplicate_identity_denial(
            side,
            owned_identity_basis,
            previous.node.node_summary(),
            indexed_node.node.node_summary(),
            counters,
        );
    }

    index.insert(owned_identity_basis, indexed_node);
    Ok(())
}

fn reject_same_side_identity_kind_mismatch(
    side: WorthUiIdentityMatchNodeSide,
    identity_basis: &str,
    previous: &IndexedIdentityNode,
    next: &WorthUiIdentityMatchNode,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    if previous.node.kind() == next.kind() {
        return Ok(());
    }

    counters.record_identity_kind_mismatch();
    match side {
        WorthUiIdentityMatchNodeSide::Active => {
            Err(WorthUiIdentityMatchDenial::ActiveIdentityKindMismatch {
                identity_basis: identity_basis.to_owned(),
                first_kind: previous.node.kind(),
                second_kind: next.kind(),
                first_node_summary: previous.node.node_summary(),
                second_node_summary: next.node_summary(),
                counters: *counters,
            })
        }
        WorthUiIdentityMatchNodeSide::Candidate => {
            Err(WorthUiIdentityMatchDenial::CandidateIdentityKindMismatch {
                identity_basis: identity_basis.to_owned(),
                first_kind: previous.node.kind(),
                second_kind: next.kind(),
                first_node_summary: previous.node.node_summary(),
                second_node_summary: next.node_summary(),
                counters: *counters,
            })
        }
    }
}

fn build_match_graph(
    active_index: BTreeMap<String, IndexedIdentityNode>,
    candidate_index: BTreeMap<String, IndexedIdentityNode>,
    mut counters: WorthUiIdentityMatchCounters,
) -> Result<WorthUiIdentityMatchGraph, WorthUiIdentityMatchDenial> {
    let mut matches = Vec::new();
    let mut repeated_template_identities = Vec::new();
    let mut moved_node_identities = Vec::new();

    for (identity_basis, active) in &active_index {
        let Some(candidate) = candidate_index.get(identity_basis) else {
            counters.record_unmatched_active();
            continue;
        };
        reject_identity_kind_mismatch(identity_basis, active, candidate, &mut counters)?;
        counters.record_match_emitted();
        matches.push(WorthUiIdentityMatchEdge::new(
            active.node.handle().clone(),
            candidate.node.handle().clone(),
            identity_basis.to_owned(),
        ));
        if active.node.handle() != candidate.node.handle() {
            moved_node_identities.push(WorthUiMovedNodeIdentity::new(
                active.node.handle().clone(),
                candidate.node.handle().clone(),
                identity_basis.to_owned(),
            ));
        }
        if let Some(repeated) = WorthUiRepeatedTemplateIdentity::from_identity_basis(identity_basis)
        {
            repeated_template_identities.push(repeated);
        }
    }

    for identity_basis in candidate_index.keys() {
        if !active_index.contains_key(identity_basis) {
            counters.record_unmatched_candidate();
        }
    }

    Ok(WorthUiIdentityMatchGraph::new(
        active_index
            .into_values()
            .map(|indexed| indexed.node)
            .collect(),
        candidate_index
            .into_values()
            .map(|indexed| indexed.node)
            .collect(),
        matches,
        repeated_template_identities,
        moved_node_identities,
        counters,
    ))
}

fn reject_identity_kind_mismatch(
    identity_basis: &str,
    active: &IndexedIdentityNode,
    candidate: &IndexedIdentityNode,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<(), WorthUiIdentityMatchDenial> {
    if active.node.kind() == candidate.node.kind() {
        return Ok(());
    }

    counters.record_identity_kind_mismatch();
    Err(WorthUiIdentityMatchDenial::IdentityKindMismatch {
        identity_basis: identity_basis.to_owned(),
        active_kind: active.node.kind(),
        candidate_kind: candidate.node.kind(),
        active_node_summary: active.node.node_summary(),
        candidate_node_summary: candidate.node.node_summary(),
        counters: *counters,
    })
}

fn node_identity_seed(node: &WorthUiArtifactNode) -> &WorthUiArtifactIdentitySeed {
    match node {
        WorthUiArtifactNode::Import(node) => node.identity_seed(),
        WorthUiArtifactNode::Component(node) => node.identity_seed(),
        WorthUiArtifactNode::Surface(node) => node.identity_seed(),
        WorthUiArtifactNode::Binding(node) => node.identity_seed(),
        WorthUiArtifactNode::Token(node) => node.identity_seed(),
    }
}

fn node_durable_state_eligibility(node: &WorthUiArtifactNode) -> &WorthUiDurableStateEligibility {
    match node {
        WorthUiArtifactNode::Import(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Component(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Surface(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Binding(node) => node.durable_state_eligibility(),
        WorthUiArtifactNode::Token(node) => node.durable_state_eligibility(),
    }
}

fn durable_state_is_eligible(eligibility: &WorthUiDurableStateEligibility) -> bool {
    matches!(eligibility, WorthUiDurableStateEligibility::Eligible { .. })
}
