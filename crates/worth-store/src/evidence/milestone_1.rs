use crate::{authority::AuthoritativeExportBundle, evidence::StoreCounterSnapshot};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone1CertificationBundle {
    pub semantic: Milestone1SemanticCertificationEvidence,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone1SemanticCertificationEvidence {
    pub truth_digest: String,
    pub history_digest: String,
    pub branch_heads_digest: String,
    pub artifact_digest: String,
    pub replay_digest: String,
}

impl Milestone1CertificationBundle {
    pub(crate) fn from_export(
        bundle: &AuthoritativeExportBundle,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let canonical = bundle.clone().into_canonicalized();

        let truth_digest = stable_digest(&TruthDigestBasis {
            canonicalization_version: canonical.canonicalization_version,
            commit_envelopes: &canonical.commit_envelopes,
        });
        let history_digest = stable_digest(&HistoryDigestBasis {
            branch_records: &canonical.branch_records,
            commit_parent_records: &canonical.commit_parent_records,
        });
        let branch_heads_digest = stable_digest(&canonical.branch_head_records);
        let artifact_digest = stable_digest(&canonical.authoritative_artifact_digests);
        let replay_digest = stable_digest(&ReplayDigestBasis {
            canonicalization_version: canonical.canonicalization_version,
            commit_envelopes: &canonical.commit_envelopes,
            branch_head_records: &canonical.branch_head_records,
        });

        Self {
            semantic: Milestone1SemanticCertificationEvidence {
                truth_digest,
                history_digest,
                branch_heads_digest,
                artifact_digest,
                replay_digest,
            },
            counter_snapshot,
        }
    }

    pub fn semantic_json(&self) -> String {
        serde_json::to_string(&self.semantic)
            .expect("milestone 1 semantic certification serialization")
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 1 certification serialization")
    }
}

#[derive(Serialize)]
struct TruthDigestBasis<'a> {
    canonicalization_version: u32,
    commit_envelopes: &'a [crate::backend::records::StoredCommitEnvelope],
}

#[derive(Serialize)]
struct HistoryDigestBasis<'a> {
    branch_records: &'a [crate::backend::records::BranchRecord],
    commit_parent_records: &'a [crate::backend::records::CommitParentRecord],
}

#[derive(Serialize)]
struct ReplayDigestBasis<'a> {
    canonicalization_version: u32,
    commit_envelopes: &'a [crate::backend::records::StoredCommitEnvelope],
    branch_head_records: &'a [crate::backend::records::BranchHeadRecord],
}

fn stable_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("certification digest serialization");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
