use super::*;

impl RuntimeBridge {
    /// Validates one bridge-owned async source declaration without yet claiming
    /// that it has been lowered through an admitted Signal family.
    pub fn validate_async_source_declaration(
        &self,
        draft: BridgeAsyncSourceDeclarationDraft,
    ) -> Result<ValidatedBridgeAsyncSourceDeclaration, BridgeAsyncSourceDeclarationRejection> {
        let _ = self;
        ValidatedBridgeAsyncSourceDeclaration::validate(draft)
    }

    /// Lowers one already-validated bridge async source declaration through one
    /// explicit admitted Signal declaration family.
    pub fn lower_async_source_declaration(
        &self,
        declaration: &ValidatedBridgeAsyncSourceDeclaration,
    ) -> Result<LoweredBridgeAsyncSourceDeclaration, BridgeAsyncSourceDeclarationRejection> {
        let _ = self;
        LoweredBridgeAsyncSourceDeclaration::lower(declaration)
    }
}
