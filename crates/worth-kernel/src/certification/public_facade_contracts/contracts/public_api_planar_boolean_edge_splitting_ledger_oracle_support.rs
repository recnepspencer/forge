use super::super::edge_splitting_decision_log_support::DecisionLogMetabossProducts;
use super::ledger_manifest_support::{
    edge_key, EdgeKey, ObservedSplitEdgeChainLedgerManifest, ObservedSplitEdgeChainLedgerRow,
};
use std::collections::{BTreeMap, BTreeSet};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitDecisionLogQueryResult, PlanarBooleanSplitEdgeChainLedger,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct MetabossSplitLedgerOracle {
    chains: BTreeMap<EdgeKey, MetabossExpectedSplitLedgerChain>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MetabossExpectedSplitLedgerChain {
    source_edge_identity: String,
    carrier_identity: String,
    expects_overlap_chain: bool,
}

impl MetabossSplitLedgerOracle {
    pub(crate) fn from_products(products: &DecisionLogMetabossProducts) -> Self {
        let overlap_keys = products
            .chains
            .chains()
            .iter()
            .flat_map(|chain| {
                chain.members().iter().map(|member| {
                    edge_key(member.source_edge_identity(), member.carrier_identity())
                })
            })
            .collect::<BTreeSet<_>>();
        let chains = products
            .fragments
            .schedules()
            .iter()
            .map(|schedule| {
                let key = edge_key(
                    schedule.source_edge_identity(),
                    schedule.carrier_identity(),
                );
                let expected = MetabossExpectedSplitLedgerChain {
                    source_edge_identity: schedule.source_edge_identity().to_string(),
                    carrier_identity: schedule.carrier_identity().to_string(),
                    expects_overlap_chain: overlap_keys.contains(&key),
                };
                (key, expected)
            })
            .collect();
        Self { chains }
    }

    pub(crate) fn assert_matches_observed_products(
        &self,
        observed: &ObservedSplitEdgeChainLedgerManifest,
    ) {
        assert_eq!(
            observed.chains.keys().collect::<Vec<_>>(),
            self.chains.keys().collect::<Vec<_>>(),
            "observed split products must cover exactly the split-fragment schedule rows"
        );
        for (key, expected) in &self.chains {
            let observed_chain = observed
                .chains
                .get(key)
                .expect("observed split products must include the oracle carrier key");
            expected.assert_observed_chain_semantics(observed_chain);
        }
    }

    pub(crate) fn assert_matches_ledger_semantics(
        &self,
        ledger: &PlanarBooleanSplitEdgeChainLedger,
        _decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    ) {
        assert_eq!(
            ledger.chains().len(),
            self.chains.len(),
            "ledger must emit one proof-bearing chain for each split-fragment schedule row"
        );
        for chain in ledger.chains() {
            let key = edge_key(chain.source_edge_identity(), chain.carrier_identity());
            let expected = self
                .chains
                .get(&key)
                .expect("ledger chain must be anchored in participation evidence");
            expected.assert_ledger_chain_semantics(chain);
        }
    }
}

impl MetabossExpectedSplitLedgerChain {
    fn assert_observed_chain_semantics(&self, observed: &ObservedSplitEdgeChainLedgerRow) {
        assert!(!observed.endpoint_boundary_schedule_identity.is_empty());
        assert!(!observed.interval_subdivision_schedule_identity.is_empty());
        assert!(!observed.split_fragment_schedule_identity.is_empty());
        assert!(!observed.fragment_identities.is_empty());
        assert!(!observed.validation_fragment_coverage_identities.is_empty());
        if self.expects_overlap_chain() {
            assert!(!observed.overlap_chain_identities.is_empty());
            assert!(!observed.validation_overlap_coverage_identities.is_empty());
        }
    }

    fn assert_ledger_chain_semantics(
        &self,
        chain: &worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeChain,
    ) {
        assert_eq!(chain.source_edge_identity(), self.source_edge_identity);
        assert_eq!(chain.carrier_identity(), self.carrier_identity);
        assert!(!chain.endpoint_boundary_schedule_identity().is_empty());
        assert!(!chain.interval_subdivision_schedule_identity().is_empty());
        assert!(!chain.split_fragment_schedule_identity().is_empty());
        assert!(!chain.fragment_identities().is_empty());
        assert!(!chain.validation_fragment_coverage_identities().is_empty());
        if self.expects_overlap_chain() {
            assert!(!chain.overlap_chain_identities().is_empty());
            assert!(!chain.validation_overlap_coverage_identities().is_empty());
        }
    }

    fn expects_overlap_chain(&self) -> bool {
        self.expects_overlap_chain
    }
}
