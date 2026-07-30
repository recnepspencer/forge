use worth_foundational::facade::{AspectValue, InternedString};
use worth_query::facade::installed::{collection, operation};

use super::{
    UiCollectionCompleteness, UiCollectionContinuation, UiCollectionProjectionChange,
    UiCollectionProjectionFactReceipt, UiCollectionProjectionRowReference,
    UiCollectionProjectionTextRow, UiCollectionProjectionValue, UiCollectionProjectionWorkCounters,
    UiNativeTextValue, UiPresentProjection, UiProjectionAvailability, UiProjectionFactReceipt,
    UiProjectionFactStopKind, UiProjectionFactStopReceipt, UiProjectionUnavailableKind,
    UiProjectionUnavailableReceipt,
};

pub(crate) struct UiCollectionDerivationContext<'a> {
    pub binding: &'a crate::UiCollectionProjectionBinding,
    pub consumer: &'a collection::WorthQueryCollectionConsumerWindow,
    pub accesses: &'a [crate::application_binding::WorthUiCollectionTextNativeAccess],
    pub budget: crate::UiCollectionProjectionBudget,
}

pub(crate) fn derive_initial_collection_projection(
    context: UiCollectionDerivationContext<'_>,
) -> UiCollectionProjectionFactReceipt {
    let rows = context.consumer.rows().iter().collect::<Vec<_>>();
    derive_collection_projection(
        context,
        &rows,
        Box::<[UiCollectionProjectionChange]>::default(),
    )
}

pub(crate) fn derive_collection_projection(
    context: UiCollectionDerivationContext<'_>,
    rows: &[&collection::WorthQueryCollectionRowHandle],
    changes: Box<[UiCollectionProjectionChange]>,
) -> UiCollectionProjectionFactReceipt {
    let result_identity = context.consumer.result_generation_identity_evidence();
    let core = UiProjectionFactReceipt::admitted(
        context.binding.query_world_identity().clone(),
        context.consumer.binding_identity_evidence(),
        context.consumer.source_generation_identity_evidence(),
        result_identity.clone(),
    );
    let mut work = UiCollectionProjectionWorkCounters::default();
    for access in context.accesses {
        work.record_key_resolution(access.resolution_counters());
    }
    let availability = if changes.len() > context.budget.max_change_operations() {
        stopped(
            UiProjectionFactStopKind::BudgetExceeded,
            &result_identity,
            "the collection change-operation budget is exhausted",
        )
    } else if changes
        .iter()
        .any(|change| matches!(change, UiCollectionProjectionChange::ResetRequired { .. }))
    {
        stopped(
            UiProjectionFactStopKind::ResetRequired,
            &result_identity,
            "Query requires explicit collection replacement",
        )
    } else {
        derive_availability(&context, rows, &result_identity, &mut work)
    };
    UiCollectionProjectionFactReceipt::admitted(core, availability, work, changes)
}

fn derive_availability(
    context: &UiCollectionDerivationContext<'_>,
    rows: &[&collection::WorthQueryCollectionRowHandle],
    result_identity: &worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    work: &mut UiCollectionProjectionWorkCounters,
) -> UiProjectionAvailability<UiCollectionProjectionValue> {
    match context.consumer.result_state() {
        operation::WorthQueryOperationResultState::Pending => {
            UiProjectionAvailability::Unavailable(UiProjectionUnavailableReceipt::query_issued(
                UiProjectionUnavailableKind::Pending,
                result_identity.clone(),
            ))
        }
        operation::WorthQueryOperationResultState::Violation => stopped(
            UiProjectionFactStopKind::PayloadShapeMismatch,
            result_identity,
            "Query reported a violating collection result",
        ),
        operation::WorthQueryOperationResultState::Ready
        | operation::WorthQueryOperationResultState::Advisory
        | operation::WorthQueryOperationResultState::Partial => {
            derive_present(context, rows, result_identity, work)
        }
    }
}

