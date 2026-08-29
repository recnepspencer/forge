use std::collections::BTreeMap;

#[path = "state/duplicate_request.rs"]
mod duplicate_request;
#[path = "state/mounted_projection.rs"]
mod mounted_projection;
#[path = "state/shutdown.rs"]
mod shutdown;

pub(crate) use shutdown::UiPortalShutdownReport;

#[cfg(test)]
pub(super) const fn duplicate_request_capacity_for_test() -> usize {
    duplicate_request::UI_PORTAL_CLOSED_REQUEST_CAPACITY
}

/// `records` holds only live portals: `Open`, `Visible`, or `Closing`. A portal
/// that reaches `Closed` leaves the live table and keeps only its bounded
/// duplicate-request row, so placement, dismissal, descendant, and command
/// routing work stays proportional to the currently active portals rather than
/// to every portal the session ever opened.
pub(crate) struct UiPortalRuntimeState {
    #[cfg(test)]
    persistence: crate::runtime::UiServiceStatePersistencePosture,
    pub(super) policy: crate::declaration::UiPortalPolicy,
    pub(super) records: BTreeMap<super::UiPortalIdentity, UiPortalRecord>,
    closed_requests: duplicate_request::UiPortalClosedRequestWindow,
    admitted_requests: u64,
    idempotent_requests: u64,
    revision: u64,
    last_closed: Option<super::UiPortalClosedInspectionRecord>,
}

pub(super) struct UiPortalRecord {
    pub(super) posture: super::UiPortalLifecyclePosture,
    pub(super) semantic_surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    last_request: crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity,
    dismissal: Option<super::UiPortalDismissalCause>,
    pub(super) placement: Option<super::UiCommittedPortalPlacement>,
    exit_retention: Option<super::UiPortalExitRetentionReceipt>,
}

impl UiPortalRuntimeState {
    pub(crate) fn new(persistence: crate::runtime::UiServiceStatePersistencePosture) -> Self {
        Self::new_with_policy(persistence, crate::declaration::UiPortalPolicy::dropdown())
    }

    pub(crate) fn new_with_policy(
        _persistence: crate::runtime::UiServiceStatePersistencePosture,
        policy: crate::declaration::UiPortalPolicy,
    ) -> Self {
        Self {
            #[cfg(test)]
            persistence: _persistence,
            policy,
            records: BTreeMap::new(),
            closed_requests: duplicate_request::UiPortalClosedRequestWindow::new(),
            admitted_requests: 0,
            idempotent_requests: 0,
            revision: 0,
            last_closed: None,
        }
    }

    pub(crate) fn apply_policy(&mut self, policy: crate::declaration::UiPortalPolicy) {
        self.policy = policy;
    }

    #[cfg(test)]
    pub(crate) const fn persistence(&self) -> crate::runtime::UiServiceStatePersistencePosture {
        self.persistence
    }

