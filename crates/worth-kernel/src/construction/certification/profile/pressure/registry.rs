#[cfg(test)]
use crate::construction::certification::profile::pressure::{
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureDeltaCase,
};

#[cfg(test)]
pub(crate) fn required_policy_pressure_direct_cases(
) -> &'static [PrimitiveConstructionPolicyPressureCase] {
    &[
        PrimitiveConstructionPolicyPressureCase::GrazingAskFirst,
        PrimitiveConstructionPolicyPressureCase::GrazingPreserveAmbiguity,
        PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnap,
        PrimitiveConstructionPolicyPressureCase::GrazingAggressiveSnapHighFidelity,
        PrimitiveConstructionPolicyPressureCase::HostFaceAskFirst,
        PrimitiveConstructionPolicyPressureCase::HostFaceBimHostFriendly,
        PrimitiveConstructionPolicyPressureCase::HostFaceBimHostHighFidelityAskFirst,
    ]
}

#[cfg(test)]
pub(crate) fn required_policy_pressure_delta_cases(
) -> &'static [PrimitiveConstructionPolicyPressureDeltaCase] {
    &[
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
    ]
}
