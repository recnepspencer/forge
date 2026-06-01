use std::collections::BTreeSet;

use crate::authority::commit::preparation::facade::PreparedInvariantExecution;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationPartitionScope, PreparationReadSetApproximation,
};
use crate::validation::engine::{
    InvariantPlanScopeClass, InvariantProofBoundarySummary, InvariantScopeWideningCause,
};

pub(crate) fn planned_proof_boundary_summary(
    planned: &PreparedInvariantExecution<'_>,
) -> InvariantProofBoundarySummary {
    let mut widened_causes = Vec::new();
    let mut touched_partitions = BTreeSet::new();
    let mut saw_touched_only = false;

    for packet in &planned.packets {
        collect_partition_scope_evidence(packet, &mut widened_causes, &mut touched_partitions);
        collect_read_set_evidence(packet, &mut widened_causes, &mut saw_touched_only);
    }

    InvariantProofBoundarySummary::new(
        proof_scope_class(&widened_causes, saw_touched_only),
        widened_causes,
        planned.packets.len(),
        touched_partitions.len(),
    )
}

fn collect_partition_scope_evidence(
    packet: &crate::authority::commit::preparation::InvariantWorkPacket<'_>,
    widened_causes: &mut Vec<InvariantScopeWideningCause>,
    touched_partitions: &mut BTreeSet<crate::identity::data::PartitionId>,
) {
    match &packet.locality.partition_scope {
        PreparationPartitionScope::AllObserved => {
            if !widened_causes.contains(&InvariantScopeWideningCause::AllObservedPartitionScope) {
                widened_causes.push(InvariantScopeWideningCause::AllObservedPartitionScope);
            }
        }
        PreparationPartitionScope::TouchedPartitions(partitions) => {
            touched_partitions.extend(partitions.iter().copied());
        }
    }
}

fn collect_read_set_evidence(
    packet: &crate::authority::commit::preparation::InvariantWorkPacket<'_>,
    widened_causes: &mut Vec<InvariantScopeWideningCause>,
    saw_touched_only: &mut bool,
) {
    match packet.locality.read_set_approximation {
        PreparationReadSetApproximation::FullObservedScan => {
            if !widened_causes.contains(&InvariantScopeWideningCause::FullObservedReadSet) {
                widened_causes.push(InvariantScopeWideningCause::FullObservedReadSet);
            }
        }
        PreparationReadSetApproximation::TouchedOnly => {
            *saw_touched_only = true;
        }
        PreparationReadSetApproximation::SharedCommittedRead => {}
    }
}

fn proof_scope_class(
    widened_causes: &[InvariantScopeWideningCause],
    saw_touched_only: bool,
) -> InvariantPlanScopeClass {
    if !widened_causes.is_empty() {
        InvariantPlanScopeClass::BroaderScope
    } else if saw_touched_only {
        InvariantPlanScopeClass::TouchedScope
    } else {
        InvariantPlanScopeClass::PartitionScope
    }
}
