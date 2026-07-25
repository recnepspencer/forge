use super::WorthQuerySemanticAspectDependencySource as Source;

pub(super) fn dependency_source_semantics_eq(subject: &Source, candidate: &Source) -> bool {
    installed_semantics_eq(subject, candidate)
        .or_else(|| realized_observation_semantics_eq(subject, candidate))
        .or_else(|| realized_workflow_semantics_eq(subject, candidate))
        .unwrap_or(false)
}

fn installed_semantics_eq(subject: &Source, candidate: &Source) -> Option<bool> {
    Some(match (subject, candidate) {
        (
            Source::InstalledOperationIdentity {
                canonical_identity: left,
                ..
            },
            Source::InstalledOperationIdentity {
                canonical_identity: right,
                ..
            },
        ) => left == right,
        (Source::NativeProjection(left), Source::NativeProjection(right)) => left == right,
        (Source::CollectionField(left), Source::CollectionField(right)) => left == right,
        (Source::CollectionWindowPolicy(left), Source::CollectionWindowPolicy(right)) => {
            left == right
        }
        (Source::ResultShape(left), Source::ResultShape(right)) => left == right,
        (Source::TouchGraphRole(left), Source::TouchGraphRole(right)) => left == right,
        (Source::TouchScope(left), Source::TouchScope(right)) => left == right,
        (Source::EffectFamily(left), Source::EffectFamily(right)) => left == right,
        (Source::InstalledInvariant(left), Source::InstalledInvariant(right)) => left == right,
        (Source::ReplayContract(left), Source::ReplayContract(right)) => left == right,
        (Source::LineageContract(left), Source::LineageContract(right)) => left == right,
        (Source::SupportContract(left), Source::SupportContract(right)) => left == right,
        (
            Source::WorkflowStageRead {
                graph_read_role: left,
            },
            Source::WorkflowStageRead {
                graph_read_role: right,
            },
        ) => left == right,
        (
            Source::WorkflowStage { predecessors: left },
            Source::WorkflowStage {
                predecessors: right,
            },
        ) => left == right,
        (Source::ConditionalNodeContract(left), Source::ConditionalNodeContract(right)) => {
            left == right
        }
        (Source::ConditionalTruth(left), Source::ConditionalTruth(right)) => left == right,
        _ if is_realized(subject) && is_realized(candidate) => return None,
        _ => false,
    })
}

fn realized_observation_semantics_eq(subject: &Source, candidate: &Source) -> Option<bool> {
    Some(match (subject, candidate) {
        (
            Source::RealizedGraphCall {
                role: left_role,
                call_kind: left_kind,
                projection_result_digest: left_projection,
                commit_graph_roles: left_commit_roles,
                ..
            },
            Source::RealizedGraphCall {
                role: right_role,
                call_kind: right_kind,
                projection_result_digest: right_projection,
                commit_graph_roles: right_commit_roles,
                ..
            },
        ) => {
            left_role == right_role
                && left_kind == right_kind
                && left_projection == right_projection
                && left_commit_roles == right_commit_roles
        }
        (
            Source::RealizedConditionalOutcome {
                class: left_class,
                observations: left_observations,
                ..
            },
            Source::RealizedConditionalOutcome {
                class: right_class,
                observations: right_observations,
                ..
            },
        ) => left_class == right_class && left_observations == right_observations,
        (
            Source::RealizedDirectOutput {
                result_state: left_state,
                output_identity: left_output,
                ..
            },
            Source::RealizedDirectOutput {
                result_state: right_state,
                output_identity: right_output,
                ..
            },
        ) => left_state == right_state && left_output == right_output,
        _ if is_realized_workflow(source_kind(subject))
            && is_realized_workflow(source_kind(candidate)) =>
        {
            return None
        }
        _ if is_realized(subject) && is_realized(candidate) => false,
        _ => return None,
    })
}

fn realized_workflow_semantics_eq(subject: &Source, candidate: &Source) -> Option<bool> {
    Some(match (subject, candidate) {
        (
            Source::RealizedWorkflowOutput {
                semantic_output: left_output,
                result_state: left_state,
                ..
            },
            Source::RealizedWorkflowOutput {
                semantic_output: right_output,
                result_state: right_state,
                ..
            },
        ) => left_output == right_output && left_state == right_state,
        (Source::RealizedWorkflowRead(left), Source::RealizedWorkflowRead(right)) => {
            left.semantic_replay_eq(right)
        }
        (Source::RealizedWorkflowEffect(left), Source::RealizedWorkflowEffect(right)) => {
            left.semantic_replay_eq(right)
        }
        (Source::RealizedWorkflowInvariant(left), Source::RealizedWorkflowInvariant(right)) => {
            left.invariant_role() == right.invariant_role()
                && left.installed_invariant_identity() == right.installed_invariant_identity()
        }
        (Source::RealizedWorkflowLineage(left), Source::RealizedWorkflowLineage(right)) => {
            left.semantic_replay_eq(right)
        }
        _ if is_realized(subject) && is_realized(candidate) => false,
        _ => return None,
    })
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum RealizedSourceKind {
    Observation,
    Workflow,
    Installed,
}

fn source_kind(source: &Source) -> RealizedSourceKind {
    match source {
        Source::RealizedGraphCall { .. }
        | Source::RealizedConditionalOutcome { .. }
        | Source::RealizedDirectOutput { .. } => RealizedSourceKind::Observation,
        Source::RealizedWorkflowRead(_)
        | Source::RealizedWorkflowEffect(_)
        | Source::RealizedWorkflowInvariant(_)
        | Source::RealizedWorkflowLineage(_)
        | Source::RealizedWorkflowOutput { .. } => RealizedSourceKind::Workflow,
        _ => RealizedSourceKind::Installed,
    }
}

fn is_realized_workflow(kind: RealizedSourceKind) -> bool {
    kind == RealizedSourceKind::Workflow
}

fn is_realized(source: &Source) -> bool {
    matches!(
        source,
        Source::RealizedGraphCall { .. }
            | Source::RealizedWorkflowRead(_)
            | Source::RealizedConditionalOutcome { .. }
            | Source::RealizedDirectOutput { .. }
            | Source::RealizedWorkflowEffect(_)
            | Source::RealizedWorkflowInvariant(_)
            | Source::RealizedWorkflowLineage(_)
            | Source::RealizedWorkflowOutput { .. }
    )
}
