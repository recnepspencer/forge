use worth_proof::PhaseMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContributionRequestedPhase;
impl PhaseMarker for ContributionRequestedPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContributionEligiblePhase;
impl PhaseMarker for ContributionEligiblePhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContributionAdmittedPhase;
impl PhaseMarker for ContributionAdmittedPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContributionMaterializationReadyPhase;
impl PhaseMarker for ContributionMaterializationReadyPhase {}
