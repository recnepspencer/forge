use super::*;

impl ProductUnpublishedOwnerEffectsRecord {
    pub(crate) fn identity(&self) -> &ProductUnpublishedOwnerEffectsIdentity {
        &self.identity
    }

    pub(crate) const fn metadata_bytes(&self) -> usize {
        self.metadata_bytes
    }

    pub(crate) fn derived_metadata_bytes(&self) -> usize {
        // Every retained field, including the bounded action array, is inline
        // in the record. Owner pins, history custody, and owner results retain
        // their own proof and are not allocator-sized hints.
        std::mem::size_of_val(self)
    }

    pub(crate) const fn catalog_affinity(&self) -> usize {
        self.catalog_affinity
    }

    pub(crate) fn settlement_required(&self) -> bool {
        matches!(
            self.progress.relational_posture(),
            crate::publication::RelationalAttemptProgressPosture::Performed
                | crate::publication::RelationalAttemptProgressPosture::SettlementRequired
                | crate::publication::RelationalAttemptProgressPosture::SettlementPending
        )
    }

    pub(crate) fn next_actions(&self) -> &[ProductUnpublishedNextAction] {
        self.next_actions.as_slice()
    }

    pub(crate) fn take_relational_recovery(&mut self) -> Result<RelationalRecoveryRecordState, ()> {
        let progress = std::mem::replace(&mut self.progress, CompositeAttemptProgress::untouched());
        let component_results = std::mem::replace(
            &mut self.component_results,
            CompositeOwnerExecutionResults::retained(),
        );
        let (commit_identity, successor_basis, recovery_route, signal_posture) =
            match progress.into_relational_recovery_parts() {
                Ok(parts) => parts,
                Err(progress) => {
                    self.progress = progress;
                    self.component_results = component_results;
                    return Err(());
                }
            };
        let route = match recovery_route {
            crate::publication::RelationalRecoveryRoute::Performed(performed) => {
                RelationalRecoveryRecordRoute::Performed {
                    performed,
                    commit_identity: commit_identity.clone(),
                    successor_basis: successor_basis.clone(),
                }
            }
            crate::publication::RelationalRecoveryRoute::SettlementPending(settlement) => {
                RelationalRecoveryRecordRoute::SettlementPending {
                    settlement,
                    commit_identity: commit_identity.clone(),
                    successor_basis: successor_basis.clone(),
                }
            }
            crate::publication::RelationalRecoveryRoute::IdentityRequired => {
                RelationalRecoveryRecordRoute::IdentityRequired {
                    commit_identity: commit_identity.clone(),
                    successor_basis: successor_basis.clone(),
                }
            }
        };
        Ok(RelationalRecoveryRecordState {
            commit_identity,
            successor_basis,
            route: Some(route),
            component_results: Some(component_results),
            signal_posture,
        })
    }

    pub(crate) fn restore_relational_recovery(&mut self, mut state: RelationalRecoveryRecordState) {
        let component_results = state
            .component_results
            .take()
            .expect("recovery result projection is restored exactly once");
        self.progress = state.into_progress();
        self.component_results = component_results;
    }

    pub(crate) fn settle_relational_recovery(
        &mut self,
        mut state: RelationalRecoveryRecordState,
        result: worth_relational::facade::transactions::CommitResult,
    ) {
        self.progress = state.settled_progress(result.clone());
        self.component_results = state
            .component_results
            .take()
            .expect("recovery result projection settles exactly once")
            .with_relational_settled(
                state.commit_identity.clone(),
                state.successor_basis.clone(),
                result,
            );
        state.route = None;
        self.cause = ProductUnpublishedCause::OwnerSettlementComplete;
        self.next_actions = super::RetainedNextActions::from_vec(vec![
            ProductUnpublishedNextAction::ReleaseObligations,
            ProductUnpublishedNextAction::Inspect,
        ]);
    }

