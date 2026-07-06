use std::collections::BTreeMap;

use crate::capability::{MosaicResizePermission, MosaicSizingContractId};
use crate::declaration::stable_text_digest;
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
                node_resize_contract_id(node),
                node_resize_permission(node),
                node_resize_shape_digest(node),
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

fn node_resize_contract_id(node: &WorthUiArtifactNode) -> Option<MosaicSizingContractId> {
    node_structure(node)
        .and_then(|structure| structure.unique_root_sizing_contract_id().ok())
        .flatten()
}

fn node_resize_permission(node: &WorthUiArtifactNode) -> Option<MosaicResizePermission> {
    let structure = node_structure(node)?;
    let root_region = structure.root_regions().first()?;
    let (_, descriptor) = root_region.sizing_contract()?;
    descriptor.resize_permission().cloned()
}

fn node_resize_shape_digest(node: &WorthUiArtifactNode) -> Option<u64> {
    let structure = node_structure(node)?;
    Some(structure_shape_digest(structure))
}

fn node_structure(
    node: &WorthUiArtifactNode,
) -> Option<&crate::source::WorthUiMosaicStructureFacts> {
    match node {
        WorthUiArtifactNode::Component(node) => Some(node.structure()),
        WorthUiArtifactNode::Surface(node) => Some(node.structure()),
        WorthUiArtifactNode::Binding(node) => Some(node.structure()),
        WorthUiArtifactNode::Import(_) | WorthUiArtifactNode::Token(_) => None,
    }
}

fn structure_shape_digest(structure: &crate::source::WorthUiMosaicStructureFacts) -> u64 {
    let mut digest = stable_text_digest("worth-ui.runtime.resize-shape");
    let mut region_count = 0_u64;
    let mut mount_count = 0_u64;
    let mut max_depth = 0_u64;
    for root_region in structure.root_regions() {
        fold_region_shape(
            root_region,
            1,
            &mut digest,
            &mut region_count,
            &mut mount_count,
            &mut max_depth,
        );
    }
    digest ^= (structure.root_regions().len() as u64).rotate_left(7);
    digest ^= region_count.rotate_left(13);
    digest ^= mount_count.rotate_left(19);
    digest ^ max_depth.rotate_left(23)
}

fn fold_region_shape(
    region: &crate::source::WorthUiMosaicRegionFacts,
    depth: u64,
    digest: &mut u64,
    region_count: &mut u64,
    mount_count: &mut u64,
    max_depth: &mut u64,
) {
    *digest ^= stable_text_digest(region.region().id().as_str()).rotate_left(3);
    *region_count += 1;
    *mount_count += region.mounts().len() as u64;
    *max_depth = (*max_depth).max(depth);
    if let Some((contract, descriptor)) = region.sizing_contract() {
        *digest ^= stable_text_digest(contract.id().as_str()).rotate_left(5);
        *digest ^= resize_permission_digest(descriptor.resize_permission()).rotate_left(7);
    }
    if let Some((slot, descriptor)) = region.state_slot() {
        *digest ^= stable_text_digest(slot.id().as_str()).rotate_left(11);
        *digest ^= state_slot_kind_digest(descriptor.kind()).rotate_left(13);
    }
    for mount in region.mounts() {
        *digest ^= stable_text_digest(mount.surface().id().as_str()).rotate_left(17);
        if let Some((slot, descriptor)) = mount.state_slot() {
            *digest ^= stable_text_digest(slot.id().as_str()).rotate_left(19);
            *digest ^= state_slot_kind_digest(descriptor.kind()).rotate_left(23);
        }
    }
    for child in region.child_regions() {
        fold_region_shape(
            child,
            depth + 1,
            digest,
            region_count,
            mount_count,
            max_depth,
        );
    }
}

fn resize_permission_digest(permission: Option<&MosaicResizePermission>) -> u64 {
    let basis = match permission {
        Some(MosaicResizePermission::FixedByRuntime) => "fixed_by_runtime",
        Some(MosaicResizePermission::UserResizable) => "user_resizable",
        Some(MosaicResizePermission::ContentDriven) => "content_driven",
        Some(MosaicResizePermission::MissingForDiagnostics) => "missing",
        None => "none",
    };
    stable_text_digest(basis)
}

fn state_slot_kind_digest(kind: &crate::capability::MosaicStateSlotKind) -> u64 {
    let basis = match kind {
        crate::capability::MosaicStateSlotKind::SplitterPosition => "splitter_position",
        crate::capability::MosaicStateSlotKind::ActiveStackItem => "active_stack_item",
        crate::capability::MosaicStateSlotKind::RegionVisibility => "region_visibility",
        crate::capability::MosaicStateSlotKind::CollapsedPosture => "collapsed_posture",
        crate::capability::MosaicStateSlotKind::PinnedPosture => "pinned_posture",
        crate::capability::MosaicStateSlotKind::ScrollPosition => "scroll_position",
        crate::capability::MosaicStateSlotKind::FocusedRegion => "focused_region",
        crate::capability::MosaicStateSlotKind::ActivePrimarySurface => "active_primary_surface",
        crate::capability::MosaicStateSlotKind::ActiveAuxiliarySurface => {
            "active_auxiliary_surface"
        }
        crate::capability::MosaicStateSlotKind::SelectionToken => "selection_token",
        crate::capability::MosaicStateSlotKind::DraftInputState => "draft_input_state",
    };
    stable_text_digest(basis)
}
