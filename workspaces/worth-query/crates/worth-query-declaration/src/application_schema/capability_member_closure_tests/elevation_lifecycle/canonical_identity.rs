use super::{elevation_contract, LifecyclePosture, ReviewPosture, StatePosture};
use crate::application_capability::application_capability_canonical_components;

#[test]
fn swapping_lifecycle_operation_roles_changes_canonical_identity() {
    let ordinary = elevation_contract(
        StatePosture::Distinct,
        ReviewPosture::Distinct,
        LifecyclePosture::Distinct,
    );
    let swapped = elevation_contract(
        StatePosture::Distinct,
        ReviewPosture::Distinct,
        LifecyclePosture::SwappedOperations,
    );
    assert_ne!(
        application_capability_canonical_components(&ordinary),
        application_capability_canonical_components(&swapped)
    );
}
