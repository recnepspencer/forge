use super::WorthQueryAdmittedApplicationOperation;

impl<Schema, Operation, Input, Scope>
    WorthQueryAdmittedApplicationOperation<Schema, Operation, Input, Scope>
{
    pub(in crate::domain_computation) const fn operation_authority_identity_bytes(
        &self,
    ) -> &[u8; 32] {
        &self.operation_authority_identity_bytes
    }

    pub(in crate::domain_computation) const fn governed_input_identity(&self) -> Option<&[u8; 32]> {
        self.governed_input_identity.as_ref()
    }

    pub(in crate::domain_computation) fn governed_proposal_identity(&self) -> Option<&[u8; 32]> {
        self.delegation_activation_binding()
            .map(|binding| binding.proposal_identity())
            .or_else(|| {
                self.capability_revocation_binding()
                    .map(|binding| binding.proposal_identity())
            })
    }
}