    pub(crate) fn retain_pending_relational_settlement(
        &mut self,
        mut state: RelationalRecoveryRecordState,
        settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
    ) {
        self.progress = state.pending_progress(settlement.clone());
        self.component_results = state
            .component_results
            .take()
            .expect("recovery result projection remains while settlement is pending")
            .with_relational_settlement_pending(
                state.commit_identity.clone(),
                state.successor_basis.clone(),
                settlement,
            );
        state.route = None;
        self.cause = ProductUnpublishedCause::SettlementPending;
        self.next_actions = super::RetainedNextActions::from_vec(vec![
            ProductUnpublishedNextAction::SettleOwnerEffects,
            ProductUnpublishedNextAction::ReleaseObligations,
            ProductUnpublishedNextAction::Inspect,
        ]);
    }

    pub(crate) fn retain_identity_repair(&mut self, mut state: RelationalRecoveryRecordState) {
        self.progress = state.identity_required_progress();
        self.component_results = state
            .component_results
            .take()
            .expect("recovery result projection remains while identity repair is required");
        state.route = None;
        self.cause = ProductUnpublishedCause::SettlementPending;
        self.next_actions = super::RetainedNextActions::from_vec(vec![
            ProductUnpublishedNextAction::SettleOwnerEffects,
            ProductUnpublishedNextAction::ReleaseObligations,
            ProductUnpublishedNextAction::Inspect,
        ]);
    }

    pub(crate) fn settle_relational_recovery_with_receipt(
        &mut self,
        mut state: RelationalRecoveryRecordState,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) {
        self.progress = state.settled_receipt_progress(receipt.clone());
        self.component_results = state
            .component_results
            .take()
            .expect("recovery result projection settles exactly once")
            .with_relational_settled_receipt(
                state.commit_identity.clone(),
                state.successor_basis.clone(),
                receipt,
            );
        state.route = None;
        self.cause = ProductUnpublishedCause::OwnerSettlementComplete;
        self.next_actions = super::RetainedNextActions::from_vec(vec![
            ProductUnpublishedNextAction::ReleaseObligations,
            ProductUnpublishedNextAction::Inspect,
        ]);
    }
}

pub(crate) struct RelationalRecoveryRecordState {
    commit_identity: worth_relational::facade::history::RelationalCommitIdentity,
    successor_basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    route: Option<RelationalRecoveryRecordRoute>,
    component_results: Option<CompositeOwnerExecutionResults>,
    signal_posture: crate::publication::SignalAttemptProgressPosture,
}

enum RelationalRecoveryRecordRoute {
    Performed {
        commit_identity: worth_relational::facade::history::RelationalCommitIdentity,
        successor_basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
        performed: worth_relational::facade::mvcc::PerformedRelationalCommit,
    },
    SettlementPending {
        commit_identity: worth_relational::facade::history::RelationalCommitIdentity,
        successor_basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
        settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
    },
    IdentityRequired {
        commit_identity: worth_relational::facade::history::RelationalCommitIdentity,
        successor_basis: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    },
}

impl RelationalRecoveryRecordRoute {
    fn settlement(
        &self,
    ) -> Option<&worth_relational::facade::publication::DeferredPublicationSettlement> {
        match self {
            Self::Performed { .. } => None,
            Self::SettlementPending { settlement, .. } => Some(settlement),
            Self::IdentityRequired { .. } => None,
        }
    }
}

impl RelationalRecoveryRecordState {
    pub(crate) fn commit_identity(
        &self,
    ) -> &worth_relational::facade::history::RelationalCommitIdentity {
        &self.commit_identity
    }

    pub(crate) fn settlement(
        &self,
    ) -> Option<&worth_relational::facade::publication::DeferredPublicationSettlement> {
        self.route
            .as_ref()
            .and_then(RelationalRecoveryRecordRoute::settlement)
    }

