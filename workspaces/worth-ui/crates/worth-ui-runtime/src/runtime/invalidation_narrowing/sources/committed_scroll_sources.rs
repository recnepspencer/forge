impl super::UiAllocationInvalidationAdmissionContext {
    pub(crate) fn committed_scroll_sources(
        &self,
    ) -> Result<
        Box<[crate::runtime::allocation_receipt::UiCommittedScrollActivationSource]>,
        crate::runtime::UiScrollContractAdmissionDenial,
    > {
        if let Some(denial) = self.scroll_planning_denial {
            return Err(denial);
        }
        let Some(planning) = self.scroll_planning() else {
            return Ok(Box::new([]));
        };
        Ok(planning.committed_sources())
    }
}
