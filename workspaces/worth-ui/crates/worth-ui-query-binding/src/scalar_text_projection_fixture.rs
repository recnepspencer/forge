use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::domain::{
    WorthQueryConsumerSupportDimension, WorthQueryConsumerSupportPosture,
};
use worth_query::facade::runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectValue};

use crate::{
    install_worth_ui_test_operation_executors, worth_ui_domain_package,
    worth_ui_native_aspect_contracts,
};

pub(crate) fn projection_workspace(
    supports_async_lifecycle: bool,
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    projection_runtime_builder(supports_async_lifecycle)
        .workspace("worth-ui-scalar-text-projection")
        .expect("scalar text Query workspace")
}

pub(crate) fn remasked_projection_workspace() -> worth_query::facade::runtime::WorthQueryWorkspace {
    projection_runtime_builder(true)
        .remask_projection(
            worth_query::facade::runtime::WorthQueryRuntimeRemaskProjection::remasked(
                worth_query::facade::runtime::WorthQueryRuntimeRemaskReasonKind::PolicyDrift,
                "policy:drifted",
                "tenant-truth:stable",
                "tenant-schema:stable",
                "relationship-proof:verified",
                "schema-context:worth-ui-projection",
            ),
        )
        .workspace("worth-ui-remasked-scalar-text-projection")
        .expect("remasked scalar text Query workspace")
}

pub(crate) fn collection_projection_workspace() -> worth_query::facade::runtime::WorthQueryWorkspace
{
    collection_projection_workspace_builder(true)
        .workspace("worth-ui-collection-text-projection")
        .expect("collection text Query workspace")
}

pub(crate) fn collection_projection_workspace_without_entity_lookup(
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    collection_projection_workspace_builder(true)
        .without_collection_entity_lookup()
        .workspace("worth-ui-collection-text-projection-without-entity-lookup")
        .expect("collection text Query workspace without entity lookup")
}

pub(crate) fn collection_projection_workspace_without_dependency_impact(
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    collection_projection_workspace_builder(false)
        .workspace("worth-ui-collection-text-projection-without-dependency-impact")
        .expect("collection text Query workspace without dependency-impact support")
}

pub(crate) fn partial_collection_projection_workspace(
) -> worth_query::facade::runtime::WorthQueryWorkspace {
    let builder = collection_projection_support(projection_runtime_base_builder(), true);
    crate::install_worth_ui_partial_collection_test_operation_executors(builder)
        .workspace("worth-ui-partial-collection-text-projection")
        .expect("partial collection text Query workspace")
}

pub(crate) fn seeded_collection_projection_workspace(
    rows: Vec<(String, String)>,
    partial: bool,
    entity_lookup: bool,
    supports_async_lifecycle: bool,
) -> (
    worth_query::facade::runtime::WorthQueryWorkspace,
    worth_query::facade::consumer_kit::WorthQueryTestSeedReceipt,
) {
    use worth_query::facade::{
        consumer_kit::WorthQueryTestSeedRow,
        runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectValue},
    };

    let identity_touch =
        WorthQueryAspectTouch::from_authoring_ingress_text("identity.id").expect("identity touch");
    let status_touch = WorthQueryAspectTouch::from_authoring_ingress_text("query_text.status")
        .expect("status touch");
    let seed_rows = rows
        .into_iter()
        .map(|(identity, status)| {
            WorthQueryTestSeedRow::new(identity.clone(), "WorthUiProjectionText", |entity| {
                entity
                    .set_aspect(
                        identity_touch.clone(),
                        WorthQueryAuthoredAspectValue::string(identity),
                    )
                    .set_aspect(
                        status_touch.clone(),
                        WorthQueryAuthoredAspectValue::string(status),
                    )
            })
            .expect("projection seed row")
        })
        .collect();
    let builder = if supports_async_lifecycle {
        projection_runtime_builder(true)
    } else {
        projection_runtime_base_builder()
    };
    let builder = collection_projection_support(builder, true);
    let builder = if partial {
        crate::install_worth_ui_partial_collection_test_operation_executors(builder)
    } else if supports_async_lifecycle {
        builder
    } else {
        install_worth_ui_test_operation_executors(builder)
    };
    let builder = if entity_lookup {
        builder
    } else {
        builder.without_collection_entity_lookup()
    };
    builder
        .seed_collection_rows(identity_touch, seed_rows)
        .expect("projection seed identities")
        .workspace_with_seed_receipt("worth-ui-seeded-collection-text-projection")
        .expect("seeded collection text Query workspace")
}

