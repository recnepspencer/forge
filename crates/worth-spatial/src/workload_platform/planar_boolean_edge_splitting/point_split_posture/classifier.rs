use std::collections::BTreeMap;

use crate::workload_platform::planar_boolean_edge_splitting::point_parameter_admission::{
    AdmittedPointSplitCandidate, PlanarBooleanAdmittedPointSplitCandidateSet,
    PlanarBooleanSplitPointEndpointPosture,
};
use crate::workload_platform::planar_boolean_events::PlanarBooleanPointEventKind;

use super::counters::PlanarBooleanPointSplitPostureCounters;
use super::denial::{
    PlanarBooleanPointSplitPostureDenial, PlanarBooleanPointSplitPostureDenialKind,
};
use super::identity::{posture_set_identity, postured_candidate_identity};
use super::posture::{PlanarBooleanPointSplitPosture, PosturedPointSplitCandidate};
use super::posture_set::PlanarBooleanPointSplitPostureSet;

impl PlanarBooleanAdmittedPointSplitCandidateSet {
    pub fn classify_point_split_postures(
        &self,
    ) -> Result<PlanarBooleanPointSplitPostureSet, PlanarBooleanPointSplitPostureDenial> {
        let groups = group_candidate_offsets_by_point_event(self.admitted_candidates());
        let mut postured = Vec::with_capacity(self.admitted_candidates().len());
        let mut counters = CounterBuild::default();
        for offsets in groups.values() {
            if offsets.is_empty() {
                return Err(PlanarBooleanPointSplitPostureDenial::new(
                    PlanarBooleanPointSplitPostureDenialKind::EmptyPointEventGroup,
                    self.point_candidate_set_identity(),
                    "point split posture classification requires non-empty event groups",
                ));
            }
            let posture = classify_group_posture(self, offsets)?;
            for offset in offsets {
                let admitted = self.admitted_candidates()[*offset].clone();
                let identity = postured_candidate_identity(
                    admitted.candidate().candidate_identity(),
                    posture.as_str(),
                );
                counters.record(posture);
                postured.push(PosturedPointSplitCandidate::new(
                    identity, admitted, posture,
                ));
            }
        }
        postured.sort_by(|left, right| {
            left.postured_candidate_identity()
                .cmp(right.postured_candidate_identity())
        });
        let counters = counters.finish(self.admitted_candidates().len(), postured.len());
        let set_identity = posture_set_identity(self.point_candidate_set_identity(), &postured);
        Ok(PlanarBooleanPointSplitPostureSet::new(
            set_identity,
            self.point_candidate_set_identity().to_string(),
            self.participation_index_identity().to_string(),
            postured,
            counters,
        ))
    }
}

fn group_candidate_offsets_by_point_event(
    candidates: &[AdmittedPointSplitCandidate],
) -> BTreeMap<String, Vec<usize>> {
    let mut groups: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (offset, candidate) in candidates.iter().enumerate() {
        groups
            .entry(candidate.candidate().point_event_identity().to_string())
            .or_default()
            .push(offset);
    }
    groups
}

fn classify_group_posture(
    set: &PlanarBooleanAdmittedPointSplitCandidateSet,
    offsets: &[usize],
) -> Result<PlanarBooleanPointSplitPosture, PlanarBooleanPointSplitPostureDenial> {
    let mut has_endpoint = false;
    let mut has_interior = false;
    let first = &set.admitted_candidates()[offsets[0]];
    let point_event_kind = first.candidate().point_event_kind();
    for offset in offsets {
        let admitted = &set.admitted_candidates()[*offset];
        if admitted.candidate().point_event_kind() != point_event_kind {
            return Err(PlanarBooleanPointSplitPostureDenial::new(
                PlanarBooleanPointSplitPostureDenialKind::MixedPointEventKind,
                first.candidate().point_event_identity(),
                "point split posture groups must contain exactly one point-event kind",
            ));
        }
        match admitted.endpoint_posture() {
            PlanarBooleanSplitPointEndpointPosture::StartEndpoint
            | PlanarBooleanSplitPointEndpointPosture::EndEndpoint => has_endpoint = true,
            PlanarBooleanSplitPointEndpointPosture::Interior => has_interior = true,
        }
    }
    match point_event_kind {
        PlanarBooleanPointEventKind::SharedEndpoint => {
            validate_shared_endpoint_posture(set, offsets)?;
            Ok(PlanarBooleanPointSplitPosture::SharedEndpoint)
        }
        PlanarBooleanPointEventKind::OperandAEndpointOnOperandBInterior
        | PlanarBooleanPointEventKind::OperandBEndpointOnOperandAInterior => {
            validate_t_junction_participants(
                first.candidate().point_event_identity(),
                has_endpoint,
                has_interior,
            )?;
            Ok(PlanarBooleanPointSplitPosture::TJunctionPromotion)
        }
        _ if has_interior => Ok(PlanarBooleanPointSplitPosture::InteriorSplit),
        _ => Ok(PlanarBooleanPointSplitPosture::EndpointNoOp),
    }
}

