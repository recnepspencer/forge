use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChain, PlanarBooleanOverlapEdgeChainCounters,
    PlanarBooleanOverlapEdgeChainMember, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentCounters,
    PlanarBooleanSplitEdgeFragmentSchedule, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitSourceEdgeCarrierCounters, PlanarBooleanSplitSourceEdgeCarrierSet,
};

pub(crate) fn empty_recovered_source_carriers_for(
    carriers: &PlanarBooleanSplitSourceEdgeCarrierSet,
) -> PlanarBooleanSplitSourceEdgeCarrierSet {
    PlanarBooleanSplitSourceEdgeCarrierSet::new(
        carriers.scope_admission_identity().to_string(),
        carriers.split_request_identity().to_string(),
        carriers.event_ledger_identity().to_string(),
        carriers.segment_carrier_set_identity().to_string(),
        carriers.candidate_index_product_identity().to_string(),
        carriers.query_index_plan_digest().to_string(),
        Vec::new(),
        PlanarBooleanSplitSourceEdgeCarrierCounters::default(),
    )
}

pub(crate) fn missing_first_fragment_from_set(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let schedules = fragments
        .schedules()
        .iter()
        .filter_map(|schedule| {
            let remaining = schedule
                .fragments()
                .iter()
                .skip(1)
                .cloned()
                .collect::<Vec<_>>();
            if remaining.is_empty() {
                None
            } else {
                Some(clone_fragment_schedule(schedule, remaining))
            }
        })
        .collect::<Vec<_>>();
    clone_fragment_set(fragments, schedules, fragments.counters())
}

pub(crate) fn with_duplicate_first_fragment(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let schedules = fragments
        .schedules()
        .iter()
        .enumerate()
        .map(|(index, schedule)| {
            let mut rows = schedule.fragments().to_vec();
            if index == 0 {
                let duplicate = rows
                    .first()
                    .cloned()
                    .expect("test support requires at least one fragment to duplicate");
                rows.push(duplicate);
            }
            clone_fragment_schedule(schedule, rows)
        })
        .collect::<Vec<_>>();
    clone_fragment_set(fragments, schedules, fragments.counters())
}

pub(crate) fn foreign_fragment_membership_set(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let schedules = fragments
        .schedules()
        .iter()
        .enumerate()
        .map(|(index, schedule)| {
            let carrier_identity = if index == 0 {
                "foreign-carrier-for-membership".to_string()
            } else {
                schedule.carrier_identity().to_string()
            };
            PlanarBooleanSplitEdgeFragmentSchedule::new(
                schedule.schedule_identity().to_string(),
                schedule
                    .interval_subdivision_schedule_identity()
                    .to_string(),
                schedule
                    .split_vertex_identity_schedule_identity()
                    .to_string(),
                schedule.source_edge_identity().to_string(),
                carrier_identity,
                schedule.fragments().to_vec(),
            )
        })
        .collect::<Vec<_>>();
    clone_fragment_set(fragments, schedules, fragments.counters())
}

pub(crate) fn uncertified_coordinate_only_fragment_set(
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let counters = PlanarBooleanSplitEdgeFragmentCounters::new(
        fragments.counters().schedules_inspected(),
        0,
        fragments.counters().split_vertices_consumed(),
        fragments
            .counters()
            .original_endpoint_boundaries_synthesized(),
        fragments.counters().fragments_emitted(),
        fragments.counters().interval_attributed_fragments(),
        fragments.counters().endpoint_noop_boundaries_skipped(),
        fragments.counters().collapsed_fragments_rejected(),
        1,
        fragments.counters().foreign_schedule_rows_rejected(),
    );
    clone_fragment_set(fragments, fragments.schedules().to_vec(), counters)
}

pub(crate) fn missing_first_overlap_chain_from_set(
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    let chains = overlap_chains
        .chains()
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>();
    clone_overlap_chain_set(overlap_chains, chains, overlap_chains.counters())
}

