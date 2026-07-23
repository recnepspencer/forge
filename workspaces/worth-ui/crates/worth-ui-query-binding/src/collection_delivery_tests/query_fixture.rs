use worth_foundational::facade::{AspectValue, CanonicalF32, FieldKey};
use worth_query::facade::{domain, foundation, runtime};

use crate::installed_domain::snapshot_measurement::{
    WorthUiSnapshotMeasurement, WorthUiSnapshotMeasurementFamily,
};
use crate::installed_operations_tests::{aspect_touch, bound_snapshot, installed_builder};

type SettledMeasurement = domain::WorthQuerySettledDomainProjection<
    crate::WorthUiDomainEntry,
    WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
    foundation::ObservationLaneWitness,
>;

pub(super) type MeasurementCollection = domain::WorthQueryBoundCollection<
    crate::WorthUiDomainEntry,
    WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
    foundation::ObservationLaneWitness,
>;

pub(super) type MeasurementLease = domain::WorthQuerySharedLiveProjectionLease<
    crate::WorthUiDomainEntry,
    WorthUiSnapshotMeasurement,
    WorthUiSnapshotMeasurementFamily,
    foundation::ObservationLaneWitness,
>;

pub(super) fn workspace_with_measurement() -> (
    runtime::WorthQueryWorkspace,
    foundation::WorthQueryEntityIdentity,
) {
    let mut workspace = live_builder()
        .workspace("worth-ui-collection-delivery")
        .unwrap();
    let entity = workspace
        .insert("WorthUiMeasurement", |measurement| {
            measurement
                .set_aspect(aspect_touch("identity.id"), "measurement-1")
                .set_aspect(
                    aspect_touch("measurement.value"),
                    AspectValue::Float32(CanonicalF32::from_f32(10.0)),
                )
        })
        .unwrap()
        .deltas()[0]
        .entity_identity()
        .clone();
    (workspace, entity)
}

pub(super) fn bound_collection(
    workspace: &mut runtime::WorthQueryWorkspace,
) -> MeasurementCollection {
    settled_measurement(workspace)
        .into_bound_collection()
        .unwrap()
}

pub(super) fn managed_lease(workspace: &mut runtime::WorthQueryWorkspace) -> MeasurementLease {
    let live = match settled_measurement(workspace)
        .into_lifecycle()
        .promote(workspace)
    {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("Worth UI collection did not promote"),
    };
    match live.into_managed_lease(workspace) {
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Admitted(lease) => lease,
        domain::WorthQueryProjectionLeaseAdmissionOutcome::Stopped(stop) => {
            panic!("Worth UI collection lease stopped: {}", stop.detail())
        }
    }
}

pub(super) fn update_measurement(
    workspace: &mut runtime::WorthQueryWorkspace,
    entity: foundation::WorthQueryEntityIdentity,
) {
    workspace
        .update(entity, |measurement| {
            measurement.set_aspect(
                aspect_touch("measurement.value"),
                AspectValue::Float32(CanonicalF32::from_f32(20.0)),
            )
        })
        .unwrap();
}

fn settled_measurement(workspace: &mut runtime::WorthQueryWorkspace) -> SettledMeasurement {
    let bound = bound_snapshot(workspace);
    let mut request = bound
        .consumer_projection_contract()
        .unwrap()
        .projection_request();
    request
        .select_display_native_field(FieldKey::new("value").unwrap())
        .unwrap();
    bound
        .execute((), workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume_bound(request.build().unwrap())
        .unwrap()
        .settle()
        .unwrap()
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
    .fold(installed_builder(), |builder, dimension| {
        builder.consumer_support_posture(
            dimension,
            domain::WorthQueryConsumerSupportPosture::Supported,
        )
    })
}
