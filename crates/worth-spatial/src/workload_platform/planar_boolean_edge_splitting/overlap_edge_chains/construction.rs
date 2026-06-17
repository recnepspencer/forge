use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::canonical_parameter::canonical_parameter_bits;
use crate::workload_platform::planar_boolean_edge_splitting::micro_interval_normalization::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanNormalizedIntervalSubdivisionRow,
};
use crate::workload_platform::planar_boolean_edge_splitting::split_edge_fragments::{
    PlanarBooleanSplitEdgeFragment, PlanarBooleanSplitEdgeFragmentSet,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanIntervalEventKind, PlanarBooleanSourceIntervalSense,
};

use super::boundary_role::{
    PlanarBooleanOverlapChainBoundaryRole, PlanarBooleanOverlapChainPosture,
};
use super::chain_member::PlanarBooleanOverlapEdgeChainMember;
use super::chain_row::PlanarBooleanOverlapEdgeChain;
use super::chain_set::PlanarBooleanOverlapEdgeChainSet;
use super::counters::PlanarBooleanOverlapEdgeChainCounters;
use super::denial::PlanarBooleanOverlapEdgeChainDenial;
use super::identity::{
    overlap_chain_identity, overlap_chain_member_identity, overlap_chain_set_identity,
};
use super::indexed_inputs::OverlapChainIndexedInputs;
use super::validation::{reject_ambiguous_chain_basis, reject_foreign_fragment_set};

impl PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
    pub fn build_overlap_edge_chains(
        &self,
        fragments: &PlanarBooleanSplitEdgeFragmentSet,
    ) -> Result<PlanarBooleanOverlapEdgeChainSet, PlanarBooleanOverlapEdgeChainDenial> {
        reject_foreign_fragment_set(self, fragments)?;
        let indexed = OverlapChainIndexedInputs::new(self, fragments)?;
        let mut counters =
            CounterBuild::new(self.schedules().len(), indexed.fragment_rows_inspected());
        let mut grouped =
            BTreeMap::<String, Vec<&PlanarBooleanNormalizedIntervalSubdivisionRow>>::new();
        for schedule in self.schedules() {
            for subdivision in schedule.interval_subdivisions() {
                counters.interval_subdivisions_inspected += 1;
                grouped
                    .entry(subdivision.interval_event_identity().to_string())
                    .or_default()
                    .push(subdivision);
            }
        }
        let mut chains = Vec::with_capacity(grouped.len());
        for (event_identity, subdivisions) in grouped {
            reject_ambiguous_chain_basis(&subdivisions)?;
            let chain = build_chain(&event_identity, &subdivisions, &indexed, &mut counters)?;
            chains.push(chain);
        }
        chains.sort_by(|a, b| a.chain_identity().cmp(b.chain_identity()));
        let chain_set_identity = overlap_chain_set_identity(
            self.schedule_set_identity(),
            fragments.fragment_set_identity(),
            &chains,
        );
        Ok(PlanarBooleanOverlapEdgeChainSet::new(
            chain_set_identity,
            self.schedule_set_identity().to_string(),
            fragments.fragment_set_identity().to_string(),
            chains,
            counters.finish(),
        ))
    }
}

fn build_chain(
    event_identity: &str,
    subdivisions: &[&PlanarBooleanNormalizedIntervalSubdivisionRow],
    indexed: &OverlapChainIndexedInputs<'_>,
    counters: &mut CounterBuild,
) -> Result<PlanarBooleanOverlapEdgeChain, PlanarBooleanOverlapEdgeChainDenial> {
    let first = subdivisions[0];
    let interval_event_kind = first.interval_event_kind();
    let posture = PlanarBooleanOverlapChainPosture::from_interval_kind(interval_event_kind);
    let mut members = Vec::new();
    for subdivision in subdivisions {
        for fragment in indexed.fragments_for_subdivision(subdivision)? {
            members.push(member_from_fragment(subdivision, fragment));
        }
    }
    members.sort_by(|a, b| {
        a.source_edge_identity()
            .cmp(b.source_edge_identity())
            .then_with(|| a.carrier_identity().cmp(b.carrier_identity()))
            .then_with(|| {
                a.fragment_parameter_range()[0].total_cmp(&b.fragment_parameter_range()[0])
            })
            .then_with(|| a.member_identity().cmp(b.member_identity()))
    });
    counters.record_chain(interval_event_kind, &members);
    let source_interval_identities = canonical_strings(
        members
            .iter()
            .map(|member| member.source_interval_identity().to_string()),
    );
    let normalized_interval_identities = canonical_strings(
        members
            .iter()
            .map(|member| member.normalized_interval_identity().to_string()),
    );
    let source_senses = canonical_source_senses(members.iter().map(|member| member.source_sense()));
    let event_group_identities = canonical_strings(
        members
            .iter()
            .flat_map(|member| member.event_group_identities().iter().cloned()),
    );
    let chain_identity =
        overlap_chain_identity(event_identity, interval_event_kind, posture, &members);
    Ok(PlanarBooleanOverlapEdgeChain::new(
        chain_identity,
        event_identity.to_string(),
        interval_event_kind,
        posture,
        source_interval_identities,
        normalized_interval_identities,
        source_senses,
        event_group_identities,
        members,
    ))
}

