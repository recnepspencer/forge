use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::execution_digest::hash_parts;

use super::call_identity::WorthQueryGraphCallAuthorityIdentity;
use super::{WorthQueryGraphProviderCall, WorthQueryGraphReadMaterial};

#[derive(Debug, PartialEq)]
pub struct WorthQueryExecutionGraphReadStreamEvidence {
    authority_identity: WorthQueryGraphCallAuthorityIdentity,
    identity: Arc<str>,
    call_identity: Arc<str>,
    provider_session_identity: Arc<str>,
    canonical_query_digest: Arc<str>,
    basis_identity: Arc<str>,
    snapshot_identity: Arc<str>,
    result_digest: Arc<str>,
    chunk_count: u64,
    row_count: u64,
}

impl WorthQueryExecutionGraphReadStreamEvidence {
    pub fn identity(&self) -> &str {
        &self.identity
    }

    pub fn call_identity(&self) -> &str {
        &self.call_identity
    }

    pub fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }

    pub fn canonical_query_digest(&self) -> &str {
        &self.canonical_query_digest
    }

    pub fn basis_identity(&self) -> &str {
        &self.basis_identity
    }

    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }

    pub fn result_digest(&self) -> &str {
        &self.result_digest
    }

    pub const fn chunk_count(&self) -> u64 {
        self.chunk_count
    }

    pub const fn row_count(&self) -> u64 {
        self.row_count
    }

    pub(super) const fn authority_identity(&self) -> WorthQueryGraphCallAuthorityIdentity {
        self.authority_identity
    }
}

pub(crate) struct WorthQueryGraphReadStreamAccumulator {
    hasher: Sha256,
    chunk_count: u64,
    row_count: u64,
}

impl WorthQueryGraphReadStreamAccumulator {
    pub(crate) fn new(call: &WorthQueryGraphProviderCall) -> Self {
        let mut hasher = Sha256::new();
        hash_part(&mut hasher, "worth_query_execution_graph_read_stream_v1");
        hash_part(&mut hasher, call.canonical_query_digest());
        hash_part(&mut hasher, call.basis_identity());
        Self {
            hasher,
            chunk_count: 0,
            row_count: 0,
        }
    }

    pub(crate) fn admit_chunk(&mut self, material: &WorthQueryGraphReadMaterial) {
        hash_part(&mut self.hasher, &format!("chunk:{}", self.chunk_count));
        hash_part(
            &mut self.hasher,
            &format!("chunk-rows:{}", material.rows().len()),
        );
        for row in material.rows() {
            hash_part(&mut self.hasher, &format!("row:{}", self.row_count));
            for part in row.digest_parts() {
                hash_part(&mut self.hasher, &part);
            }
            self.row_count = self.row_count.saturating_add(1);
        }
        self.chunk_count = self.chunk_count.saturating_add(1);
    }

    pub(crate) fn finish(
        mut self,
        call: &WorthQueryGraphProviderCall,
    ) -> WorthQueryExecutionGraphReadStreamEvidence {
        hash_part(&mut self.hasher, &format!("chunks:{}", self.chunk_count));
        hash_part(&mut self.hasher, &format!("rows:{}", self.row_count));
        let result_digest = Arc::<str>::from(format!("{:x}", self.hasher.finalize()));
        let identity = Arc::<str>::from(hash_parts(&[
            "worth_query_execution_graph_read_stream_identity_v1".into(),
            format!("call-authority:{}", call.authority_identity().as_u64()),
            format!("result:{result_digest}"),
        ]));
        WorthQueryExecutionGraphReadStreamEvidence {
            authority_identity: call.authority_identity(),
            identity,
            call_identity: Arc::from(call.call_identity()),
            provider_session_identity: Arc::from(call.provider_session_identity()),
            canonical_query_digest: Arc::from(call.canonical_query_digest()),
            basis_identity: Arc::from(call.basis_identity()),
            snapshot_identity: Arc::from(call.snapshot_identity()),
            result_digest,
            chunk_count: self.chunk_count,
            row_count: self.row_count,
        }
    }
}

fn hash_part(hasher: &mut Sha256, part: &str) {
    hasher.update(part.len().to_le_bytes());
    hasher.update(part.as_bytes());
}
