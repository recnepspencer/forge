mod adoption;
mod capability;
mod closeout;
mod execution_matrix;
mod handoff;
mod hostile_program;
mod phase_defects;
mod phase_invocation;
mod qos_evidence;
mod scale_comparison;
mod scenario;
mod scenario_audit_binding;
mod scenario_counter_binding;
mod scenario_identity;
mod scenario_mutation_requirements;
mod scenario_owner_topology;
mod scenario_scale;
mod scenario_topology;
mod scenario_trace_binding;
mod structural_preflight;
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
pub use execution_matrix::{S10ScenarioExecutionMatrix, S10ScenarioExecutionMatrixDenial};
pub use handoff::{
    S11StructuredAuditHardeningHandoff, S11UnimplementedSecurityStrengthening,
    S12PhysicalQualificationHandoff, S12UnqualifiedDimension,
};
pub use hostile_program::{
    PublishedReadmissionRecoveryReceipt, RevokedAuthorizationRecoveryReceipt,
    S10HostileProgramDenial, S10HostileProgramEvidence, S10HostileProgramRequirement,
};
pub use phase_defects::{
    localize_s10_audit_phase_defect, localize_s10_closeout_join_phase_defect,
    localize_s10_control_selection_phase_defect, localize_s10_counter_phase_defect,
    localize_s10_formal_phase_defect, localize_s10_harness_join_omission,
    localize_s10_harness_phase_defect, localize_s10_observation_join_omission,
    localize_s10_observation_phase_defect, localize_s10_runtime_phase_defect,
    localize_s10_runtime_record_omission, localize_s10_structural_phase_defect,
    S10PhaseDefectDenial, S10PhaseDefectLocalization, S10PhaseDefectSourceKind,
    S10PhaseDefectSuite, S10PhaseDefectSuiteDenial,
};
pub use phase_invocation::{
    S10PhaseInvocationDenial, S10PhaseInvocationEvidence, S10ScenarioProductionEvidence,
};
pub use qos_evidence::{S10OperationalQosDenial, S10OperationalQosEvidence};
pub use scale_comparison::{
    S10ScaleComparisonDenial, S10ScaleComparisonMatrix, S10ScaleComparisonRow,
};
pub use scenario::{
    certify_s10_operational_scenario, required_s10_crash_reopen_yieldpoints,
    S10OperationalScenarioEvidence, S10ScenarioCertificationDenial,
};
pub use scenario_scale::{
    ScenarioScaleDenial, ScenarioScaleEvidence, ScenarioScaleProfile, ScenarioWorkloadDimensions,
};
pub use scenario_topology::{S10OperationalScenarioKind, S10OperationalScenarioProgram, S10Phase};
pub use structural_preflight::{
    require_s10_structural_preflight, S10StructuralPreflightDenial, S10StructuralPreflightEvidence,
};
