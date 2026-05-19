mod audits;
mod bundle;
mod fixtures;
mod manifests;
mod oracles;
mod output_manifest;
mod reports;

pub(crate) const INTENT_ADMISSION_CERTIFICATION_MODULE_ROOT: &str =
    "intent_admission/certification/mod.rs";
pub(crate) const INTENT_ADMISSION_CERTIFICATION_CHILD_MODULES: &[&str] = &[
    "audits",
    "bundle",
    "fixtures",
    "manifests",
    "oracles",
    "output_manifest",
    "reports",
];
pub(crate) const INTENT_ADMISSION_CERTIFICATION_EXPORTED_SURFACE: &[&str] = &[
    "ForgeQueryIntentAdmissionProofShapeAudit",
    "ForgeQueryIntentAdmissionPublicBoundaryAudit",
    "ForgeQueryIntentAdmissionTopologyAudit",
    "ForgeQueryIntentAdmissionTopologyAuditRow",
    "ForgeQueryIntentAdmissionTopologyDomain",
    "certify_intent_admission",
    "ForgeQueryIntentAdmissionCertificationBundle",
    "ForgeQueryIntentAdmissionCertificationOutput",
    "forge_query_intent_admission_compile_fail_targets",
    "forge_query_intent_admission_crate_doc_example_targets",
    "forge_query_intent_admission_golden_transcripts",
    "ForgeQueryIntentAdmissionCompileFailTarget",
    "ForgeQueryIntentAdmissionCrateDocExampleTarget",
    "ForgeQueryIntentAdmissionGoldenTranscript",
    "forge_query_intent_admission_oracle_report",
    "ForgeQueryIntentAdmissionOracleComparisonRow",
    "ForgeQueryIntentAdmissionOracleLane",
    "ForgeQueryIntentAdmissionOracleManifestRow",
    "ForgeQueryIntentAdmissionOracleReport",
    "forge_query_intent_admission_certification_output_manifest",
    "forge_query_intent_admission_doc_example_report",
    "ForgeQueryIntentAdmissionDocExampleReport",
    "ForgeQueryIntentAdmissionDocExampleRow",
    "forge_query_intent_admission_legacy_parity_report",
    "ForgeQueryIntentAdmissionLegacyParityCheck",
    "ForgeQueryIntentAdmissionLegacyParityLane",
    "ForgeQueryIntentAdmissionLegacyParityReport",
    "ForgeQueryIntentAdmissionLegacyParityRow",
    "forge_query_intent_admission_representative_family_report",
    "ForgeQueryIntentAdmissionRepresentativeFamilyLane",
    "ForgeQueryIntentAdmissionRepresentativeFamilyReport",
    "ForgeQueryIntentAdmissionRepresentativeFamilyRow",
    "forge_query_intent_admission_representative_output_report",
    "ForgeQueryIntentAdmissionRepresentativeOutputReport",
    "forge_query_intent_admission_seeded_certification_report",
    "ForgeQueryIntentAdmissionSeedGeneratorClass",
    "ForgeQueryIntentAdmissionSeedReplayRow",
    "ForgeQueryIntentAdmissionSeededCertificationReport",
    "ForgeQueryIntentAdmissionSlopeLane",
    "ForgeQueryIntentAdmissionWidthRunRow",
    "ForgeQueryIntentAdmissionWidthRunScale",
    "forge_query_intent_admission_slope_report",
    "ForgeQueryIntentAdmissionCertificationCounterSnapshot",
    "ForgeQueryIntentAdmissionSlopeReport",
    "forge_query_intent_admission_support_traceability_report",
    "ForgeQueryIntentAdmissionSupportTraceabilityReport",
    "ForgeQueryIntentAdmissionSupportTraceabilityRow",
];

pub use audits::{
    ForgeQueryIntentAdmissionProofShapeAudit, ForgeQueryIntentAdmissionPublicBoundaryAudit,
    ForgeQueryIntentAdmissionTopologyAudit, ForgeQueryIntentAdmissionTopologyAuditRow,
    ForgeQueryIntentAdmissionTopologyDomain,
};
pub use bundle::{
    certify_intent_admission, ForgeQueryIntentAdmissionCertificationBundle,
    ForgeQueryIntentAdmissionCertificationOutput,
};
pub(crate) use fixtures::{certification_bridge, certification_runtime};
pub use manifests::{
    forge_query_intent_admission_compile_fail_targets,
    forge_query_intent_admission_crate_doc_example_targets,
    forge_query_intent_admission_golden_transcripts, ForgeQueryIntentAdmissionCompileFailTarget,
    ForgeQueryIntentAdmissionCrateDocExampleTarget, ForgeQueryIntentAdmissionGoldenTranscript,
};
pub use oracles::{
    forge_query_intent_admission_oracle_report, ForgeQueryIntentAdmissionOracleComparisonRow,
    ForgeQueryIntentAdmissionOracleLane, ForgeQueryIntentAdmissionOracleManifestRow,
    ForgeQueryIntentAdmissionOracleReport,
};
pub use output_manifest::{
    forge_query_intent_admission_certification_output_manifest,
    forge_query_intent_admission_closeout_extension_outputs,
    forge_query_intent_admission_required_certification_outputs,
};
pub use reports::*;
