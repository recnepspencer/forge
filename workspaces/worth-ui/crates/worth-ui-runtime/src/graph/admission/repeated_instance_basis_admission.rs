use crate::declaration::UiDeclarationIdentity;
use crate::graph::UiRepeatedInstanceBasisDenial;
use crate::graph::identity::{UiRuntimeDataInstanceKey, UiRuntimeDataInstanceKeyToken};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiRuntimeInstanceBasisAdmission {
    declaration_identity: UiDeclarationIdentity,
    runtime_data_key: UiRuntimeDataInstanceKey,
}

impl UiRuntimeInstanceBasisAdmission {
    pub fn admit_runtime_data_keyed(
        declaration_identity: &UiDeclarationIdentity,
        runtime_data_key: UiRuntimeDataInstanceKeyToken,
    ) -> Result<Self, UiRepeatedInstanceBasisDenial> {
        Ok(Self {
            declaration_identity: declaration_identity.clone(),
            runtime_data_key: UiRuntimeDataInstanceKey::admit(runtime_data_key)?,
        })
    }

    pub fn declaration_identity(&self) -> &UiDeclarationIdentity {
        &self.declaration_identity
    }

    pub(crate) fn runtime_data_key(&self) -> &UiRuntimeDataInstanceKey {
        &self.runtime_data_key
    }
}
