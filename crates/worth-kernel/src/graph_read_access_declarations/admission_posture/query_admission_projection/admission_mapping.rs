use super::super::posture_outcome::WorthGraphReadAccessAdmissionPostureOutcome;
use super::admission_attempt::WorthGraphReadAdmissionAttempt;
use super::capability_gap::WorthGraphReadAdmissionCapabilityGap;
use crate::graph_read_access_declarations::{
    WorthGraphReadRequirementDerivationOutcome, WorthGraphReadRequirementDerivationRecord,
};
use forge_query::facade::admit_graph_read_access_for_family;

pub(crate) fn admission_outcome_for_requirement_record(
    record: &WorthGraphReadRequirementDerivationRecord,
) -> WorthGraphReadAccessAdmissionPostureOutcome {
    match record.derivation_outcome() {
        WorthGraphReadRequirementDerivationOutcome::QueryCapabilityGap(gap) => {
            WorthGraphReadAccessAdmissionPostureOutcome::RequirementDerivationGapCarriedForward {
                admission_attempt:
                    WorthGraphReadAdmissionAttempt::blocked_by_requirement_derivation_gap(record),
                admission_gap: WorthGraphReadAdmissionCapabilityGap::requirement_derivation_blocked(
                    record,
                    gap.blocker(),
                    gap.removal_trigger(),
                ),
                requirement_gap: gap.clone(),
            }
        }
        WorthGraphReadRequirementDerivationOutcome::QueryDerived(_) => {
            if let Some(query_read_family) = record.query_read_family_artifact() {
                match admit_graph_read_access_for_family(query_read_family) {
                    Ok(admission) => {
                        if let Some(admission_gap) =
                            WorthGraphReadAdmissionCapabilityGap::from_query_admission_denial(
                                record, &admission,
                            )
                        {
                            WorthGraphReadAccessAdmissionPostureOutcome::RequiredSupportCapabilityGap {
                                admission_attempt:
                                    WorthGraphReadAdmissionAttempt::query_admission_inspected(
                                        record,
                                    ),
                                admission_gap,
                            }
                        } else {
                            WorthGraphReadAccessAdmissionPostureOutcome::QueryAdmissionEvidence(
                                super::super::posture_outcome::WorthGraphReadQueryAdmissionEvidence::from_query_admission(&admission),
                            )
                        }
                    }
                    Err(_) => {
                        WorthGraphReadAccessAdmissionPostureOutcome::RequiredSupportCapabilityGap {
                            admission_attempt:
                                WorthGraphReadAdmissionAttempt::missing_query_read_family_artifact(
                                    record,
                                ),
                            admission_gap:
                                WorthGraphReadAdmissionCapabilityGap::missing_query_read_family_artifact(
                                    record,
                                ),
                        }
                    }
                }
            } else {
                WorthGraphReadAccessAdmissionPostureOutcome::RequiredSupportCapabilityGap {
                    admission_attempt:
                        WorthGraphReadAdmissionAttempt::missing_query_read_family_artifact(record),
                    admission_gap:
                        WorthGraphReadAdmissionCapabilityGap::missing_query_read_family_artifact(
                            record,
                        ),
                }
            }
        }
    }
}
