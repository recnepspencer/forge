#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationRuntimeCloseoutDigest {
    canonical_digest: String,
}

impl WorthServerOperationRuntimeCloseoutDigest {
    pub(crate) fn new(
        support_row_digest: &str,
        editor_readiness_digest: &str,
        no_product_semantics_digest: &str,
    ) -> Self {
        Self {
            canonical_digest: format!(
                "worth-server-operation-runtime-closeout-v1|support={support_row_digest}|editor={editor_readiness_digest}|no-product-semantics={no_product_semantics_digest}"
            ),
        }
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