    pub(crate) fn prepare(
        &self,
        request: super::UiPortalServiceRequest,
    ) -> Result<super::UiPreparedPortalServiceTransition, super::UiPortalServiceTransitionDenial>
    {
        let request = request.with_policy(self.policy);
        let committed_revision = self
            .revision
            .checked_add(1)
            .ok_or(super::UiPortalServiceTransitionDenial::RevisionExhausted)?;
        let parent = request
            .parent()
            .and_then(|parent| self.records.get(&parent))
            .and_then(|record| record.placement);
        let placement = super::UiPreparedPortalPlacement::for_request(&request, parent)
            .map_err(super::UiPortalServiceTransitionDenial::Placement)?;
        let (staged_posture, disposition) =
            duplicate_request::classify(self.prior_request(request.portal()), &request, placement);
        let closed_descendants = match request.operation() {
            super::request::UiPortalServiceOperation::Open => Box::default(),
            super::request::UiPortalServiceOperation::Close(_) => self
                .records
                .keys()
                .copied()
                .filter(|portal| self.portal_descends_from(*portal, request.portal()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        };
        Ok(super::UiPreparedPortalServiceTransition::new(
            request,
            self.revision,
            committed_revision,
            staged_posture,
            disposition,
            placement,
            closed_descendants,
        ))
    }

    pub(crate) fn commit_published(
        &mut self,
        transition: super::UiPreparedPortalServiceTransition,
    ) -> Result<super::UiPortalServiceReceipt, super::UiPortalServiceTransitionDenial> {
        self.commit_published_with_exit_retention(transition, false)
            .map(|(receipt, _)| receipt)
    }

    pub(crate) fn commit_published_with_exit_retention(
        &mut self,
        transition: super::UiPreparedPortalServiceTransition,
        retain_exit: bool,
    ) -> Result<
        (
            super::UiPortalServiceReceipt,
            Option<super::UiPortalExitRetentionReceipt>,
        ),
        super::UiPortalServiceTransitionDenial,
    > {
        self.require_current(&transition)?;
        let existing_retention = self
            .records
            .get(&transition.portal())
            .and_then(|record| record.exit_retention);
        if transition.is_idempotent() {
            return self
                .commit(transition, None, existing_retention)
                .map(|receipt| (receipt, existing_retention));
        }
        let posture = match transition.request().operation() {
            super::request::UiPortalServiceOperation::Open => {
                super::UiPortalLifecyclePosture::Visible
            }
            super::request::UiPortalServiceOperation::Close(_) if retain_exit => {
                super::UiPortalLifecyclePosture::Closing
            }
            super::request::UiPortalServiceOperation::Close(_) => {
                super::UiPortalLifecyclePosture::Closed
            }
        };
        let exit_retention = (posture == super::UiPortalLifecyclePosture::Closing).then(|| {
            super::UiPortalExitRetentionReceipt::new(
                transition.portal(),
                transition.committed_revision(),
                transition.request().idempotency().lineage(),
            )
        });
        self.commit(transition, Some(posture), exit_retention)
            .map(|receipt| (receipt, exit_retention))
    }

    pub(crate) fn prepare_exit_terminal(
        &self,
        retention: super::UiPortalExitRetentionReceipt,
        idempotency: crate::runtime::intent_execution::UiIntentExecutionIdempotencyIdentity,
    ) -> Result<super::UiPreparedPortalServiceTransition, super::UiPortalExitTerminalDenial> {
        let record = self
            .records
            .get(&retention.portal())
            .filter(|record| {
                record.posture == super::UiPortalLifecyclePosture::Closing
                    && record.exit_retention == Some(retention)
            })
            .ok_or(super::UiPortalExitTerminalDenial::RetentionMismatch)?;
        self.prepare(super::UiPortalServiceRequest::close(
            retention.portal(),
            idempotency,
            record
                .dismissal
                .expect("a Closing portal retains its dismissal cause"),
            record.semantic_surface,
        ))
        .map_err(super::UiPortalExitTerminalDenial::Transition)
    }

    pub(crate) fn validate_prepared(
        &self,
        transition: &super::UiPreparedPortalServiceTransition,
    ) -> Result<(), super::UiPortalServiceTransitionDenial> {
        self.require_current(transition)
    }

    fn commit(
        &mut self,
        transition: super::UiPreparedPortalServiceTransition,
        posture: Option<super::UiPortalLifecyclePosture>,
        exit_retention: Option<super::UiPortalExitRetentionReceipt>,
    ) -> Result<super::UiPortalServiceReceipt, super::UiPortalServiceTransitionDenial> {
        self.require_current(&transition)?;
        let request = transition.request();
        let posture = posture.unwrap_or(transition.staged_posture());
        let dismissal = match request.operation() {
            super::request::UiPortalServiceOperation::Open => None,
            super::request::UiPortalServiceOperation::Close(cause) => Some(cause),
        };
        if matches!(
            posture,
            super::UiPortalLifecyclePosture::Closed | super::UiPortalLifecyclePosture::Closing
        ) {
            for descendant in transition.closed_descendants().iter().copied() {
                let mut record = self
                    .records
                    .remove(&descendant)
                    .expect("prepared descendant closure retains its portal record");
                record.posture = posture;
                record.dismissal = Some(super::UiPortalDismissalCause::ParentClosed);
                record.exit_retention = None;
                if posture == super::UiPortalLifecyclePosture::Closed {
                    record.placement = None;
                }
                self.retain_record(descendant, record);
            }
        }
        let placement = transition
            .placement()
            .map(super::UiCommittedPortalPlacement::from_prepared)
            .or_else(|| {
                (posture == super::UiPortalLifecyclePosture::Closing)
                    .then(|| self.records.get(&request.portal())?.placement)
                    .flatten()
            });
        self.retain_committed_record(
            request,
            UiPortalRecord {
                posture,
                semantic_surface: request.semantic_surface(),
                last_request: request.idempotency(),
                dismissal,
                placement,
                exit_retention,
            },
        );
        self.admitted_requests = self.admitted_requests.saturating_add(1);
        if transition.disposition() == super::UiPortalServiceDisposition::Idempotent {
            self.idempotent_requests = self.idempotent_requests.saturating_add(1);
        }
        self.revision = transition.committed_revision();
        if let super::request::UiPortalServiceOperation::Close(cause) = request.operation() {
            self.last_closed = Some(super::UiPortalClosedInspectionRecord::new(
                request.portal(),
                cause,
                transition.closed_descendants().len(),
                self.revision,
            ));
        }
        Ok(super::UiPortalServiceReceipt::new(
            request.portal(),
            posture,
            transition.disposition(),
        ))
    }

    /// A terminally closed portal leaves the live table and keeps only its
    /// bounded duplicate-request row; every other posture stays live.
    fn retain_record(&mut self, portal: super::UiPortalIdentity, record: UiPortalRecord) {
        if record.posture == super::UiPortalLifecyclePosture::Closed {
            self.records.remove(&portal);
            self.closed_requests.retain(
                portal,
                record.semantic_surface,
                record.last_request,
                record
                    .dismissal
                    .expect("a Closed portal record retains its dismissal cause"),
            );
        } else {
            self.closed_requests.forget(portal);
            self.records.insert(portal, record);
        }
    }

    fn retain_committed_record(
        &mut self,
        request: super::UiPortalServiceRequest,
        record: UiPortalRecord,
    ) {
        self.retain_record(request.portal(), record);
    }

    fn prior_request(
        &self,
        portal: super::UiPortalIdentity,
    ) -> Option<duplicate_request::UiPortalPriorRequest> {
        self.records
            .get(&portal)
            .map(|record| duplicate_request::UiPortalPriorRequest {
                posture: record.posture,
                semantic_surface: record.semantic_surface,
                last_request: record.last_request,
                dismissal: record.dismissal,
                placement: record.placement,
            })
            .or_else(|| self.closed_requests.prior_request(portal))
    }

    fn require_current(
        &self,
        transition: &super::UiPreparedPortalServiceTransition,
    ) -> Result<(), super::UiPortalServiceTransitionDenial> {
        if transition.expected_revision() == self.revision {
            Ok(())
        } else {
            Err(super::UiPortalServiceTransitionDenial::StalePlan)
        }
    }

    #[cfg(test)]
    pub(crate) fn posture(
        &self,
        portal: super::UiPortalIdentity,
    ) -> super::UiPortalLifecyclePosture {
        self.records
            .get(&portal)
            .map_or(super::UiPortalLifecyclePosture::Closed, |record| {
                record.posture
            })
    }

    /// The live table holds exactly the active portals, so this is a length
    /// rather than a scan.
    pub(crate) fn active_count(&self) -> usize {
        self.records.len()
    }

    /// Graph nodes exposed by active Portal scopes are the Portal owner/anchor
    /// nodes in 3.15, not child content mounted inside the Portal. Bounded by
    /// the active portals because terminal portals leave the live table.
    pub(crate) fn active_portal_owner_graph_nodes(
        &self,
    ) -> impl Iterator<Item = crate::graph::UiGraphNodeIdentity> + '_ {
        self.records
            .keys()
            .map(|identity| identity.owner().graph_node())
    }

    pub(crate) fn posture_count(&self, posture: super::UiPortalLifecyclePosture) -> usize {
        self.records
            .values()
            .filter(|record| record.posture == posture)
            .count()
    }

    pub(crate) const fn admitted_requests(&self) -> u64 {
        self.admitted_requests
    }

    pub(crate) const fn idempotent_requests(&self) -> u64 {
        self.idempotent_requests
    }

    pub(crate) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(crate) const fn last_closed(&self) -> Option<super::UiPortalClosedInspectionRecord> {
        self.last_closed
    }

    /// Live portal records plus the bounded duplicate-request rows retained for
    /// terminally closed portals. Both must reach zero at shutdown.
    pub(crate) fn record_count(&self) -> usize {
        self.records.len() + self.closed_requests.len()
    }

    #[cfg(test)]
    pub(super) fn live_record_count(&self) -> usize {
        self.records.len()
    }

    pub(super) fn clear_closed_requests(&mut self) {
        self.closed_requests.clear();
    }
}
