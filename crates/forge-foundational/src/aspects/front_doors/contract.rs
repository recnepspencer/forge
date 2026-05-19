use super::super::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEquivalenceBasis, AspectIdentity,
    AspectKey, AspectMaskContract, AspectShape, OpaqueAspectType, ReferenceAspectType,
    StructAspectShape,
};
use crate::aspects::evolution::AspectEvolutionPolicy;
use crate::values::ScalarAspectType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectContractFrontDoor;

impl AspectContractFrontDoor {
    pub fn for_key(self, key: AspectKey) -> AspectContractIdentityStep {
        AspectContractIdentityStep { key }
    }
}

pub struct AspectContractIdentityStep {
    key: AspectKey,
}

impl AspectContractIdentityStep {
    pub fn identified_by(self, identity: AspectIdentity) -> AspectContractRevisionStep {
        AspectContractRevisionStep {
            key: self.key,
            identity,
        }
    }
}

pub struct AspectContractRevisionStep {
    key: AspectKey,
    identity: AspectIdentity,
}

impl AspectContractRevisionStep {
    pub fn at_revision(self, revision: AspectContractRevision) -> AspectContractShapeStep {
        AspectContractShapeStep {
            key: self.key,
            identity: self.identity,
            revision,
        }
    }
}

pub struct AspectContractShapeStep {
    key: AspectKey,
    identity: AspectIdentity,
    revision: AspectContractRevision,
}

impl AspectContractShapeStep {
    pub fn scalar(self, scalar: ScalarAspectType) -> AspectContract {
        AspectContract::scalar(self.key, self.identity, self.revision, scalar)
    }

    pub fn scalar_with(
        self,
        scalar: ScalarAspectType,
        masks: AspectMaskContract,
        absence: AbsenceLaw,
        equivalence: AspectEquivalenceBasis,
        evolution: AspectEvolutionPolicy,
    ) -> AspectContract {
        self.custom(
            AspectShape::Scalar(scalar),
            masks,
            absence,
            equivalence,
            evolution,
        )
    }

    pub fn struct_aspect(self, shape: StructAspectShape) -> AspectContract {
        AspectContract::struct_aspect(self.key, self.identity, self.revision, shape)
    }

    pub fn struct_with(
        self,
        shape: StructAspectShape,
        masks: AspectMaskContract,
        absence: AbsenceLaw,
        equivalence: AspectEquivalenceBasis,
        evolution: AspectEvolutionPolicy,
    ) -> AspectContract {
        self.custom(
            AspectShape::Struct(shape),
            masks,
            absence,
            equivalence,
            evolution,
        )
    }

    pub fn reference_entity(self) -> AspectContract {
        AspectContract::reference_entity(self.key, self.identity, self.revision)
    }

    pub fn content_ref(self) -> AspectContract {
        AspectContract::content_ref(self.key, self.identity, self.revision)
    }

    pub fn opaque_token(self) -> AspectContract {
        AspectContract::opaque_token(self.key, self.identity, self.revision)
    }

    pub fn custom(
        self,
        shape: AspectShape,
        masks: AspectMaskContract,
        absence: AbsenceLaw,
        equivalence: AspectEquivalenceBasis,
        evolution: AspectEvolutionPolicy,
    ) -> AspectContract {
        AspectContract::new(
            self.key,
            self.identity,
            self.revision,
            shape,
            masks,
            absence,
            equivalence,
            evolution,
        )
    }

    pub fn opaque_with(
        self,
        opaque: OpaqueAspectType,
        masks: AspectMaskContract,
        absence: AbsenceLaw,
        equivalence: AspectEquivalenceBasis,
        evolution: AspectEvolutionPolicy,
    ) -> AspectContract {
        self.custom(
            AspectShape::Opaque(opaque),
            masks,
            absence,
            equivalence,
            evolution,
        )
    }

    pub fn reference_with(
        self,
        reference: ReferenceAspectType,
        masks: AspectMaskContract,
        absence: AbsenceLaw,
        equivalence: AspectEquivalenceBasis,
        evolution: AspectEvolutionPolicy,
    ) -> AspectContract {
        self.custom(
            AspectShape::Reference(reference),
            masks,
            absence,
            equivalence,
            evolution,
        )
    }
}
