use super::counters::PlanarBooleanLoopSourceProvenanceCounters;
use super::denial::PlanarBooleanLoopSourceProvenanceDenial;
use super::input::PlanarBooleanLoopSourceProvenanceRecoveryInput;
use super::recovery::recover_loop_source_provenance;
use super::{
    PlanarBooleanFragmentMembershipMap, PlanarBooleanLoopOverlapChainLineageMap,
    PlanarBooleanLoopSourceCarrierSet,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopSourceProvenanceBundle {
    bundle_identity: String,
    request_identity: String,
    split_ledger_receipt_identity: String,
    source_loop_carriers: PlanarBooleanLoopSourceCarrierSet,
    fragment_membership_map: PlanarBooleanFragmentMembershipMap,
    overlap_chain_lineage_map: PlanarBooleanLoopOverlapChainLineageMap,
    counters: PlanarBooleanLoopSourceProvenanceCounters,
}

impl PlanarBooleanLoopSourceProvenanceBundle {
    pub fn recover(
        input: PlanarBooleanLoopSourceProvenanceRecoveryInput<'_>,
    ) -> Result<Self, PlanarBooleanLoopSourceProvenanceDenial> {
        recover_loop_source_provenance(input)
    }

    pub(crate) fn new(
        bundle_identity: String,
        request_identity: String,
        split_ledger_receipt_identity: String,
        source_loop_carriers: PlanarBooleanLoopSourceCarrierSet,
        fragment_membership_map: PlanarBooleanFragmentMembershipMap,
        overlap_chain_lineage_map: PlanarBooleanLoopOverlapChainLineageMap,
        counters: PlanarBooleanLoopSourceProvenanceCounters,
    ) -> Self {
        Self {
            bundle_identity,
            request_identity,
            split_ledger_receipt_identity,
            source_loop_carriers,
            fragment_membership_map,
            overlap_chain_lineage_map,
            counters,
        }
    }

    pub fn bundle_identity(&self) -> &str {
        &self.bundle_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn split_ledger_receipt_identity(&self) -> &str {
        &self.split_ledger_receipt_identity
    }

    pub fn source_loop_carriers(&self) -> &PlanarBooleanLoopSourceCarrierSet {
        &self.source_loop_carriers
    }

    pub fn fragment_membership_map(&self) -> &PlanarBooleanFragmentMembershipMap {
        &self.fragment_membership_map
    }

    pub fn overlap_chain_lineage_map(&self) -> &PlanarBooleanLoopOverlapChainLineageMap {
        &self.overlap_chain_lineage_map
    }

    pub fn counters(&self) -> PlanarBooleanLoopSourceProvenanceCounters {
        self.counters
    }
}