pub(crate) fn overlap_chain_set_with_missing_member_membership(
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    let chains = overlap_chains
        .chains()
        .iter()
        .enumerate()
        .map(|(index, chain)| {
            if index != 0 {
                return chain.clone();
            }
            let members = chain
                .members()
                .iter()
                .enumerate()
                .map(|(member_index, member)| {
                    if member_index != 0 {
                        return member.clone();
                    }
                    PlanarBooleanOverlapEdgeChainMember::new(
                        member.member_identity().to_string(),
                        "missing-fragment-membership".to_string(),
                        member.interval_subdivision_identity().to_string(),
                        member.source_edge_identity().to_string(),
                        member.carrier_identity().to_string(),
                        member.fragment_parameter_range(),
                        member.source_interval_identity().to_string(),
                        member.source_parameter_range(),
                        member.source_sense(),
                        member.normalized_interval_identity().to_string(),
                        member.normalized_parameter_range(),
                        member.boundary_role(),
                        member.local_frame_identity().to_string(),
                        member.precision_basis_identity().to_string(),
                        member.event_group_identities().to_vec(),
                        member.provenance_identities().to_vec(),
                    )
                })
                .collect::<Vec<_>>();
            clone_overlap_chain(chain, members)
        })
        .collect::<Vec<_>>();
    clone_overlap_chain_set(overlap_chains, chains, overlap_chains.counters())
}

pub(crate) fn duplicate_overlap_chain_identity_set(
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    let mut chains = overlap_chains.chains().to_vec();
    let duplicate = chains
        .first()
        .cloned()
        .expect("test support requires at least one overlap chain to duplicate");
    chains.push(duplicate);
    clone_overlap_chain_set(overlap_chains, chains, overlap_chains.counters())
}

pub(crate) fn overlap_chain_set_with_topology_truth(
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    let counters = PlanarBooleanOverlapEdgeChainCounters::new(
        overlap_chains.counters().schedules_inspected(),
        overlap_chains.counters().interval_subdivisions_inspected(),
        overlap_chains.counters().fragment_rows_inspected(),
        overlap_chains.counters().chains_emitted(),
        overlap_chains.counters().chain_members_emitted(),
        overlap_chains.counters().partial_overlap_chains(),
        overlap_chains.counters().identical_parallel_chains(),
        overlap_chains.counters().identical_antiparallel_chains(),
        overlap_chains
            .counters()
            .different_parameterization_chains(),
        overlap_chains.counters().opposite_sense_chains(),
        overlap_chains
            .counters()
            .missing_fragment_references_rejected(),
        overlap_chains
            .counters()
            .missing_subdivision_references_rejected(),
        overlap_chains
            .counters()
            .mismatched_fragment_authority_rejected(),
        overlap_chains.counters().foreign_fragment_sets_rejected(),
        1,
    );
    clone_overlap_chain_set(overlap_chains, overlap_chains.chains().to_vec(), counters)
}

fn clone_fragment_set(
    original: &PlanarBooleanSplitEdgeFragmentSet,
    schedules: Vec<PlanarBooleanSplitEdgeFragmentSchedule>,
    counters: PlanarBooleanSplitEdgeFragmentCounters,
) -> PlanarBooleanSplitEdgeFragmentSet {
    PlanarBooleanSplitEdgeFragmentSet::new(
        original.fragment_set_identity().to_string(),
        original
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        original.split_vertex_identity_set_identity().to_string(),
        schedules,
        counters,
    )
}

fn clone_fragment_schedule(
    schedule: &PlanarBooleanSplitEdgeFragmentSchedule,
    fragments: Vec<PlanarBooleanSplitEdgeFragment>,
) -> PlanarBooleanSplitEdgeFragmentSchedule {
    PlanarBooleanSplitEdgeFragmentSchedule::new(
        schedule.schedule_identity().to_string(),
        schedule
            .interval_subdivision_schedule_identity()
            .to_string(),
        schedule
            .split_vertex_identity_schedule_identity()
            .to_string(),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        fragments,
    )
}

fn clone_overlap_chain_set(
    original: &PlanarBooleanOverlapEdgeChainSet,
    chains: Vec<PlanarBooleanOverlapEdgeChain>,
    counters: PlanarBooleanOverlapEdgeChainCounters,
) -> PlanarBooleanOverlapEdgeChainSet {
    PlanarBooleanOverlapEdgeChainSet::new(
        original.chain_set_identity().to_string(),
        original
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        original.split_edge_fragment_set_identity().to_string(),
        chains,
        counters,
    )
}

fn clone_overlap_chain(
    chain: &PlanarBooleanOverlapEdgeChain,
    members: Vec<PlanarBooleanOverlapEdgeChainMember>,
) -> PlanarBooleanOverlapEdgeChain {
    PlanarBooleanOverlapEdgeChain::new(
        chain.chain_identity().to_string(),
        chain.interval_event_identity().to_string(),
        chain.interval_event_kind(),
        chain.posture(),
        chain.source_interval_identities().to_vec(),
        chain.normalized_interval_identities().to_vec(),
        chain.source_senses().to_vec(),
        chain.event_group_identities().to_vec(),
        members,
    )
}
