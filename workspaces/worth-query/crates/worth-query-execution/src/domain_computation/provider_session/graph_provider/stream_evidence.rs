use std::sync::Arc;

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
    chunk_count: u64,
    row_count: u64,
}

impl WorthQueryGraphReadStreamAccumulator {
    pub(crate) fn new(_call: &WorthQueryGraphProviderCall) -> Self {
        Self {
            chunk_count: 0,
            row_count: 0,
        }
    }

    pub(crate) fn admit_chunk(&mut self, material: &WorthQueryGraphReadMaterial) {
        self.row_count = self
            .row_count
            .saturating_add(u64::try_from(material.rows().len()).unwrap_or(u64::MAX));
        self.chunk_count = self.chunk_count.saturating_add(1);
    }

    pub(crate) fn finish(
        self,
        call: &WorthQueryGraphProviderCall,
    ) -> WorthQueryExecutionGraphReadStreamEvidence {
        WorthQueryExecutionGraphReadStreamEvidence {
            authority_identity: call.authority_identity(),
            identity: Arc::from(call.call_identity()),
            call_identity: Arc::from(call.call_identity()),
            provider_session_identity: Arc::from(call.provider_session_identity()),
            canonical_query_digest: Arc::from(call.canonical_query_digest()),
            basis_identity: Arc::from(call.basis_identity()),
            snapshot_identity: Arc::from(call.snapshot_identity()),
            chunk_count: self.chunk_count,
            row_count: self.row_count,
        }
    }
}