fn validate_shared_endpoint_posture(
    set: &PlanarBooleanAdmittedPointSplitCandidateSet,
    offsets: &[usize],
) -> Result<(), PlanarBooleanPointSplitPostureDenial> {
    for offset in offsets {
        let admitted = &set.admitted_candidates()[*offset];
        if admitted.endpoint_posture() == PlanarBooleanSplitPointEndpointPosture::Interior {
            return Err(PlanarBooleanPointSplitPostureDenial::new(
                PlanarBooleanPointSplitPostureDenialKind::SharedEndpointInteriorParticipant,
                admitted.candidate().point_event_identity(),
                "shared endpoint split posture cannot carry an interior participant",
            ));
        }
        let candidate = admitted.candidate();
        if candidate.shared_endpoint_source_identities().is_empty()
            || candidate
                .shared_endpoint_projection_fact_digests()
                .is_empty()
        {
            return Err(PlanarBooleanPointSplitPostureDenial::new(
                PlanarBooleanPointSplitPostureDenialKind::SharedEndpointMissingProvenance,
                candidate.point_event_identity(),
                "shared endpoint split posture must carry source and projection provenance",
            ));
        }
        if candidate.shared_endpoint_source_identities().len()
            != candidate.shared_endpoint_projection_fact_digests().len()
        {
            return Err(PlanarBooleanPointSplitPostureDenial::new(
                PlanarBooleanPointSplitPostureDenialKind::SharedEndpointProvenanceMismatch,
                candidate.point_event_identity(),
                "shared endpoint source identities and projection facts must have matching cardinality",
            ));
        }
        validate_admitted_endpoint_matches_shared_endpoint_provenance(admitted)?;
    }
    if offsets.len() < 2 {
        let first = &set.admitted_candidates()[offsets[0]];
        return Err(PlanarBooleanPointSplitPostureDenial::new(
            PlanarBooleanPointSplitPostureDenialKind::SharedEndpointMissingParticipant,
            first.candidate().point_event_identity(),
            "shared endpoint split posture requires at least two admitted endpoint participants",
        ));
    }
    Ok(())
}

fn validate_admitted_endpoint_matches_shared_endpoint_provenance(
    admitted: &AdmittedPointSplitCandidate,
) -> Result<(), PlanarBooleanPointSplitPostureDenial> {
    let candidate = admitted.candidate();
    let exact_endpoint = admitted.exact_endpoint_source_identity().ok_or_else(|| {
        PlanarBooleanPointSplitPostureDenial::new(
            PlanarBooleanPointSplitPostureDenialKind::SharedEndpointExactEndpointMismatch,
            candidate.point_event_identity(),
            "shared endpoint posture requires an admitted exact source endpoint identity",
        )
    })?;
    let exact_projection = admitted
        .exact_projected_endpoint_fact_identity()
        .ok_or_else(|| {
            PlanarBooleanPointSplitPostureDenial::new(
                PlanarBooleanPointSplitPostureDenialKind::SharedEndpointExactEndpointMismatch,
                candidate.point_event_identity(),
                "shared endpoint posture requires an admitted exact projected endpoint fact",
            )
        })?;
    if !candidate
        .shared_endpoint_source_identities()
        .iter()
        .any(|identity| identity == exact_endpoint)
    {
        return Err(PlanarBooleanPointSplitPostureDenial::new(
            PlanarBooleanPointSplitPostureDenialKind::SharedEndpointExactEndpointMismatch,
            candidate.point_event_identity(),
            format!(
                "admitted exact endpoint `{exact_endpoint}` must be present in shared endpoint source provenance {:?}; exact projection `{exact_projection}` remains preserved on the admitted candidate",
                candidate.shared_endpoint_source_identities()
            ),
        ));
    }
    Ok(())
}

fn validate_t_junction_participants(
    point_event_identity: &str,
    has_endpoint: bool,
    has_interior: bool,
) -> Result<(), PlanarBooleanPointSplitPostureDenial> {
    if !has_endpoint {
        return Err(PlanarBooleanPointSplitPostureDenial::new(
            PlanarBooleanPointSplitPostureDenialKind::TJunctionMissingEndpointParticipant,
            point_event_identity,
            "endpoint-on-interior split posture requires an endpoint participant",
        ));
    }
    if !has_interior {
        return Err(PlanarBooleanPointSplitPostureDenial::new(
            PlanarBooleanPointSplitPostureDenialKind::TJunctionMissingInteriorParticipant,
            point_event_identity,
            "endpoint-on-interior split posture requires an interior participant",
        ));
    }
    Ok(())
}

#[derive(Default)]
struct CounterBuild {
    interior_splits: usize,
    t_junction_promotions: usize,
    shared_endpoint_noops: usize,
    endpoint_noops: usize,
}

impl CounterBuild {
    fn record(&mut self, posture: PlanarBooleanPointSplitPosture) {
        match posture {
            PlanarBooleanPointSplitPosture::InteriorSplit => self.interior_splits += 1,
            PlanarBooleanPointSplitPosture::TJunctionPromotion => self.t_junction_promotions += 1,
            PlanarBooleanPointSplitPosture::SharedEndpoint => self.shared_endpoint_noops += 1,
            PlanarBooleanPointSplitPosture::EndpointNoOp => self.endpoint_noops += 1,
        }
    }

    fn finish(
        self,
        admitted_point_candidates: usize,
        postured_point_candidates: usize,
    ) -> PlanarBooleanPointSplitPostureCounters {
        PlanarBooleanPointSplitPostureCounters::new(
            admitted_point_candidates,
            postured_point_candidates,
            self.interior_splits,
            self.t_junction_promotions,
            self.shared_endpoint_noops,
            self.endpoint_noops,
        )
    }
}
