mod adoption;
mod capability;
mod closeout;
mod handoff;
mod qos_evidence;
mod scale_comparison;
mod scenario;
mod scenario_scale;
mod scenario_topology;
#[cfg(test)]
mod tests;

pub use adoption::{
    ProofFoundationalAdoptionRow, S10ProofFoundationalAdoptionMatrix, SharedArtifactCategory,
};
pub use capability::{
    OperationalRecoveryCapability, OperationalRecoveryCapabilityMatrix,
    OperationalRecoveryCapabilityRow,
};
pub use closeout::{
    close_s10_certification, PromotionRemoteExclusionEvidence, S10CertificationCloseout,
    S10CloseoutDenial, S10ScenarioSuiteDenial, S10ScenarioSuiteEvidence,
};
pub use handoff::{
    S11StructuredAuditHardeningHandoff, S11UnimplementedSecurityStrengthening,
    S12PhysicalQualificationHandoff, S12UnqualifiedDimension,
};
pub use qos_evidence::{S10OperationalQosDenial, S10OperationalQosEvidence};
pub use scale_comparison::{
    S10ScaleComparisonDenial, S10ScaleComparisonMatrix, S10ScaleComparisonRow,
};
pub use scenario::{
    certify_s10_operational_scenario, S10OperationalScenarioEvidence,
    S10ScenarioCertificationDenial,
};
pub use scenario_scale::{
    ScenarioScaleDenial, ScenarioScaleEvidence, ScenarioScaleProfile, ScenarioWorkloadDimensions,
};
pub use scenario_topology::{S10OperationalScenarioKind, S10OperationalScenarioProgram, S10Phase};
