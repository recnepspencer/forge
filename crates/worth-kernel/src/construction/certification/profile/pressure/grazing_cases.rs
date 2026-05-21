use super::case_fixtures::PrimitiveConstructionPolicyPressureFixture;
use super::report::{
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureSetup,
};
use crate::spatial_intent::{
    SpatialArbitrationPosture, SpatialAuthoredActKind, SpatialIntentCapabilitySet,
    SpatialIntentPolicyProfile, SpatialIntentPolicyProfileOverride, SpatialObservedRelationFact,
    SpatialPreviewRichness,
};

pub(super) fn grazing_pressure_fixture(
    case: PrimitiveConstructionPolicyPressureCase,
) -> PrimitiveConstructionPolicyPressureFixture {
    match case {
        PrimitiveConstructionPolicyPressureCase::GrazingAskFirst => (
            PrimitiveConstructionPolicyPressureSetup::GrazingContactMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::GrazingContact],
            SpatialIntentCapabilitySet::blocked_defaults(),
            SpatialIntentPolicyProfile::ask_first_arbitration(),
        ),
        PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity => (
            PrimitiveConstructionPolicyPressureSetup::GrazingContactMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::GrazingContact],
            SpatialIntentCapabilitySet::blocked_defaults(),
            SpatialIntentPolicyProfile::conservative_exact_modeling().derive(
                SpatialIntentPolicyProfileOverride::new()
                    .with_name("conservative_preserve_ambiguity")
                    .with_arbitration_posture(SpatialArbitrationPosture::PreserveAmbiguity),
            ),
        ),
        PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap => (
            PrimitiveConstructionPolicyPressureSetup::GrazingContactMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::GrazingContact],
            SpatialIntentCapabilitySet::blocked_defaults(),
            SpatialIntentPolicyProfile::aggressive_snap(),
        ),
        PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity => (
            PrimitiveConstructionPolicyPressureSetup::GrazingContactMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::GrazingContact],
            SpatialIntentCapabilitySet::blocked_defaults(),
            SpatialIntentPolicyProfile::aggressive_snap().derive(
                SpatialIntentPolicyProfileOverride::new()
                    .with_name("aggressive_snap_high_fidelity")
                    .with_preview_richness(SpatialPreviewRichness::HighFidelity),
            ),
        ),
        _ => unreachable!("non-grazing policy pressure case routed to grazing fixtures"),
    }
}
