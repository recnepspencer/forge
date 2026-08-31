impl super::UiPortalRuntimeState {
    pub(crate) fn prepare(
        &self,
        request: super::super::UiPortalServiceRequest,
    ) -> Result<
        super::super::UiPreparedPortalServiceTransition,
        super::super::UiPortalServiceTransitionDenial,
    > {
        let request = request.with_policy(self.policy);
        let committed_revision = self
            .revision
            .checked_add(1)
            .ok_or(super::super::UiPortalServiceTransitionDenial::RevisionExhausted)?;
        let parent = request
            .parent()
            .and_then(|parent| self.records.get(&parent))
            .and_then(|record| record.placement);
        let placement = super::super::UiPreparedPortalPlacement::for_request(&request, parent)
            .map_err(super::super::UiPortalServiceTransitionDenial::Placement)?;
        let (staged_posture, disposition) = super::duplicate_request::classify(
            self.prior_request(request.portal()),
            &request,
            placement,
        );
        let staged_stack_ordinal = match (request.operation(), disposition) {
            (
                super::super::request::UiPortalServiceOperation::Open,
                super::super::UiPortalServiceDisposition::Idempotent,
            ) => self
                .records
                .get(&request.portal())
                .map(|record| record.stack_ordinal),
            (super::super::request::UiPortalServiceOperation::Open, _) => {
                self.next_stack_ordinal
                    .checked_add(1)
                    .ok_or(super::super::UiPortalServiceTransitionDenial::StackOrdinalExhausted)?;
                Some(super::super::UiPortalStackOrdinal::minted(
                    self.next_stack_ordinal,
                ))
            }
            (super::super::request::UiPortalServiceOperation::Close(_), _) => None,
        };
        let closed_descendants = match request.operation() {
            super::super::request::UiPortalServiceOperation::Open => Box::default(),
            super::super::request::UiPortalServiceOperation::Close(_) => self
                .records
                .keys()
                .copied()
                .filter(|portal| self.portal_descends_from(*portal, request.portal()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        };
        Ok(super::super::UiPreparedPortalServiceTransition::new(
            request,
            self.revision,
            committed_revision,
            staged_posture,
            disposition,
            placement,
            staged_stack_ordinal,
            closed_descendants,
        ))
    }

    fn prior_request(
        &self,
        portal: super::super::UiPortalIdentity,
    ) -> Option<super::duplicate_request::UiPortalPriorRequest> {
        self.records
            .get(&portal)
            .map(|record| super::duplicate_request::UiPortalPriorRequest {
                posture: record.posture,
                semantic_surface: record.semantic_surface,
                last_request: record.last_request,
                dismissal: record.dismissal,
                placement: record.placement,
            })
            .or_else(|| self.closed_requests.prior_request(portal))
    }
}
