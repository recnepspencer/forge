use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

pub(super) const FIELDS: &[&str] = &[
    "phase",
    "requirement",
    "owner",
    "production_boundary",
    "world_identity",
    "world_version",
    "proof_kind",
    "evidence_schema",
    "baseline_digest",
    "scenario_delta",
    "generated_seed",
    "authority_provenance",
    "production_entry",
    "independent_oracle",
    "mutation_control",
    "fault_injection_boundary",
    "retained_failure_artifact",
    "teardown_result",
    "construction_cost",
    "execution_cost",
    "source_identity",
    "font_profile_identity",
    "font_profile_digest",
    "native_profile_identity",
    "native_profile_digest",
    "platform_versions",
    "structural_counters",
    "presented_source_readback",
    "client_area_observation",
];

pub(super) fn calculate(row: &BTreeMap<String, String>) -> String {
    let mut digest = Sha256::new();
    for field in FIELDS {
        digest.update(field.as_bytes());
        digest.update([0]);
        digest.update(row[*field].as_bytes());
        digest.update([0]);
    }
    format!("{:x}", digest.finalize())
}
