use std::sync::Arc;

use crate::execution_digest::hash_parts;

use super::call_identity::WorthQueryGraphCallAuthorityIdentity;
use super::{WorthQueryGraphProviderCall, WorthQueryGraphReadMaterial, WorthQueryGraphReadRow};

#[derive(Debug, PartialEq)]
pub struct WorthQueryExecutionGraphReadProduct {
    authority_identity: WorthQueryGraphCallAuthorityIdentity,
    identity: Arc<str>,
    call_identity: Arc<str>,
    provider_session_identity: Arc<str>,
    canonical_query_digest: Arc<str>,
    basis_identity: Arc<str>,
    snapshot_identity: Arc<str>,
    result_digest: Arc<str>,
    rows: Box<[WorthQueryGraphReadRow]>,
}

impl WorthQueryExecutionGraphReadProduct {
    pub(super) fn seal(
        call: &WorthQueryGraphProviderCall,
        material: WorthQueryGraphReadMaterial,
    ) -> Self {
        let rows = material.into_rows().into_boxed_slice();
        let mut digest_parts = vec![
            "worth_query_execution_graph_read_product_v1".into(),
            format!("call:{}", call.call_identity()),
            format!("query:{}", call.canonical_query_digest()),
            format!("basis:{}", call.basis_identity()),
            format!("snapshot:{}", call.snapshot_identity()),
            format!("rows:{}", rows.len()),
        ];
        for (index, row) in rows.iter().enumerate() {
            digest_parts.push(format!("row-index:{index}"));
            digest_parts.extend(row.digest_parts());
        }
        let result_digest = Arc::<str>::from(hash_parts(&digest_parts));
        let identity = Arc::<str>::from(hash_parts(&[
            "worth_query_execution_graph_read_product_identity_v1".into(),
            format!("call-authority:{}", call.authority_identity().as_u64()),
            format!("result:{result_digest}"),
        ]));
        Self {
            authority_identity: call.authority_identity(),
            identity,
            call_identity: Arc::from(call.call_identity()),
            provider_session_identity: Arc::from(call.provider_session_identity()),
            canonical_query_digest: Arc::from(call.canonical_query_digest()),
            basis_identity: Arc::from(call.basis_identity()),
            snapshot_identity: Arc::from(call.snapshot_identity()),
            result_digest,
            rows,
        }
    }

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

    pub fn rows(&self) -> &[WorthQueryGraphReadRow] {
        &self.rows
    }

    pub(super) fn authority_identity(&self) -> WorthQueryGraphCallAuthorityIdentity {
        self.authority_identity
    }
}
