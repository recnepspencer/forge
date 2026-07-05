use super::{
    PlanarBooleanFragmentMembershipMap, PlanarBooleanLoopIslandPartition,
    PlanarBooleanLoopOverlapChainLineageMap, PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopReconstructionLedger, PlanarBooleanLoopReconstructionLedgerReceipt,
    PlanarBooleanLoopReconstructionLedgerRow, PlanarBooleanLoopRoleOutcomeSet,
    PlanarBooleanLoopSourceCarrierSet,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanLoopReconstructionParticipationSupportDenialKind {
    RequestIdentityMismatch,
    IslandPartitionIdentityMismatch,
    RoleOutcomeIdentityMismatch,
    PersistentNameMapIdentityMismatch,
    LedgerRowIdentityMismatch,
    OverlapChainLineageRequestMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionParticipationSupportDenial {
    kind: PlanarBooleanLoopReconstructionParticipationSupportDenialKind,
    rejected_identity: String,
    human_reason: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopReconstructionParticipationSupport {
    loop_ledger_receipt: PlanarBooleanLoopReconstructionLedgerReceipt,
    ledger_rows: Vec<PlanarBooleanLoopReconstructionLedgerRow>,
    role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
    island_partition: PlanarBooleanLoopIslandPartition,
    persistent_name_map: PlanarBooleanLoopPersistentNamePropagationMap,
    fragment_membership_map: PlanarBooleanFragmentMembershipMap,
    overlap_chain_lineage_map: PlanarBooleanLoopOverlapChainLineageMap,
    source_loop_carriers: PlanarBooleanLoopSourceCarrierSet,
}

impl PlanarBooleanLoopReconstructionParticipationSupport {
    pub fn admit_from_ledger_and_products(
        ledger: &PlanarBooleanLoopReconstructionLedger,
        role_outcomes: &PlanarBooleanLoopRoleOutcomeSet,
        island_partition: &PlanarBooleanLoopIslandPartition,
        persistent_name_map: &PlanarBooleanLoopPersistentNamePropagationMap,
        fragment_membership_map: &PlanarBooleanFragmentMembershipMap,
        overlap_chain_lineage_map: &PlanarBooleanLoopOverlapChainLineageMap,
        source_loop_carriers: &PlanarBooleanLoopSourceCarrierSet,
    ) -> Result<Self, PlanarBooleanLoopReconstructionParticipationSupportDenial> {
        let receipt = ledger.receipt();
        reject_mismatch(
            role_outcomes.request_identity(),
            receipt.request_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::RequestIdentityMismatch,
            "role outcome request identity",
            "overlap participation support requires role outcomes from the same loop-reconstruction request",
        )?;
        reject_mismatch(
            island_partition.request_identity(),
            receipt.request_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::RequestIdentityMismatch,
            "island partition request identity",
            "overlap participation support requires island partition from the same loop-reconstruction request",
        )?;
        reject_mismatch(
            persistent_name_map.request_identity(),
            receipt.request_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::RequestIdentityMismatch,
            "persistent name map request identity",
            "overlap participation support requires persistent names from the same loop-reconstruction request",
        )?;
        reject_mismatch(
            fragment_membership_map.request_identity(),
            receipt.request_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::RequestIdentityMismatch,
            "fragment membership request identity",
            "overlap participation support requires fragment membership from the same loop-reconstruction request",
        )?;
        reject_mismatch(
            overlap_chain_lineage_map.request_identity(),
            receipt.request_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::OverlapChainLineageRequestMismatch,
            "overlap chain lineage request identity",
            "overlap participation support requires overlap-chain lineage from the same loop-reconstruction request",
        )?;
        reject_mismatch(
            island_partition.partition_identity(),
            ledger.island_partition_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::IslandPartitionIdentityMismatch,
            "island partition identity",
            "overlap participation support requires the canonical loop-island partition consumed by the real 7.4 ledger",
        )?;
        reject_mismatch(
            role_outcomes.role_outcome_set_identity(),
            ledger.role_outcome_set_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::RoleOutcomeIdentityMismatch,
            "role outcome set identity",
            "overlap participation support requires the canonical loop-role outcomes consumed by the real 7.4 ledger",
        )?;
        reject_mismatch(
            persistent_name_map.map_identity(),
            receipt.persistent_name_map_identity(),
            PlanarBooleanLoopReconstructionParticipationSupportDenialKind::PersistentNameMapIdentityMismatch,
            "persistent name map identity",
            "overlap participation support requires the canonical persistent-name map carried by the real 7.4 ledger receipt",
        )?;

        let ledger_row_identities = ledger
            .rows()
            .iter()
            .map(|row| row.ledger_row_identity().to_string())
            .collect::<Vec<_>>();
        if ledger_row_identities != receipt.ledger_row_identities() {
            return Err(PlanarBooleanLoopReconstructionParticipationSupportDenial::new(
                PlanarBooleanLoopReconstructionParticipationSupportDenialKind::LedgerRowIdentityMismatch,
                "loop ledger row identities",
                "overlap participation support requires the exact canonical loop-ledger rows carried by the real 7.4 receipt",
            ));
        }

        Ok(Self {
            loop_ledger_receipt: receipt,
            ledger_rows: ledger.rows().to_vec(),
            role_outcomes: role_outcomes.clone(),
            island_partition: island_partition.clone(),
            persistent_name_map: persistent_name_map.clone(),
            fragment_membership_map: fragment_membership_map.clone(),
            overlap_chain_lineage_map: overlap_chain_lineage_map.clone(),
            source_loop_carriers: source_loop_carriers.clone(),
        })
    }

    pub fn loop_ledger_receipt(&self) -> &PlanarBooleanLoopReconstructionLedgerReceipt {
        &self.loop_ledger_receipt
    }

    pub fn ledger_rows(&self) -> &[PlanarBooleanLoopReconstructionLedgerRow] {
        &self.ledger_rows
    }

    pub fn role_outcomes(&self) -> &PlanarBooleanLoopRoleOutcomeSet {
        &self.role_outcomes
    }

    pub fn island_partition(&self) -> &PlanarBooleanLoopIslandPartition {
        &self.island_partition
    }

    pub fn persistent_name_map(&self) -> &PlanarBooleanLoopPersistentNamePropagationMap {
        &self.persistent_name_map
    }

    pub fn fragment_membership_map(&self) -> &PlanarBooleanFragmentMembershipMap {
        &self.fragment_membership_map
    }

    pub fn overlap_chain_lineage_map(&self) -> &PlanarBooleanLoopOverlapChainLineageMap {
        &self.overlap_chain_lineage_map
    }

    pub fn source_loop_carriers(&self) -> &PlanarBooleanLoopSourceCarrierSet {
        &self.source_loop_carriers
    }

    #[cfg(test)]
    pub(crate) fn with_role_outcomes_for_tests(
        &self,
        role_outcomes: PlanarBooleanLoopRoleOutcomeSet,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.role_outcomes = role_outcomes;
        cloned
    }

    #[cfg(test)]
    pub(crate) fn with_island_partition_for_tests(
        &self,
        island_partition: PlanarBooleanLoopIslandPartition,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.island_partition = island_partition;
        cloned
    }

    #[cfg(test)]
    pub(crate) fn with_overlap_chain_lineage_map_for_tests(
        &self,
        overlap_chain_lineage_map: PlanarBooleanLoopOverlapChainLineageMap,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.overlap_chain_lineage_map = overlap_chain_lineage_map;
        cloned
    }

    #[cfg(test)]
    pub(crate) fn with_loop_ledger_receipt_for_tests(
        &self,
        loop_ledger_receipt: PlanarBooleanLoopReconstructionLedgerReceipt,
    ) -> Self {
        let mut cloned = self.clone();
        cloned.loop_ledger_receipt = loop_ledger_receipt;
        cloned
    }
}

impl PlanarBooleanLoopReconstructionParticipationSupportDenial {
    fn new(
        kind: PlanarBooleanLoopReconstructionParticipationSupportDenialKind,
        rejected_identity: impl Into<String>,
        human_reason: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            human_reason,
        }
    }

    pub fn kind(&self) -> PlanarBooleanLoopReconstructionParticipationSupportDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn human_reason(&self) -> &'static str {
        self.human_reason
    }
}

fn reject_mismatch(
    left: &str,
    right: &str,
    kind: PlanarBooleanLoopReconstructionParticipationSupportDenialKind,
    rejected_identity: &'static str,
    human_reason: &'static str,
) -> Result<(), PlanarBooleanLoopReconstructionParticipationSupportDenial> {
    if left != right {
        return Err(
            PlanarBooleanLoopReconstructionParticipationSupportDenial::new(
                kind,
                rejected_identity,
                human_reason,
            ),
        );
    }
    Ok(())
}
