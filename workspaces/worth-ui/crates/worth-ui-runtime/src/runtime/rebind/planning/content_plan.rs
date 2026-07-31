use std::sync::Arc;

use crate::graph::UiGraphFactConsumerIdentity;

mod collection;
mod schema_transition;

pub(super) fn compile_content_plan(
    predecessor: &crate::facade::prepared_application_authority::
        WorthUiPreparedApplicationAuthority,
    candidate: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    scope: &super::super::UiResolvedAffectedScope,
) -> Result<crate::mounting::UiMountedSemanticContentInput, super::UiRebindPlanningDenial> {
    let mut content = crate::mounting::UiMountedSemanticContentInput::empty();
    content.set_projection_input_capacity(candidate.query_binding_plan().projection_input_count());
    retain_projection_inputs(candidate, scope, &mut content)?;
    let governed_nodes = schema_transition::compile(predecessor, candidate, scope, &mut content)?;
    for lookup in scope.lookups() {
        let Some(query) = scope
            .facts()
            .get(lookup.fact_ordinal())
            .and_then(crate::fact_contract::UiProducedFact::query)
        else {
            continue;
        };
        let projection = match (query.scalar_projection(), query.collection_projection()) {
            (Some(scalar), None) => UiProjectedSemanticContent::Scalar(project_scalar(scalar)),
            (None, Some(collection)) => {
                UiProjectedSemanticContent::Collection(collection::project_collection(collection)?)
            }
            (None, None) => continue,
            (Some(_), Some(_)) => unreachable!("a Query projection fact has one sealed shape"),
        };
        for entry in lookup.candidate().entries() {
            let UiGraphFactConsumerIdentity::GraphNode(graph_node) = entry.consumer() else {
                continue;
            };
            if governed_nodes.contains(&graph_node) {
                continue;
            }
            let inserted = match &projection {
                UiProjectedSemanticContent::Scalar((value, posture)) => {
                    content.insert_scalar(graph_node, value.clone(), Arc::clone(posture))
                }
                UiProjectedSemanticContent::Collection((value, posture)) => {
                    content.insert_collection(graph_node, value.clone(), Arc::clone(posture))
                }
            };
            if inserted.is_err() {
                return Err(super::UiRebindPlanningDenial::AmbiguousProjectionContent {
                    graph_node,
                });
            }
        }
    }
    Ok(content)
}

fn retain_projection_inputs(
    candidate: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    scope: &super::super::UiResolvedAffectedScope,
    content: &mut crate::mounting::UiMountedSemanticContentInput,
) -> Result<(), super::UiRebindPlanningDenial> {
    for fact in scope.facts() {
        let Some(query) = fact.query() else {
            continue;
        };
        let input = match (query.scalar_projection(), query.collection_projection()) {
            (Some(scalar), None) => {
                projection_input(candidate, scalar.core().projection_identity(), |slot| {
                    scalar.intent_input_reference(slot)
                })?
            }
            (None, Some(collection)) => {
                projection_input(candidate, collection.core().projection_identity(), |slot| {
                    collection.intent_input_reference(slot)
                })?
            }
            (None, None) => continue,
            (Some(_), Some(_)) => unreachable!("a Query projection fact has one sealed shape"),
        };
        let projection = input.revision().projection_identity().clone();
        content
            .insert_projection_input(input)
            .map_err(|()| super::UiRebindPlanningDenial::AmbiguousProjectionInput { projection })?;
    }
    Ok(())
}

fn projection_input(
    candidate: &crate::facade::prepared_application_authority::WorthUiPreparedApplicationAuthority,
    projection: &worth_ui_query_binding::WorthUiQueryViewIdentity,
    materialize: impl FnOnce(
        worth_ui_query_binding::UiProjectionInputSlot,
    ) -> worth_ui_query_binding::UiProjectionInputFactReference,
) -> Result<worth_ui_query_binding::UiProjectionInputFactReference, super::UiRebindPlanningDenial> {
    let slot = candidate
        .query_binding_plan()
        .projection_input_slot(projection)
        .ok_or_else(
            || super::UiRebindPlanningDenial::ProjectionInputSlotUnavailable {
                projection: projection.clone(),
            },
        )?;
    Ok(materialize(slot))
}

