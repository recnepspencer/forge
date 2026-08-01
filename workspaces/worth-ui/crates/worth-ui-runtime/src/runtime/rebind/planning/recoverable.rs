use super::{
    budget::{require_compiled_plan_budget, require_terminal_decision_budget},
    cost::compile_cost,
    currentness::require_scope_currentness,
    effect_compiler::{compile_conflicts, compile_effects, compile_parallel_admission},
    policy::require_policy_session,
    subsystem_compiler::compile_subsystems,
    UiRebindConflictFootprint, UiRebindExecutionPolicy, UiRebindParallelAdmission,
    UiRebindPlanningContext, UiRebindPlanningDenial,
};

pub(crate) struct UiRebindPlanningRecoveryStop {
    denial: UiRebindPlanningDenial,
    lifecycle: Box<crate::runtime::rebind::UiResolvedIdentityLifecycle>,
}

pub(super) struct PreparedNonSourcePlan {
    pub(super) subsystems: Box<[super::UiRebindSubsystemPlan]>,
    pub(super) effects: super::UiRebindEffectSet,
    pub(super) conflicts: UiRebindConflictFootprint,
    pub(super) parallel: UiRebindParallelAdmission,
    pub(super) content: crate::mounting::UiMountedSemanticContentInput,
    pub(super) cost: super::UiRebindPlanCost,
}

pub(super) fn prepare_non_source(
    context: &UiRebindPlanningContext<'_>,
    lifecycle: &crate::runtime::rebind::UiResolvedIdentityLifecycle,
    policy: UiRebindExecutionPolicy,
) -> Result<PreparedNonSourcePlan, UiRebindPlanningDenial> {
    if lifecycle.scope().source_succession().is_some() {
        return Err(UiRebindPlanningDenial::WrongSourceSuccessionPosture);
    }
    require_scope_currentness(context, lifecycle.scope())?;
    require_policy_session(context.session(), policy)?;
    let budget = context.budget();
    require_terminal_decision_budget(lifecycle.selected(), budget)?;
    let subsystems = compile_subsystems(lifecycle.scope(), lifecycle.selected(), Vec::new());
    require_compiled_plan_budget(lifecycle.scope(), &subsystems, budget)?;
    let effects = compile_effects(&subsystems);
    let conflicts = compile_conflicts(&subsystems, &effects);
    let parallel = compile_parallel_admission(&effects);
    let content = super::content_plan::compile_content_plan(
        context.predecessor(),
        context.predecessor(),
        lifecycle.scope(),
    )?;
    let cost = compile_cost(lifecycle.selected(), &subsystems, &effects);
    Ok(PreparedNonSourcePlan {
        subsystems,
        effects,
        conflicts,
        parallel,
        content,
        cost,
    })
}

impl UiRebindPlanningRecoveryStop {
    pub(super) fn new(
        denial: UiRebindPlanningDenial,
        lifecycle: crate::runtime::rebind::UiResolvedIdentityLifecycle,
    ) -> Self {
        Self {
            denial,
            lifecycle: Box::new(lifecycle),
        }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        UiRebindPlanningDenial,
        crate::runtime::rebind::UiResolvedIdentityLifecycle,
    ) {
        (self.denial, *self.lifecycle)
    }
}
