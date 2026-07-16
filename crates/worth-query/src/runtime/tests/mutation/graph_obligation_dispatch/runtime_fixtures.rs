use super::super::super::support::*;

pub(super) fn runtime_with_relation_obligation(relation_kind: &str) -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(
            WorthQueryGraphObligationRegistration::schema_contract_validator(
                WorthQueryGraphObligationRuleIdentity::new(
                    "test.graph-obligation-dispatch",
                    relation_kind,
                    "v1",
                )
                .unwrap(),
                WorthQueryGraphTouchSelector::relation_kind(relation_kind).unwrap(),
                WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
            ),
        )
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with relation graph obligation")
}

pub(super) fn runtime_with_collection_obligation(collection: &str) -> WorthQueryRuntime {
    runtime_with_collection_registration(collection_registration(
        collection,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    ))
}

pub(super) fn runtime_with_blocking_collection_obligation(collection: &str) -> WorthQueryRuntime {
    runtime_with_collection_registration(collection_registration(
        collection,
        WorthQueryGraphObligationSupportPosture::unsupported(
            WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    ))
}

pub(super) fn runtime_with_scalar_collection_obligation(collection: &str) -> WorthQueryRuntime {
    runtime_with_collection_registration(collection_registration(
        collection,
        WorthQueryGraphObligationSupportPosture::supported(
            WorthQueryGraphObligationSupportLane::ScalarMutation,
        ),
    ))
}

fn runtime_with_collection_registration(
    registration: WorthQueryGraphObligationRegistration,
) -> WorthQueryRuntime {
    complete_backend_from_parts_builder()
        .graph_obligation(registration)
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with collection graph obligation")
}

fn collection_registration(
    collection: &str,
    support_posture: WorthQueryGraphObligationSupportPosture,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new(
            "test.graph-obligation-dispatch",
            collection,
            "v1",
        )
        .unwrap(),
        WorthQueryGraphTouchSelector::collection(collection).unwrap(),
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
    .with_support_posture(support_posture)
}
