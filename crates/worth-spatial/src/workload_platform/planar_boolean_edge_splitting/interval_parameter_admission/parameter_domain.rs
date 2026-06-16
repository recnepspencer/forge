use crate::workload_platform::planar_boolean_edge_splitting::interval_split_candidates::{
    PlanarBooleanIntervalSplitCandidate, PlanarBooleanIntervalSplitCandidateSet,
};

use super::admitted_candidate::{
    AdmittedIntervalSplitCandidate, PlanarBooleanAdmittedIntervalSplitCandidateSet,
};
use super::counters::PlanarBooleanSplitIntervalAdmissionCounters;
use super::denial::PlanarBooleanSplitIntervalAdmissionDenial;
use super::range_domain::SplitIntervalRangeDomain;
use super::source_sense_admission::admit_source_sense_ordered_range;

impl PlanarBooleanIntervalSplitCandidateSet {
    pub fn admit_parameter_domain(
        &self,
    ) -> Result<
        PlanarBooleanAdmittedIntervalSplitCandidateSet,
        PlanarBooleanSplitIntervalAdmissionDenial,
    > {
        let mut admitted = Vec::with_capacity(self.candidates().len());
        for candidate in self.candidates() {
            admitted.push(admit_interval_candidate(candidate)?);
        }
        Ok(PlanarBooleanAdmittedIntervalSplitCandidateSet::new(
            self.candidate_set_identity().to_string(),
            self.participation_index_identity().to_string(),
            admitted,
            successful_admission_counters(self.candidates().len()),
        ))
    }
}

fn admit_interval_candidate(
    candidate: &PlanarBooleanIntervalSplitCandidate,
) -> Result<AdmittedIntervalSplitCandidate, PlanarBooleanSplitIntervalAdmissionDenial> {
    let range_domain = SplitIntervalRangeDomain::new(
        candidate.candidate_identity(),
        candidate.source_parameter_range(),
    )?;
    let admitted_range = admit_source_sense_ordered_range(
        candidate.candidate_identity(),
        range_domain,
        candidate.source_sense(),
    )?;
    Ok(AdmittedIntervalSplitCandidate::new(
        candidate.clone(),
        admitted_range,
    ))
}

fn successful_admission_counters(
    candidate_count: usize,
) -> PlanarBooleanSplitIntervalAdmissionCounters {
    PlanarBooleanSplitIntervalAdmissionCounters::new(candidate_count, candidate_count, 0, 0, 0, 0)
}
