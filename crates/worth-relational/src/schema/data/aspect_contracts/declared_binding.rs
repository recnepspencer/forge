use serde::{Deserialize, Serialize};
use worth_foundational::facade::AspectKey;
use worth_foundational::{AspectContract, FieldKey};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclaredAspectContractBinding {
    pub binding: AspectBinding,
    pub contract: AspectContract,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AspectBinding {
    EntityField { field: FieldKey },
    RelationField { field: FieldKey },
    RelationSourceEndpoint,
    RelationTargetEndpoint,
    LifecycleTransition,
}

impl DeclaredAspectContractBinding {
    pub fn aspect_key(&self) -> AspectKey {
        self.contract.key().clone()
    }

    pub fn foundational_key(&self) -> &worth_foundational::AspectKey {
        self.contract.key()
    }
}
