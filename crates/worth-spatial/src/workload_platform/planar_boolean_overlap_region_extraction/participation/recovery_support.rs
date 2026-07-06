use std::collections::{BTreeMap, BTreeSet};

use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIslandPartitionRow, PlanarBooleanLoopPersistentNamePropagationRow,
    PlanarBooleanLoopReconstructionLedgerRow, PlanarBooleanLoopReconstructionParticipationSupport,
};

use super::chain_lineage::PlanarBooleanOverlapChainRegionLineageRow;
use super::counters::PlanarBooleanOverlapParticipationRecoveryCounters;
use super::denial::{
    PlanarBooleanOverlapParticipationRecoveryDenial,
    PlanarBooleanOverlapParticipationRecoveryDenialKind as Kind,
};
use super::island_participation::PlanarBooleanLoopIslandOverlapParticipationRow;
use super::lineage_binding::{
    ledger_source_edge_identities, lineage_binds_to_loop, lineage_touches_participating_surface,
};
use super::loop_participation::{
    PlanarBooleanLoopOverlapParticipationMap, PlanarBooleanLoopOverlapParticipationRow,
};
use super::source_loop_witnesses::{
    aligned_witnesses_for_lineage_row, witnesses_for_lineage_row, SourceLoopWitness,
};

pub(super) fn recover_island_row<'a>(
    ledger_row: &PlanarBooleanLoopReconstructionLedgerRow,
    island_rows: &'a BTreeMap<&str, &'a PlanarBooleanLoopIslandPartitionRow>,
    island_membership_counts: &BTreeMap<String, usize>,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<&'a PlanarBooleanLoopIslandPartitionRow, PlanarBooleanOverlapParticipationRecoveryDenial>
{
    if ledger_row.island_identities().len() != 1 {
        return Err(contradictory(ledger_row.tracked_loop_identity(), counters));
    }
    let island_identity = &ledger_row.island_identities()[0];
    let island_row = island_rows
        .get(island_identity.as_str())
        .copied()
        .ok_or_else(|| dangling(ledger_row.tracked_loop_identity(), counters))?;
    if island_membership_counts
        .get(ledger_row.tracked_loop_identity())
        .copied()
        .unwrap_or_default()
        != 1
    {
        return Err(contradictory(ledger_row.tracked_loop_identity(), counters));
    }
    if !island_row
        .member_loop_identities()
        .contains(&ledger_row.tracked_loop_identity().to_string())
    {
        return Err(contradictory(ledger_row.tracked_loop_identity(), counters));
    }
    Ok(island_row)
}

