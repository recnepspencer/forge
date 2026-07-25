use std::collections::BTreeMap;
use std::rc::Rc;

use crate::evidence::{
    UiAllocationNeighborhood, UiAllocationNeighborhoodIdentity, UiMeasurementBasis,
    UiMeasurementBasisGeneration,
};
use crate::graph::{UiGraphGeneration, UiGraphNodeIdentity};

use super::{
    UiAdmittedAllocationPlanReference, UiGraphNeighborhoodFootprint, UiReplanGenerationKey,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmittedAllocationInvalidationTarget {
    graph_node_identity: UiGraphNodeIdentity,
    graph_generation: UiGraphGeneration,
    neighborhood_identity: UiAllocationNeighborhoodIdentity,
    neighborhood_footprint: Rc<UiGraphNeighborhoodFootprint>,
    measurement_basis_generation: UiMeasurementBasisGeneration,
    allocation_plan: Option<UiAdmittedAllocationPlanReference>,
    pub(super) graph_membership_probes: u16,
    pub(in crate::graph::allocation_neighborhood) replacement_impact:
        Option<Rc<crate::runtime::WorthUiReplacementImpactClassification>>,
    pub(in crate::graph::allocation_neighborhood) impact_narrowing:
        Option<Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>>,
    consequence: super::UiReplanWidenReason,
    disposition: UiGraphReplanTargetDisposition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAdmittedAllocationInvalidationTargetSet {
    pub(super) touched_graph_node_identities: Box<[UiGraphNodeIdentity]>,
    pub(super) primary: UiAdmittedAllocationInvalidationTarget,
    pub(super) widened: Box<[UiAdmittedAllocationInvalidationTarget]>,
}

struct UiAllocationInvalidationTargetAdmissionInput<'a> {
    graph_node_identity: UiGraphNodeIdentity,
    neighborhood: &'a UiAllocationNeighborhood,
    basis: &'a UiMeasurementBasis,
    allocation_plan: Option<&'a UiAdmittedAllocationPlanReference>,
    membership_probes: u16,
    replacement_impact: Option<&'a Rc<crate::runtime::WorthUiReplacementImpactClassification>>,
    impact_narrowing: Option<&'a Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>>,
    neighborhood_footprint: Rc<UiGraphNeighborhoodFootprint>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiGraphReplanTargetDisposition {
    LocalPrimaryEligible,
    RootPrimaryEligible,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct UiGraphReplanAuthority {
    pub(in crate::graph::allocation_neighborhood) active_identity_digest: u64,
    pub(in crate::graph::allocation_neighborhood) active_neighborhoods:
        crate::runtime::persistent_index::UiPersistentOrdMap<
            crate::evidence::UiAllocationNeighborhoodScope,
            super::activation_lifecycle::UiGraphNeighborhoodLifecycleEntry,
        >,
    pub(in crate::graph::allocation_neighborhood) generations_by_digest:
        crate::runtime::persistent_index::UiPersistentOrdMap<u64, Box<[UiReplanGenerationKey]>>,
    pub(in crate::graph::allocation_neighborhood) targets_by_node:
        crate::runtime::persistent_index::UiPersistentOrdMap<
            UiGraphNodeIdentity,
            Box<[UiAdmittedAllocationInvalidationTarget]>,
        >,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct UiGraphReplanAdmission {
    targets: BTreeMap<UiGraphNodeIdentity, UiAdmittedAllocationInvalidationTarget>,
}

impl UiGraphReplanAdmission {
    pub(crate) fn seal(
        neighborhood: &UiAllocationNeighborhood,
        basis: &UiMeasurementBasis,
        allocation_plan: Option<&UiAdmittedAllocationPlanReference>,
        replacement_impact: Option<&Rc<crate::runtime::WorthUiReplacementImpactClassification>>,
        impact_narrowing: Option<&Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>>,
    ) -> Self {
        let mut targets = BTreeMap::new();
        let footprint = Rc::new(UiGraphNeighborhoodFootprint::seal(neighborhood));
        for member in neighborhood.members() {
            let identity = member.graph_node_identity();
            targets.insert(
                identity,
                UiAdmittedAllocationInvalidationTarget::admit(
                    UiAllocationInvalidationTargetAdmissionInput {
                        graph_node_identity: identity,
                        neighborhood,
                        basis,
                        allocation_plan,
                        membership_probes: 1,
                        replacement_impact,
                        impact_narrowing,
                        neighborhood_footprint: Rc::clone(&footprint),
                    },
                ),
            );
        }
        Self { targets }
    }

    pub(crate) fn targets(&self) -> impl Iterator<Item = &UiAdmittedAllocationInvalidationTarget> {
        self.targets.values()
    }
}

impl UiAdmittedAllocationInvalidationTarget {
    fn admit(input: UiAllocationInvalidationTargetAdmissionInput<'_>) -> Self {
        let UiAllocationInvalidationTargetAdmissionInput {
            graph_node_identity,
            neighborhood,
            basis,
            allocation_plan,
            membership_probes,
            replacement_impact,
            impact_narrowing,
            neighborhood_footprint,
        } = input;
        Self {
            graph_node_identity,
            graph_generation: neighborhood.graph_generation(),
            neighborhood_identity: neighborhood.identity().clone(),
            neighborhood_footprint,
            measurement_basis_generation: basis.generation(),
            allocation_plan: allocation_plan.cloned(),
            graph_membership_probes: membership_probes,
            replacement_impact: replacement_impact.cloned(),
            impact_narrowing: impact_narrowing.cloned(),
            consequence: super::replan_consequence::consequence_for(
                graph_node_identity,
                neighborhood,
                replacement_impact.map(Rc::as_ref),
            ),
            disposition: if graph_node_identity == neighborhood.root_graph_node_identity() {
                UiGraphReplanTargetDisposition::RootPrimaryEligible
            } else {
                UiGraphReplanTargetDisposition::LocalPrimaryEligible
            },
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }
    pub fn neighborhood_identity(&self) -> &UiAllocationNeighborhoodIdentity {
        &self.neighborhood_identity
    }
    pub(crate) fn neighborhood_footprint(&self) -> Rc<UiGraphNeighborhoodFootprint> {
        Rc::clone(&self.neighborhood_footprint)
    }
    pub fn graph_generation(&self) -> UiGraphGeneration {
        self.graph_generation
    }
    pub fn measurement_basis_generation(&self) -> UiMeasurementBasisGeneration {
        self.measurement_basis_generation
    }
    pub(crate) fn allocation_plan(&self) -> Option<&UiAdmittedAllocationPlanReference> {
        self.allocation_plan.as_ref()
    }
    pub(crate) fn graph_membership_probes(&self) -> u16 {
        self.graph_membership_probes
    }
    pub(crate) fn with_graph_index_probes(mut self, probes: u16) -> Self {
        self.graph_membership_probes = probes;
        self
    }
    pub(crate) fn graph_consequence(&self) -> super::UiReplanWidenReason {
        self.consequence
    }
    pub(crate) fn disposition(&self) -> UiGraphReplanTargetDisposition {
        self.disposition
    }

    pub(crate) fn generation_key(&self) -> Option<UiReplanGenerationKey> {
        self.allocation_plan
            .as_ref()
            .map(UiAdmittedAllocationPlanReference::generation_key)
    }
}

impl UiGraphReplanAuthority {
    pub(crate) fn active_identity_digest(&self) -> u64 {
        self.active_identity_digest
    }

    pub(crate) fn replace<'a>(
        &mut self,
        targets: impl Iterator<Item = &'a UiAdmittedAllocationInvalidationTarget>,
    ) {
        let mut next = BTreeMap::<u64, Vec<UiReplanGenerationKey>>::new();
        let mut targets_by_node =
            BTreeMap::<UiGraphNodeIdentity, Vec<UiAdmittedAllocationInvalidationTarget>>::new();
        for target in targets {
            let node_targets = targets_by_node
                .entry(target.graph_node_identity())
                .or_default();
            if !node_targets
                .iter()
                .any(|existing| existing.neighborhood_identity() == target.neighborhood_identity())
            {
                node_targets.push(target.clone());
            }
            let Some(key) = target.generation_key() else {
                continue;
            };
            let bucket = next
                .entry(key.neighborhood_identity.identity_digest())
                .or_default();
            if !bucket.contains(&key) {
                bucket.push(key);
            }
        }
        self.generations_by_digest = Default::default();
        for (digest, keys) in next {
            self.generations_by_digest
                .insert(digest, keys.into_boxed_slice());
        }
        self.targets_by_node = Default::default();
        for (node, mut targets) in targets_by_node {
            targets.sort_by(|left, right| {
                let left_root =
                    left.disposition == UiGraphReplanTargetDisposition::RootPrimaryEligible;
                let right_root =
                    right.disposition == UiGraphReplanTargetDisposition::RootPrimaryEligible;
                left_root
                    .cmp(&right_root)
                    .then_with(|| left.consequence.cmp(&right.consequence))
                    .then_with(|| {
                        left.neighborhood_identity
                            .identity_digest()
                            .cmp(&right.neighborhood_identity.identity_digest())
                    })
            });
            self.targets_by_node
                .insert(node, targets.into_boxed_slice());
        }
    }

    pub(crate) fn target_set(
        &self,
        identity: UiGraphNodeIdentity,
    ) -> Option<UiAdmittedAllocationInvalidationTargetSet> {
        self.target_set_for_nodes(&[identity])
    }

    pub(crate) fn target_set_for_neighborhood(
        &self,
        identity: UiGraphNodeIdentity,
        neighborhood_identity: &crate::evidence::UiAllocationNeighborhoodIdentity,
    ) -> Option<UiAdmittedAllocationInvalidationTargetSet> {
        let target = self
            .targets_by_node
            .get(&identity)?
            .iter()
            .find(|target| target.neighborhood_identity() == neighborhood_identity)?
            .clone()
            .with_graph_index_probes(1);
        Some(UiAdmittedAllocationInvalidationTargetSet {
            touched_graph_node_identities: vec![identity].into_boxed_slice(),
            primary: target,
            widened: Box::new([]),
        })
    }

    pub(crate) fn target_set_for_nodes(
        &self,
        identities: &[UiGraphNodeIdentity],
    ) -> Option<UiAdmittedAllocationInvalidationTargetSet> {
        let mut touched = identities.to_vec();
        touched.sort_unstable();
        touched.dedup();
        let mut targets = Vec::new();
        for identity in &touched {
            targets.extend(self.targets_by_node.get(identity)?.iter().cloned());
        }
        targets.sort_by_key(causal_rank);
        targets.dedup_by(|left, right| {
            left.neighborhood_identity() == right.neighborhood_identity()
                && left.generation_key() == right.generation_key()
        });
        let primary_ordinal = targets
            .iter()
            .enumerate()
            .min_by(|(_, left), (_, right)| causal_rank(left).cmp(&causal_rank(right)))?
            .0;
        let primary = targets
            .get(primary_ordinal)?
            .clone()
            .with_graph_index_probes(1);
        let widened = targets
            .iter()
            .enumerate()
            .filter(|(ordinal, _)| *ordinal != primary_ordinal)
            .map(|(_, target)| target.clone().with_graph_index_probes(1))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Some(UiAdmittedAllocationInvalidationTargetSet {
            touched_graph_node_identities: touched.into_boxed_slice(),
            primary,
            widened,
        })
    }

    pub(crate) fn certifies(&self, key: &UiReplanGenerationKey) -> bool {
        self.generations_by_digest
            .get(&key.neighborhood_identity.identity_digest())
            .is_some_and(|bucket| bucket.contains(key))
    }
}

pub(super) fn causal_rank(
    target: &UiAdmittedAllocationInvalidationTarget,
) -> (bool, usize, super::UiReplanWidenReason, u64) {
    (
        target.disposition == UiGraphReplanTargetDisposition::RootPrimaryEligible,
        target.neighborhood_footprint.members().len(),
        target.consequence,
        target.neighborhood_identity.identity_digest(),
    )
}
