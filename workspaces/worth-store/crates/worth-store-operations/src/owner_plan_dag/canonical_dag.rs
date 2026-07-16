use std::collections::{BTreeMap, BTreeSet};

use sha2::{Digest, Sha256};
use worth_proof::{CanonicalVec, NonEmpty, UniqueVec};

use super::topological_order::first_irreversible_node_in_execution_order;
use super::{
    OwnerPlanAccess, OwnerPlanEffect, OwnerPlanExecutionStage, OwnerPlanFootprint, OwnerPlanNode,
    OwnerPlanNodeIdentity, StoreOwnerKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct OwnerPlanPrerequisite {
    prerequisite: OwnerPlanNodeIdentity,
    dependent: OwnerPlanNodeIdentity,
    durability_barrier: bool,
}

impl OwnerPlanPrerequisite {
    pub(crate) const fn new(
        prerequisite: OwnerPlanNodeIdentity,
        dependent: OwnerPlanNodeIdentity,
        durability_barrier: bool,
    ) -> Self {
        Self {
            prerequisite,
            dependent,
            durability_barrier,
        }
    }

    pub(super) const fn prerequisite(self) -> OwnerPlanNodeIdentity {
        self.prerequisite
    }

    pub(super) const fn dependent(self) -> OwnerPlanNodeIdentity {
        self.dependent
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnerPlanDagDenial {
    Empty,
    DuplicateNodeIdentity,
    DuplicateEdge,
    UnknownEdgeEndpoint,
    SelfDependency,
    Cycle,
    AmbiguousOverlappingMutation,
    AllocationFailed,
    WorkEstimateOverflow,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CanonicalOwnerPlanDagExplanation {
    plan_fingerprint: [u8; 32],
    node_count: u64,
    edge_count: u64,
    estimated_work_units: u64,
    first_irreversible_node: Option<OwnerPlanNodeIdentity>,
    nodes: Vec<OwnerPlanNodeExplanation>,
    prerequisites: Vec<OwnerPlanPrerequisiteExplanation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnerPlanNodeExplanation {
    identity: OwnerPlanNodeIdentity,
    owner: StoreOwnerKind,
    effect: OwnerPlanEffect,
    stage: OwnerPlanExecutionStage,
    footprint: OwnerPlanFootprint,
    estimated_work_units: u64,
    irreversible: bool,
    access: OwnerPlanAccess,
    expected_receipt_fingerprint: [u8; 32],
}

impl OwnerPlanNodeExplanation {
    pub const fn identity(self) -> OwnerPlanNodeIdentity {
        self.identity
    }
    pub const fn owner(self) -> StoreOwnerKind {
        self.owner
    }
    pub const fn effect(self) -> OwnerPlanEffect {
        self.effect
    }
    pub const fn stage(self) -> OwnerPlanExecutionStage {
        self.stage
    }
    pub const fn footprint(self) -> OwnerPlanFootprint {
        self.footprint
    }
    pub const fn estimated_work_units(self) -> u64 {
        self.estimated_work_units
    }
    pub const fn irreversible(self) -> bool {
        self.irreversible
    }
    pub const fn access(self) -> OwnerPlanAccess {
        self.access
    }
    pub const fn expected_receipt_fingerprint(self) -> [u8; 32] {
        self.expected_receipt_fingerprint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnerPlanPrerequisiteExplanation {
    prerequisite: OwnerPlanNodeIdentity,
    dependent: OwnerPlanNodeIdentity,
    durability_barrier: bool,
}

impl OwnerPlanPrerequisiteExplanation {
    pub const fn prerequisite(self) -> OwnerPlanNodeIdentity {
        self.prerequisite
    }
    pub const fn dependent(self) -> OwnerPlanNodeIdentity {
        self.dependent
    }
    pub const fn durability_barrier(self) -> bool {
        self.durability_barrier
    }
}

impl CanonicalOwnerPlanDagExplanation {
    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn node_count(&self) -> u64 {
        self.node_count
    }
    pub const fn edge_count(&self) -> u64 {
        self.edge_count
    }
    pub const fn estimated_work_units(&self) -> u64 {
        self.estimated_work_units
    }
    pub const fn first_irreversible_node(&self) -> Option<OwnerPlanNodeIdentity> {
        self.first_irreversible_node
    }
    pub fn nodes(&self) -> &[OwnerPlanNodeExplanation] {
        &self.nodes
    }
    pub fn prerequisites(&self) -> &[OwnerPlanPrerequisiteExplanation] {
        &self.prerequisites
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CanonicalOwnerPlanDag {
    nodes: CanonicalVec<OwnerPlanNode>,
    non_empty_node_identities: NonEmpty<OwnerPlanNodeIdentity>,
    unique_node_identities: UniqueVec<OwnerPlanNodeIdentity>,
    edges: CanonicalVec<OwnerPlanPrerequisite>,
    explanation: CanonicalOwnerPlanDagExplanation,
}

impl CanonicalOwnerPlanDag {
    pub(crate) fn admit(
        mut nodes: Vec<OwnerPlanNode>,
        mut edges: Vec<OwnerPlanPrerequisite>,
    ) -> Result<Self, OwnerPlanDagDenial> {
        if nodes.is_empty() {
            return Err(OwnerPlanDagDenial::Empty);
        }
        nodes.sort();
        edges.sort();
        if edges.windows(2).any(|window| window[0] == window[1]) {
            return Err(OwnerPlanDagDenial::DuplicateEdge);
        }
        let identities = nodes
            .iter()
            .map(OwnerPlanNode::identity)
            .collect::<Vec<_>>();
        let non_empty_node_identities =
            NonEmpty::try_from_vec(identities.clone()).map_err(|_| OwnerPlanDagDenial::Empty)?;
        let unique_node_identities = UniqueVec::try_from_unique(identities)
            .map_err(|_| OwnerPlanDagDenial::DuplicateNodeIdentity)?;
        validate_edges(&nodes, &edges)?;
        validate_overlapping_mutations(&nodes, &edges)?;
        let explanation = explain(&nodes, &edges)?;
        Ok(Self {
            nodes: CanonicalVec::try_from_sorted(nodes)
                .map_err(|_| OwnerPlanDagDenial::AllocationFailed)?,
            non_empty_node_identities,
            unique_node_identities,
            edges: CanonicalVec::try_from_sorted(edges)
                .map_err(|_| OwnerPlanDagDenial::AllocationFailed)?,
            explanation,
        })
    }

    pub(crate) const fn explanation(&self) -> &CanonicalOwnerPlanDagExplanation {
        &self.explanation
    }
}

fn validate_edges(
    nodes: &[OwnerPlanNode],
    edges: &[OwnerPlanPrerequisite],
) -> Result<(), OwnerPlanDagDenial> {
    let known = nodes
        .iter()
        .map(OwnerPlanNode::identity)
        .collect::<BTreeSet<_>>();
    let mut indegree = known
        .iter()
        .copied()
        .map(|id| (id, 0_u64))
        .collect::<BTreeMap<_, _>>();
    let mut outgoing = BTreeMap::<OwnerPlanNodeIdentity, Vec<OwnerPlanNodeIdentity>>::new();
    for edge in edges {
        if edge.prerequisite == edge.dependent {
            return Err(OwnerPlanDagDenial::SelfDependency);
        }
        if !known.contains(&edge.prerequisite) || !known.contains(&edge.dependent) {
            return Err(OwnerPlanDagDenial::UnknownEdgeEndpoint);
        }
        *indegree
            .get_mut(&edge.dependent)
            .ok_or(OwnerPlanDagDenial::UnknownEdgeEndpoint)? += 1;
        outgoing
            .entry(edge.prerequisite)
            .or_default()
            .push(edge.dependent);
    }
    let mut ready = indegree
        .iter()
        .filter_map(|(id, degree)| (*degree == 0).then_some(*id))
        .collect::<BTreeSet<_>>();
    let mut visited = 0_usize;
    while let Some(next) = ready.pop_first() {
        visited = visited
            .checked_add(1)
            .ok_or(OwnerPlanDagDenial::WorkEstimateOverflow)?;
        if let Some(dependents) = outgoing.get(&next) {
            for dependent in dependents {
                let degree = indegree
                    .get_mut(dependent)
                    .ok_or(OwnerPlanDagDenial::UnknownEdgeEndpoint)?;
                *degree = degree.checked_sub(1).ok_or(OwnerPlanDagDenial::Cycle)?;
                if *degree == 0 {
                    ready.insert(*dependent);
                }
            }
        }
    }
    if visited == nodes.len() {
        Ok(())
    } else {
        Err(OwnerPlanDagDenial::Cycle)
    }
}

fn validate_overlapping_mutations(
    nodes: &[OwnerPlanNode],
    edges: &[OwnerPlanPrerequisite],
) -> Result<(), OwnerPlanDagDenial> {
    for (index, left) in nodes.iter().enumerate() {
        for right in &nodes[index + 1..] {
            if left.footprint().overlaps(right.footprint())
                && (left.access() == OwnerPlanAccess::Mutate
                    || right.access() == OwnerPlanAccess::Mutate)
                && !ordered(left.identity(), right.identity(), edges)
            {
                return Err(OwnerPlanDagDenial::AmbiguousOverlappingMutation);
            }
        }
    }
    Ok(())
}

fn ordered(
    left: OwnerPlanNodeIdentity,
    right: OwnerPlanNodeIdentity,
    edges: &[OwnerPlanPrerequisite],
) -> bool {
    reachable(left, right, edges) || reachable(right, left, edges)
}

fn reachable(
    start: OwnerPlanNodeIdentity,
    target: OwnerPlanNodeIdentity,
    edges: &[OwnerPlanPrerequisite],
) -> bool {
    let mut pending = vec![start];
    let mut visited = BTreeSet::new();
    while let Some(next) = pending.pop() {
        if !visited.insert(next) {
            continue;
        }
        for edge in edges.iter().filter(|edge| edge.prerequisite == next) {
            if edge.dependent == target {
                return true;
            }
            pending.push(edge.dependent);
        }
    }
    false
}

fn explain(
    nodes: &[OwnerPlanNode],
    edges: &[OwnerPlanPrerequisite],
) -> Result<CanonicalOwnerPlanDagExplanation, OwnerPlanDagDenial> {
    let mut digest = Sha256::new();
    digest.update(b"worth-store-owner-plan-dag-v1");
    let mut estimated_work_units = 0_u64;
    let first_irreversible_node = first_irreversible_node_in_execution_order(nodes, edges);
    let mut explained_nodes = Vec::new();
    explained_nodes
        .try_reserve_exact(nodes.len())
        .map_err(|_| OwnerPlanDagDenial::AllocationFailed)?;
    for node in nodes {
        digest.update(node.identity().fingerprint());
        digest.update(node.expected_receipt_fingerprint());
        estimated_work_units = estimated_work_units
            .checked_add(node.estimated_work_units())
            .ok_or(OwnerPlanDagDenial::WorkEstimateOverflow)?;
        explained_nodes.push(OwnerPlanNodeExplanation {
            identity: node.identity(),
            owner: node.owner(),
            effect: node.effect(),
            stage: node.stage(),
            footprint: node.footprint(),
            estimated_work_units: node.estimated_work_units(),
            irreversible: node.irreversible(),
            access: node.access(),
            expected_receipt_fingerprint: node.expected_receipt_fingerprint(),
        });
    }
    let mut explained_prerequisites = Vec::new();
    explained_prerequisites
        .try_reserve_exact(edges.len())
        .map_err(|_| OwnerPlanDagDenial::AllocationFailed)?;
    for edge in edges {
        digest.update(edge.prerequisite.fingerprint());
        digest.update(edge.dependent.fingerprint());
        digest.update([u8::from(edge.durability_barrier)]);
        explained_prerequisites.push(OwnerPlanPrerequisiteExplanation {
            prerequisite: edge.prerequisite,
            dependent: edge.dependent,
            durability_barrier: edge.durability_barrier,
        });
    }
    Ok(CanonicalOwnerPlanDagExplanation {
        plan_fingerprint: digest.finalize().into(),
        node_count: u64::try_from(nodes.len())
            .map_err(|_| OwnerPlanDagDenial::WorkEstimateOverflow)?,
        edge_count: u64::try_from(edges.len())
            .map_err(|_| OwnerPlanDagDenial::WorkEstimateOverflow)?,
        estimated_work_units,
        first_irreversible_node,
        nodes: explained_nodes,
        prerequisites: explained_prerequisites,
    })
}
