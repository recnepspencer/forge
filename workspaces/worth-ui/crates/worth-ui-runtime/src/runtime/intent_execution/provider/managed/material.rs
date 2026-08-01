use crate::capability::{UiIntentProductOutcome, UiIntentSchema};

use super::{UiManagedIntentOutcomeMaterial, UiManagedIntentPartialEffect};
use crate::runtime::intent_execution::{UiIntentPartialEffect, UiIntentProviderStop};

struct UiTypedManagedIntentOutcome<O: UiIntentProductOutcome> {
    outcome: O,
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

pub(super) fn partial_effect<O: UiIntentProductOutcome>(
    effect: UiIntentPartialEffect<O>,
) -> UiManagedIntentPartialEffect {
    let detail: UiIntentProviderStop = effect.detail();
    UiManagedIntentPartialEffect::new(effect.into_outcome().map(outcome_material), detail)
}
