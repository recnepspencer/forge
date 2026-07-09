use super::*;

pub(in crate::harness::tests::pricing_shock) fn pricing_snapshot(
    snapshot_identity: TruthSnapshotIdentity,
    steel_cost: &str,
    rubber_cost: &str,
) -> SnapshotFixture {
    SnapshotFixture::new(
        snapshot_identity,
        vec![
            SnapshotReadRecord::for_request(
                &pricing_coarse_read_request("steel"),
                worth_foundational::facade::AspectValue::String((steel_cost).into()),
            ),
            SnapshotReadRecord::for_request(
                &pricing_coarse_read_request("rubber"),
                worth_foundational::facade::AspectValue::String((rubber_cost).into()),
            ),
        ],
    )
}

pub(in crate::harness::tests::pricing_shock) fn pricing_aspect_snapshot(
    snapshot_identity: TruthSnapshotIdentity,
    steel_cost: &str,
    rubber_cost: &str,
) -> SnapshotFixture {
    SnapshotFixture::new(
        snapshot_identity,
        vec![
            SnapshotReadRecord::for_request(
                &pricing_field_slice_read_request("steel"),
                worth_foundational::facade::AspectValue::String(steel_cost.into()),
            ),
            SnapshotReadRecord::for_request(
                &pricing_field_slice_read_request("rubber"),
                worth_foundational::facade::AspectValue::String(rubber_cost.into()),
            ),
        ],
    )
}

fn pricing_coarse_read_request(component: &str) -> SnapshotReadRequest {
    SnapshotReadRequest::for_coarse(
        format!("component:{component}"),
        crate::snapshot::SnapshotReadContract::scalar(
            worth_foundational::facade::AspectKey::new("cost").expect("valid pricing aspect key"),
            worth_foundational::facade::ScalarAspectType::String,
        ),
    )
}

fn pricing_field_slice_read_request(component: &str) -> SnapshotReadRequest {
    let entity = format!("component:{component}");
    let aspect_key =
        worth_foundational::facade::AspectKey::new("cost").expect("valid pricing aspect key");
    let aspect_locator = worth_foundational::facade::AspectLocator::new(
        worth_foundational::facade::LocatorAuthority::Authoritative,
        aspect_key.clone(),
    );
    let field_path = worth_foundational::facade::CanonicalFieldPath::single(
        worth_foundational::facade::FieldKey::new("usd".to_owned())
            .expect("valid pricing field key"),
    );
    let field_locator = worth_foundational::facade::AspectFieldLocator::from_aspect(
        aspect_locator.clone(),
        field_path.clone(),
    );
    SnapshotReadRequest::for_native_subscription_slice(
        entity,
        crate::snapshot::SnapshotReadContract::scalar(
            aspect_key,
            worth_foundational::facade::ScalarAspectType::String,
        ),
        aspect_locator,
        Some(field_locator),
        worth_foundational::facade::AspectMask::new([field_path]),
        SubscriptionSliceKind::SignalField,
    )
}
