use crate::aspects::{AspectContractRevision, AspectIdentity, AspectKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AspectContractFrontDoor;

impl AspectContractFrontDoor {
    pub fn for_key(self, key: AspectKey) -> AspectContractIdentityStep {
        AspectContractIdentityStep { key }
    }
}

pub struct AspectContractIdentityStep {
    pub(super) key: AspectKey,
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
    pub(super) key: AspectKey,
    pub(super) identity: AspectIdentity,
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
    pub(super) key: AspectKey,
    pub(super) identity: AspectIdentity,
    pub(super) revision: AspectContractRevision,
}
