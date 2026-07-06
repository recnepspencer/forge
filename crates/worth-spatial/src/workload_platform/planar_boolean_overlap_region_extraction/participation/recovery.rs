use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIslandPartitionRow, PlanarBooleanLoopPersistentNamePropagationRow,
    PlanarBooleanLoopReconstructionLedgerRow, PlanarBooleanLoopReconstructionParticipationSupport,
    PlanarBooleanLoopRoleOutcome,
};

use super::chain_lineage::{
    PlanarBooleanOverlapChainRegionLineageMap, PlanarBooleanOverlapChainRegionLineageRow,
};
use super::counters::PlanarBooleanOverlapParticipationRecoveryCounters;
use super::denial::{
    PlanarBooleanOverlapParticipationRecoveryDenial,
    PlanarBooleanOverlapParticipationRecoveryDenialKind as Kind,
};
use super::input::PlanarBooleanOverlapParticipationRecoveryInput;
use super::island_participation::{
    PlanarBooleanLoopIslandOverlapParticipationMap, PlanarBooleanLoopIslandOverlapParticipationRow,
};
use super::loop_participation::{
    PlanarBooleanLoopOverlapParticipationMap, PlanarBooleanLoopOverlapParticipationRow,
};
use super::recovery_support::{
    island_membership_counts, map_identity, matching_chain_lineage_identities,
    persistent_names_by_canonical_loop, recover_chain_lineage_rows,
    recover_island_participation_rows, recover_island_row,
};
use super::source_loop_witnesses::witnesses_for_ledger_row;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapParticipationRecovery {
    loop_participation_map: PlanarBooleanLoopOverlapParticipationMap,
    island_participation_map: PlanarBooleanLoopIslandOverlapParticipationMap,
    chain_lineage_map: PlanarBooleanOverlapChainRegionLineageMap,
    counters: PlanarBooleanOverlapParticipationRecoveryCounters,
}

impl PlanarBooleanOverlapParticipationRecovery {
    pub fn recover(
        input: PlanarBooleanOverlapParticipationRecoveryInput<'_>,
    ) -> Result<Self, PlanarBooleanOverlapParticipationRecoveryDenial> {
        let mut counters = PlanarBooleanOverlapParticipationRecoveryCounters::default();
        let support = input.loop_participation_support();
        validate_request_binding(input.request(), support, &mut counters)?;

        let role_outcomes = support
            .role_outcomes()
            .rows()
            .iter()
            .map(|row| (row.role_outcome_identity(), row))
            .collect::<BTreeMap<_, _>>();
        let island_rows = support
            .island_partition()
            .rows()
            .iter()
            .map(|row| (row.island_identity(), row))
            .collect::<BTreeMap<_, _>>();
        let island_membership_counts = island_membership_counts(support);
        let persistent_name_rows = persistent_names_by_canonical_loop(support);

        let mut recovered_loop_rows = Vec::new();
        for ledger_row in support.ledger_rows() {
            let role_outcome = role_outcomes
                .get(ledger_row.role_outcome_identity())
                .copied()
                .ok_or_else(|| dangling(ledger_row.tracked_loop_identity(), &mut counters))?;
            if role_outcome.loop_identity() != ledger_row.tracked_loop_identity() {
                return Err(dangling(ledger_row.tracked_loop_identity(), &mut counters));
            }
            let loop_role = role_outcome
                .preserved_source_role()
                .ok_or_else(|| contradictory(ledger_row.tracked_loop_identity(), &mut counters))?;
            let island_row = recover_island_row(
                ledger_row,
                &island_rows,
                &island_membership_counts,
                &mut counters,
            )?;
            let source_loop_witnesses =
                witnesses_for_ledger_row(ledger_row, support, &mut counters)?;
            let chain_lineage_ids = matching_chain_lineage_identities(ledger_row, support);
            recovered_loop_rows.push(PlanarBooleanLoopOverlapParticipationRow::new(
                format!(
                    "overlap-participation:loop:{}:{}",
                    ledger_row.canonical_loop_identity(),
                    island_row.island_identity()
                ),
                ledger_row.ledger_row_identity().to_string(),
                ledger_row.canonical_loop_identity().to_string(),
                ledger_row.tracked_loop_identity().to_string(),
                ledger_row.loop_kind(),
                loop_role,
                role_outcome.role_outcome_identity().to_string(),
                island_row.island_identity().to_string(),
                island_row.source_loop_identity().to_string(),
                island_row.kind(),
                source_loop_witnesses
                    .iter()
                    .map(|witness| witness.source_loop_identity.clone())
                    .collect(),
                source_loop_witnesses
                    .iter()
                    .map(|witness| witness.operand_side)
                    .collect(),
                source_loop_witnesses
                    .iter()
                    .map(|witness| witness.winding_sign)
                    .collect(),
                persistent_name_rows
                    .get(ledger_row.canonical_loop_identity())
                    .map(|rows| {
                        rows.iter()
                            .map(|row| row.propagated_persistent_name_identity().to_string())
                            .collect()
                    })
                    .unwrap_or_default(),
                chain_lineage_ids,
            ));
            counters.recovered_loop_row();
        }

        let loop_participation_map = PlanarBooleanLoopOverlapParticipationMap::new(
            map_identity(
                input.request().request_identity(),
                recovered_loop_rows
                    .iter()
                    .map(|row| row.participation_identity()),
            ),
            input.request().request_identity().to_string(),
            recovered_loop_rows,
        );

        let island_participation_rows =
            recover_island_participation_rows(&loop_participation_map, support, &mut counters)?;
        let island_participation_map = PlanarBooleanLoopIslandOverlapParticipationMap::new(
            map_identity(
                input.request().request_identity(),
                island_participation_rows
                    .iter()
                    .map(|row| row.participation_identity()),
            ),
            input.request().request_identity().to_string(),
            island_participation_rows,
        );

        let chain_lineage_rows =
            recover_chain_lineage_rows(&loop_participation_map, support, &mut counters)?;
        let chain_lineage_map = PlanarBooleanOverlapChainRegionLineageMap::new(
            map_identity(
                input.request().request_identity(),
                chain_lineage_rows
                    .iter()
                    .map(|row| row.lineage_row_identity()),
            ),
            input.request().request_identity().to_string(),
            chain_lineage_rows,
        );

        Ok(Self {
            loop_participation_map,
            island_participation_map,
            chain_lineage_map,
            counters,
        })
    }

