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
    "WorthQueryIntentAdmissionProofShapeAudit",
    "WorthQueryIntentAdmissionPublicBoundaryAudit",
    "WorthQueryIntentAdmissionTopologyAudit",
    "WorthQueryIntentAdmissionTopologyAuditRow",
    "WorthQueryIntentAdmissionTopologyDomain",
    "certify_intent_admission",
    "WorthQueryIntentAdmissionCertificationBundle",
    "WorthQueryIntentAdmissionCertificationOutput",
    "worth_query_intent_admission_compile_fail_targets",
    "worth_query_intent_admission_crate_doc_example_targets",
    "worth_query_intent_admission_golden_transcripts",
    "WorthQueryIntentAdmissionCompileFailTarget",
    "WorthQueryIntentAdmissionCrateDocExampleTarget",
    "WorthQueryIntentAdmissionGoldenTranscript",
    "worth_query_intent_admission_oracle_report",
    "WorthQueryIntentAdmissionOracleComparisonRow",
    "WorthQueryIntentAdmissionOracleLane",
    "WorthQueryIntentAdmissionOracleManifestRow",
    "WorthQueryIntentAdmissionOracleReport",
    "worth_query_intent_admission_certification_output_manifest",
    "worth_query_intent_admission_doc_example_report",
    "WorthQueryIntentAdmissionDocExampleReport",
    "WorthQueryIntentAdmissionDocExampleRow",
    "worth_query_intent_admission_legacy_parity_report",
    "WorthQueryIntentAdmissionLegacyParityCheck",
    "WorthQueryIntentAdmissionLegacyParityLane",
    "WorthQueryIntentAdmissionLegacyParityReport",
    "WorthQueryIntentAdmissionLegacyParityRow",
    "worth_query_intent_admission_representative_family_report",
    "WorthQueryIntentAdmissionRepresentativeFamilyLane",
    "WorthQueryIntentAdmissionRepresentativeFamilyReport",
    "WorthQueryIntentAdmissionRepresentativeFamilyRow",
    "worth_query_intent_admission_representative_output_report",
    "WorthQueryIntentAdmissionRepresentativeOutputReport",
    "worth_query_intent_admission_seeded_certification_report",
    "WorthQueryIntentAdmissionSeedGeneratorClass",
    "WorthQueryIntentAdmissionSeedReplayRow",
    "WorthQueryIntentAdmissionSeededCertificationReport",
    "WorthQueryIntentAdmissionSlopeLane",
    "WorthQueryIntentAdmissionWidthRunRow",
    "WorthQueryIntentAdmissionWidthRunScale",
    "worth_query_intent_admission_slope_report",
    "WorthQueryIntentAdmissionCertificationCounterSnapshot",
    "WorthQueryIntentAdmissionSlopeReport",
    "worth_query_intent_admission_support_traceability_report",
    "WorthQueryIntentAdmissionSupportTraceabilityReport",
    "WorthQueryIntentAdmissionSupportTraceabilityRow",
];

pub use audits::{
    WorthQueryIntentAdmissionProofShapeAudit, WorthQueryIntentAdmissionPublicBoundaryAudit,
    WorthQueryIntentAdmissionTopologyAudit, WorthQueryIntentAdmissionTopologyAuditRow,
    WorthQueryIntentAdmissionTopologyDomain,
};
pub use bundle::{
    certify_intent_admission, WorthQueryIntentAdmissionCertificationBundle,
    WorthQueryIntentAdmissionCertificationOutput,
};
pub(crate) use fixtures::{certification_bridge, certification_runtime};
pub use manifests::{
    worth_query_intent_admission_compile_fail_targets,
    worth_query_intent_admission_crate_doc_example_targets,
    worth_query_intent_admission_golden_transcripts, WorthQueryIntentAdmissionCompileFailTarget,
    WorthQueryIntentAdmissionCrateDocExampleTarget, WorthQueryIntentAdmissionGoldenTranscript,
};
pub use oracles::{
    worth_query_intent_admission_oracle_report, WorthQueryIntentAdmissionOracleComparisonRow,
    WorthQueryIntentAdmissionOracleLane, WorthQueryIntentAdmissionOracleManifestRow,
    WorthQueryIntentAdmissionOracleReport,
};
pub use output_manifest::{
    worth_query_intent_admission_certification_output_manifest,
    worth_query_intent_admission_closeout_extension_outputs,
    worth_query_intent_admission_required_certification_outputs,
};
pub use reports::*;
