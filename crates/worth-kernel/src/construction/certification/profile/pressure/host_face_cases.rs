use super::case_fixtures::PrimitiveConstructionPolicyPressureFixture;
use super::report::{
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureSetup,
};
use crate::spatial_intent::{
    SpatialArbitrationPosture, SpatialAuthoredActKind, SpatialIntentCapabilitySet,
    SpatialIntentPolicyProfile, SpatialIntentPolicyProfileOverride, SpatialObservedRelationFact,
    SpatialPreviewRichness,
};

pub(super) fn host_face_pressure_fixture(
    case: PrimitiveConstructionPolicyPressureCase,
) -> PrimitiveConstructionPolicyPressureFixture {
    match case {
        PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst => (
            PrimitiveConstructionPolicyPressureSetup::HostFaceAttachMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::HostFaceContact],
            SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
            SpatialIntentPolicyProfile::ask_first_arbitration(),
        ),
        PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly => (
            PrimitiveConstructionPolicyPressureSetup::HostFaceAttachMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::HostFaceContact],
            SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
            SpatialIntentPolicyProfile::bim_host_friendly(),
        ),
        PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst => (
            PrimitiveConstructionPolicyPressureSetup::HostFaceAttachMove,
            SpatialAuthoredActKind::Move,
            vec![SpatialObservedRelationFact::HostFaceContact],
            SpatialIntentCapabilitySet::blocked_defaults().with_host_attach(),
            SpatialIntentPolicyProfile::bim_host_friendly().derive(
                SpatialIntentPolicyProfileOverride::new()
                    .with_name("bim_host_friendly_high_fidelity_ask_first")
                    .with_arbitration_posture(SpatialArbitrationPosture::AskFirst)
                    .with_preview_richness(SpatialPreviewRichness::HighFidelity),
            ),
        ),
        _ => unreachable!("non-host-face policy pressure case routed to host-face fixtures"),
    }
}
