use super::admitted_candidate::{
    AdmittedPointSplitCandidate, PlanarBooleanAdmittedPointSplitCandidateSet,
};
use super::counters::PlanarBooleanSplitPointAdmissionCounters;
use super::denial::PlanarBooleanSplitPointAdmissionDenial;
use super::endpoint_posture::PlanarBooleanSplitPointEndpointPosture;
use crate::workload_platform::planar_boolean_edge_splitting::point_split_candidates::{
    PlanarBooleanPointSplitCandidate, PlanarBooleanPointSplitCandidateSet,
};

impl PlanarBooleanPointSplitCandidateSet {
    pub fn admit_parameter_domain(
        &self,
    ) -> Result<PlanarBooleanAdmittedPointSplitCandidateSet, PlanarBooleanSplitPointAdmissionDenial>
    {
        let mut admitted = Vec::with_capacity(self.candidates().len());
        let mut endpoint_candidates = 0;
        let mut interior_candidates = 0;
        for candidate in self.candidates() {
            let posture = classify_candidate_parameter(candidate)?;
            count_endpoint_posture(posture, &mut endpoint_candidates, &mut interior_candidates);
            let (source_endpoint_identity, projected_endpoint_fact_identity) =
                bind_exact_endpoint_identity(candidate, posture)?;
            admitted.push(AdmittedPointSplitCandidate::new(
                candidate.clone(),
                posture,
                source_endpoint_identity,
                projected_endpoint_fact_identity,
            ));
        }
        let counters = PlanarBooleanSplitPointAdmissionCounters::new(
            self.candidates().len(),
            admitted.len(),
            endpoint_candidates,
            interior_candidates,
            0,
        );
        Ok(PlanarBooleanAdmittedPointSplitCandidateSet::new(
            self.candidate_set_identity().to_string(),
            self.participation_index_identity().to_string(),
            admitted,
            counters,
        ))
    }
}

fn classify_candidate_parameter(
    candidate: &PlanarBooleanPointSplitCandidate,
) -> Result<PlanarBooleanSplitPointEndpointPosture, PlanarBooleanSplitPointAdmissionDenial> {
    let parameter = candidate.parameter();
    if !parameter.is_finite() {
        return Err(
            PlanarBooleanSplitPointAdmissionDenial::non_finite_parameter(
                candidate.candidate_identity(),
                "point split parameter must be finite",
            ),
        );
    }
    if !(0.0..=1.0).contains(&parameter) {
        return Err(
            PlanarBooleanSplitPointAdmissionDenial::out_of_domain_parameter(
                candidate.candidate_identity(),
                "point split parameter must be inside the source-edge domain",
            ),
        );
    }
    Ok(classify_in_domain_parameter(parameter))
}

fn count_endpoint_posture(
    posture: PlanarBooleanSplitPointEndpointPosture,
    endpoint_candidates: &mut usize,
    interior_candidates: &mut usize,
) {
    match posture {
        PlanarBooleanSplitPointEndpointPosture::StartEndpoint
        | PlanarBooleanSplitPointEndpointPosture::EndEndpoint => *endpoint_candidates += 1,
        PlanarBooleanSplitPointEndpointPosture::Interior => *interior_candidates += 1,
    }
}

fn bind_exact_endpoint_identity(
    candidate: &PlanarBooleanPointSplitCandidate,
    posture: PlanarBooleanSplitPointEndpointPosture,
) -> Result<(Option<String>, Option<String>), PlanarBooleanSplitPointAdmissionDenial> {
    match posture {
        PlanarBooleanSplitPointEndpointPosture::StartEndpoint => endpoint_binding(
            candidate,
            candidate.start_source_endpoint_identity(),
            candidate.start_projected_endpoint_fact_identity(),
        ),
        PlanarBooleanSplitPointEndpointPosture::EndEndpoint => endpoint_binding(
            candidate,
            candidate.end_source_endpoint_identity(),
            candidate.end_projected_endpoint_fact_identity(),
        ),
        PlanarBooleanSplitPointEndpointPosture::Interior => Ok((None, None)),
    }
}

fn endpoint_binding(
    candidate: &PlanarBooleanPointSplitCandidate,
    source_endpoint_identity: &str,
    projected_endpoint_fact_identity: &str,
) -> Result<(Option<String>, Option<String>), PlanarBooleanSplitPointAdmissionDenial> {
    if source_endpoint_identity.is_empty() || projected_endpoint_fact_identity.is_empty() {
        return Err(
            PlanarBooleanSplitPointAdmissionDenial::missing_exact_endpoint_identity(
                candidate.candidate_identity(),
                "exact endpoint split point requires source and projected endpoint identity",
            ),
        );
    }
    Ok((
        Some(source_endpoint_identity.to_string()),
        Some(projected_endpoint_fact_identity.to_string()),
    ))
}

fn classify_in_domain_parameter(parameter: f64) -> PlanarBooleanSplitPointEndpointPosture {
    if parameter == 0.0 {
        PlanarBooleanSplitPointEndpointPosture::StartEndpoint
    } else if parameter == 1.0 {
        PlanarBooleanSplitPointEndpointPosture::EndEndpoint
    } else {
        PlanarBooleanSplitPointEndpointPosture::Interior
    }
}
