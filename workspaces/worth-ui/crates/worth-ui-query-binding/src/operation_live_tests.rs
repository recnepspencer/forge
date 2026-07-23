use worth_foundational::facade::{AspectValue, CanonicalF32};
use worth_query::facade::{domain, foundation};

use crate::{
    WorthUiCollectionAllocationPolicy, WorthUiOperationLiveCloseOutcome,
    WorthUiOperationLiveOpenRequest, WorthUiOperationLiveRefreshOutcome,
    WorthUiQueryAllocationDetail, WorthUiQueryConsumerRequirements, WorthUiQueryDenialPresentation,
    WorthUiQueryInspectionRelevance, WorthUiQueryViewShape, WorthUiQueryWorkspaceExt,
};

#[test]
fn installed_live_view_uses_operation_lease_window_and_query_patch() {
    let mut workspace = live_builder().workspace("worth-ui-operation-live").unwrap();
    let entity = insert_measurement(&mut workspace);
    let view = workspace
        .worth_ui()
        .unwrap()
        .live_measurement_view("dashboard.measurements")
        .unwrap();
    let mut resource = view.open_operation(request(), &mut workspace).unwrap();

    assert_eq!(resource.rows().len(), 1);
    assert!(!resource.lease_identity().is_empty());
    assert!(matches!(
        resource.refresh(&mut workspace).unwrap(),
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery
    ));

    update_measurement(&mut workspace, entity.clone());
    let consequences = match resource.refresh(&mut workspace).unwrap() {
        WorthUiOperationLiveRefreshOutcome::Applied(consequences) => consequences,
        WorthUiOperationLiveRefreshOutcome::NoSemanticDelivery => {
            panic!("semantic measurement update produced no UI patch")
        }
    };
    assert!(matches!(
        consequences.graph_mutations(),
        [crate::WorthUiCollectionGraphMutation::Update { row }]
            if row.entity_identity() == &entity
    ));
    assert_eq!(consequences.native_fact_touches(), 1);
    assert_eq!(consequences.measurement_invalidations(), 1);

    assert!(matches!(
        resource.close(&mut workspace),
        WorthUiOperationLiveCloseOutcome::Closed(_)
    ));
}

fn request() -> WorthUiOperationLiveOpenRequest {
    WorthUiOperationLiveOpenRequest::new(
        WorthUiQueryConsumerRequirements::new(
            domain::WorthQueryConsumerBoundaryRequirements {
                presentation: domain::WorthQueryConsumerPresentationPosture::Interactive,
                allocation: domain::WorthQueryConsumerAllocationPosture::Borrowed,
            },
            WorthUiQueryAllocationDetail::BorrowedFactSlice,
            WorthUiQueryViewShape::Collection,
            WorthUiQueryDenialPresentation::StructuredStatus,
            WorthUiQueryInspectionRelevance::Relevant,
        ),
        domain::WorthQueryCollectionWindowBreadth::new(1, 0, 0, 1).unwrap(),
        WorthUiCollectionAllocationPolicy::PreserveMounted,
    )
}

fn live_builder() -> worth_query::facade::consumer_kit::WorthQueryInMemoryTestRuntimeBuilder {
    [
        domain::WorthQueryConsumerSupportDimension::Live,
        domain::WorthQueryConsumerSupportDimension::Sharing,
        domain::WorthQueryConsumerSupportDimension::Invalidation,
        domain::WorthQueryConsumerSupportDimension::DependencyImpact,
        domain::WorthQueryConsumerSupportDimension::CollectionDelivery,
    ]
    .into_iter()
    .fold(
        crate::installed_operations_tests::installed_builder(),
        |builder, dimension| {
            builder.consumer_support_posture(
                dimension,
                domain::WorthQueryConsumerSupportPosture::Supported,
            )
        },
    )
}

fn insert_measurement(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
) -> foundation::WorthQueryEntityIdentity {
    workspace
        .insert("WorthUiMeasurement", |measurement| {
            measurement
                .set_aspect(
                    crate::installed_operations_tests::aspect_touch("identity.id"),
                    "measurement-1",
                )
                .set_aspect(
                    crate::installed_operations_tests::aspect_touch("measurement.value"),
                    AspectValue::Float32(CanonicalF32::from_f32(10.0)),
                )
        })
        .unwrap()
        .deltas()[0]
        .entity_identity()
        .clone()
}

fn update_measurement(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    entity: foundation::WorthQueryEntityIdentity,
) {
    workspace
        .update(entity, |measurement| {
            measurement.set_aspect(
                crate::installed_operations_tests::aspect_touch("measurement.value"),
                AspectValue::Float32(CanonicalF32::from_f32(20.0)),
            )
        })
        .unwrap();
}
