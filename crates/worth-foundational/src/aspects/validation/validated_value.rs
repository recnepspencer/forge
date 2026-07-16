use worth_proof::{Artifact, PhaseMarker};

use crate::aspects::identity::{AspectContractRevision, AspectIdentity};
use crate::aspects::keys::AspectKey;
use crate::aspects::structs::StructAspectValue;
use crate::values::AspectValue;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ContractValidatedAspectValue {
    key: AspectKey,
    contract_identity: AspectIdentity,
    contract_revision: AspectContractRevision,
    kind: ContractValidatedAspectValueKind,
}

impl ContractValidatedAspectValue {
    pub(crate) fn scalar(
        key: AspectKey,
        value: AspectValue,
        contract_identity: AspectIdentity,
        contract_revision: AspectContractRevision,
    ) -> Self {
        Self {
            key,
            contract_identity,
            contract_revision,
            kind: ContractValidatedAspectValueKind::Scalar(value),
        }
    }

    pub(crate) fn struct_value(
        key: AspectKey,
        value: StructAspectValue,
        contract_identity: AspectIdentity,
        contract_revision: AspectContractRevision,
    ) -> Self {
        Self {
            key,
            contract_identity,
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

    pub fn contract_identity(&self) -> AspectIdentity {
        self.contract_identity
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
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
