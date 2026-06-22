use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::overlap_edge_chains::{
    PlanarBooleanOverlapEdgeChain, PlanarBooleanOverlapEdgeChainMember,
    PlanarBooleanOverlapEdgeChainSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentSet,
};

pub(super) struct SplitChainValidationIndexedInputs<'a> {
    fragments_by_identity: BTreeMap<&'a str, &'a PlanarBooleanSplitEdgeFragment>,
    chains_by_identity: BTreeMap<&'a str, &'a PlanarBooleanOverlapEdgeChain>,
    overlap_members_by_interval:
        BTreeMap<OverlapCoverageKey<'a>, Vec<&'a PlanarBooleanOverlapEdgeChainMember>>,
}

impl<'a> SplitChainValidationIndexedInputs<'a> {
    pub(super) fn new(
        fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        chains: &'a PlanarBooleanOverlapEdgeChainSet,
    ) -> Self {
        let mut fragments_by_identity = BTreeMap::new();
        for fragment in fragments.fragments() {
            fragments_by_identity.insert(fragment.fragment_identity(), fragment);
        }
        let mut chains_by_identity = BTreeMap::new();
        let mut overlap_members_by_interval =
            BTreeMap::<OverlapCoverageKey<'a>, Vec<&PlanarBooleanOverlapEdgeChainMember>>::new();
        for chain in chains.chains() {
            chains_by_identity.insert(chain.chain_identity(), chain);
            for member in chain.members() {
                overlap_members_by_interval
                    .entry(OverlapCoverageKey::from_member(
                        chain.chain_identity(),
                        member,
                    ))
                    .or_default()
                    .push(member);
            }
        }
        for members in overlap_members_by_interval.values_mut() {
            members.sort_by(|a, b| {
                a.fragment_parameter_range()[0]
                    .total_cmp(&b.fragment_parameter_range()[0])
                    .then_with(|| a.member_identity().cmp(b.member_identity()))
            });
        }
        Self {
            fragments_by_identity,
            chains_by_identity,
            overlap_members_by_interval,
        }
    }

    pub(super) fn fragment(
        &self,
        fragment_identity: &str,
    ) -> Option<&'a PlanarBooleanSplitEdgeFragment> {
        self.fragments_by_identity.get(fragment_identity).copied()
    }

    pub(super) fn overlap_groups(
        &self,
    ) -> &BTreeMap<OverlapCoverageKey<'a>, Vec<&'a PlanarBooleanOverlapEdgeChainMember>> {
        &self.overlap_members_by_interval
    }

    pub(super) fn chain(&self, chain_identity: &str) -> Option<&'a PlanarBooleanOverlapEdgeChain> {
        self.chains_by_identity.get(chain_identity).copied()
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct OverlapCoverageKey<'a> {
    pub(super) chain_identity: &'a str,
    pub(super) source_interval_identity: &'a str,
    pub(super) source_edge_identity: &'a str,
    pub(super) carrier_identity: &'a str,
}

impl<'a> OverlapCoverageKey<'a> {
    fn from_member(
        chain_identity: &'a str,
        member: &'a PlanarBooleanOverlapEdgeChainMember,
    ) -> Self {
        Self {
            chain_identity,
            source_interval_identity: member.source_interval_identity(),
            source_edge_identity: member.source_edge_identity(),
            carrier_identity: member.carrier_identity(),
        }
    }
}
