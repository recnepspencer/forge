use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::{
    PlanarBooleanNormalizedEdgeSplitSchedule, PlanarBooleanNormalizedEdgeSplitScheduleSet,
    PlanarBooleanNormalizedSplitCut,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanPointSplitPosture, PlanarBooleanRawEdgeSplitScheduleEntryKind,
};

use super::action::PlanarBooleanEndpointBoundarySplitAction;
use super::boundary_position::PlanarBooleanSplitBoundaryPosition;
use super::counters::PlanarBooleanEndpointBoundaryNormalizationCounters;
use super::decision_record::PlanarBooleanEndpointContactDecision;
use super::denial::{
    PlanarBooleanEndpointBoundaryNormalizationDenial,
    PlanarBooleanEndpointBoundaryNormalizationDenialKind,
};
use super::identity::{
    endpoint_boundary_schedule_identity, endpoint_boundary_schedule_set_identity,
    endpoint_contact_decision_identity, EndpointContactDecisionIdentityBasis,
};
use super::normalized_schedule::{
    PlanarBooleanEndpointBoundaryNormalizedSplitSchedule,
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
};

impl PlanarBooleanNormalizedEdgeSplitScheduleSet {
    pub fn normalize_endpoint_boundary_splits(
        &self,
    ) -> Result<
        PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        PlanarBooleanEndpointBoundaryNormalizationDenial,
    > {
        let mut schedules = Vec::with_capacity(self.schedules().len());
        let mut counters = CounterBuild::default();
        for schedule in self.schedules() {
            schedules.push(normalize_schedule(schedule, &mut counters)?);
        }
        let set_identity =
            endpoint_boundary_schedule_set_identity(self.schedule_set_identity(), &schedules);
        Ok(
            PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet::new(
                set_identity,
                self.schedule_set_identity().to_string(),
                schedules,
                counters.finish(self.schedules().len()),
            ),
        )
    }
}

fn normalize_schedule(
    schedule: &PlanarBooleanNormalizedEdgeSplitSchedule,
    counters: &mut CounterBuild,
) -> Result<
    PlanarBooleanEndpointBoundaryNormalizedSplitSchedule,
    PlanarBooleanEndpointBoundaryNormalizationDenial,
> {
    let mut fragment_cuts = Vec::new();
    let mut decisions = Vec::new();
    for cut in schedule.cuts() {
        counters.inspected_point_cuts += 1;
        match classify_cut_action(cut)? {
            PlanarBooleanEndpointBoundarySplitAction::FragmentCut => {
                counters.fragment_point_cuts += 1;
                fragment_cuts.push(cut.clone());
            }
            PlanarBooleanEndpointBoundarySplitAction::EndpointContactDecision => {
                decisions.push(endpoint_contact_decision_from_cut(cut)?);
                counters.record_decision(cut);
            }
        }
    }
    let retained_interval_entries = schedule.retained_interval_entries().to_vec();
    let retained_interval_entry_identities = retained_interval_entries
        .iter()
        .map(|entry| entry.entry_identity().to_string())
        .collect::<Vec<_>>();
    counters.retained_interval_entries += retained_interval_entries.len();
    let fragment_cut_identities = fragment_cuts
        .iter()
        .map(|cut| cut.cut_identity().to_string())
        .collect::<Vec<_>>();
    let schedule_identity = endpoint_boundary_schedule_identity(
        schedule.schedule_identity(),
        &fragment_cut_identities,
        &decisions,
        &retained_interval_entry_identities,
    );
    Ok(PlanarBooleanEndpointBoundaryNormalizedSplitSchedule::new(
        schedule_identity,
        schedule.schedule_identity().to_string(),
        schedule.source_edge_identity().to_string(),
        schedule.carrier_identity().to_string(),
        fragment_cuts,
        decisions,
        retained_interval_entries,
    ))
}

fn classify_cut_action(
    cut: &PlanarBooleanNormalizedSplitCut,
) -> Result<
    PlanarBooleanEndpointBoundarySplitAction,
    PlanarBooleanEndpointBoundaryNormalizationDenial,
> {
    let boundary_position = PlanarBooleanSplitBoundaryPosition::from_parameter(cut.parameter());
    let PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) = cut.kind() else {
        return Ok(PlanarBooleanEndpointBoundarySplitAction::FragmentCut);
    };
    match (posture, boundary_position.is_boundary()) {
        (PlanarBooleanPointSplitPosture::EndpointNoOp, true)
        | (PlanarBooleanPointSplitPosture::SharedEndpoint, true)
        | (PlanarBooleanPointSplitPosture::TJunctionPromotion, true) => {
            Ok(PlanarBooleanEndpointBoundarySplitAction::EndpointContactDecision)
        }
        (PlanarBooleanPointSplitPosture::InteriorSplit, true) => Err(
            PlanarBooleanEndpointBoundaryNormalizationDenial::new(
                PlanarBooleanEndpointBoundaryNormalizationDenialKind::EndpointSplitWouldCreateZeroLengthFragment,
                cut.cut_identity(),
                "interior split posture at a source endpoint would create a zero-length fragment",
            ),
        ),
        (PlanarBooleanPointSplitPosture::EndpointNoOp, false)
        | (PlanarBooleanPointSplitPosture::SharedEndpoint, false) => Err(
            PlanarBooleanEndpointBoundaryNormalizationDenial::new(
                PlanarBooleanEndpointBoundaryNormalizationDenialKind::ContradictoryBoundaryAction,
                cut.cut_identity(),
                "endpoint no-op and shared endpoint split postures must occur at a source boundary",
            ),
        ),
        _ => Ok(PlanarBooleanEndpointBoundarySplitAction::FragmentCut),
    }
}

