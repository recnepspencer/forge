use super::{
    ForgeServerNoProductSemanticsCertification, ForgeServerOperationRuntimeCloseoutDigest,
    ForgeServerProductEditorReadinessCertification, ForgeServerProductOperationRuntimeSupportRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerProductOperationRuntimeCertification {
    support_row: ForgeServerProductOperationRuntimeSupportRow,
    editor_readiness: ForgeServerProductEditorReadinessCertification,
    no_product_semantics: ForgeServerNoProductSemanticsCertification,
    closeout_digest: ForgeServerOperationRuntimeCloseoutDigest,
}

impl ForgeServerProductOperationRuntimeCertification {
    pub(crate) fn new(
        support_row: ForgeServerProductOperationRuntimeSupportRow,
        editor_readiness: ForgeServerProductEditorReadinessCertification,
        no_product_semantics: ForgeServerNoProductSemanticsCertification,
    ) -> Self {
        let closeout_digest = ForgeServerOperationRuntimeCloseoutDigest::new(
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

    pub fn support_row(&self) -> &ForgeServerProductOperationRuntimeSupportRow {
        &self.support_row
    }

    pub fn editor_readiness(&self) -> &ForgeServerProductEditorReadinessCertification {
        &self.editor_readiness
    }

    pub fn no_product_semantics_proof(&self) -> &ForgeServerNoProductSemanticsCertification {
        &self.no_product_semantics
    }

    pub fn closeout_digest(&self) -> &ForgeServerOperationRuntimeCloseoutDigest {
        &self.closeout_digest
    }
}
