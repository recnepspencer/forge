use std::collections::BTreeMap;

#[path = "state/mounted_projection.rs"]
mod mounted_projection;
#[path = "state/shutdown.rs"]
mod shutdown;

pub(crate) use shutdown::UiPortalShutdownReport;

pub(crate) struct UiPortalRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
    pub(super) policy: crate::declaration::UiPortalPolicy,
    pub(super) records: BTreeMap<super::UiPortalIdentity, UiPortalRecord>,
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
        persistence: crate::runtime::UiServiceStatePersistencePosture,
        policy: crate::declaration::UiPortalPolicy,
    ) -> Self {
        Self {
            persistence,
            policy,
            records: BTreeMap::new(),
            admitted_requests: 0,
            idempotent_requests: 0,
            revision: 0,
            last_closed: None,
        }
    }

    pub(crate) fn apply_policy(&mut self, policy: crate::declaration::UiPortalPolicy) {
        self.policy = policy;
    }

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
        let record = self.records.get(&request.portal());
        let exact_request = record.is_some_and(|record| {
            record.last_request == request.idempotency()
                && record.semantic_surface == request.semantic_surface()
                && match request.operation() {
                    super::request::UiPortalServiceOperation::Open => {
                        record.dismissal.is_none()
                            && record.placement.map(|placement| placement.prepared()) == placement
                    }
                    super::request::UiPortalServiceOperation::Close(cause) => {
                        record.dismissal == Some(cause)
                    }
                }
        });
        let idempotent = exact_request
            && record.is_some_and(|record| {
                matches!(
                    (record.posture, request.operation()),
                    (
                        super::UiPortalLifecyclePosture::Open
                            | super::UiPortalLifecyclePosture::Visible,
                        super::request::UiPortalServiceOperation::Open
                    ) | (
                        super::UiPortalLifecyclePosture::Closing
                            | super::UiPortalLifecyclePosture::Closed,
                        super::request::UiPortalServiceOperation::Close(_)
                    )
                )
            });
        let (staged_posture, disposition) = if idempotent {
            (
                record.expect("exact request has a portal record").posture,
                super::UiPortalServiceDisposition::Idempotent,
            )
        } else {
            match request.operation() {
                super::request::UiPortalServiceOperation::Open => (
                    super::UiPortalLifecyclePosture::Open,
                    super::UiPortalServiceDisposition::Opened,
                ),
                super::request::UiPortalServiceOperation::Close(_) => (
                    super::UiPortalLifecyclePosture::Closing,
                    super::UiPortalServiceDisposition::Closing,
                ),
            }
        };
        let closed_descendants = match request.operation() {
            super::request::UiPortalServiceOperation::Open => Box::default(),
            super::request::UiPortalServiceOperation::Close(_) => self
                .records
                .iter()
                .filter_map(|(portal, record)| {
                    (record.posture != super::UiPortalLifecyclePosture::Closed
                        && self.portal_descends_from(*portal, request.portal()))
                    .then_some(*portal)
                })
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
            for descendant in transition.closed_descendants() {
                let record = self
                    .records
                    .get_mut(descendant)
                    .expect("prepared descendant closure retains its portal record");
                record.posture = posture;
                record.dismissal = Some(super::UiPortalDismissalCause::ParentClosed);
                record.exit_retention = None;
                if posture == super::UiPortalLifecyclePosture::Closed {
                    record.placement = None;
                }
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
        self.records.insert(
            request.portal(),
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

    pub(crate) fn active_count(&self) -> usize {
        self.records
            .values()
            .filter(|record| record.posture != super::UiPortalLifecyclePosture::Closed)
            .count()
    }

    /// Graph nodes exposed by active Portal scopes are the Portal owner/anchor
    /// nodes in 3.15, not child content mounted inside the Portal.
    pub(crate) fn active_portal_owner_graph_nodes(
        &self,
    ) -> impl Iterator<Item = crate::graph::UiGraphNodeIdentity> + '_ {
        self.records.iter().filter_map(|(identity, record)| {
            (record.posture != super::UiPortalLifecyclePosture::Closed)
                .then(|| identity.owner().graph_node())
        })
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

    pub(crate) fn record_count(&self) -> usize {
        self.records.len()
    }
}
