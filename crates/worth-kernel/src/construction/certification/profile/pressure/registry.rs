use crate::construction::certification::profile::pressure::{
    PrimitiveConstructionPolicyPressureCase, PrimitiveConstructionPolicyPressureDeltaCase,
};
use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PrimitiveConstructionPolicyPressureRegistry {
    direct_cases: &'static [PrimitiveConstructionPolicyPressureCase],
    delta_cases: &'static [PrimitiveConstructionPolicyPressureDeltaCase],
    registry_digest: String,
}

impl PrimitiveConstructionPolicyPressureRegistry {
    pub(crate) fn direct_cases(&self) -> &'static [PrimitiveConstructionPolicyPressureCase] {
        self.direct_cases
    }

    pub(crate) fn delta_cases(&self) -> &'static [PrimitiveConstructionPolicyPressureDeltaCase] {
        self.delta_cases
    }

    pub(crate) fn registry_digest(&self) -> &str {
        &self.registry_digest
    }
}

pub(crate) fn policy_pressure_registry() -> PrimitiveConstructionPolicyPressureRegistry {
    let direct_cases = required_direct_cases();
    let delta_cases = required_delta_cases();
    let registry_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ParityIdentity,
        &direct_cases
            .iter()
            .map(|case| format!("{case:?}"))
            .chain(delta_cases.iter().map(|case| format!("{case:?}")))
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionPolicyPressureRegistry {
        direct_cases,
        delta_cases,
        registry_digest,
    }
}

fn required_direct_cases() -> &'static [PrimitiveConstructionPolicyPressureCase] {
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

fn required_delta_cases() -> &'static [PrimitiveConstructionPolicyPressureDeltaCase] {
    &[
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsPreservedAmbiguity,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingClarificationVsAggressiveSnap,
        PrimitiveConstructionPolicyPressureDeltaCase::GrazingAggressiveSnapVsHighFidelity,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceAskFirstVsBimHostFriendly,
        PrimitiveConstructionPolicyPressureDeltaCase::HostFaceBimHostFriendlyVsHighFidelityAskFirst,
    ]
}