fn collection_projection_workspace_builder(
    supports_dependency_impact: bool,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    collection_projection_support(
        projection_runtime_builder(false),
        supports_dependency_impact,
    )
}

fn collection_projection_support(
    builder: worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder,
    supports_dependency_impact: bool,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let builder = [
        WorthQueryConsumerSupportDimension::Live,
        WorthQueryConsumerSupportDimension::Continuation,
        WorthQueryConsumerSupportDimension::ProjectionConsumption,
        WorthQueryConsumerSupportDimension::Sharing,
        WorthQueryConsumerSupportDimension::Invalidation,
        WorthQueryConsumerSupportDimension::CollectionDelivery,
    ]
    .into_iter()
    .fold(builder, |builder, dimension| {
        builder.consumer_support_posture(dimension, WorthQueryConsumerSupportPosture::Supported)
    });
    if supports_dependency_impact {
        builder.consumer_support_posture(
            WorthQueryConsumerSupportDimension::DependencyImpact,
            WorthQueryConsumerSupportPosture::Supported,
        )
    } else {
        builder
    }
}

fn projection_runtime_builder(
    supports_async_lifecycle: bool,
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let builder = install_worth_ui_test_operation_executors(projection_runtime_base_builder());
    if supports_async_lifecycle {
        [
            WorthQueryConsumerSupportDimension::AsyncResultState,
            WorthQueryConsumerSupportDimension::Recovery,
        ]
        .into_iter()
        .fold(builder, |builder, dimension| {
            builder.consumer_support_posture(dimension, WorthQueryConsumerSupportPosture::Supported)
        })
    } else {
        builder
    }
}

fn projection_runtime_base_builder(
) -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    let schema = WorthQueryTestBackendSchema::single_collection("WorthUiProjectionText")
        .aspect_contracts(worth_ui_native_aspect_contracts())
        .expect("Worth UI native aspect contracts")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect")
        .aspect("query_text.status", "query_text.status")
        .expect("projection status aspect");
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(worth_ui_domain_package())
}

#[cfg(test)]
pub(crate) fn insert_status(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    status: &str,
) -> worth_query::facade::foundation::WorthQueryEntityIdentity {
    insert_collection_status(workspace, "platform.pulse.status", status)
}

pub(crate) fn insert_collection_status(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    identity: &str,
    status: &str,
) -> worth_query::facade::foundation::WorthQueryEntityIdentity {
    let receipt = workspace
        .insert("WorthUiProjectionText", |entity| {
            entity
                .set_aspect(
                    WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                        .expect("identity touch"),
                    WorthQueryAuthoredAspectValue::string(identity),
                )
                .set_aspect(
                    WorthQueryAspectTouch::from_authoring_ingress_text("query_text.status")
                        .expect("projection status touch"),
                    WorthQueryAuthoredAspectValue::string(status),
                )
        })
        .expect("projection text insertion");
    receipt.deltas()[0].entity_identity().clone()
}

pub(crate) fn update_status(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    entity: worth_query::facade::foundation::WorthQueryEntityIdentity,
    status: &str,
) {
    workspace
        .update(entity, |record| {
            record.set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("query_text.status")
                    .expect("projection status touch"),
                WorthQueryAuthoredAspectValue::string(status),
            )
        })
        .expect("projection text update");
}

pub(crate) fn update_identity(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    entity: worth_query::facade::foundation::WorthQueryEntityIdentity,
    identity: &str,
) {
    workspace
        .update(entity, |record| {
            record.set_aspect(
                WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")
                    .expect("identity touch"),
                WorthQueryAuthoredAspectValue::string(identity),
            )
        })
        .expect("projection identity update");
}
