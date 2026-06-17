use crate::workload_platform::planar_boolean_edge_splitting::overlap_edge_chains::PlanarBooleanOverlapEdgeChainSet;
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::PlanarBooleanSplitEdgeFragmentSet;

use super::counters::PlanarBooleanSplitChainValidationCounters;
use super::denial::{
    PlanarBooleanSplitChainValidationDenial, PlanarBooleanSplitChainValidationDenialKind as Kind,
};
use super::fragment_domain::validate_fragment_domains;
use super::identity::validation_receipt_identity;
use super::indexed_inputs::SplitChainValidationIndexedInputs;
use super::overlap_references::validate_overlap_references;
use super::receipt::PlanarBooleanSplitChainValidationReceipt;

impl PlanarBooleanSplitEdgeFragmentSet {
    pub fn validate_split_edge_chains(
        &self,
        chains: &PlanarBooleanOverlapEdgeChainSet,
    ) -> Result<PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitChainValidationDenial>
    {
        let mut counters = CounterBuild::default();
        if chains.split_edge_fragment_set_identity() != self.fragment_set_identity()
            || chains.interval_subdivision_schedule_set_identity()
                != self.interval_subdivision_schedule_set_identity()
        {
            counters.foreign_chain_sets_rejected += 1;
            return Err(counters.deny(
                Kind::ForeignOverlapChainSet,
                chains.chain_set_identity(),
                "split chain validation requires overlap chains from the same split fragment authority",
            ));
        }
        let indexed = SplitChainValidationIndexedInputs::new(self, chains);
        let fragment_rows = validate_fragment_domains(self, &mut counters)?;
        let overlap_rows = validate_overlap_references(chains, &indexed, &mut counters)?;
        let receipt_identity = validation_receipt_identity(
            self.fragment_set_identity(),
            chains.chain_set_identity(),
            &fragment_rows,
            &overlap_rows,
        );
        Ok(PlanarBooleanSplitChainValidationReceipt::new(
            receipt_identity,
            self.fragment_set_identity().to_string(),
            chains.chain_set_identity().to_string(),
            self.interval_subdivision_schedule_set_identity()
                .to_string(),
            fragment_rows,
            overlap_rows,
            counters.finish(),
        ))
    }
}

#[derive(Default)]
pub(super) struct CounterBuild {
    pub(super) source_edges_checked: usize,
    pub(super) fragment_schedules_checked: usize,
    pub(super) fragments_checked: usize,
    pub(super) overlap_chains_checked: usize,
    pub(super) overlap_members_checked: usize,
    pub(super) gaps_rejected: usize,
    pub(super) overlaps_rejected: usize,
    pub(super) dangling_references_rejected: usize,
    pub(super) mismatched_interval_basis_rejected: usize,
    pub(super) foreign_chain_sets_rejected: usize,
    pub(super) out_of_interval_references_rejected: usize,
    pub(super) denied_chains: usize,
}

impl CounterBuild {
    pub(super) fn deny(
        &mut self,
        kind: Kind,
        evidence_identity: impl Into<String>,
        reason: impl Into<String>,
    ) -> PlanarBooleanSplitChainValidationDenial {
        self.denied_chains += 1;
        PlanarBooleanSplitChainValidationDenial::new(kind, evidence_identity, self.finish(), reason)
    }

    fn finish(&self) -> PlanarBooleanSplitChainValidationCounters {
        PlanarBooleanSplitChainValidationCounters::new(
            self.source_edges_checked,
            self.fragment_schedules_checked,
            self.fragments_checked,
            self.overlap_chains_checked,
            self.overlap_members_checked,
            self.gaps_rejected,
            self.overlaps_rejected,
            self.dangling_references_rejected,
            self.mismatched_interval_basis_rejected,
            self.foreign_chain_sets_rejected,
            self.out_of_interval_references_rejected,
            self.denied_chains,
        )
    }
}
