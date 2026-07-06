use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense::{
    Forward, Reversed,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopOverlapChainLineageRow, PlanarBooleanLoopReconstructionLedgerRow,
    PlanarBooleanLoopReconstructionParticipationSupport,
};

use super::counters::PlanarBooleanOverlapParticipationRecoveryCounters;
use super::denial::{
    PlanarBooleanOverlapParticipationRecoveryDenial,
    PlanarBooleanOverlapParticipationRecoveryDenialKind as Kind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct SourceLoopWitness {
    pub source_loop_identity: String,
    pub operand_side: PlanarBooleanCommonPlaneOperandSide,
    pub winding_sign: i8,
}

pub(super) fn witnesses_for_ledger_row(
    ledger_row: &PlanarBooleanLoopReconstructionLedgerRow,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    witnesses_for_fragments(
        ledger_row.fragment_identities(),
        ledger_row.tracked_loop_identity(),
        support,
        counters,
    )
    .or_else(|_| {
        witnesses_for_source_loops(
            ledger_row.source_loop_identities(),
            ledger_row.tracked_loop_identity(),
            support,
            counters,
        )
    })
}

pub(super) fn witnesses_for_lineage_row(
    lineage_row: &PlanarBooleanLoopOverlapChainLineageRow,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    witnesses_for_fragments(
        lineage_row.fragment_identities(),
        lineage_row.lineage_identity(),
        support,
        counters,
    )
    .or_else(|_| {
        witnesses_for_source_edges_or_loops(
            lineage_row.source_edge_identities(),
            lineage_row.source_loop_identities(),
            lineage_row.lineage_identity(),
            support,
            counters,
        )
    })
}

pub(super) fn aligned_witnesses_for_lineage_row(
    lineage_row: &PlanarBooleanLoopOverlapChainLineageRow,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    let mut witnesses = Vec::new();
    for fragment_identity in lineage_row.fragment_identities() {
        let Some(membership) = support
            .fragment_membership_map()
            .membership_for_fragment_identity(fragment_identity)
        else {
            return Err(dangling(lineage_row.lineage_identity(), counters));
        };
        let source_loop_carrier = support
            .source_loop_carriers()
            .carrier_for_identity(membership.carrier_identity())
            .ok_or_else(|| dangling(lineage_row.lineage_identity(), counters))?;
        let winding_sign =
            canonical_source_loop_winding_sign(membership.source_senses()).unwrap_or(0);
        witnesses.push(SourceLoopWitness {
            source_loop_identity: membership.source_loop_identity().to_string(),
            operand_side: source_loop_carrier.operand_side(),
            winding_sign,
        });
    }
    if witnesses.is_empty() {
        return Err(dangling(lineage_row.lineage_identity(), counters));
    }
    Ok(witnesses)
}

fn witnesses_for_fragments(
    fragment_identities: &[String],
    rejected_identity: &str,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    let fragment_identity_set = fragment_identities
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut witnesses = support
        .fragment_membership_map()
        .rows()
        .iter()
        .filter(|row| fragment_identity_set.contains(row.fragment_identity()))
        .map(|row| {
            let source_loop_carrier = support
                .source_loop_carriers()
                .carrier_for_identity(row.carrier_identity())
                .ok_or_else(|| dangling(rejected_identity, counters))?;
            let winding_sign = canonical_source_loop_winding_sign(row.source_senses())
                .ok_or_else(|| contradictory(rejected_identity, counters))?;
            Ok(SourceLoopWitness {
                source_loop_identity: row.source_loop_identity().to_string(),
                operand_side: source_loop_carrier.operand_side(),
                winding_sign,
            })
        })
        .collect::<Result<Vec<_>, PlanarBooleanOverlapParticipationRecoveryDenial>>()?;
    if witnesses.is_empty() {
        return Err(dangling(rejected_identity, counters));
    }
    witnesses.sort_by_key(witness_sort_key);
    witnesses.dedup_by(|left, right| witness_sort_key(left) == witness_sort_key(right));
    Ok(witnesses)
}

fn witnesses_for_source_edges_or_loops(
    source_edge_identities: &[String],
    source_loop_identities: &[String],
    rejected_identity: &str,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    let source_edge_identity_set = source_edge_identities
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let witnesses = support
        .fragment_membership_map()
        .rows()
        .iter()
        .filter(|row| source_edge_identity_set.contains(row.source_edge_identity()))
        .map(|row| {
            let source_loop_carrier = support
                .source_loop_carriers()
                .carrier_for_identity(row.carrier_identity())
                .ok_or_else(|| dangling(rejected_identity, counters))?;
            let winding_sign = canonical_source_loop_winding_sign(row.source_senses())
                .ok_or_else(|| contradictory(rejected_identity, counters))?;
            Ok(SourceLoopWitness {
                source_loop_identity: row.source_loop_identity().to_string(),
                operand_side: source_loop_carrier.operand_side(),
                winding_sign,
            })
        })
        .collect::<Result<Vec<_>, PlanarBooleanOverlapParticipationRecoveryDenial>>()?;
    if witnesses.is_empty() {
        return witnesses_for_source_loops(
            source_loop_identities,
            rejected_identity,
            support,
            counters,
        );
    }
    dedup_witnesses(witnesses, rejected_identity, counters)
}

fn witnesses_for_source_loops(
    source_loop_identities: &[String],
    rejected_identity: &str,
    support: &PlanarBooleanLoopReconstructionParticipationSupport,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    let source_loop_identity_set = source_loop_identities
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let mut witnesses = Vec::new();
    for source_loop_identity in source_loop_identities {
        let source_senses = support
            .fragment_membership_map()
            .rows()
            .iter()
            .filter(|row| row.source_loop_identity() == source_loop_identity)
            .flat_map(|row| row.source_senses().iter().cloned())
            .collect::<Vec<_>>();
        let winding_sign = canonical_source_loop_winding_sign(&source_senses)
            .ok_or_else(|| contradictory(rejected_identity, counters))?;
        let matching_carriers = support
            .source_loop_carriers()
            .rows()
            .iter()
            .filter(|row| source_loop_identity_set.contains(row.source_loop_identity()))
            .filter(|row| row.source_loop_identity() == source_loop_identity)
            .collect::<Vec<_>>();
        if matching_carriers.is_empty() {
            return Err(dangling(rejected_identity, counters));
        }
        for carrier in matching_carriers {
            witnesses.push(SourceLoopWitness {
                source_loop_identity: source_loop_identity.clone(),
                operand_side: carrier.operand_side(),
                winding_sign,
            });
        }
    }
    dedup_witnesses(witnesses, rejected_identity, counters)
}

fn dedup_witnesses(
    mut witnesses: Vec<SourceLoopWitness>,
    rejected_identity: &str,
    counters: &mut PlanarBooleanOverlapParticipationRecoveryCounters,
) -> Result<Vec<SourceLoopWitness>, PlanarBooleanOverlapParticipationRecoveryDenial> {
    if witnesses.is_empty() {
        return Err(dangling(rejected_identity, counters));
    }
    witnesses.sort_by_key(witness_sort_key);
    witnesses.dedup_by(|left, right| witness_sort_key(left) == witness_sort_key(right));
    Ok(witnesses)
}

fn canonical_source_loop_winding_sign(
    source_senses: &[crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense],
) -> Option<i8> {
    let mut sign = None;
    for source_sense in source_senses {
        let next_sign = match source_sense {
            Forward => 1,
            Reversed => -1,
        };
        match sign {
            Some(existing) if existing != next_sign => return None,
            Some(_) => {}
            None => sign = Some(next_sign),
        }
    }
    sign
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

fn witness_sort_key(witness: &SourceLoopWitness) -> (String, &'static str, i8) {
    (
        witness.source_loop_identity.clone(),
        witness.operand_side.query_key(),
        witness.winding_sign,
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
