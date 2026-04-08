use super::*;
use super::validation::{validate_registration_set, validate_registration_values};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenBridgeMappingRegistration {
    registration: BridgeMappingRegistration,
}

impl FrozenBridgeMappingRegistration {
    fn new(registration: BridgeMappingRegistration) -> Self {
        Self { registration }
    }

    pub fn mapping_id(&self) -> &crate::mapping::registration::BridgeMappingId {
        self.registration.mapping_id()
    }

    pub fn truth_scope(&self) -> &crate::mapping::registration::TruthPatchScope {
        self.registration.truth_scope()
    }

    pub fn signal_scope(&self) -> &crate::mapping::registration::SignalInvalidationScope {
        self.registration.signal_scope()
    }

    pub fn routing_mode(&self) -> crate::mapping::registration::CoarseRoutingMode {
        self.registration.routing_mode()
    }

    pub fn fallback_class(&self) -> Option<BridgeMappingFallbackClass> {
        self.registration.truth_scope().fallback_class()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrozenMappingRegistry {
    pub(crate) registrations: Vec<FrozenBridgeMappingRegistration>,
}

impl FrozenMappingRegistry {
    pub(crate) fn freeze(mut registrations: Vec<BridgeMappingRegistration>) -> Result<Self, BridgeBuildError> {
        if registrations.is_empty() {
            return Err(BridgeBuildError::new(
                BridgeBuildErrorKind::MissingMappingRegistrations,
                "RuntimeBridgeBuilder requires at least one bridge mapping registration before build.",
            ));
        }

        registrations.sort_by(canonical_registration_order);
        validate_registration_values(&registrations)?;
        validate_registration_set(&registrations)?;

        Ok(Self {
            registrations: registrations
                .into_iter()
                .map(FrozenBridgeMappingRegistration::new)
                .collect(),
        })
    }

    pub(crate) fn registrations(&self) -> &[FrozenBridgeMappingRegistration] {
        &self.registrations
    }
}

fn canonical_registration_order(
    left: &BridgeMappingRegistration,
    right: &BridgeMappingRegistration,
) -> std::cmp::Ordering {
    right
        .truth_scope()
        .specificity_rank()
        .cmp(&left.truth_scope().specificity_rank())
        .then_with(|| left.truth_scope().cmp(right.truth_scope()))
        .then_with(|| left.signal_scope().cmp(right.signal_scope()))
        .then_with(|| left.routing_mode().cmp(&right.routing_mode()))
        .then_with(|| left.mapping_id().cmp(right.mapping_id()))
}
