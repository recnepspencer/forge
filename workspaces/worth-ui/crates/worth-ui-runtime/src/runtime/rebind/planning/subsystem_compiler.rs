use std::collections::BTreeMap;

use crate::declaration::UiAspectFamily;
use crate::graph::UiGraphFactConsumerKind;
use crate::runtime::rebind::UiIdentityLifecycleDecision;

use super::{UiRebindPlanTarget, UiRebindSubsystemKind, UiRebindSubsystemPlan};

pub(super) fn compile_subsystems(
    scope: &super::super::UiResolvedAffectedScope,
    decisions: &[super::super::UiIdentityLifecycleEntry],
    binding_targets: Vec<UiRebindPlanTarget>,
) -> Box<[UiRebindSubsystemPlan]> {
    let mut targets = BTreeMap::<UiRebindSubsystemKind, Vec<UiRebindPlanTarget>>::new();
    for decision in decisions {
        record_identity_subsystems(scope, decision, &mut targets);
    }
    targets.insert(UiRebindSubsystemKind::Binding, binding_targets);
    UiRebindSubsystemKind::all()
        .into_iter()
        .map(|kind| UiRebindSubsystemPlan::new(kind, targets.remove(&kind).unwrap_or_default()))
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn record_identity_subsystems(
    scope: &super::super::UiResolvedAffectedScope,
    entry: &super::super::UiIdentityLifecycleEntry,
    targets: &mut BTreeMap<UiRebindSubsystemKind, Vec<UiRebindPlanTarget>>,
) {
    let target = UiRebindPlanTarget::Consumer(entry.key().clone());
    let structural = structural_effect(entry.decision());
    if structural {
        match entry.key().kind() {
            UiGraphFactConsumerKind::GraphNode => {
                push(targets, UiRebindSubsystemKind::Graph, target.clone())
            }
            UiGraphFactConsumerKind::MountEligibilitySlot => {
                push(targets, UiRebindSubsystemKind::Mount, target.clone())
            }
        }
    }
    if entry.decision() == UiIdentityLifecycleDecision::Preserve {
        push(targets, UiRebindSubsystemKind::Preservation, target.clone());
    }
    if entry.decision() == UiIdentityLifecycleDecision::Retire {
        push(targets, UiRebindSubsystemKind::Retirement, target.clone());
    }
    let families = affected_families(scope, entry);
    if structural || families.iter().any(|family| allocation_family(*family)) {
        push(targets, UiRebindSubsystemKind::Allocation, target.clone());
    }
    if families.contains(&UiAspectFamily::Layout) {
        push(targets, UiRebindSubsystemKind::Measurement, target.clone());
    }
    if entry.key().kind() == UiGraphFactConsumerKind::MountEligibilitySlot
        || families
            .iter()
            .any(|family| matches!(family, UiAspectFamily::Appearance | UiAspectFamily::Content))
    {
        push(targets, UiRebindSubsystemKind::Surface, target.clone());
    }
    if families.iter().any(|family| {
        matches!(
            family,
            UiAspectFamily::Interaction | UiAspectFamily::Service
        )
    }) {
        push(targets, UiRebindSubsystemKind::Obligation, target);
    }
}

fn affected_families(
    scope: &super::super::UiResolvedAffectedScope,
    entry: &super::super::UiIdentityLifecycleEntry,
) -> Vec<UiAspectFamily> {
    scope
        .consumers()
        .binary_search_by(|consumer| consumer.key().cmp(entry.key()))
        .ok()
        .map(|index| scope.consumers()[index].affected_aspects())
        .unwrap_or(&[])
        .iter()
        .map(|aspect| aspect.family())
        .collect()
}

fn structural_effect(decision: UiIdentityLifecycleDecision) -> bool {
    matches!(
        decision,
        UiIdentityLifecycleDecision::Create
            | UiIdentityLifecycleDecision::Retire
            | UiIdentityLifecycleDecision::Rebind
            | UiIdentityLifecycleDecision::Move
            | UiIdentityLifecycleDecision::Remount
    )
}

fn allocation_family(family: UiAspectFamily) -> bool {
    matches!(
        family,
        UiAspectFamily::Structure
            | UiAspectFamily::Presence
            | UiAspectFamily::Participation
            | UiAspectFamily::Layout
    )
}

fn push(
    targets: &mut BTreeMap<UiRebindSubsystemKind, Vec<UiRebindPlanTarget>>,
    kind: UiRebindSubsystemKind,
    target: UiRebindPlanTarget,
) {
    targets.entry(kind).or_default().push(target);
}
