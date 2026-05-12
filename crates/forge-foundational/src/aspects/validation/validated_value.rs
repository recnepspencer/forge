use forge_proof::{Artifact, PhaseMarker};

use crate::aspects::identity::AspectContractRevision;
use crate::aspects::keys::AspectKey;
use crate::aspects::structs::StructAspectValue;
use crate::values::AspectValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractValidatedAspectValue {
    Scalar {
        key: AspectKey,
        value: AspectValue,
        contract_revision: AspectContractRevision,
    },
    Struct {
        key: AspectKey,
        value: StructAspectValue,
        contract_revision: AspectContractRevision,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractValidated;

impl PhaseMarker for ContractValidated {}

pub type ContractValidatedAspectArtifact =
    Artifact<ContractValidated, ContractValidatedAspectValue>;
