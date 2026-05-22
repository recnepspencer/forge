use forge_proof::PhaseMarker;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestedLoweringIntentPhase;
impl PhaseMarker for RequestedLoweringIntentPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedLoweringIntentPhase;
impl PhaseMarker for AdmittedLoweringIntentPhase {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoweredSpatialIntentPhase;
impl PhaseMarker for LoweredSpatialIntentPhase {}
