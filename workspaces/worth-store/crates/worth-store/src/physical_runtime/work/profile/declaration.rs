use worth_signal::facade::MAX_ASPECTS;
use worth_store_aspect_native::StoreAspectContractAdmission;

use super::{
    identity::profile_identity, PhysicalSignalAspectDeclaration, PhysicalSignalProfileIdentity,
    PhysicalWorkCapacity,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkProfileDenial {
    DuplicateAspectContract,
    DependencyProjectionMaskAbsent,
    OutputMutationMaskAbsent,
    WorkFamilySetEmpty,
    SignalAspectCapacityExceeded,
}

#[derive(Debug, Clone)]
pub struct PhysicalWorkProfileDeclaration {
    security_authorities: Box<[[u8; 32]]>,
    aspects: Box<[PhysicalSignalAspectDeclaration]>,
    capacity: PhysicalWorkCapacity,
}

pub(super) struct PhysicalWorkProfileParts {
    pub(super) security_authorities: Box<[[u8; 32]]>,
    pub(super) aspects: Box<[PhysicalSignalAspectDeclaration]>,
    pub(super) capacity: PhysicalWorkCapacity,
}

impl PhysicalWorkProfileDeclaration {
    pub fn new(
        security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
        source: impl IntoIterator<Item = StoreAspectContractAdmission>,
    ) -> Result<Self, PhysicalWorkProfileDenial> {
        Self::from_signal_aspects(
            security,
            source
                .into_iter()
                .map(PhysicalSignalAspectDeclaration::from_contract),
        )
    }

    pub fn from_signal_aspects(
        security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
        source: impl IntoIterator<Item = PhysicalSignalAspectDeclaration>,
    ) -> Result<Self, PhysicalWorkProfileDenial> {
        Self::from_security_authorities([security.authority_identity().fingerprint()], source)
    }

    fn from_security_authorities(
        security_authorities: impl IntoIterator<Item = [u8; 32]>,
        source: impl IntoIterator<Item = PhysicalSignalAspectDeclaration>,
    ) -> Result<Self, PhysicalWorkProfileDenial> {
        let mut security_authorities = security_authorities.into_iter().collect::<Vec<_>>();
        security_authorities.sort_unstable();
        security_authorities.dedup();
        let mut aspects = Vec::new();
        for aspect in source {
            if aspects.len() == MAX_ASPECTS {
                return Err(PhysicalWorkProfileDenial::SignalAspectCapacityExceeded);
            }
            aspects.push(aspect);
        }
        for aspect in &aspects {
            if aspect.families().is_empty() {
                return Err(PhysicalWorkProfileDenial::WorkFamilySetEmpty);
            }
            if matches!(
                aspect.role(),
                super::PhysicalSignalAspectRole::Dependency
                    | super::PhysicalSignalAspectRole::DependencyAndOutput
            ) && aspect.contract().projection_mask().is_none()
            {
                return Err(PhysicalWorkProfileDenial::DependencyProjectionMaskAbsent);
            }
            if matches!(
                aspect.role(),
                super::PhysicalSignalAspectRole::Output
                    | super::PhysicalSignalAspectRole::DependencyAndOutput
            ) && aspect.contract().mutation_mask().is_none()
            {
                return Err(PhysicalWorkProfileDenial::OutputMutationMaskAbsent);
            }
        }
        aspects.sort_by(|left, right| left.contract().identity().cmp(right.contract().identity()));
        if aspects
            .windows(2)
            .any(|pair| pair[0].contract().identity() == pair[1].contract().identity())
        {
            return Err(PhysicalWorkProfileDenial::DuplicateAspectContract);
        }
        Ok(Self {
            security_authorities: security_authorities.into_boxed_slice(),
            aspects: aspects.into_boxed_slice(),
            capacity: PhysicalWorkCapacity::default(),
        })
    }

    pub(in crate::physical_runtime) fn with_native_extensions(
        self,
        security: worth_store_security::StoreAuthorityBoundSecurityScopeReceipt,
        source: impl IntoIterator<Item = PhysicalSignalAspectDeclaration>,
    ) -> Result<Self, PhysicalWorkProfileDenial> {
        let Self {
            security_authorities,
            aspects,
            capacity,
        } = self;
        let security_authorities = security_authorities
            .into_vec()
            .into_iter()
            .chain([security.authority_identity().fingerprint()]);
        let aspects = aspects.into_vec().into_iter().chain(source);
        Self::from_security_authorities(security_authorities, aspects)
            .map(|profile| profile.with_capacity(capacity))
    }

    pub const fn with_capacity(mut self, capacity: PhysicalWorkCapacity) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn identity(&self) -> PhysicalSignalProfileIdentity {
        profile_identity(&self.security_authorities, &self.aspects, self.capacity)
    }

    pub const fn contract_count(&self) -> usize {
        self.aspects.len()
    }
    pub const fn capacity(&self) -> PhysicalWorkCapacity {
        self.capacity
    }

    pub(super) fn into_parts(self) -> PhysicalWorkProfileParts {
        PhysicalWorkProfileParts {
            security_authorities: self.security_authorities,
            aspects: self.aspects,
            capacity: self.capacity,
        }
    }
}

impl Default for PhysicalWorkProfileDeclaration {
    fn default() -> Self {
        Self::from_security_authorities([], [])
            .expect("empty physical work profile is within Signal capacity")
    }
}
