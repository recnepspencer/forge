use crate::capability::{UiIntentProductOutcome, UiIntentSchema};

use super::{UiManagedIntentOutcomeMaterial, UiManagedIntentPartialEffect};
use crate::runtime::intent_execution::{UiIntentPartialEffect, UiIntentProviderStop};

struct UiTypedManagedIntentOutcome<O: UiIntentProductOutcome> {
    outcome: O,
}

struct UiRuntimeServiceIntentOutcome<I: crate::capability::UiIntent> {
    destination: crate::capability::UiIntentRuntimeServiceDestination,
    intent: core::marker::PhantomData<fn() -> I>,
}

impl<O: UiIntentProductOutcome> UiManagedIntentOutcomeMaterial for UiTypedManagedIntentOutcome<O> {
    fn schema(&self) -> UiIntentSchema {
        let _ = &self.outcome;
        O::SCHEMA
    }

    fn into_consequences(self: Box<Self>) -> crate::capability::UiIntentProductConsequences {
        self.outcome.into_consequences()
    }
}

pub(in crate::runtime::intent_execution::provider) fn outcome_material<
    O: UiIntentProductOutcome,
>(
    outcome: O,
) -> Box<dyn UiManagedIntentOutcomeMaterial> {
    Box::new(UiTypedManagedIntentOutcome { outcome })
}

impl<I: crate::capability::UiIntent> UiManagedIntentOutcomeMaterial
    for UiRuntimeServiceIntentOutcome<I>
{
    fn schema(&self) -> UiIntentSchema {
        I::ProductOutcome::SCHEMA
    }

    fn runtime_service_destination(
        &self,
    ) -> Option<crate::capability::UiIntentRuntimeServiceDestination> {
        Some(self.destination)
    }

    fn into_consequences(self: Box<Self>) -> crate::capability::UiIntentProductConsequences {
        crate::capability::UiIntentProductConsequences::none()
    }
}

pub(in crate::runtime::intent_execution::provider) fn runtime_service_material<
    I: crate::capability::UiIntent,
>(
    destination: crate::capability::UiIntentRuntimeServiceDestination,
) -> Box<dyn UiManagedIntentOutcomeMaterial> {
    Box::new(UiRuntimeServiceIntentOutcome::<I> {
        destination,
        intent: core::marker::PhantomData,
    })
}

pub(super) fn partial_effect<O: UiIntentProductOutcome>(
    effect: UiIntentPartialEffect<O>,
) -> UiManagedIntentPartialEffect {
    let detail: UiIntentProviderStop = effect.detail();
    UiManagedIntentPartialEffect::new(effect.into_outcome().map(outcome_material), detail)
}