fn member_from_fragment(
    subdivision: &PlanarBooleanNormalizedIntervalSubdivisionRow,
    fragment: &PlanarBooleanSplitEdgeFragment,
) -> PlanarBooleanOverlapEdgeChainMember {
    let role = boundary_role(
        subdivision.admitted_parameter_range(),
        fragment.parameter_range(),
    );
    let member_identity = overlap_chain_member_identity(
        subdivision.interval_event_identity(),
        subdivision.subdivision_identity(),
        fragment.fragment_identity(),
        subdivision.source_sense(),
        role,
        fragment.parameter_range(),
    );
    let event_group_identities = canonical_strings(
        subdivision
            .event_group_identities()
            .iter()
            .cloned()
            .chain(fragment.event_group_identities().iter().cloned()),
    );
    let provenance_identities = canonical_strings(
        subdivision
            .provenance_entry_identities()
            .iter()
            .cloned()
            .chain(fragment.cause_provenance_identities().iter().cloned()),
    );
    PlanarBooleanOverlapEdgeChainMember::new(
        member_identity,
        fragment.fragment_identity().to_string(),
        subdivision.subdivision_identity().to_string(),
        subdivision.source_edge_identity().to_string(),
        subdivision.carrier_identity().to_string(),
        fragment.parameter_range(),
        subdivision.source_interval_identity().to_string(),
        subdivision.source_parameter_range(),
        subdivision.source_sense(),
        subdivision.normalized_interval_identity().to_string(),
        subdivision.normalized_parameter_range(),
        role,
        subdivision.local_frame_identity().to_string(),
        subdivision.precision_basis_identity().to_string(),
        event_group_identities,
        provenance_identities,
    )
}

fn boundary_role(
    subdivision_range: [f64; 2],
    fragment_range: [f64; 2],
) -> PlanarBooleanOverlapChainBoundaryRole {
    let same_start = canonical_parameter_bits(subdivision_range[0])
        == canonical_parameter_bits(fragment_range[0]);
    let same_end = canonical_parameter_bits(subdivision_range[1])
        == canonical_parameter_bits(fragment_range[1]);
    match (same_start, same_end) {
        (true, true) => PlanarBooleanOverlapChainBoundaryRole::FullOverlapSpan,
        (true, false) => PlanarBooleanOverlapChainBoundaryRole::OverlapStartBoundary,
        (false, true) => PlanarBooleanOverlapChainBoundaryRole::OverlapEndBoundary,
        (false, false) => PlanarBooleanOverlapChainBoundaryRole::OverlapInteriorFragment,
    }
}

fn canonical_strings(values: impl Iterator<Item = String>) -> Vec<String> {
    let mut values = values.collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

fn canonical_source_senses(
    values: impl Iterator<Item = PlanarBooleanSourceIntervalSense>,
) -> Vec<PlanarBooleanSourceIntervalSense> {
    let mut values = values.collect::<Vec<_>>();
    values.sort();
    values.dedup();
    values
}

struct CounterBuild {
    schedules_inspected: usize,
    interval_subdivisions_inspected: usize,
    fragment_rows_inspected: usize,
    chains_emitted: usize,
    chain_members_emitted: usize,
    partial_overlap_chains: usize,
    identical_parallel_chains: usize,
    identical_antiparallel_chains: usize,
    different_parameterization_chains: usize,
    opposite_sense_chains: usize,
}

impl CounterBuild {
    fn new(schedules_inspected: usize, fragment_rows_inspected: usize) -> Self {
        Self {
            schedules_inspected,
            interval_subdivisions_inspected: 0,
            fragment_rows_inspected,
            chains_emitted: 0,
            chain_members_emitted: 0,
            partial_overlap_chains: 0,
            identical_parallel_chains: 0,
            identical_antiparallel_chains: 0,
            different_parameterization_chains: 0,
            opposite_sense_chains: 0,
        }
    }

    fn record_chain(
        &mut self,
        interval_event_kind: PlanarBooleanIntervalEventKind,
        members: &[PlanarBooleanOverlapEdgeChainMember],
    ) {
        self.chains_emitted += 1;
        self.chain_members_emitted += members.len();
        match interval_event_kind {
            PlanarBooleanIntervalEventKind::PartialOverlap => self.partial_overlap_chains += 1,
            PlanarBooleanIntervalEventKind::ContainmentOverlap => {
                self.different_parameterization_chains += 1
            }
            PlanarBooleanIntervalEventKind::IdenticalSameDirection => {
                self.identical_parallel_chains += 1
            }
            PlanarBooleanIntervalEventKind::IdenticalAntiParallel => {
                self.identical_antiparallel_chains += 1
            }
        }
        if members
            .iter()
            .any(|member| member.source_sense() == PlanarBooleanSourceIntervalSense::Reversed)
        {
            self.opposite_sense_chains += 1;
        }
    }

    fn finish(self) -> PlanarBooleanOverlapEdgeChainCounters {
        PlanarBooleanOverlapEdgeChainCounters::new(
            self.schedules_inspected,
            self.interval_subdivisions_inspected,
            self.fragment_rows_inspected,
            self.chains_emitted,
            self.chain_members_emitted,
            self.partial_overlap_chains,
            self.identical_parallel_chains,
            self.identical_antiparallel_chains,
            self.different_parameterization_chains,
            self.opposite_sense_chains,
            0,
            0,
            0,
            0,
            0,
        )
    }
}
