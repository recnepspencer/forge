use super::{
    build_from_members, elevation_contract_with_duration, elevation_members, LifecyclePosture,
    ReviewPosture, StatePosture,
};
use crate::application_capability::application_capability_canonical_components;
use crate::application_schema::ApplicationSchemaDeclarationDenial;

#[test]
fn zero_or_timeline_inexact_maximum_duration_cannot_install() {
    for duration in [
        std::time::Duration::ZERO,
        std::time::Duration::from_millis(1_001),
    ] {
        let contract = elevation_contract_with_duration(
            StatePosture::Distinct,
            ReviewPosture::Distinct,
            LifecyclePosture::Distinct,
            duration,
        );
        assert_eq!(
            build_from_members(elevation_members(contract)),
            Err(ApplicationSchemaDeclarationDenial::InvalidApplicationCapability)
        );
    }
}

#[test]
fn maximum_duration_changes_capability_identity() {
    let contract = |duration| {
        elevation_contract_with_duration(
            StatePosture::Distinct,
            ReviewPosture::Distinct,
            LifecyclePosture::Distinct,
            duration,
        )
    };
    assert_ne!(
        application_capability_canonical_components(&contract(std::time::Duration::from_secs(60))),
        application_capability_canonical_components(&contract(std::time::Duration::from_secs(61)))
    );
}
