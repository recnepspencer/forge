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
    aspects: Box<[PhysicalSignalAspectDeclaration]>,
    capacity: PhysicalWorkCapacity,
}

impl PhysicalWorkProfileDeclaration {
    pub fn new(
        source: impl IntoIterator<Item = StoreAspectContractAdmission>,
    ) -> Result<Self, PhysicalWorkProfileDenial> {
        Self::from_signal_aspects(
            source
                .into_iter()
                .map(PhysicalSignalAspectDeclaration::from_contract),
        )
    }

    pub fn from_signal_aspects(
        source: impl IntoIterator<Item = PhysicalSignalAspectDeclaration>,
    ) -> Result<Self, PhysicalWorkProfileDenial> {
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
            aspects: aspects.into_boxed_slice(),
            capacity: PhysicalWorkCapacity::default(),
        })
    }

    pub const fn with_capacity(mut self, capacity: PhysicalWorkCapacity) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn identity(&self) -> PhysicalSignalProfileIdentity {
        profile_identity(&self.aspects, self.capacity)
    }

    pub const fn contract_count(&self) -> usize {
        self.aspects.len()
    }
    pub const fn capacity(&self) -> PhysicalWorkCapacity {
        self.capacity
    }

    pub(in crate::physical_runtime) fn into_parts(
        self,
    ) -> (Box<[PhysicalSignalAspectDeclaration]>, PhysicalWorkCapacity) {
        (self.aspects, self.capacity)
    }
}

impl Default for PhysicalWorkProfileDeclaration {
    fn default() -> Self {
        Self::new([]).expect("empty physical work profile is within Signal capacity")
    }
}
