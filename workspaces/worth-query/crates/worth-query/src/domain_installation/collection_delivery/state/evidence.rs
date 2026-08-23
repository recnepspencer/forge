use crate::domain_installation::{
    WorthQueryCollectionDeliveryCounters, WorthQueryCollectionDeliveryDenial,
    WorthQueryCollectionDeliveryDenialKind,
};

pub(super) fn collection_evidence(
    shape: &'static str,
) -> crate::evidence_identity::WorthQueryEvidenceIdentityEncoder {
    crate::WorthQueryEvidenceIdentity::compose(
        crate::WorthQueryEvidenceScope::ProjectionConsumptionIdentity,
    )
    .field_shape(crate::WorthQueryEvidenceTag::new("collection"), shape)
}

pub(super) fn maintenance_ordinal(value: Option<u64>) -> String {
    value
        .map(|ordinal| ordinal.to_string())
        .unwrap_or_else(|| "initial".to_owned())
}

pub(super) fn carry_planning_counters(
    applied: &mut WorthQueryCollectionDeliveryCounters,
    planned: WorthQueryCollectionDeliveryCounters,
) {
    applied.invalidation_authority_checks += planned.invalidation_authority_checks;
    applied.lease_checks += planned.lease_checks;
    applied.generation_checks += planned.generation_checks;
    applied.cursor_checks += planned.cursor_checks;
    applied.semantic_contract_checks += planned.semantic_contract_checks;
    applied.pending_patch_checks += planned.pending_patch_checks;
    applied.prior_window_rows_visited += planned.prior_window_rows_visited;
    applied.fresh_window_rows_visited += planned.fresh_window_rows_visited;
    applied.affected_identity_lookups += planned.affected_identity_lookups;
    applied.entity_point_lookups += planned.entity_point_lookups;
    applied.ordering_index_updates += planned.ordering_index_updates;
    applied.operations_materialized += planned.operations_materialized;
    applied.native_facts_materialized += planned.native_facts_materialized;
    applied.full_collection_scans += planned.full_collection_scans;
    applied.unrelated_consumer_scans += planned.unrelated_consumer_scans;
}

pub(crate) fn denial(
    kind: WorthQueryCollectionDeliveryDenialKind,
    counters: WorthQueryCollectionDeliveryCounters,
) -> WorthQueryCollectionDeliveryDenial {
    WorthQueryCollectionDeliveryDenial::new(kind, counters)
}