fn derive_present(
    context: &UiCollectionDerivationContext<'_>,
    selected_rows: &[&collection::WorthQueryCollectionRowHandle],
    result_identity: &worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    work: &mut UiCollectionProjectionWorkCounters,
) -> UiProjectionAvailability<UiCollectionProjectionValue> {
    let completeness =
        if context.consumer.result_state() == operation::WorthQueryOperationResultState::Partial {
            UiCollectionCompleteness::Partial
        } else {
            UiCollectionCompleteness::Complete
        };
    if completeness == UiCollectionCompleteness::Partial
        && context.binding.requirement().requires_complete_result()
    {
        return stopped(
            UiProjectionFactStopKind::PayloadShapeMismatch,
            result_identity,
            "the binding requires a complete Query result",
        );
    }
    let continuation = match context.consumer.continuation().identity_evidence() {
        Some(_identity) if !context.binding.requirement().permits_continuation() => {
            return stopped(
                UiProjectionFactStopKind::PayloadShapeMismatch,
                result_identity,
                "the binding forbids Query continuation",
            );
        }
        Some(_) if context.budget.max_continuation_operations() == 0 => {
            return stopped(
                UiProjectionFactStopKind::BudgetExceeded,
                result_identity,
                "the collection continuation budget is exhausted",
            );
        }
        Some(identity) => {
            work.record_continuation();
            Some(UiCollectionContinuation::query_issued(identity))
        }
        None => None,
    };
    let rows = match derive_rows(context, selected_rows, work) {
        Ok(rows) => rows,
        Err(stop) => return UiProjectionAvailability::Stopped(stop),
    };
    UiProjectionAvailability::Present(UiPresentProjection::Current(
        UiCollectionProjectionValue::admitted(rows, completeness, continuation),
    ))
}

fn derive_rows(
    context: &UiCollectionDerivationContext<'_>,
    selected_rows: &[&collection::WorthQueryCollectionRowHandle],
    work: &mut UiCollectionProjectionWorkCounters,
) -> Result<Box<[UiCollectionProjectionTextRow]>, UiProjectionFactStopReceipt> {
    let mut rows = Vec::with_capacity(selected_rows.len());
    for row in selected_rows {
        work.visit_row();
        let mut values = Vec::with_capacity(context.accesses.len());
        for access in context.accesses {
            let fact = context
                .consumer
                .native_value(row, access.key())
                .map_err(|_| {
                    stop_receipt(
                        UiProjectionFactStopKind::WrongWorld,
                        &context.consumer.result_generation_identity_evidence(),
                        "Query rejected the collection row or native key authority",
                    )
                })?;
            let text = match fact.native_value().scalar() {
                Some(AspectValue::String(InternedString::Raw(text))) => text.clone(),
                _ => {
                    return Err(stop_receipt(
                        UiProjectionFactStopKind::NativeFamilyMismatch,
                        &context.consumer.result_generation_identity_evidence(),
                        "the selected Query native value is not direct raw text",
                    ));
                }
            };
            if work.native_bytes_retained().saturating_add(text.len())
                > context.budget.max_native_bytes()
            {
                return Err(stop_receipt(
                    UiProjectionFactStopKind::BudgetExceeded,
                    &context.consumer.result_generation_identity_evidence(),
                    "the collection native-text byte budget is exhausted",
                ));
            }
            work.record_native_access(fact.counters(), text.len());
            values.push(UiNativeTextValue::from_raw(text));
        }
        rows.push(UiCollectionProjectionTextRow::admitted(
            UiCollectionProjectionRowReference::query_issued(
                row.entity_identity().evidence_identity(),
            ),
            values,
        ));
    }
    Ok(rows.into_boxed_slice())
}

fn stopped(
    kind: UiProjectionFactStopKind,
    result_identity: &worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    summary: &'static str,
) -> UiProjectionAvailability<UiCollectionProjectionValue> {
    UiProjectionAvailability::Stopped(stop_receipt(kind, result_identity, summary))
}

fn stop_receipt(
    kind: UiProjectionFactStopKind,
    result_identity: &worth_query::facade::runtime::WorthQueryEvidenceIdentity,
    summary: &'static str,
) -> UiProjectionFactStopReceipt {
    UiProjectionFactStopReceipt::query_issued(kind, result_identity.clone(), None, summary)
}
