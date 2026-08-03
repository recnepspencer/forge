use std::collections::BTreeMap;

use super::{
    UiRebindConflictFootprint, UiRebindDeclarativeEffect, UiRebindEffectSet,
    UiRebindParallelAdmission, UiRebindPlanTarget, UiRebindResourceAccess, UiRebindSubsystemKind,
    UiRebindSubsystemPlan,
};

pub(super) fn compile_effects(subsystems: &[UiRebindSubsystemPlan]) -> UiRebindEffectSet {
    UiRebindEffectSet::new(
        subsystems
            .iter()
            .filter(|plan| plan.kind() != UiRebindSubsystemKind::Preservation)
            .flat_map(|plan| {
                plan.targets()
                    .iter()
                    .cloned()
                    .map(|target| UiRebindDeclarativeEffect::new(plan.kind(), target))
            })
            .collect(),
    )
}

pub(super) fn compile_conflicts(
    subsystems: &[UiRebindSubsystemPlan],
    effects: &UiRebindEffectSet,
) -> UiRebindConflictFootprint {
    let reads = subsystems
        .iter()
        .flat_map(|plan| {
            plan.targets()
                .iter()
                .cloned()
                .map(|target| UiRebindResourceAccess::new(plan.kind(), target))
        })
        .collect();
    let writes = effects
        .effects()
        .iter()
        .map(|effect| UiRebindResourceAccess::new(effect.subsystem(), effect.target().clone()))
        .collect();
    let invalidations = effects
        .effects()
        .iter()
        .filter(|effect| {
            matches!(
                effect.subsystem(),
                UiRebindSubsystemKind::Allocation
                    | UiRebindSubsystemKind::Binding
                    | UiRebindSubsystemKind::Surface
            )
        })
        .map(|effect| UiRebindResourceAccess::new(effect.subsystem(), effect.target().clone()))
        .collect();
    UiRebindConflictFootprint::new(reads, writes, invalidations)
}

pub(super) fn compile_parallel_admission(effects: &UiRebindEffectSet) -> UiRebindParallelAdmission {
    let mut target_counts = BTreeMap::<UiRebindPlanTarget, usize>::new();
    for effect in effects.effects() {
        *target_counts.entry(effect.target().clone()).or_default() += 1;
    }
    UiRebindParallelAdmission::new(
        effects
            .effects()
            .iter()
            .filter(|effect| target_counts.get(effect.target()) == Some(&1))
            .map(|effect| effect.subsystem())
            .collect(),
    )
}