    pub(crate) fn take_performed(
        &mut self,
    ) -> Option<worth_relational::facade::mvcc::PerformedRelationalCommit> {
        let route = self.route.take()?;
        match route {
            RelationalRecoveryRecordRoute::Performed { performed, .. } => Some(performed),
            route => {
                self.route = Some(route);
                None
            }
        }
    }

    pub(crate) fn take_identity_repair(
        &mut self,
    ) -> Option<worth_relational::facade::history::RelationalCommitIdentity> {
        let route = self.route.take()?;
        match route {
            RelationalRecoveryRecordRoute::IdentityRequired {
                commit_identity, ..
            } => Some(commit_identity),
            route => {
                self.route = Some(route);
                None
            }
        }
    }

    pub(crate) fn restore_identity_repair(&mut self) {
        assert!(self.route.is_none(), "identity repair is restored once");
        self.route = Some(RelationalRecoveryRecordRoute::IdentityRequired {
            commit_identity: self.commit_identity.clone(),
            successor_basis: self.successor_basis.clone(),
        });
    }

    fn into_progress(mut self) -> CompositeAttemptProgress {
        let signal_posture = self.signal_posture;
        match self
            .route
            .take()
            .expect("a recoverable route is restored exactly once")
        {
            RelationalRecoveryRecordRoute::Performed { performed, .. } => {
                CompositeAttemptProgress::new(
                    crate::publication::RelationalAttemptProgress::performed(performed),
                    crate::publication::SignalAttemptProgress::summary(signal_posture),
                )
            }
            RelationalRecoveryRecordRoute::SettlementPending {
                commit_identity,
                successor_basis,
                settlement,
            } => CompositeAttemptProgress::new(
                crate::publication::RelationalAttemptProgress::settlement_pending(
                    commit_identity,
                    successor_basis,
                    settlement,
                ),
                crate::publication::SignalAttemptProgress::summary(signal_posture),
            ),
            RelationalRecoveryRecordRoute::IdentityRequired {
                commit_identity,
                successor_basis,
            } => CompositeAttemptProgress::new(
                crate::publication::RelationalAttemptProgress::settlement_required(
                    commit_identity,
                    successor_basis,
                ),
                crate::publication::SignalAttemptProgress::summary(signal_posture),
            ),
        }
    }

    fn settled_progress(
        &self,
        result: worth_relational::facade::transactions::CommitResult,
    ) -> CompositeAttemptProgress {
        CompositeAttemptProgress::new(
            crate::publication::RelationalAttemptProgress::settled(
                self.commit_identity.clone(),
                self.successor_basis.clone(),
                result,
            ),
            crate::publication::SignalAttemptProgress::summary(self.signal_posture),
        )
    }

    fn pending_progress(
        &self,
        settlement: worth_relational::facade::publication::DeferredPublicationSettlement,
    ) -> CompositeAttemptProgress {
        CompositeAttemptProgress::new(
            crate::publication::RelationalAttemptProgress::settlement_pending(
                self.commit_identity.clone(),
                self.successor_basis.clone(),
                settlement,
            ),
            crate::publication::SignalAttemptProgress::summary(self.signal_posture),
        )
    }

    fn identity_required_progress(&self) -> CompositeAttemptProgress {
        CompositeAttemptProgress::new(
            crate::publication::RelationalAttemptProgress::settlement_required(
                self.commit_identity.clone(),
                self.successor_basis.clone(),
            ),
            crate::publication::SignalAttemptProgress::summary(self.signal_posture),
        )
    }

    fn settled_receipt_progress(
        &self,
        receipt: worth_relational::facade::history::RelationalCommitReceipt,
    ) -> CompositeAttemptProgress {
        CompositeAttemptProgress::new(
            crate::publication::RelationalAttemptProgress::settled_receipt(
                self.commit_identity.clone(),
                self.successor_basis.clone(),
                receipt,
            ),
            crate::publication::SignalAttemptProgress::summary(self.signal_posture),
        )
    }
}
