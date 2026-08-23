use serde_json::Value;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    proved_record, record_claim_digest, result_artifact, set, source_digest, write_artifact,
    ORACLE_SOURCE, PRODUCTION_SOURCE,
};

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

pub(super) struct ProvedFixture {
    pub(super) record: Vec<String>,
    artifact_identity: String,
}

impl ProvedFixture {
    pub(super) fn new() -> Self {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let artifact_identity = format!(
            "workspaces/worth-ui/target/milestone-3141-ledger-fixtures/{}-{sequence}.json",
            std::process::id()
        );
        let revision = result_artifact::current_revision().unwrap();
        let sources = format!("{PRODUCTION_SOURCE};{ORACLE_SOURCE}");
        let digest = source_digest::calculate(&sources).unwrap();
        let state_digest = source_digest::calculate_source_state(&revision).unwrap();
        let run_nonce = format!("{:032x}", sequence + 1);
        let evidence = ProvedEvidence {
            artifact: &artifact_identity,
            revision: &revision,
            digest: &digest,
            state_digest: &state_digest,
            run_nonce: &run_nonce,
            sources: &sources,
        };
        let mut record = proved_record(&evidence);
        let claim_digest = record_claim_digest(&record);
        write_artifact(&evidence, &claim_digest);
        let artifact_digest = source_digest::file_digest(&artifact_identity).unwrap();
        set(&mut record, "result_artifact_digest", &artifact_digest);
        Self {
            record,
            artifact_identity,
        }
    }

    pub(super) fn mutate_artifact(&mut self, field: &str, value: Value) {
        let path = source_digest::repository_file(&self.artifact_identity).unwrap();
        let mut artifact: Value = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
        artifact[field] = value;
        std::fs::write(path, serde_json::to_vec_pretty(&artifact).unwrap()).unwrap();
        let artifact_digest = source_digest::file_digest(&self.artifact_identity).unwrap();
        set(&mut self.record, "result_artifact_digest", &artifact_digest);
    }
}

impl Drop for ProvedFixture {
    fn drop(&mut self) {
        let _ =
            std::fs::remove_file(source_digest::repository_root().join(&self.artifact_identity));
    }
}

pub(super) struct ProvedEvidence<'a> {
    pub(super) artifact: &'a str,
    pub(super) revision: &'a str,
    pub(super) digest: &'a str,
    pub(super) state_digest: &'a str,
    pub(super) run_nonce: &'a str,
    pub(super) sources: &'a str,
}
