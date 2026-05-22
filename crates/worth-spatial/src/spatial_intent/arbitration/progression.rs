use forge_proof::{Artifact, PhaseMarker, TransitionOutcome};

use crate::spatial_intent::policy::SpatialIntentPolicyProfile;

use super::analysis::compute_spatial_intent_arbitration_declaration;
use super::capabilities::SpatialIntentCapabilitySet;
use super::declared_analysis::SpatialIntentArbitrationDeclaration;
use super::facts::{SpatialAuthoredActKind, SpatialObservedRelationFact};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RequestedSpatialArbitrationIntentPhase;
impl PhaseMarker for RequestedSpatialArbitrationIntentPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdmittedSpatialArbitrationIntentPhase;
impl PhaseMarker for AdmittedSpatialArbitrationIntentPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DeclaredSpatialArbitrationIntentPhase;
impl PhaseMarker for DeclaredSpatialArbitrationIntentPhase {}

pub(crate) type RequestedSpatialArbitrationIntentArtifact =
    Artifact<RequestedSpatialArbitrationIntentPhase, RequestedSpatialArbitrationIntent>;
pub(crate) type AdmittedSpatialArbitrationIntentArtifact =
    Artifact<AdmittedSpatialArbitrationIntentPhase, AdmittedSpatialArbitrationIntent>;
pub(crate) type DeclaredSpatialArbitrationIntentArtifact =
    Artifact<DeclaredSpatialArbitrationIntentPhase, SpatialIntentArbitrationDeclaration>;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RequestedSpatialArbitrationIntent {
    pub authored_act: SpatialAuthoredActKind,
    pub observed_relation_facts: Vec<SpatialObservedRelationFact>,
    pub capabilities: SpatialIntentCapabilitySet,
    pub profile: SpatialIntentPolicyProfile,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct AdmittedSpatialArbitrationIntent {
    pub authored_act: SpatialAuthoredActKind,
    pub observed_relation_facts: Vec<SpatialObservedRelationFact>,
    pub capabilities: SpatialIntentCapabilitySet,
    pub profile: SpatialIntentPolicyProfile,
}

pub(crate) fn request_spatial_arbitration_intent(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
    profile: SpatialIntentPolicyProfile,
) -> RequestedSpatialArbitrationIntentArtifact {
    Artifact::new(RequestedSpatialArbitrationIntent {
        authored_act,
        observed_relation_facts: observed_relation_facts.to_vec(),
        capabilities,
        profile,
    })
}

pub(crate) fn admit_requested_spatial_arbitration_intent(
    requested: RequestedSpatialArbitrationIntentArtifact,
) -> TransitionOutcome<AdmittedSpatialArbitrationIntentArtifact, core::convert::Infallible> {
    TransitionOutcome::success(Artifact::new(AdmittedSpatialArbitrationIntent {
        authored_act: requested.payload().authored_act,
        observed_relation_facts: requested.payload().observed_relation_facts.clone(),
        capabilities: requested.payload().capabilities,
        profile: requested.payload().profile,
    }))
}

pub(crate) fn declare_admitted_spatial_arbitration_intent(
    admitted: AdmittedSpatialArbitrationIntentArtifact,
) -> TransitionOutcome<DeclaredSpatialArbitrationIntentArtifact, core::convert::Infallible> {
    TransitionOutcome::success(Artifact::new(
        compute_spatial_intent_arbitration_declaration(
            admitted.payload().authored_act,
            &admitted.payload().observed_relation_facts,
            admitted.payload().capabilities,
            admitted.payload().profile,
        ),
    ))
}

#[cfg(test)]
#[path = "progression_tests.rs"]
mod progression_tests;
