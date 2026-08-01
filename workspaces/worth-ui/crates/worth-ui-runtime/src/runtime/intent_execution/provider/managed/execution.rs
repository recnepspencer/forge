use std::sync::Arc;

use crate::capability::UiIntent;

use super::material::{outcome_material, partial_effect};
use super::recovery::{protocol_recovery, provider_recovery};
use super::{UiManagedIntentExecution, UiManagedIntentExecutionPoll, UiManagedIntentSettlement};
use crate::runtime::intent_execution::{
    UiIntentExecutionAttempt, UiIntentExecutionCancellationContext, UiIntentExecutionPollContext,
    UiIntentExecutionProvider, UiIntentProviderPoll, UiIntentProviderSettlement,
    UiIntentProviderStop,
};

pub(in crate::runtime::intent_execution::provider) struct UiTypedManagedIntentExecution<I, Provider>
where
    I: UiIntent,
    Provider: UiIntentExecutionProvider<I>,
{
    attempt: Box<dyn UiIntentExecutionAttempt<I>>,
    provider: Arc<Provider>,
    effect_may_have_begun: bool,
}

impl<I, Provider> UiTypedManagedIntentExecution<I, Provider>
where
    I: UiIntent,
    Provider: UiIntentExecutionProvider<I>,
{
    pub(in crate::runtime::intent_execution::provider) fn new(
        attempt: Box<dyn UiIntentExecutionAttempt<I>>,
        provider: Arc<Provider>,
    ) -> Self {
        Self {
            attempt,
            provider,
            effect_may_have_begun: false,
        }
    }

    fn translate(self, poll: UiIntentProviderPoll<I>) -> UiManagedIntentExecutionPoll {
        match poll {
            UiIntentProviderPoll::PendingBeforeEffect => {
                UiManagedIntentExecutionPoll::PendingBeforeEffect(Box::new(self))
            }
            UiIntentProviderPoll::PendingEffectMayHaveBegun => {
                let mut pending = self;
                pending.effect_may_have_begun = true;
                UiManagedIntentExecutionPoll::PendingEffectMayHaveBegun(Box::new(pending))
            }
            UiIntentProviderPoll::Settled(settlement) => {
                UiManagedIntentExecutionPoll::Settled(self.settle(settlement))
            }
        }
    }

    fn settle(self, settlement: UiIntentProviderSettlement<I>) -> UiManagedIntentSettlement {
        if self.effect_may_have_begun && is_before_effect(&settlement) {
            return UiManagedIntentSettlement::Indeterminate {
                detail: Some(UiIntentProviderStop::stable(
                    "worth_ui.provider.pre_effect_after_effect_may_begin",
                )),
                recovery: protocol_recovery(self.attempt, self.provider),
            };
        }
        match settlement {
            UiIntentProviderSettlement::Completed(outcome) => {
                UiManagedIntentSettlement::Completed(outcome_material(outcome))
            }
            UiIntentProviderSettlement::RejectedBeforeEffect(detail) => {
                UiManagedIntentSettlement::RejectedBeforeEffect(detail)
            }
            UiIntentProviderSettlement::FailedBeforeEffect(detail) => {
                UiManagedIntentSettlement::FailedBeforeEffect(detail)
            }
            UiIntentProviderSettlement::CancelledBeforeEffect(detail) => {
                UiManagedIntentSettlement::CancelledBeforeEffect(detail)
            }
            UiIntentProviderSettlement::TimedOutBeforeEffect(detail) => {
                UiManagedIntentSettlement::TimedOutBeforeEffect(detail)
            }
            UiIntentProviderSettlement::Partial(effect, recovery) => {
                UiManagedIntentSettlement::Partial {
                    effect: partial_effect(effect),
                    recovery: provider_recovery(recovery, self.provider),
                }
            }
            UiIntentProviderSettlement::Indeterminate(recovery) => {
                UiManagedIntentSettlement::Indeterminate {
                    detail: None,
                    recovery: provider_recovery(recovery, self.provider),
                }
            }
        }
    }
}

impl<I, Provider> UiManagedIntentExecution for UiTypedManagedIntentExecution<I, Provider>
where
    I: UiIntent,
    Provider: UiIntentExecutionProvider<I>,
{
    fn poll(
        mut self: Box<Self>,
        context: UiIntentExecutionPollContext,
    ) -> UiManagedIntentExecutionPoll {
        let poll = self.attempt.poll(context);
        (*self).translate(poll)
    }

    fn cancel(
        mut self: Box<Self>,
        context: UiIntentExecutionCancellationContext,
    ) -> UiManagedIntentExecutionPoll {
        let poll = self.attempt.cancel(context);
        (*self).translate(poll)
    }
}

fn is_before_effect<I: UiIntent>(settlement: &UiIntentProviderSettlement<I>) -> bool {
    matches!(
        settlement,
        UiIntentProviderSettlement::RejectedBeforeEffect(_)
            | UiIntentProviderSettlement::FailedBeforeEffect(_)
            | UiIntentProviderSettlement::CancelledBeforeEffect(_)
            | UiIntentProviderSettlement::TimedOutBeforeEffect(_)
    )
}
