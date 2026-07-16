use crate::aspects::{
    AbsenceLaw, AspectContractRevision, AspectIdentity, AspectKey,
    AuthoritativePatchConstructionDenial, AuthoritativeStateAdmissionDenial,
    ContractValidationDenial,
};
use serde::{Deserialize, Serialize};

use super::PortablePatchReadmissionPurpose;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortableAspectReadmissionDenial {
    MissingContract(AspectKey),
    ContractIdentityMismatch {
        key: AspectKey,
        expected: AspectIdentity,
        found: AspectIdentity,
    },
    ContractRevisionMismatch {
        key: AspectKey,
        expected: AspectContractRevision,
        found: AspectContractRevision,
    },
    DuplicateAspectOperation(AspectKey),
    WholeClearDenied {
        key: AspectKey,
        purpose: PortablePatchReadmissionPurpose,
        absence: AbsenceLaw,
    },
    FieldPatchDeniedForCreation(AspectKey),
    ValueValidation {
        key: AspectKey,
        denial: ContractValidationDenial,
    },
    PatchConstruction(AuthoritativePatchConstructionDenial),
    StateAdmission(AuthoritativeStateAdmissionDenial),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PortableAspectExportDenial {
    MissingContract(AspectKey),
    ContractIdentityDrift {
        key: AspectKey,
        expected: AspectIdentity,
        found: AspectIdentity,
    },
    ContractRevisionDrift {
        key: AspectKey,
        expected: AspectContractRevision,
        found: AspectContractRevision,
    },
}
