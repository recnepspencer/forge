use serde::Serialize;

#[derive(Serialize)]
pub(super) struct MutationObservation {
    pub(super) id: u8,
    pub(super) source_binding: &'static str,
    pub(super) source_sha256: String,
    pub(super) mutant_sha256: String,
    pub(super) binary_binding: String,
    pub(super) binary_sha256: String,
    pub(super) profile_binding: &'static str,
    pub(super) scenario_binding: &'static str,
    pub(super) expected_failing_predicate: &'static str,
    pub(super) actual_failing_predicate: String,
    pub(super) localization: String,
}

pub(super) fn encode(observation: &MutationObservation) -> Result<String, String> {
    serde_json::to_string(observation)
        .map_err(|error| format!("cannot encode mutation evidence: {error}"))
}
