use forge_proof::TransitionOutcome;

use crate::spatial_intent::arbitration::progression::{
    admit_requested_spatial_arbitration_intent, declare_admitted_spatial_arbitration_intent,
    request_spatial_arbitration_intent,
};
use crate::spatial_intent::arbitration::{
    SpatialAuthoredActKind, SpatialIntentCapabilitySet, SpatialObservedRelationFact,
};
use crate::spatial_intent::policy::SpatialIntentPolicyProfile;

#[test]
fn arbitration_progression_preserves_requested_to_declared_flow() {
    let requested = request_spatial_arbitration_intent(
        SpatialAuthoredActKind::Move,
        &[SpatialObservedRelationFact::GrazingContact],
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    );
    let admitted = match admit_requested_spatial_arbitration_intent(requested) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("expected admitted request, got {outcome:?}"),
    };
    let declared = match declare_admitted_spatial_arbitration_intent(admitted) {
        TransitionOutcome::Success(value) => value,
        outcome => panic!("expected declared arbitration, got {outcome:?}"),
    };

    assert_eq!(
        declared.payload().authored_act(),
        SpatialAuthoredActKind::Move
    );
    assert_eq!(
        declared.payload().observed_relation_facts(),
        [SpatialObservedRelationFact::GrazingContact]
    );
}
