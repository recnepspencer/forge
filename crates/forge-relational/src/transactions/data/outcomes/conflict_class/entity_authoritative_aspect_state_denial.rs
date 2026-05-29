use forge_foundational::facade::{
    AuthoritativePatchConstructionDenial, AuthoritativeStateAdmissionDenial, BoundarySourceLocator,
    ContractValidationDenial,
};
use serde::{Deserialize, Serialize};

use crate::identity::data::KindId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityAuthoritativeAspectStateDenial {
    MissingAspectPlan {
        kind_id: KindId,
    },
    ContractValidationDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
        denial: ContractValidationDenial,
    },
    PatchConstructionDenied {
        denial: AuthoritativePatchConstructionDenial,
    },
    UnsupportedAspectValue {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
        value_family: String,
    },
    StructValueConstructionDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
    },
    StructBindingShapeMismatch {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
        shape: String,
    },
    StructContractValidationDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
        denial: ContractValidationDenial,
    },
    StateAdmissionDenied {
        denial: AuthoritativeStateAdmissionDenial,
    },
}
