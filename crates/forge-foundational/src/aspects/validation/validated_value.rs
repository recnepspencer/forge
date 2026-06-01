use forge_proof::{Artifact, PhaseMarker};

use crate::aspects::identity::AspectContractRevision;
use crate::aspects::keys::AspectKey;
use crate::aspects::structs::StructAspectValue;
use crate::values::AspectValue;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContractValidatedAspectValue {
    key: AspectKey,
    contract_revision: AspectContractRevision,
    kind: ContractValidatedAspectValueKind,
}

impl ContractValidatedAspectValue {
    pub(crate) fn scalar(
        key: AspectKey,
        value: AspectValue,
        contract_revision: AspectContractRevision,
    ) -> Self {
        Self {
            key,
            contract_revision,
            kind: ContractValidatedAspectValueKind::Scalar(value),
        }
    }

    pub(crate) fn struct_value(
        key: AspectKey,
        value: StructAspectValue,
        contract_revision: AspectContractRevision,
    ) -> Self {
        Self {
            key,
            contract_revision,
            kind: ContractValidatedAspectValueKind::Struct(value),
        }
    }

    pub fn key(&self) -> &AspectKey {
        &self.key
    }

    pub fn contract_revision(&self) -> AspectContractRevision {
        self.contract_revision
    }

    pub fn view(&self) -> ContractValidatedAspectValueView<'_> {
        match &self.kind {
            ContractValidatedAspectValueKind::Scalar(value) => {
                ContractValidatedAspectValueView::Scalar(value)
            }
            ContractValidatedAspectValueKind::Struct(value) => {
                ContractValidatedAspectValueView::Struct(value)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum ContractValidatedAspectValueKind {
    Scalar(AspectValue),
    Struct(StructAspectValue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContractValidatedAspectValueView<'a> {
    Scalar(&'a AspectValue),
    Struct(&'a StructAspectValue),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractValidated;

impl PhaseMarker for ContractValidated {}

pub type ContractValidatedAspectArtifact =
    Artifact<ContractValidated, ContractValidatedAspectValue>;
