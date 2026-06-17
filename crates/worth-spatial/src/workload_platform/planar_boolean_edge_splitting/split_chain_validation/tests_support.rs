use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_interval_entry, raw_schedule, raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::overlap_edge_chains::{
    PlanarBooleanOverlapEdgeChain, PlanarBooleanOverlapEdgeChainCounters,
    PlanarBooleanOverlapEdgeChainMember, PlanarBooleanOverlapEdgeChainSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentCounters,
    PlanarBooleanSplitEdgeFragmentEndpointRef, PlanarBooleanSplitEdgeFragmentSchedule,
    PlanarBooleanSplitEdgeFragmentSet,
};
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanMicroIntervalPolicy;

pub(super) fn prepared_split_products() -> (
    PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanOverlapEdgeChainSet,
) {
    let normalized = raw_set_from_schedules(vec![raw_schedule(
        "raw schedule",
        "source edge",
        "carrier",
        vec![raw_interval_entry(
            "interval",
            "source edge",
            "carrier",
            "event:interval",
            0.25,
        )],
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicate normalization should pass")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint normalization should pass")
    .normalize_overlap_interval_subdivisions(PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance)
    .expect("interval subdivisions should normalize");
    let vertices = normalized
        .mint_split_vertex_identities()
        .expect("split vertices should mint");
    let fragments = normalized
        .build_split_edge_fragments(&vertices)
        .expect("split fragments should build");
    let chains = normalized
        .build_overlap_edge_chains(&fragments)
        .expect("overlap chains should build");
    (fragments, chains)
}

pub(super) fn fragment_set_with_ranges(
    original: &PlanarBooleanSplitEdgeFragmentSet,
    ranges: &[[f64; 2]],
) -> PlanarBooleanSplitEdgeFragmentSet {
    let schedule = &original.schedules()[0];
    let fragments = ranges
        .iter()
        .enumerate()
        .map(|(index, range)| fragment_for_schedule(schedule, index, *range))
        .collect::<Vec<_>>();
    fragment_set_from_schedules(original, vec![schedule_with_fragments(schedule, fragments)])
}

pub(super) fn fragment_set_with_duplicate_identity_across_schedules(
    original: &PlanarBooleanSplitEdgeFragmentSet,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let schedule = &original.schedules()[0];
    let first = fragment_for_schedule(schedule, 0, [0.0, 1.0]);
    let duplicate = fragment_for_schedule(schedule, 99, [0.0, 1.0]);
    let duplicate = PlanarBooleanSplitEdgeFragment::new(
        first.fragment_identity().to_string(),
        duplicate.source_edge_identity().to_string(),
        duplicate.carrier_identity().to_string(),
        duplicate.start_endpoint().clone(),
        duplicate.end_endpoint().clone(),
        duplicate.parameter_range(),
        duplicate.parameter_range_bits(),
        duplicate.local_frame_identity().to_string(),
        duplicate.precision_basis_identity().to_string(),
        duplicate.source_senses().to_vec(),
        duplicate.point_cut_identities().to_vec(),
        duplicate.interval_subdivision_identities().to_vec(),
        duplicate.normalized_interval_identities().to_vec(),
        duplicate.event_group_identities().to_vec(),
        duplicate.cause_provenance_identities().to_vec(),
    );
    let second_schedule = PlanarBooleanSplitEdgeFragmentSchedule::new(
        "duplicate schedule".to_string(),
        "duplicate interval schedule".to_string(),
        "duplicate vertex schedule".to_string(),
        "other source edge".to_string(),
        "other carrier".to_string(),
        vec![duplicate],
    );
    fragment_set_from_schedules(
        original,
        vec![
            schedule_with_fragments(schedule, vec![first]),
            second_schedule,
        ],
    )
}

fn fragment_set_from_schedules(
    original: &PlanarBooleanSplitEdgeFragmentSet,
    schedules: Vec<PlanarBooleanSplitEdgeFragmentSchedule>,
) -> PlanarBooleanSplitEdgeFragmentSet {
    let fragments = schedules
        .iter()
        .map(|schedule| schedule.fragments().len())
        .sum();
    PlanarBooleanSplitEdgeFragmentSet::new(
        original.fragment_set_identity().to_string(),
        original
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        original.split_vertex_identity_set_identity().to_string(),
        schedules,
        PlanarBooleanSplitEdgeFragmentCounters::new(1, 1, 0, fragments, 0, 0, 0, 0, 0, 0),
    )
}

fn schedule_with_fragments(
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

fn fragment_for_schedule(
    schedule: &PlanarBooleanSplitEdgeFragmentSchedule,
    index: usize,
    range: [f64; 2],
) -> PlanarBooleanSplitEdgeFragment {
    let basis = &schedule.fragments()[0];
    PlanarBooleanSplitEdgeFragment::new(
        format!("forged-fragment:{index}:{}:{}", range[0], range[1]),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_start(
            schedule.source_edge_identity(),
            schedule.carrier_identity(),
            basis.local_frame_identity(),
            basis.precision_basis_identity(),
        ),
        PlanarBooleanSplitEdgeFragmentEndpointRef::original_source_end(
            schedule.source_edge_identity(),
            schedule.carrier_identity(),
            basis.local_frame_identity(),
            basis.precision_basis_identity(),
        ),
        range,
        [range[0].to_bits(), range[1].to_bits()],
        basis.local_frame_identity().to_string(),
        basis.precision_basis_identity().to_string(),
        basis.source_senses().to_vec(),
        Vec::new(),
        basis.interval_subdivision_identities().to_vec(),
        basis.normalized_interval_identities().to_vec(),
        basis.event_group_identities().to_vec(),
        basis.cause_provenance_identities().to_vec(),
    )
}

pub(super) fn chain_set_with_first_member_fragment(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    fragment_identity: &str,
) -> PlanarBooleanOverlapEdgeChainSet {
    chain_set_with_first_member(chains, |member| {
        member_from(
            member,
            fragment_identity.to_string(),
            member.fragment_parameter_range(),
        )
    })
}

pub(super) fn chain_set_with_first_member_fragment_and_range(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    fragment_identity: &str,
    range: [f64; 2],
) -> PlanarBooleanOverlapEdgeChainSet {
    chain_set_with_first_member(chains, |member| {
        member_from(member, fragment_identity.to_string(), range)
    })
}

pub(super) fn chain_set_with_foreign_interval_schedule(
    chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    chain_set_from_chains(
        chains,
        "foreign interval schedule".to_string(),
        chains.chains().to_vec(),
    )
}

pub(super) fn chain_set_with_conflicting_source_interval_basis(
    chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    let chain = &chains.chains()[0];
    let mut members = chain.members().to_vec();
    let first = members[0].clone();
    let source_range = first.source_parameter_range();
    members.push(member_from_with_source_range(
        &first,
        first.fragment_identity().to_string(),
        first.fragment_parameter_range(),
        [source_range[0], 1.0],
    ));
    chain_set_with_members(chains, members)
}

pub(super) fn chain_set_with_malformed_source_interval_basis(
    chains: &PlanarBooleanOverlapEdgeChainSet,
) -> PlanarBooleanOverlapEdgeChainSet {
    chain_set_with_first_member(chains, |member| {
        member_from_with_source_range(
            member,
            member.fragment_identity().to_string(),
            member.fragment_parameter_range(),
            [f64::NAN, member.source_parameter_range()[1]],
        )
    })
}

fn chain_set_with_first_member(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    replace: impl FnOnce(&PlanarBooleanOverlapEdgeChainMember) -> PlanarBooleanOverlapEdgeChainMember,
) -> PlanarBooleanOverlapEdgeChainSet {
    let chain = &chains.chains()[0];
    let mut members = chain.members().to_vec();
    members[0] = replace(&members[0]);
    chain_set_with_members(chains, members)
}

fn chain_set_with_members(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    members: Vec<PlanarBooleanOverlapEdgeChainMember>,
) -> PlanarBooleanOverlapEdgeChainSet {
    let chain = &chains.chains()[0];
    let forged_chain = PlanarBooleanOverlapEdgeChain::new(
        chain.chain_identity().to_string(),
        chain.interval_event_identity().to_string(),
        chain.interval_event_kind(),
        chain.posture(),
        chain.source_interval_identities().to_vec(),
        chain.normalized_interval_identities().to_vec(),
        chain.source_senses().to_vec(),
        chain.event_group_identities().to_vec(),
        members,
    );
    chain_set_from_chains(
        chains,
        chains
            .interval_subdivision_schedule_set_identity()
            .to_string(),
        vec![forged_chain],
    )
}

fn chain_set_from_chains(
    chains: &PlanarBooleanOverlapEdgeChainSet,
    interval_subdivision_schedule_set_identity: String,
    chain_rows: Vec<PlanarBooleanOverlapEdgeChain>,
) -> PlanarBooleanOverlapEdgeChainSet {
    let members = chain_rows.iter().map(|chain| chain.members().len()).sum();
    PlanarBooleanOverlapEdgeChainSet::new(
        chains.chain_set_identity().to_string(),
        interval_subdivision_schedule_set_identity,
        chains.split_edge_fragment_set_identity().to_string(),
        chain_rows,
        PlanarBooleanOverlapEdgeChainCounters::new(
            1, 1, 3, 1, members, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ),
    )
}

fn member_from(
    member: &PlanarBooleanOverlapEdgeChainMember,
    fragment_identity: String,
    fragment_range: [f64; 2],
) -> PlanarBooleanOverlapEdgeChainMember {
    member_from_with_source_range(
        member,
        fragment_identity,
        fragment_range,
        member.source_parameter_range(),
    )
}

fn member_from_with_source_range(
    member: &PlanarBooleanOverlapEdgeChainMember,
    fragment_identity: String,
    fragment_range: [f64; 2],
    source_range: [f64; 2],
) -> PlanarBooleanOverlapEdgeChainMember {
    PlanarBooleanOverlapEdgeChainMember::new(
        format!("{}:mutated-source-range", member.member_identity()),
        fragment_identity,
        member.interval_subdivision_identity().to_string(),
        member.source_edge_identity().to_string(),
        member.carrier_identity().to_string(),
        fragment_range,
        member.source_interval_identity().to_string(),
        source_range,
        member.source_sense(),
        member.normalized_interval_identity().to_string(),
        member.normalized_parameter_range(),
        member.boundary_role(),
        member.local_frame_identity().to_string(),
        member.precision_basis_identity().to_string(),
        member.event_group_identities().to_vec(),
        member.provenance_identities().to_vec(),
    )
}
