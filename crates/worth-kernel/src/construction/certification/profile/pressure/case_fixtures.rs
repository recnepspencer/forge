use super::grazing_cases::grazing_pressure_fixture;
use super::host_face_cases::host_face_pressure_fixture;
use super::report::{
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureSetup,
};
use crate::spatial_intent::{
    SpatialAuthoredActKind, SpatialIntentCapabilitySet, SpatialIntentPolicyProfile,
    SpatialObservedRelationFact,
};

pub(super) type PrimitiveConstructionPolicyPressureFixture = (
    PrimitiveConstructionPolicyPressureSetup,
    SpatialAuthoredActKind,
    Vec<SpatialObservedRelationFact>,
    SpatialIntentCapabilitySet,
    SpatialIntentPolicyProfile,
);

impl PrimitiveConstructionPolicyPressureCase {
    pub(super) fn fixture(&self) -> PrimitiveConstructionPolicyPressureFixture {
        match self {
            Self::GrazingAskFirst
            | Self::GrazingPreserveAmbiguity
            | Self::GrazingAggressiveSnap
            | Self::GrazingAggressiveSnapHighFidelity => grazing_pressure_fixture(*self),
            Self::HostFaceAskFirst
            | Self::HostFaceBimHostFriendly
            | Self::HostFaceBimHostHighFidelityAskFirst => host_face_pressure_fixture(*self),
        }
    }
}
