use crate::branch::{ProductBranchCreationIntent, RuntimeWorldBootstrapIntent};
use crate::budget::RuntimeWorldBudgets;
use crate::lifecycle::{RuntimeWorldClock, RuntimeWorldOwnerInputs};
use worth_relational::facade::mvcc::{
    PerformedRelationalCommit, RelationalPublicationOutcome, RelationalTransactionIntent,
};
use worth_relational::facade::transactions::WorkerIntentBatch;

use super::RealReferenceFixture;

impl RealReferenceFixture {
    pub(crate) fn retention_owner(
        &self,
    ) -> &crate::retention::RuntimeWorldRetentionOwner<(), (), ()> {
        &self.owner
    }

    pub(crate) fn owner_identity(&self) -> crate::identity::RuntimeWorldOwnerIdentity {
        self.owner_identity
    }

    pub(crate) fn next_publication_attempt(
        &mut self,
    ) -> crate::identity::CompositePublicationAttemptIdentity {
        self.identities
            .issuer_mut()
            .publication_attempt()
            .expect("publication attempt identity")
    }

    pub(crate) fn next_product_unpublished(
        &mut self,
    ) -> crate::identity::ProductUnpublishedOwnerEffectsIdentity {
        self.identities
            .issuer_mut()
            .product_unpublished()
            .expect("product-unpublished identity")
    }

    pub(crate) fn perform_relational_owner_change(&self) -> PerformedRelationalCommit {
        let identity = self._relational_runtime.main_branch_identity();
        let (_, basis) = self
            ._relational_runtime
            .observe_branch(&identity)
            .expect("real Relational owner observes its current basis");
        let mut transaction = self
            ._relational_runtime
            .begin_branch_transaction(&basis, RelationalTransactionIntent::ordinary())
            .expect("current owner basis admits a transaction");
        transaction
            .push_batch(WorkerIntentBatch::new(
                "runtime-world-cas-loss-owner-effect",
            ))
            .expect("bounded empty batch stages through the production transaction path");
        let candidate = self
            ._relational_runtime
            .prepare_branch_transaction(transaction)
            .expect("production Relational owner prepares the change");
        match self
            ._relational_runtime
            .publication_port()
            .compare_and_publish(candidate)
        {
            RelationalPublicationOutcome::Performed(performed) => performed,
            outcome => {
                panic!("production Relational owner must perform the test change: {outcome:?}")
            }
        }
    }

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
