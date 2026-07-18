pub(in crate::runtime::allocation_frame_dispatch::framework_turn) fn execute<'runtime>(
    ledger: &'runtime crate::runtime::allocation_receipt::UiAllocationReceiptLedger,
    invalidation_authority: &'runtime std::cell::RefCell<
        crate::runtime::invalidation_narrowing::UiAllocationInvalidationAuthority,
    >,
    execution: super::super::transition_planning::UiDurableResizeExecutionPlan,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    let super::super::transition_planning::UiDurableResizeExecutionPlan {
        plan,
        selection,
        transaction,
        extent,
        previous_extent,
        requested_mutation,
    } = execution;
    let completion_basis = UiDurableResizeCompletionBasis {
        plan,
        selection,
        extent,
        previous_extent,
        requested_mutation,
    };
    let transaction = super::super::allocation_transaction::publish_pending(
        ledger,
        &mut invalidation_authority.borrow_mut(),
        transaction,
    );
    let durable_state = ledger.durable_semantic_state();
    match transaction {
        crate::runtime::UiAllocationReplanTransactionOutcome::Committed(committed) => {
            committed_completion(completion_basis, committed, durable_state, counters, false)
        }
        crate::runtime::UiAllocationReplanTransactionOutcome::Replayed(committed) => {
            committed_completion(completion_basis, committed, durable_state, counters, true)
        }
        crate::runtime::UiAllocationReplanTransactionOutcome::Denied(_) => {
            super::super::WorthUiFrameworkTurnCompletion::AllocationInvalidationsNarrowed {
                plan: completion_basis.plan,
                selection: completion_basis.selection,
                transaction,
                planning_counters: counters,
            }
        }
    }
}

struct UiDurableResizeCompletionBasis {
    plan: crate::runtime::UiNarrowedAllocationFramePlan,
    selection: crate::graph::UiAdmittedReplanNeighborhoodSet,
    extent: crate::runtime::UiResizeLogicalExtent,
    previous_extent: Option<crate::runtime::UiResizeLogicalExtent>,
    requested_mutation: bool,
}

fn committed_completion<'runtime>(
    basis: UiDurableResizeCompletionBasis,
    committed: crate::runtime::UiCommittedAllocationReplan,
    durable_state: Option<crate::runtime::UiAllocationDurableSemanticState>,
    counters: super::super::UiFrameworkTransitionPlanningCounters,
    replayed: bool,
) -> super::super::WorthUiFrameworkTurnCompletion<'runtime> {
    let Some(durable_state) = durable_state else {
        return super::super::WorthUiFrameworkTurnCompletion::FrameworkTransitionExecutionDenied {
            denial: super::super::UiFrameworkTransitionExecutionDenial::DurableSemanticStateMissing,
        };
    };
    super::super::WorthUiFrameworkTurnCompletion::DurableResizeCommitted {
        outcome: crate::runtime::UiDurableResizeCommitOutcome::new(
            basis.extent,
            committed,
            durable_state,
            !replayed && basis.requested_mutation && basis.previous_extent != Some(basis.extent),
            replayed,
        ),
        selection: basis.selection,
        planning_counters: counters,
    }
}
