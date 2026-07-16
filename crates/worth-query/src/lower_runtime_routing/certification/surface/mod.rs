mod acceptance;
mod acceptance_cardinality;
mod acceptance_checks;
#[cfg(test)]
mod acceptance_checks_tests;
mod acceptance_policy;
mod evidence;
#[cfg(test)]
mod evidence_constructor_tests;
#[cfg(test)]
mod evidence_tests;
#[cfg(test)]
mod evidence_width_tests;
mod fixtures;
mod synthetic_tail_report;
mod transcripts;

pub use acceptance::{
    worth_query_lower_runtime_acceptance_suite, WorthQueryLowerRuntimeAcceptanceLane,
    WorthQueryLowerRuntimeAcceptanceRow, WorthQueryLowerRuntimeAcceptanceSuite,
};
pub(super) use acceptance_policy::{
    allowed_phase_six_synthetic_seams, required_phase_six_concrete_seams,
};
pub(super) use evidence::{
    worth_query_lower_runtime_representative_surface,
    WorthQueryLowerRuntimeRepresentativeEvidenceSource,
    WorthQueryLowerRuntimeRepresentativeSurface,
};
pub use synthetic_tail_report::{
    worth_query_lower_runtime_synthetic_tail_report, WorthQueryLowerRuntimeSyntheticTailReport,
    WorthQueryLowerRuntimeSyntheticTailRow,
};
pub(crate) use transcripts::worth_query_lower_runtime_golden_transcript_digest;
pub use transcripts::{
    worth_query_lower_runtime_golden_transcripts, worth_query_lower_runtime_target_dx_digest,
};
