impl super::UiAllocationInvalidationAdmissionContext {
    pub(crate) fn committed_portal_source(
        &self,
    ) -> Option<Option<crate::runtime::allocation_receipt::UiCommittedPortalActivationSource>> {
        let Some(planning) = self.portal_planning() else {
            return Some(None);
        };
        let contract = planning.bind(&self.basis)?;
        Some(Some(
            crate::runtime::allocation_receipt::UiCommittedPortalActivationSource::Host {
                witness: contract.witness(),
                contract,
            },
        ))
    }
}
