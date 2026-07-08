use super::denial::S8SelectionCandidateRejection;
use super::selection_basis::{S8PlanningCapabilityGrant, S8SelectionCandidateEligibility};
use crate::access_shape::{S8AccessAuthorityPosture, S8AccessShape, S8AccessShapeContract};
use crate::artifact_family::{
    declare_authority_role, ArtifactFamilyLifecycleAdmission, AuthorityRole,
};
use crate::key_domain::PhysicalKeyDomainWitness;
use crate::strategy::S8LayoutStrategyFamily;
use crate::strategy_registry::{
    layout_admission_registry, S8LayoutAdmissionDeferred, S8LayoutAdmissionDenial,
    S8LayoutAdmissionRequest, S8LayoutRequestedCapability, S8LayoutStrategyRegistrySnapshot,
};
use forge_proof::TransitionOutcome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8SelectionCandidateAudit {
    family: S8LayoutStrategyFamily,
    authority_role: AuthorityRole,
    outcome: S8SelectionCandidateOutcome,
}

impl S8SelectionCandidateAudit {
    pub(crate) const fn new(
        family: S8LayoutStrategyFamily,
        authority_role: AuthorityRole,
        outcome: S8SelectionCandidateOutcome,
    ) -> Self {
        Self {
            family,
            authority_role,
            outcome,
        }
    }

    pub const fn family(self) -> S8LayoutStrategyFamily {
        self.family
    }

    pub const fn authority_role(self) -> AuthorityRole {
        self.authority_role
    }

