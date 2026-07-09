use super::{
    WorthServerNoProductSemanticsCertification, WorthServerOperationRuntimeCloseoutDigest,
    WorthServerProductEditorReadinessCertification, WorthServerProductOperationRuntimeSupportRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerProductOperationRuntimeCertification {
    support_row: WorthServerProductOperationRuntimeSupportRow,
    editor_readiness: WorthServerProductEditorReadinessCertification,
    no_product_semantics: WorthServerNoProductSemanticsCertification,
    closeout_digest: WorthServerOperationRuntimeCloseoutDigest,
}

impl WorthServerProductOperationRuntimeCertification {
    pub(crate) fn new(
        support_row: WorthServerProductOperationRuntimeSupportRow,
        editor_readiness: WorthServerProductEditorReadinessCertification,
        no_product_semantics: WorthServerNoProductSemanticsCertification,
    ) -> Self {
        let closeout_digest = WorthServerOperationRuntimeCloseoutDigest::new(
            support_row.canonical_digest(),
            editor_readiness.canonical_digest(),
            no_product_semantics.canonical_digest(),
        );
        Self {
            support_row,
            editor_readiness,
            no_product_semantics,
            closeout_digest,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.support_row.is_ready()
            && self.editor_readiness.is_ready()
            && self.no_product_semantics.is_ready()
    }

    pub fn support_row(&self) -> &WorthServerProductOperationRuntimeSupportRow {
        &self.support_row
    }

    pub fn editor_readiness(&self) -> &WorthServerProductEditorReadinessCertification {
        &self.editor_readiness
    }

    pub fn no_product_semantics_proof(&self) -> &WorthServerNoProductSemanticsCertification {
        &self.no_product_semantics
    }

    pub fn closeout_digest(&self) -> &WorthServerOperationRuntimeCloseoutDigest {
        &self.closeout_digest
    }
}
