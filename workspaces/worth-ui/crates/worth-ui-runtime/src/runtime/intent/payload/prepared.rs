use std::sync::Arc;

#[must_use]
pub struct UiPreparedIntentPayload {
    definition_id: crate::capability::UiIntentId,
    declaration: Arc<crate::declaration::UiCanonicalIntentDeclaration>,
    basis: super::UiIntentInputBasis,
    payload: crate::capability::UiSealedIntentPayload,
}

impl UiPreparedIntentPayload {
    pub(crate) const fn new(
        definition_id: crate::capability::UiIntentId,
        declaration: Arc<crate::declaration::UiCanonicalIntentDeclaration>,
        basis: super::UiIntentInputBasis,
        payload: crate::capability::UiSealedIntentPayload,
    ) -> Self {
        Self {
            definition_id,
            declaration,
            basis,
            payload,
        }
    }

    pub const fn definition_id(&self) -> crate::capability::UiIntentId {
        self.definition_id
    }

    pub fn declaration_identity(&self) -> &str {
        self.declaration.identity().as_str()
    }

    pub const fn input_basis(&self) -> &super::UiIntentInputBasisReceipt {
        self.basis.receipt()
    }

    pub fn retained_owner_reference_count(&self) -> usize {
        self.basis.retained_owner_reference_count()
    }

    pub fn retained_payload_count(&self) -> usize {
        self.payload.retained_payload_count()
    }
}
