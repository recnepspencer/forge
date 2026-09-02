use crate::branch::{ProductBranchCreationIntent, RuntimeWorldBootstrapIntent};
use crate::budget::RuntimeWorldBudgets;
use crate::lifecycle::{RuntimeWorldClock, RuntimeWorldOwnerInputs};

use super::RealReferenceFixture;

impl RealReferenceFixture {
    pub(crate) fn owner_inputs(
        &mut self,
        budgets: RuntimeWorldBudgets,
        clock: RuntimeWorldClock,
    ) -> RuntimeWorldOwnerInputs<(), (), (), (), ()> {
        let relational = self._relational_runtime.owner_component_services();
        let signal = self
            ._signal_runtime
            .owner_component_services()
            .expect("real Signal owner issues its sealed services again");
        RuntimeWorldOwnerInputs::new(
            relational,
            signal,
            self._correspondence_port.clone(),
            budgets,
            clock,
        )
    }

    pub(crate) fn bootstrap_intent(&self) -> RuntimeWorldBootstrapIntent {
        RuntimeWorldBootstrapIntent::new(
            ProductBranchCreationIntent::named("root").expect("valid root branch name"),
            self.basis.relational_basis().clone(),
            self.basis.signal_basis().clone(),
            self.basis.correspondence_basis().clone(),
        )
    }
}
