use std::collections::HashMap;

use super::{
    dependency_locus::WorthQuerySemanticAspectDependencyLocus, WorthQuerySemanticDependencyRole,
};

pub(super) fn dependency_stage_identity(
    locus: &WorthQuerySemanticAspectDependencyLocus,
) -> Option<&str> {
    use WorthQuerySemanticAspectDependencyLocus as Locus;
    match locus {
        Locus::WorkflowStage { stage_identity }
        | Locus::WorkflowStageRead { stage_identity, .. }
        | Locus::WorkflowGraphCall { stage_identity, .. }
        | Locus::WorkflowPrimaryRead { stage_identity, .. }
        | Locus::WorkflowEffect { stage_identity, .. }
        | Locus::WorkflowInvariant { stage_identity, .. }
        | Locus::WorkflowLineage { stage_identity, .. }
        | Locus::WorkflowOutput { stage_identity } => Some(stage_identity),
        _ => None,
    }
}

pub(super) fn propagate_stage_consequences(
    predecessors: &HashMap<String, Vec<String>>,
    masks: &mut HashMap<String, u16>,
) -> usize {
    let mut inspected_edges = 0;
    let mut indegree = predecessors
        .iter()
        .map(|(stage, parents)| (stage.clone(), parents.len()))
        .collect::<HashMap<_, _>>();
    let mut successors = HashMap::<String, Vec<String>>::new();
    for (stage, parents) in predecessors {
        for parent in parents {
            inspected_edges += 1;
            successors
                .entry(parent.clone())
                .or_default()
                .push(stage.clone());
        }
    }
    let mut ready = std::collections::VecDeque::from(
        indegree
            .iter()
            .filter_map(|(stage, degree)| (*degree == 0).then_some(stage.clone()))
            .collect::<Vec<_>>(),
    );
    let mut order = Vec::with_capacity(predecessors.len());
    while let Some(stage) = ready.pop_front() {
        order.push(stage.clone());
        for successor in successors.get(&stage).into_iter().flatten() {
            inspected_edges += 1;
            let degree = indegree
                .get_mut(successor)
                .expect("installed workflow successor is indexed");
            *degree -= 1;
            if *degree == 0 {
                ready.push_back(successor.clone());
            }
        }
    }
    for stage in order.into_iter().rev() {
        let successor_mask = successors
            .get(&stage)
            .into_iter()
            .flatten()
            .filter_map(|successor| {
                inspected_edges += 1;
                masks.get(successor)
            })
            .copied()
            .fold(0, |left, right| left | right);
        *masks.entry(stage).or_default() |= successor_mask;
    }
    inspected_edges
}

pub(super) fn conditional_output_mask(
    declaration: &worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
) -> u16 {
    use worth_query_installation::facade::{
        WorthQueryConditionalConsequenceRole as Consequence,
        WorthQueryConditionalNodeOutput as Output,
    };
    let mut mask =
        role_bit(WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness);
    for output in declaration.outputs() {
        match output {
            Output::OperationOutput { .. } | Output::WorkflowStageOutput { .. } => {
                mask |= role_bit(WorthQuerySemanticDependencyRole::ProjectedValue);
            }
            Output::DerivedAspect { consequences, .. } => {
                mask |= role_bit(WorthQuerySemanticDependencyRole::ProjectedValue);
                for consequence in consequences {
                    match consequence {
                        Consequence::DerivedOnly => {}
                        Consequence::Touch(_) => {
                            mask |=
                                role_bit(WorthQuerySemanticDependencyRole::SelectionOrMembership)
                                    | role_bit(
                                        WorthQuerySemanticDependencyRole::SupportAndLifecycle,
                                    );
                        }
                        Consequence::Effect(_) => {
                            mask |= role_bit(WorthQuerySemanticDependencyRole::SupportAndLifecycle);
                        }
                    }
                }
            }
        }
    }
    mask
}

pub(super) fn role_bit(role: WorthQuerySemanticDependencyRole) -> u16 {
    1 << role.canonical_ordinal()
}

pub(super) fn roles_from_mask(mask: u16) -> Vec<WorthQuerySemanticDependencyRole> {
    use WorthQuerySemanticDependencyRole as Role;
    [
        Role::OperationalIdentity,
        Role::SelectionOrMembership,
        Role::Ordering,
        Role::ProjectedValue,
        Role::Grouping,
        Role::WindowBoundary,
        Role::SupportAndLifecycle,
        Role::ConditionalEligibilityOrSemanticCleanliness,
        Role::InstalledDomainInvariant,
        Role::AdvisoryOnlyContext,
    ]
    .into_iter()
    .filter(|role| mask & role_bit(*role) != 0)
    .collect()
}
