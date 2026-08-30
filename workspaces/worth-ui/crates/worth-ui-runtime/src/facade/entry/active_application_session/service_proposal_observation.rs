#[cfg(any(test, feature = "certification-support"))]
impl super::WorthUiActiveApplicationSession {
    pub(crate) fn inspect_service_proposals_for_certification(
        &self,
    ) -> crate::certification_support::UiServiceProposalCertificationSnapshot {
        let (entries, live_occupancies, live_cancellations) = self
            .application
            .inspect_service_proposal_resources_for_certification();
        crate::certification_support::UiServiceProposalCertificationSnapshot::new(
            entries[0],
            entries[1],
            entries[2],
            entries[3],
            live_occupancies,
            live_cancellations,
        )
    }
}