enum UiProjectedSemanticContent {
    Scalar(
        (
            crate::mounting::UiMountedSemanticTextValueDirective,
            Arc<str>,
        ),
    ),
    Collection((crate::mounting::UiMountedCollectionTextDirective, Arc<str>)),
}

fn project_scalar(
    fact: &worth_ui_query_binding::UiScalarProjectionFactReceipt,
) -> (
    crate::mounting::UiMountedSemanticTextValueDirective,
    Arc<str>,
) {
    use worth_ui_query_binding::{UiPresentProjection, UiProjectionAvailability};

    match fact.availability() {
        UiProjectionAvailability::Unavailable(receipt) => (
            crate::mounting::UiMountedSemanticTextValueDirective::Clear,
            unavailable_label(receipt.kind()),
        ),
        UiProjectionAvailability::Present(UiPresentProjection::Current(value)) => (
            crate::mounting::UiMountedSemanticTextValueDirective::Replace(Arc::from(
                value.as_str(),
            )),
            Arc::from("CURRENT"),
        ),
        UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            value,
            activity,
        }) => (
            crate::mounting::UiMountedSemanticTextValueDirective::Replace(Arc::from(
                value.as_str(),
            )),
            retained_label(activity.kind()),
        ),
        UiProjectionAvailability::Stopped(receipt) => (
            crate::mounting::UiMountedSemanticTextValueDirective::Preserve,
            stopped_label(receipt.kind()),
        ),
    }
}

fn unavailable_label(kind: worth_ui_query_binding::UiProjectionUnavailableKind) -> Arc<str> {
    use worth_ui_query_binding::UiProjectionUnavailableKind as Kind;
    Arc::from(match kind {
        Kind::Pending => "PENDING",
        Kind::Failed => "FAILED",
        Kind::Cancelled => "CANCELLED",
        Kind::Retried => "RETRIED",
        Kind::Superseded => "SUPERSEDED",
        Kind::Denied => "DENIED",
        Kind::Unsupported => "UNSUPPORTED",
        Kind::Remasked => "REMASKED",
        Kind::BasisDrift => "BASIS DRIFT",
        Kind::GenerationDrift => "GENERATION DRIFT",
    })
}

fn retained_label(kind: worth_ui_query_binding::UiProjectionRetainedActivityKind) -> Arc<str> {
    use worth_ui_query_binding::UiProjectionRetainedActivityKind as Kind;
    Arc::from(match kind {
        Kind::Idle => "STALE",
        Kind::Revalidating => "REVALIDATING",
    })
}

fn stopped_label(kind: worth_ui_query_binding::UiProjectionFactStopKind) -> Arc<str> {
    use worth_ui_query_binding::UiProjectionFactStopKind as Kind;
    Arc::from(match kind {
        Kind::SchemaMismatch => "SCHEMA MISMATCH",
        Kind::PayloadShapeMismatch => "PAYLOAD SHAPE MISMATCH",
        Kind::NativeFamilyMismatch => "NATIVE FAMILY MISMATCH",
        Kind::WrongWorld => "WRONG WORLD",
        Kind::StaleBindingGeneration => "STALE BINDING",
        Kind::StaleResultGeneration => "STALE RESULT",
        Kind::BasisMismatch => "BASIS MISMATCH",
        Kind::Unsupported => "UNSUPPORTED",
        Kind::Remasked => "REMASKED",
        Kind::BudgetExceeded => "BUDGET EXCEEDED",
        Kind::ResetRequired => "RESET REQUIRED",
    })
}

#[cfg(test)]
#[path = "content_plan/tests.rs"]
mod tests;

#[cfg(test)]
#[path = "content_plan/schema_transition_tests.rs"]
mod schema_transition_tests;
