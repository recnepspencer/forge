use crate::{SubscriptionSupportAccessStructureReport, SubscriptionSupportCatalog};

use super::super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn subscription_support_counters(&self) -> crate::SubscriptionSupportCounterSnapshot {
        self.state.subscription_support_counter_snapshot.clone()
    }

    pub fn subscription_support_access_structure_report(
        &self,
    ) -> SubscriptionSupportAccessStructureReport {
        let required = SubscriptionSupportCatalog::first_ship()
            .access_structures()
            .required()
            .to_vec();
        if self.state.subscription_support_access_structures_verified {
            SubscriptionSupportCatalog::first_ship().access_structures()
        } else {
            let debted = if self
                .state
                .subscription_support_access_structure_debts
                .is_empty()
            {
                required
            } else {
                self.state
                    .subscription_support_access_structure_debts
                    .clone()
            };
            SubscriptionSupportAccessStructureReport::debt_for(debted)
        }
    }
}
