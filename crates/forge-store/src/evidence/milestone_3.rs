use crate::{
    authority::AuthoritativeExportBundle, evidence::StoreCounterSnapshot, failure::StoreErrorKind,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ObservedRecoveryFailure {
    pub kind: StoreErrorKind,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone3CertificationBundle {
    pub truth_digest: String,
    pub replay_digest: String,
    pub restore_digest: String,
    pub failure_digest: String,
    pub counter_snapshot: StoreCounterSnapshot,
}

impl ObservedRecoveryFailure {
    pub fn from_error(error: &crate::StoreError) -> Self {
        Self {
            kind: error.kind().clone(),
            message: error.message().to_string(),
        }
    }
}

impl Milestone3CertificationBundle {
    pub fn new(
        recovered_export: &AuthoritativeExportBundle,
        rebuilt_export: &AuthoritativeExportBundle,
        counter_snapshot: StoreCounterSnapshot,
        failures: &[ObservedRecoveryFailure],
    ) -> Self {
        let truth_digest = stable_digest(recovered_export);
        let replay_digest = stable_digest(
            &recovered_export
                .clone()
                .into_canonicalized()
                .commit_envelopes,
        );
        let restore_digest = stable_digest(rebuilt_export);
        let failure_digest = stable_digest(failures);

        Self {
            truth_digest,
            replay_digest,
            restore_digest,
            failure_digest,
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 3 certification serialization")
    }
}

fn stable_digest<T: Serialize + ?Sized>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("milestone 3 digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
