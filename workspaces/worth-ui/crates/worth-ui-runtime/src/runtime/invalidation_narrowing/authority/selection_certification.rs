impl super::UiAllocationInvalidationAuthority {
    pub(crate) fn certifies_selection(
        &self,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> bool {
        selection
            .ordered_neighborhoods()
            .iter()
            .all(|selected| self.graph_replan.certifies(&selected.generation_key()))
    }

    pub(crate) fn certifies_completed_replay(
        &self,
        outcome: &crate::runtime::UiAllocationReplanTransactionOutcome,
    ) -> bool {
        let crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(committed) = outcome
        else {
            return false;
        };
        !committed.receipts().is_empty()
            && committed.receipts().iter().all(|receipt| {
                let allocation = receipt.committed_allocation();
                self.graph_replan
                    .target_set_for_neighborhood(
                        receipt.identity().graph_node_identity(),
                        allocation.allocation_neighborhood().identity(),
                    )
                    .is_some_and(|target| {
                        target.primary().allocation_plan().is_some_and(|plan| {
                            plan.planning_identity_digest()
                                == receipt.generation().planning_evidence_digest()
                        })
                    })
            })
    }
}