    pub fn loop_participation_map(&self) -> &PlanarBooleanLoopOverlapParticipationMap {
        &self.loop_participation_map
    }

    pub fn island_participation_map(&self) -> &PlanarBooleanLoopIslandOverlapParticipationMap {
        &self.island_participation_map
    }

    pub fn chain_lineage_map(&self) -> &PlanarBooleanOverlapChainRegionLineageMap {
        &self.chain_lineage_map
    }

    pub fn counters(&self) -> PlanarBooleanOverlapParticipationRecoveryCounters {
        self.counters
    }
}

fn validate_request_binding(
    request: &super::super::readiness_boundary::PlanarBooleanOverlapRegionExtractionRequest,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<(), PlanarBooleanOverlapParticipationRecoveryDenial> {
    let binding = request.readiness_loop_ledger_binding();
    let receipt = support.loop_ledger_receipt();
    let mismatched = binding.loop_ledger_receipt_identity() != receipt.receipt_identity()
        || binding.loop_ledger_request_identity() != receipt.request_identity()
        || binding.persistent_name_map_identity() != receipt.persistent_name_map_identity()
        || binding.loop_ledger_row_identities() != receipt.ledger_row_identities();
    if mismatched {
        counters.denied_participation();
        return Err(PlanarBooleanOverlapParticipationRecoveryDenial::new(
            Kind::LoopLedgerParticipationSupportMismatch,
            request.request_identity(),
            *counters,
            "overlap participation requires the admitted overlap request to consume the same real 7.4 ledger products supplied for participation recovery",
        ));
    }
    if !support
        .overlap_chain_lineage_map()
        .certifies_canonical_identities()
    {
        counters.denied_participation();
        return Err(PlanarBooleanOverlapParticipationRecoveryDenial::new(
            Kind::ForeignOverlapChainLineageDenied,
            support
                .overlap_chain_lineage_map()
                .lineage_map_identity(),
            *counters,
            "overlap participation denies overlap-chain lineage whose preserved source provenance no longer matches the canonical real 7.4 lineage product",
        ));
    }
    Ok(())
}

fn dangling(
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> PlanarBooleanOverlapParticipationRecoveryDenial {
    counters.denied_participation();
    PlanarBooleanOverlapParticipationRecoveryDenial::new(
        Kind::DanglingLoopParticipationDenied,
        rejected_identity,
        *counters,
        "overlap participation denies loop references that cannot be recovered from the admitted request and real 7.4 ledger products",
    )
}

fn contradictory(
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> PlanarBooleanOverlapParticipationRecoveryDenial {
    counters.denied_participation();
    PlanarBooleanOverlapParticipationRecoveryDenial::new(
        Kind::ContradictoryIslandMembershipDenied,
        rejected_identity,
        *counters,
        "overlap participation denies contradictory loop-island or loop-role membership before adjacency construction",
    )
}
