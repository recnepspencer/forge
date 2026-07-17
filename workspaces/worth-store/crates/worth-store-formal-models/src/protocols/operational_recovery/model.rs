use std::collections::{BTreeMap, BTreeSet};

use super::{OperationalRecoveryAction, OperationalRecoveryActionKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalRecoveryControlledDefect {
    ExecutionWithoutAuthorization,
    OwnerReceiptWithoutEffectStart,
    PublicationWithoutPreparation,
    PromotionWithoutExternalFence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalRecoveryInvariant {
    UniqueTransitionIdentity,
    AuthorizationBeforeExecution,
    OwnerEffectBeforeReceipt,
    PreparationBeforePublication,
    PendingBeforeDisposition,
    DispositionBeforeFenceRelease,
    ExternalFenceBeforePromotion,
    BootstrapTransferBeforeCompletion,
    PromotionBeforePublication,
    PromotionPublicationBeforeReadmission,
    PromotionReadmissionBeforeRejoin,
    RejoinPlanBeforeCompletion,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalRecoveryCounterexample {
    operation_identity: String,
    transition_identity: String,
    invariant: OperationalRecoveryInvariant,
}

impl OperationalRecoveryCounterexample {
    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }
    pub fn transition_identity(&self) -> &str {
        &self.transition_identity
    }
    pub const fn invariant(&self) -> OperationalRecoveryInvariant {
        self.invariant
    }
}

#[derive(Debug, Default)]
struct OperationState {
    transitions: BTreeSet<String>,
    authorized: bool,
    owner_execution_opened: bool,
    owner_effect_started: bool,
    publication_prepared: bool,
    publication_pending: bool,
    publication_disposition: bool,
    promotion_fenced: bool,
    bootstrap_transferred: bool,
    promotion_recorded: bool,
    promotion_published: bool,
    promotion_readmitted: bool,
    rejoin_planned: bool,
}

#[derive(Debug, Default)]
pub struct OperationalRecoveryModel {
    operations: BTreeMap<String, OperationState>,
    reached: BTreeSet<OperationalRecoveryActionKind>,
}

impl OperationalRecoveryModel {
    pub fn apply(
        &mut self,
        action: &OperationalRecoveryAction,
        controlled_defect: Option<OperationalRecoveryControlledDefect>,
    ) -> Result<(), OperationalRecoveryCounterexample> {
        let state = self
            .operations
            .entry(action.operation_identity().to_owned())
            .or_default();
        if !state
            .transitions
            .insert(action.transition_identity().to_owned())
        {
            return Err(counterexample(
                action,
                OperationalRecoveryInvariant::UniqueTransitionIdentity,
            ));
        }
        use OperationalRecoveryActionKind as Action;
        match action.kind() {
            Action::AuthorizationConsumed => state.authorized = true,
            Action::OwnerExecutionOpened => {
                require_authorized(action, state, controlled_defect)?;
                state.owner_execution_opened = true;
            }
            Action::OwnerEffectStarted => {
                require_authorized(action, state, controlled_defect)?;
                state.owner_effect_started = true;
            }
            Action::OwnerReceiptPersisted => {
                if state.owner_execution_opened
                    && !state.owner_effect_started
                    && controlled_defect
                        != Some(OperationalRecoveryControlledDefect::OwnerReceiptWithoutEffectStart)
                {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::OwnerEffectBeforeReceipt,
                    ));
                }
            }
            Action::StagingCompleted => {
                require_authorized(action, state, controlled_defect)?;
            }
            Action::ReplicaBootstrapTransferRecorded => {
                require_authorized(action, state, controlled_defect)?;
                state.bootstrap_transferred = true;
            }
            Action::ReplicaBootstrapCompleted => {
                if !state.bootstrap_transferred {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::BootstrapTransferBeforeCompletion,
                    ));
                }
            }
            Action::PublicationPrepared => state.publication_prepared = true,
            Action::PublicationPending => {
                if !state.publication_prepared
                    && controlled_defect
                        != Some(OperationalRecoveryControlledDefect::PublicationWithoutPreparation)
                {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::PreparationBeforePublication,
                    ));
                }
                state.publication_pending = true;
            }
            Action::PublicationDisposition => {
                if !state.publication_pending {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::PendingBeforeDisposition,
                    ));
                }
                state.publication_disposition = true;
            }
            Action::FenceReleased => {
                if !state.publication_disposition {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::DispositionBeforeFenceRelease,
                    ));
                }
            }
            Action::ReplicaPromotionFenceRecorded => {
                require_authorized(action, state, controlled_defect)?;
                state.promotion_fenced = true;
            }
            Action::ReplicaPromotionRecorded => {
                require_authorized(action, state, controlled_defect)?;
                if !state.promotion_fenced
                    && controlled_defect
                        != Some(OperationalRecoveryControlledDefect::PromotionWithoutExternalFence)
                {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::ExternalFenceBeforePromotion,
                    ));
                }
                state.promotion_recorded = true;
            }
            Action::ReplicaPromotionPublished => {
                if !state.promotion_recorded {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::PromotionBeforePublication,
                    ));
                }
                state.promotion_published = true;
            }
            Action::ReplicaPromotionReadmitted => {
                if !state.promotion_published {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::PromotionPublicationBeforeReadmission,
                    ));
                }
                state.promotion_readmitted = true;
            }
            Action::OldPrimaryRejoinPlanned => {
                if !state.promotion_readmitted {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::PromotionReadmissionBeforeRejoin,
                    ));
                }
                state.rejoin_planned = true;
            }
            Action::OldPrimaryRejoinCompleted => {
                if !state.rejoin_planned {
                    return Err(counterexample(
                        action,
                        OperationalRecoveryInvariant::RejoinPlanBeforeCompletion,
                    ));
                }
            }
            _ => {}
        }
        self.reached.insert(action.kind());
        Ok(())
    }

    pub fn reached_transitions(&self) -> &BTreeSet<OperationalRecoveryActionKind> {
        &self.reached
    }
}

fn require_authorized(
    action: &OperationalRecoveryAction,
    state: &OperationState,
    defect: Option<OperationalRecoveryControlledDefect>,
) -> Result<(), OperationalRecoveryCounterexample> {
    if !state.authorized
        && defect != Some(OperationalRecoveryControlledDefect::ExecutionWithoutAuthorization)
    {
        return Err(counterexample(
            action,
            OperationalRecoveryInvariant::AuthorizationBeforeExecution,
        ));
    }
    Ok(())
}

fn counterexample(
    action: &OperationalRecoveryAction,
    invariant: OperationalRecoveryInvariant,
) -> OperationalRecoveryCounterexample {
    OperationalRecoveryCounterexample {
        operation_identity: action.operation_identity().to_owned(),
        transition_identity: action.transition_identity().to_owned(),
        invariant,
    }
}
