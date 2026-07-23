use super::UiAllocationInvalidationNarrowingCounters;
use crate::runtime::{
    UiAllocationInvalidationFamily, UiAllocationInvalidationNarrowingDenial,
    UiAllocationInvalidationTarget,
};

pub(super) fn narrow_settled_query_fact(
    ingress_key: &crate::runtime::UiAllocationFrameIngressKey,
    family: UiAllocationInvalidationFamily,
    view_binding_id: &crate::capability::ViewBindingId,
    fact: &worth_ui_query_binding::WorthUiSettledSnapshotFact,
    ordinal: u16,
    authority: &std::cell::RefCell<super::authority::UiAllocationInvalidationAuthority>,
    counters: &mut UiAllocationInvalidationNarrowingCounters,
) -> Result<UiAllocationInvalidationTarget, UiAllocationInvalidationNarrowingDenial> {
    counted(counters.visit_query_settlement(), ordinal)?;
    let source_generation = fact
        .source_generation()
        .expect("retained settled facts carry source generation")
        .as_u64();
    let source_order = fact
        .source_order()
        .expect("retained settled facts carry source order")
        .as_u64();
    if ingress_key.source_generation().as_u64() != source_generation {
        return Err(
            UiAllocationInvalidationNarrowingDenial::QuerySourceGenerationMismatch { ordinal },
        );
    }
    if ingress_key.source_order().as_u64() != source_order {
        return Err(UiAllocationInvalidationNarrowingDenial::QuerySourceOrderMismatch { ordinal });
    }
    if ingress_key.ingress_identity().as_u64() != source_order {
        return Err(
            UiAllocationInvalidationNarrowingDenial::QueryConsumptionReceiptMismatch { ordinal },
        );
    }
    let batch = fact.measurement_facts().map_err(|_| {
        UiAllocationInvalidationNarrowingDenial::QuerySettlementFamilyMissing { ordinal }
    })?;
    if batch.observations().is_empty() {
        return Err(
            UiAllocationInvalidationNarrowingDenial::QuerySettlementFamilyMissing { ordinal },
        );
    }
    counted(
        counters.visit_query_observations(batch.observations().len()),
        ordinal,
    )?;
    if batch
        .observations()
        .iter()
        .any(|value| value.extent().as_f32().is_nan())
    {
        return Err(UiAllocationInvalidationNarrowingDenial::QueryExtentUnordered { ordinal });
    }
    let source_key =
        crate::evidence::measurement::basis::UiQueryAllocationSourceKey::from_settled_fact(
            view_binding_id.clone(),
            fact,
        );
    if family == UiAllocationInvalidationFamily::ContentExtentChange {
        let authority = authority.borrow();
        let lookup = authority.scroll_settled_query_target(&source_key);
        counted(counters.record_authority_probes(lookup.probes()), ordinal)?;
        if lookup.is_empty() {
            return Err(
                UiAllocationInvalidationNarrowingDenial::ScrollOwnershipNotAdmitted { ordinal },
            );
        }
        return Ok(UiAllocationInvalidationTarget::ScrollOwnedContentExtent {
            bindings: lookup.materialize_bindings(),
        });
    }
    counted(counters.lookup_graph_target(), ordinal)?;
    let lookup = authority
        .borrow()
        .settled_query_target(&source_key)
        .map_err(|denial| super::consumer_support::map_lookup_denial(denial, ordinal))?;
    counted(counters.record_authority_probes(lookup.probes), ordinal)?;
    let target = lookup
        .target
        .ok_or(UiAllocationInvalidationNarrowingDenial::QueryTargetNotAdmitted { ordinal })?;
    Ok(UiAllocationInvalidationTarget::SettledQueryFact { target })
}

fn counted(
    result: Result<(), ()>,
    ordinal: u16,
) -> Result<(), UiAllocationInvalidationNarrowingDenial> {
    result.map_err(
        |()| UiAllocationInvalidationNarrowingDenial::AuthorityCounterExhausted { ordinal },
    )
}
