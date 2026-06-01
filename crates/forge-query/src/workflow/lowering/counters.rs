use crate::workflow::WorkflowLoweringCounters;

pub(super) fn mutation_lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_mutation_lowering_count: 1,
        ..lowering_success_counters(width)
    }
}

pub(super) fn merge_lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_merge_lowering_count: 1,
        ..lowering_success_counters(width)
    }
}

pub(super) fn writeback_lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_writeback_declaration_count: 1,
        workflow_writeback_causality_binding_count: 1,
        ..lowering_success_counters(width)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum LoweringDenialClass {
    General,
    MergeDenied,
    WritebackDenied,
    StaleDenied,
    ExplicitRebind,
    WritebackExplicitRebind,
    AmbientBasisFallback,
}

pub(super) fn lowering_denial_counters(
    width: usize,
    denial_class: LoweringDenialClass,
) -> WorkflowLoweringCounters {
    let is_rebind = matches!(
        denial_class,
        LoweringDenialClass::ExplicitRebind | LoweringDenialClass::WritebackExplicitRebind
    );
    let is_stale_denial = matches!(denial_class, LoweringDenialClass::StaleDenied);
    let is_merge_denial = matches!(denial_class, LoweringDenialClass::MergeDenied);
    let is_writeback_denial = matches!(
        denial_class,
        LoweringDenialClass::WritebackDenied | LoweringDenialClass::WritebackExplicitRebind
    );
    let is_ambient_basis_fallback =
        matches!(denial_class, LoweringDenialClass::AmbientBasisFallback);
    let budget_cross = usize::from(is_rebind || is_ambient_basis_fallback || is_stale_denial);

    WorkflowLoweringCounters {
        workflow_declaration_count: 1,
        workflow_lowering_count: 1,
        workflow_mutation_lowering_count: 0,
        workflow_merge_lowering_count: 0,
        workflow_lowering_width: width,
        workflow_lowering_denial_count: 1,
        workflow_merge_denial_count: usize::from(is_merge_denial),
        workflow_writeback_declaration_count: 0,
        workflow_writeback_denial_count: usize::from(is_writeback_denial),
        workflow_writeback_causality_binding_count: 0,
        workflow_staleness_check_count: 1,
        workflow_stale_denial_count: usize::from(is_stale_denial),
        workflow_lowering_staleness_denial_count: usize::from(
            is_stale_denial || is_rebind || is_ambient_basis_fallback,
        ),
        workflow_explicit_rebind_required_count: usize::from(is_rebind),
        workflow_authority_override_denial_count: 0,
        workflow_ambient_basis_fallback_denial_count: usize::from(is_ambient_basis_fallback),
        workflow_replay_bundle_count: 0,
        workflow_budget_cross_count: budget_cross,
        workflow_work_avoided_by_query_lowering_count: width,
        workflow_executor_rediscovery_count: 0,
    }
}

fn lowering_success_counters(width: usize) -> WorkflowLoweringCounters {
    WorkflowLoweringCounters {
        workflow_declaration_count: 1,
        workflow_lowering_count: 1,
        workflow_mutation_lowering_count: 0,
        workflow_merge_lowering_count: 0,
        workflow_lowering_width: width,
        workflow_lowering_denial_count: 0,
        workflow_merge_denial_count: 0,
        workflow_writeback_declaration_count: 0,
        workflow_writeback_denial_count: 0,
        workflow_writeback_causality_binding_count: 0,
        workflow_staleness_check_count: 1,
        workflow_stale_denial_count: 0,
        workflow_lowering_staleness_denial_count: 0,
        workflow_explicit_rebind_required_count: 0,
        workflow_authority_override_denial_count: 0,
        workflow_ambient_basis_fallback_denial_count: 0,
        workflow_replay_bundle_count: 0,
        workflow_budget_cross_count: 0,
        workflow_work_avoided_by_query_lowering_count: width,
        workflow_executor_rediscovery_count: 0,
    }
}