pub(super) fn recover_island_participation_rows(
    loop_map: &PlanarBooleanLoopOverlapParticipationMap,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<
    Vec<PlanarBooleanLoopIslandOverlapParticipationRow>,
    PlanarBooleanOverlapParticipationRecoveryDenial,
> {
    let rows_by_island = loop_map.rows().iter().fold(
        BTreeMap::<&str, Vec<&PlanarBooleanLoopOverlapParticipationRow>>::new(),
        |mut map, row| {
            map.entry(row.island_identity()).or_default().push(row);
            map
        },
    );
    let mut recovered = Vec::new();
    for island_row in support.island_partition().rows() {
        let Some(member_rows) = rows_by_island.get(island_row.island_identity()) else {
            continue;
        };
        let source_loop_witnesses = unique_source_loop_witnesses(member_rows);
        let mut propagated_names = member_rows
            .iter()
            .flat_map(|row| row.propagated_persistent_name_identities().iter().cloned())
            .collect::<Vec<_>>();
        propagated_names.sort();
        propagated_names.dedup();
        recovered.push(PlanarBooleanLoopIslandOverlapParticipationRow::new(
            format!(
                "overlap-participation:island:{}",
                island_row.island_identity()
            ),
            island_row.island_identity().to_string(),
            island_row.source_loop_identity().to_string(),
            island_row.kind(),
            member_rows
                .iter()
                .map(|row| row.tracked_loop_identity().to_string())
                .collect(),
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
            member_rows
                .iter()
                .map(|row| row.role_outcome_identity().to_string())
                .collect(),
            propagated_names,
        ));
        counters.recovered_island_row();
    }
    Ok(recovered)
}

pub(super) fn recover_chain_lineage_rows(
    loop_map: &PlanarBooleanLoopOverlapParticipationMap,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<
    Vec<PlanarBooleanOverlapChainRegionLineageRow>,
    PlanarBooleanOverlapParticipationRecoveryDenial,
> {
    if loop_map.rows().is_empty()
        && support.ledger_rows().is_empty()
        && !support.overlap_chain_lineage_map().rows().is_empty()
    {
        return recover_source_only_boundary_lineage_rows(support, counters);
    }

    let ledger_rows_by_canonical = support
        .ledger_rows()
        .iter()
        .map(|row| (row.canonical_loop_identity(), row))
        .collect::<BTreeMap<_, _>>();
    let participating_source_loop_identities = loop_map
        .rows()
        .iter()
        .flat_map(|row| row.source_loop_identities().iter().cloned())
        .collect::<BTreeSet<_>>();
    let participating_fragment_identities = loop_map
        .rows()
        .iter()
        .filter_map(|row| ledger_rows_by_canonical.get(row.canonical_loop_identity()))
        .flat_map(|row| row.fragment_identities().iter().cloned())
        .collect::<BTreeSet<_>>();
    let participating_source_edge_identities = loop_map
        .rows()
        .iter()
        .filter_map(|row| ledger_rows_by_canonical.get(row.canonical_loop_identity()))
        .flat_map(|ledger_row| ledger_source_edge_identities(ledger_row, support))
        .collect::<BTreeSet<_>>();
    let mut recovered = Vec::new();
    for lineage_row in support.overlap_chain_lineage_map().rows() {
        let matched_rows = loop_map
            .rows()
            .iter()
            .filter(|row| {
                ledger_rows_by_canonical
                    .get(row.canonical_loop_identity())
                    .is_some_and(|ledger_row| {
                        lineage_binds_to_loop(ledger_row, lineage_row, support)
                    })
            })
            .collect::<Vec<_>>();
        if matched_rows.is_empty()
            && lineage_touches_participating_surface(
                lineage_row,
                &participating_source_loop_identities,
                &participating_fragment_identities,
                &participating_source_edge_identities,
            )
        {
            counters.denied_participation();
            return Err(PlanarBooleanOverlapParticipationRecoveryDenial::new(
                Kind::ForeignOverlapChainLineageDenied,
                lineage_row.lineage_identity(),
                *counters,
                "overlap participation denies overlap-chain lineage that cannot be bound to participating loops from the real 7.4 ledger products",
            ));
        }
        if matched_rows.is_empty() {
            continue;
        }
        let source_loop_witnesses = witnesses_for_lineage_row(lineage_row, support, counters)?;
        let mut propagated_names = matched_rows
            .iter()
            .flat_map(|row| row.propagated_persistent_name_identities().iter().cloned())
            .collect::<Vec<_>>();
        propagated_names.sort();
        propagated_names.dedup();
        recovered.push(PlanarBooleanOverlapChainRegionLineageRow::new(
            format!(
                "overlap-participation:chain:{}",
                lineage_row.chain_identity()
            ),
            lineage_row.lineage_identity().to_string(),
            lineage_row.chain_identity().to_string(),
            lineage_row.fragment_identities().to_vec(),
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
            lineage_row.source_edge_identities().to_vec(),
            lineage_row.boundary_roles().to_vec(),
            matched_rows
                .iter()
                .map(|row| row.canonical_loop_identity().to_string())
                .collect(),
            matched_rows
                .iter()
                .map(|row| row.island_identity().to_string())
                .collect(),
            propagated_names,
        ));
        counters.recovered_chain_row();
    }
    Ok(recovered)
}

fn recover_source_only_boundary_lineage_rows(
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<
    Vec<PlanarBooleanOverlapChainRegionLineageRow>,
    PlanarBooleanOverlapParticipationRecoveryDenial,
> {
    let mut recovered = Vec::new();
    for lineage_row in support.overlap_chain_lineage_map().rows() {
        let aligned_witnesses = aligned_witnesses_for_lineage_row(lineage_row, support, counters)?;
        recovered.push(PlanarBooleanOverlapChainRegionLineageRow::new(
            format!(
                "overlap-participation:source-only-chain:{}",
                lineage_row.chain_identity()
            ),
            lineage_row.lineage_identity().to_string(),
            lineage_row.chain_identity().to_string(),
            lineage_row.fragment_identities().to_vec(),
            aligned_witnesses
                .iter()
                .map(|witness| witness.source_loop_identity.clone())
                .collect(),
            aligned_witnesses
                .iter()
                .map(|witness| witness.operand_side)
                .collect(),
            aligned_witnesses
                .iter()
                .map(|witness| witness.winding_sign)
                .collect(),
            lineage_row.source_edge_identities().to_vec(),
            lineage_row.boundary_roles().to_vec(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ));
        counters.recovered_chain_row();
    }
    Ok(recovered)
}

pub(super) fn persistent_names_by_canonical_loop(
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
) -> BTreeMap<&str, Vec<&PlanarBooleanLoopPersistentNamePropagationRow>> {
    let mut rows_by_loop =
        BTreeMap::<&str, Vec<&PlanarBooleanLoopPersistentNamePropagationRow>>::new();
    for row in support.persistent_name_map().rows() {
        rows_by_loop
            .entry(row.canonical_loop_identity())
            .or_default()
            .push(row);
    }
    rows_by_loop
}

pub(super) fn matching_chain_lineage_identities(
    ledger_row: &PlanarBooleanLoopReconstructionLedgerRow,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
) -> Vec<String> {
    support
        .overlap_chain_lineage_map()
        .rows()
        .iter()
        .filter(|row| lineage_binds_to_loop(ledger_row, row, support))
        .map(|row| row.lineage_identity().to_string())
        .collect()
}

pub(super) fn map_identity<'a>(
    request_identity: &str,
    row_identities: impl Iterator<Item = &'a str>,
) -> String {
    let joined = row_identities.collect::<Vec<_>>().join("|");
    format!("{request_identity}:{joined}")
}

pub(super) fn island_membership_counts(
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for island_row in support.island_partition().rows() {
        for tracked_loop_identity in island_row.member_loop_identities() {
            *counts.entry(tracked_loop_identity.clone()).or_insert(0) += 1;
        }
    }
    counts
}

fn unique_source_loop_witnesses(
    member_rows: &[&PlanarBooleanLoopOverlapParticipationRow],
) -> Vec<SourceLoopWitness> {
    let mut witnesses = member_rows
        .iter()
        .flat_map(|row| {
            row.source_loop_identities()
                .iter()
                .cloned()
                .zip(row.source_loop_operand_sides().iter().copied())
                .zip(row.source_loop_winding_signs().iter().copied())
                .map(
                    |((source_loop_identity, operand_side), winding_sign)| SourceLoopWitness {
                        source_loop_identity,
                        operand_side,
                        winding_sign,
                    },
                )
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    witnesses.sort_by_key(|witness| {
        (
            witness.source_loop_identity.clone(),
            witness.operand_side.query_key(),
            witness.winding_sign,
        )
    });
    witnesses.dedup_by(|left, right| {
        left.source_loop_identity == right.source_loop_identity
            && left.operand_side == right.operand_side
            && left.winding_sign == right.winding_sign
    });
    witnesses
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
