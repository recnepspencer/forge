use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::{
    PlanarBooleanEventExtractionCounters, PlanarBooleanEventExtractionDenial,
    PlanarBooleanEventExtractionDenialInput, PlanarBooleanEventExtractionDenialKind,
    PlanarBooleanEventExtractionPolicyExit, PlanarBooleanEventExtractionPolicyExitInput,
    PlanarBooleanEventExtractionPolicyExitKind,
};
#[cfg(test)]
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanCanonicalSegmentSetDenial, PlanarBooleanCanonicalSegmentSetDenialKind,
};
use crate::workload_platform::planar_boolean_events::{
    PlanarBooleanEventPredicateBinding, PlanarBooleanPointEventExtractionDenial,
    PlanarBooleanPointEventExtractionDenialKind, PlanarBooleanPredicateBoundPair,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventExtractionPhaseStop {
    Denied(PlanarBooleanEventExtractionDenial),
    PolicyExit(PlanarBooleanEventExtractionPolicyExit),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEventExtractionPhaseStopError {
    PredicateBindingIdentityMismatch,
    SegmentPairMissingFromPredicateBinding,
    ReducedPairIdentityMismatch,
    BoundPairBindingIdentityMismatch,
}

impl PlanarBooleanEventExtractionPhaseStop {
    #[cfg(test)]
    pub(crate) fn from_canonical_segment_denial(
        reduced_pair_identity: impl Into<String>,
        denial: &PlanarBooleanCanonicalSegmentSetDenial,
    ) -> Self {
        let kind = match denial.kind() {
            PlanarBooleanCanonicalSegmentSetDenialKind::CollapsedProjectedSegment => {
                PlanarBooleanEventExtractionDenialKind::ZeroLengthProjectedCarrier
            }
            PlanarBooleanCanonicalSegmentSetDenialKind::NonFiniteEndpointCoordinate => {
                PlanarBooleanEventExtractionDenialKind::NearCoincidentWithoutCertifiedContact
            }
        };
        Self::Denied(PlanarBooleanEventExtractionDenial::new(
            PlanarBooleanEventExtractionDenialInput {
                kind,
                reduced_pair_identity: reduced_pair_identity.into(),
                carrier_identity: Some(denial.carrier_identity().to_string()),
                segment_pair_identity: None,
                predicate_binding_identity: None,
                precision_basis_identity: Some(denial.precision_basis_identity().to_string()),
                workload_evidence_stage: WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
                counters: PlanarBooleanEventExtractionCounters::default()
                    .inspect_carriers(1)
                    .deny_micro_event(),
                human_reason: denial.human_reason().to_string(),
            },
        ))
    }

    pub fn from_point_event_denial(
        predicate_binding: &PlanarBooleanEventPredicateBinding,
        denial: &PlanarBooleanPointEventExtractionDenial,
    ) -> Result<Self, PlanarBooleanEventExtractionPhaseStopError> {
        if denial.predicate_binding_identity() != predicate_binding.predicate_binding_identity() {
            return Err(
                PlanarBooleanEventExtractionPhaseStopError::PredicateBindingIdentityMismatch,
            );
        }
        let bound_pair = predicate_binding
            .bound_pair(denial.segment_pair_identity())
            .ok_or(
                PlanarBooleanEventExtractionPhaseStopError::SegmentPairMissingFromPredicateBinding,
            )?;
        Ok(Self::Denied(PlanarBooleanEventExtractionDenial::new(
            PlanarBooleanEventExtractionDenialInput {
                kind: point_denial_kind(denial.kind()),
                reduced_pair_identity: predicate_binding.reduced_pair_identity().to_string(),
                carrier_identity: None,
                segment_pair_identity: Some(denial.segment_pair_identity().to_string()),
                predicate_binding_identity: Some(denial.predicate_binding_identity().to_string()),
                precision_basis_identity: Some(bound_pair.precision_basis_identity().to_string()),
                workload_evidence_stage: WorkloadEvidenceStage::BooleanEventExtractionRequest,
                counters: PlanarBooleanEventExtractionCounters::default()
                    .inspect_segment_pairs(denial.counters().inspected_bound_pairs())
                    .deny_micro_event(),
                human_reason: denial.human_reason().to_string(),
            },
        )))
    }

    pub fn policy_exit_for_collinear_overlap(
        predicate_binding: &PlanarBooleanEventPredicateBinding,
        bound_pair: &PlanarBooleanPredicateBoundPair,
        human_reason: impl Into<String>,
    ) -> Result<Self, PlanarBooleanEventExtractionPhaseStopError> {
        if bound_pair.reduced_pair_identity() != predicate_binding.reduced_pair_identity() {
            return Err(PlanarBooleanEventExtractionPhaseStopError::ReducedPairIdentityMismatch);
        }
        if bound_pair.predicate_binding_identity() != predicate_binding.predicate_binding_identity()
        {
            return Err(
                PlanarBooleanEventExtractionPhaseStopError::BoundPairBindingIdentityMismatch,
            );
        }
        if predicate_binding
            .bound_pair(bound_pair.segment_pair_identity())
            .is_none()
        {
            return Err(
                PlanarBooleanEventExtractionPhaseStopError::SegmentPairMissingFromPredicateBinding,
            );
        }
        Ok(Self::PolicyExit(PlanarBooleanEventExtractionPolicyExit::new(
            PlanarBooleanEventExtractionPolicyExitInput {
                kind:
                    PlanarBooleanEventExtractionPolicyExitKind::ImprintRequiredForCollinearOverlap,
                reduced_pair_identity: predicate_binding.reduced_pair_identity().to_string(),
                carrier_identity: None,
                segment_pair_identity: Some(bound_pair.segment_pair_identity().to_string()),
                predicate_binding_identity: Some(
                    predicate_binding.predicate_binding_identity().to_string(),
                ),
                precision_basis_identity: Some(bound_pair.precision_basis_identity().to_string()),
                workload_evidence_stage: WorkloadEvidenceStage::BooleanEventExtractionRequest,
                counters: PlanarBooleanEventExtractionCounters::default().policy_exit(),
                human_reason: human_reason.into(),
            },
        )))
    }

    pub fn evidence_identity(&self) -> &str {
        match self {
            Self::Denied(denial) => denial.denial_identity(),
            Self::PolicyExit(policy_exit) => policy_exit.policy_exit_identity(),
        }
    }

    pub fn reduced_pair_identity(&self) -> &str {
        match self {
            Self::Denied(denial) => denial.reduced_pair_identity(),
            Self::PolicyExit(policy_exit) => policy_exit.reduced_pair_identity(),
        }
    }

    pub fn carrier_identity(&self) -> Option<&str> {
        match self {
            Self::Denied(denial) => denial.carrier_identity(),
            Self::PolicyExit(policy_exit) => policy_exit.carrier_identity(),
        }
    }

    pub fn segment_pair_identity(&self) -> Option<&str> {
        match self {
            Self::Denied(denial) => denial.segment_pair_identity(),
            Self::PolicyExit(policy_exit) => policy_exit.segment_pair_identity(),
        }
    }

    pub fn predicate_binding_identity(&self) -> Option<&str> {
        match self {
            Self::Denied(denial) => denial.predicate_binding_identity(),
            Self::PolicyExit(policy_exit) => policy_exit.predicate_binding_identity(),
        }
    }

    pub fn precision_basis_identity(&self) -> Option<&str> {
        match self {
            Self::Denied(denial) => denial.precision_basis_identity(),
            Self::PolicyExit(policy_exit) => policy_exit.precision_basis_identity(),
        }
    }

    pub fn workload_evidence_stage(&self) -> WorkloadEvidenceStage {
        match self {
            Self::Denied(denial) => denial.workload_evidence_stage(),
            Self::PolicyExit(policy_exit) => policy_exit.workload_evidence_stage(),
        }
    }

    pub fn counters(&self) -> PlanarBooleanEventExtractionCounters {
        match self {
            Self::Denied(denial) => denial.counters(),
            Self::PolicyExit(policy_exit) => policy_exit.counters(),
        }
    }

    pub fn human_reason(&self) -> &str {
        match self {
            Self::Denied(denial) => denial.human_reason(),
            Self::PolicyExit(policy_exit) => policy_exit.human_reason(),
        }
    }
}

fn point_denial_kind(
    kind: PlanarBooleanPointEventExtractionDenialKind,
) -> PlanarBooleanEventExtractionDenialKind {
    match kind {
        PlanarBooleanPointEventExtractionDenialKind::MissingPredicateBindingIdentity => {
            PlanarBooleanEventExtractionDenialKind::MissingTopologyProvenance
        }
        PlanarBooleanPointEventExtractionDenialKind::AmbiguousPredicateRelation => {
            PlanarBooleanEventExtractionDenialKind::PredicateAmbiguousNearContact
        }
        PlanarBooleanPointEventExtractionDenialKind::DegenerateSegmentParameterBasis => {
            PlanarBooleanEventExtractionDenialKind::NearCoincidentWithoutCertifiedContact
        }
        PlanarBooleanPointEventExtractionDenialKind::MissingInteriorEndpointWitness => {
            PlanarBooleanEventExtractionDenialKind::MissingTopologyProvenance
        }
        PlanarBooleanPointEventExtractionDenialKind::NonFinitePointEventCoordinate => {
            PlanarBooleanEventExtractionDenialKind::NearCoincidentWithoutCertifiedContact
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
    use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegmentSetDenialKind;
    use crate::workload_platform::planar_boolean_events::{
        PlanarBooleanCanonicalSegmentSetDenial, PlanarBooleanEventExtractionDenialKind,
        PlanarBooleanEventExtractionPhaseStop, PlanarBooleanSegmentCarrier,
        PlanarBooleanSegmentCarrierEndpointFacts,
    };

    #[test]
    fn event_extraction_denies_zero_length_carrier_before_pair_enumeration() {
        let endpoint = PlanarBooleanSegmentCarrierEndpointFacts::for_canonical_segment_test(
            [2.0, 3.0],
            "zero-length-endpoint",
            "zero-length-projection-fact",
        );
        let carrier =
            PlanarBooleanSegmentCarrier::for_canonical_segment_test(endpoint.clone(), endpoint);
        let canonical_denial = PlanarBooleanCanonicalSegmentSetDenial::from_carrier(
            PlanarBooleanCanonicalSegmentSetDenialKind::CollapsedProjectedSegment,
            &carrier,
            "zero-length projected carrier cannot safely enter pair enumeration",
        );
        let stop = PlanarBooleanEventExtractionPhaseStop::from_canonical_segment_denial(
            "reduced-pair:zero-length",
            &canonical_denial,
        );

        let PlanarBooleanEventExtractionPhaseStop::Denied(denial) = stop else {
            panic!("zero-length carrier must deny, not policy-exit");
        };
        assert_eq!(
            denial.kind(),
            PlanarBooleanEventExtractionDenialKind::ZeroLengthProjectedCarrier
        );
        assert_eq!(denial.reduced_pair_identity(), "reduced-pair:zero-length");
        assert_eq!(denial.carrier_identity(), Some(carrier.carrier_identity()));
        assert_eq!(
            denial.workload_evidence_stage(),
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration
        );
        assert_eq!(denial.counters().inspected_carriers(), 1);
        assert_eq!(denial.counters().denied_micro_events(), 1);
        assert!(!denial.denial_identity().trim().is_empty());
    }
}