    pub const fn outcome(self) -> S8SelectionCandidateOutcome {
        self.outcome
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8SelectionCandidateOutcome {
    Eligible(S8SelectionCandidateEligibility),
    Rejected(S8SelectionCandidateRejection),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8PlanningAlternative {
    snapshot: S8LayoutStrategyRegistrySnapshot,
    audit: S8SelectionCandidateAudit,
}

impl S8PlanningAlternative {
    pub(crate) const fn snapshot(self) -> S8LayoutStrategyRegistrySnapshot {
        self.snapshot
    }

    pub(crate) const fn audit(self) -> S8SelectionCandidateAudit {
        self.audit
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct S8PlanningAlternativeSet {
    primary: Option<S8PlanningAlternative>,
    secondary: Option<S8PlanningAlternative>,
    primary_audit: S8SelectionCandidateAudit,
    secondary_audit: S8SelectionCandidateAudit,
}

impl S8PlanningAlternativeSet {
    pub(crate) fn derive(
        lifecycle: ArtifactFamilyLifecycleAdmission,
        key_domain: PhysicalKeyDomainWitness,
        shape: S8AccessShapeContract,
    ) -> Self {
        let role = declare_authority_role(lifecycle.authority().classification()).role();
        let btree = derive_candidate(
            lifecycle,
            key_domain,
            shape,
            S8LayoutStrategyFamily::BaselineBTreeRange,
            role,
        );
        let lsm = derive_candidate(
            lifecycle,
            key_domain,
            shape,
            S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
            role,
        );

        Self {
            primary: btree.0,
            secondary: lsm.0,
            primary_audit: btree.1,
            secondary_audit: lsm.1,
        }
    }

    pub(crate) const fn primary(self) -> Option<S8PlanningAlternative> {
        self.primary
    }

    pub(crate) const fn secondary(self) -> Option<S8PlanningAlternative> {
        self.secondary
    }

    pub(crate) const fn primary_audit(self) -> S8SelectionCandidateAudit {
        self.primary_audit
    }

    pub(crate) const fn secondary_audit(self) -> S8SelectionCandidateAudit {
        self.secondary_audit
    }
}

fn derive_candidate(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    shape: S8AccessShapeContract,
    family: S8LayoutStrategyFamily,
    authority_role: AuthorityRole,
) -> (Option<S8PlanningAlternative>, S8SelectionCandidateAudit) {
    let request = build_request(lifecycle, key_domain, shape, family);
    match layout_admission_registry().admit_with(request) {
        TransitionOutcome::Success(snapshot) => {
            let planned_counter_envelope = crate::strategy::planned_counter_envelope_for(
                snapshot.admitted_strategy().family(),
                shape.detail(),
            );
            if planned_counter_envelope.is_none() {
                return (
                    None,
                    S8SelectionCandidateAudit::new(
                        family,
                        authority_role,
                        S8SelectionCandidateOutcome::Rejected(
                            S8SelectionCandidateRejection::MissingPlannedCounterEnvelope,
                        ),
                    ),
                );
            }
            let audit = S8SelectionCandidateAudit::new(
                family,
                authority_role,
                S8SelectionCandidateOutcome::Eligible(
                    S8SelectionCandidateEligibility::RegistryAdmitted {
                        granted_capability: planning_capability(snapshot.granted_capability()),
                        planned_counter_envelope: planned_counter_envelope
                            .expect("eligible alternatives declare planned envelopes"),
                    },
                ),
            );
            (Some(S8PlanningAlternative { snapshot, audit }), audit)
        }
        TransitionOutcome::Denied(denial) => (
            None,
            S8SelectionCandidateAudit::new(
                family,
                authority_role,
                S8SelectionCandidateOutcome::Rejected(map_denial(denial)),
            ),
        ),
        TransitionOutcome::Deferred(deferred) => (
            None,
            S8SelectionCandidateAudit::new(
                family,
                authority_role,
                S8SelectionCandidateOutcome::Rejected(map_deferred(deferred)),
            ),
        ),
        TransitionOutcome::Stale(stale) => match stale {},
        TransitionOutcome::RebindRequired(rebind) => match rebind {},
        TransitionOutcome::Failed(failed) => match failed {},
    }
}

const fn planning_capability(
    capability: crate::strategy_registry::S8LayoutStrategyCapability,
) -> S8PlanningCapabilityGrant {
    match capability {
        crate::strategy_registry::S8LayoutStrategyCapability::PointLookup => {
            S8PlanningCapabilityGrant::PointLookup
        }
        crate::strategy_registry::S8LayoutStrategyCapability::OrderedRange => {
            S8PlanningCapabilityGrant::OrderedRange
        }
        crate::strategy_registry::S8LayoutStrategyCapability::PrefixTraversal => {
            S8PlanningCapabilityGrant::PrefixTraversal
        }
        crate::strategy_registry::S8LayoutStrategyCapability::BlobStreaming => {
            S8PlanningCapabilityGrant::BlobStreaming
        }
        crate::strategy_registry::S8LayoutStrategyCapability::ExactScan => {
            S8PlanningCapabilityGrant::ExactScan
        }
    }
}

fn build_request(
    lifecycle: ArtifactFamilyLifecycleAdmission,
    key_domain: PhysicalKeyDomainWitness,
    shape: S8AccessShapeContract,
    family: S8LayoutStrategyFamily,
) -> S8LayoutAdmissionRequest {
    let mut request = S8LayoutAdmissionRequest::new(
        lifecycle,
        key_domain,
        family,
        requested_capability(shape.shape()),
        shape.lane().admitted_lane(),
    );

    if let Some(mutation_shape) = shape.mutation_shape() {
        request = request.for_mutation_shape(mutation_shape);
    }
    if let Some(coverage) = shape.coverage() {
        request = request.require_exact_materialization(coverage);
    } else if shape.authority_posture() == S8AccessAuthorityPosture::ExactMaterialized {
        request = request.require_exact_readiness();
    }

    request
}

const fn requested_capability(shape: S8AccessShape) -> S8LayoutRequestedCapability {
    match shape {
        S8AccessShape::PointLookup
        | S8AccessShape::BatchPointLookup
        | S8AccessShape::SortedBatchLookup
        | S8AccessShape::Append => S8LayoutRequestedCapability::PointLookup,
        S8AccessShape::RangeLookup
        | S8AccessShape::MultiRangeLookup
        | S8AccessShape::CoalescedPageRead
        | S8AccessShape::CompactionRead => S8LayoutRequestedCapability::OrderedRange,
        S8AccessShape::PrefixLookup | S8AccessShape::GroupedPrefixLookup => {
            S8LayoutRequestedCapability::PrefixTraversal
        }
        S8AccessShape::ChunkTreeWalk
        | S8AccessShape::StreamingRead
        | S8AccessShape::StreamingContinuationRead => S8LayoutRequestedCapability::BlobStreaming,
        _ => S8LayoutRequestedCapability::ExactScan,
    }
}

const fn map_denial(denial: S8LayoutAdmissionDenial) -> S8SelectionCandidateRejection {
    match denial {
        S8LayoutAdmissionDenial::StrategyVocabularyDenied(_) => {
            S8SelectionCandidateRejection::StrategyUnsupported
        }
        S8LayoutAdmissionDenial::RequestedLaneDoesNotMatchFamilyLane { .. }
        | S8LayoutAdmissionDenial::MaintenanceModeIncompatibleWithRequestedLane { .. } => {
            S8SelectionCandidateRejection::LaneUnsupported
        }
        S8LayoutAdmissionDenial::MutationShapeIncompatibleWithStrategy { .. } => {
            S8SelectionCandidateRejection::MutationShapeUnsupported
        }
        S8LayoutAdmissionDenial::StrategyDoesNotSupportRequestedCapability { .. }
        | S8LayoutAdmissionDenial::ComparatorLawRequired { .. }
        | S8LayoutAdmissionDenial::PrefixLawRequired { .. }
        | S8LayoutAdmissionDenial::RangeBoundLawRequired { .. } => {
            S8SelectionCandidateRejection::CapabilityUnsupported
        }
        S8LayoutAdmissionDenial::ExactCoverageDenied(_)
        | S8LayoutAdmissionDenial::ExactAbsenceProofDenied(_)
        | S8LayoutAdmissionDenial::CoverageFamilyDoesNotMatchStrategy { .. } => {
            S8SelectionCandidateRejection::MaterializationInexact
        }
        _ => S8SelectionCandidateRejection::StrategyUnsupported,
    }
}

const fn map_deferred(deferred: S8LayoutAdmissionDeferred) -> S8SelectionCandidateRejection {
    match deferred {
        S8LayoutAdmissionDeferred::ExactCoverageEvidenceRequired { .. }
        | S8LayoutAdmissionDeferred::LiveExactMaintenanceWitnessRequired { .. } => {
            S8SelectionCandidateRejection::MaterializationRequired
        }
    }
}