fn endpoint_contact_decision_from_cut(
    cut: &PlanarBooleanNormalizedSplitCut,
) -> Result<PlanarBooleanEndpointContactDecision, PlanarBooleanEndpointBoundaryNormalizationDenial>
{
    let boundary_position = PlanarBooleanSplitBoundaryPosition::from_parameter(cut.parameter());
    let PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) = cut.kind() else {
        unreachable!("only point cuts can become endpoint contact decisions");
    };
    let source_endpoint_identity = cut
        .exact_endpoint_source_identity()
        .ok_or_else(|| missing_endpoint_authority(cut, "source endpoint identity"))?;
    let projected_endpoint_fact_identity = cut
        .exact_projected_endpoint_fact_identity()
        .ok_or_else(|| missing_endpoint_authority(cut, "projected endpoint fact identity"))?;
    if posture == PlanarBooleanPointSplitPosture::SharedEndpoint {
        validate_shared_endpoint_authority(cut)?;
    }
    let decision_identity =
        endpoint_contact_decision_identity(EndpointContactDecisionIdentityBasis {
            normalized_cut_identity: cut.cut_identity(),
            duplicate_report_identity: cut.duplicate_report_identity(),
            boundary_position,
            source_endpoint_identity,
            projected_endpoint_fact_identity,
            provenance_entry_identities: cut.provenance_entry_identities(),
            event_group_identities: cut.event_group_identities(),
        });
    Ok(PlanarBooleanEndpointContactDecision::new(
        decision_identity,
        cut.cut_identity().to_string(),
        cut.duplicate_report_identity().to_string(),
        cut.source_edge_identity().to_string(),
        cut.carrier_identity().to_string(),
        boundary_position,
        posture,
        source_endpoint_identity.to_string(),
        projected_endpoint_fact_identity.to_string(),
        cut.provenance_entry_identities().to_vec(),
        cut.event_group_identities().to_vec(),
        cut.shared_endpoint_source_identities().to_vec(),
        cut.shared_endpoint_projection_fact_digests().to_vec(),
    ))
}

fn validate_shared_endpoint_authority(
    cut: &PlanarBooleanNormalizedSplitCut,
) -> Result<(), PlanarBooleanEndpointBoundaryNormalizationDenial> {
    if cut.shared_endpoint_source_identities().is_empty()
        || cut.shared_endpoint_projection_fact_digests().is_empty()
        || cut.shared_endpoint_source_identities().len()
            != cut.shared_endpoint_projection_fact_digests().len()
    {
        return Err(PlanarBooleanEndpointBoundaryNormalizationDenial::new(
            PlanarBooleanEndpointBoundaryNormalizationDenialKind::MissingEndpointBoundaryAuthority,
            cut.cut_identity(),
            "shared endpoint contact decisions require source and projection endpoint authority",
        ));
    }
    Ok(())
}

fn missing_endpoint_authority(
    cut: &PlanarBooleanNormalizedSplitCut,
    missing_field: &'static str,
) -> PlanarBooleanEndpointBoundaryNormalizationDenial {
    PlanarBooleanEndpointBoundaryNormalizationDenial::new(
        PlanarBooleanEndpointBoundaryNormalizationDenialKind::MissingEndpointBoundaryAuthority,
        cut.cut_identity(),
        format!("endpoint contact decision is missing {missing_field}"),
    )
}

#[derive(Default)]
struct CounterBuild {
    inspected_point_cuts: usize,
    fragment_point_cuts: usize,
    endpoint_noop_decisions: usize,
    shared_endpoint_decisions: usize,
    t_junction_boundary_decisions: usize,
    retained_interval_entries: usize,
}

impl CounterBuild {
    fn record_decision(&mut self, cut: &PlanarBooleanNormalizedSplitCut) {
        if let PlanarBooleanRawEdgeSplitScheduleEntryKind::Point(posture) = cut.kind() {
            match posture {
                PlanarBooleanPointSplitPosture::EndpointNoOp => self.endpoint_noop_decisions += 1,
                PlanarBooleanPointSplitPosture::SharedEndpoint => {
                    self.shared_endpoint_decisions += 1
                }
                PlanarBooleanPointSplitPosture::TJunctionPromotion => {
                    self.t_junction_boundary_decisions += 1
                }
                PlanarBooleanPointSplitPosture::InteriorSplit => {}
            }
        }
    }

    fn finish(
        self,
        normalized_schedules: usize,
    ) -> PlanarBooleanEndpointBoundaryNormalizationCounters {
        PlanarBooleanEndpointBoundaryNormalizationCounters::new(
            normalized_schedules,
            self.inspected_point_cuts,
            self.fragment_point_cuts,
            self.endpoint_noop_decisions,
            self.shared_endpoint_decisions,
            self.t_junction_boundary_decisions,
            self.retained_interval_entries,
        )
    }
}
