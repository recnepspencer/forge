use forge_foundational::facade::{
    AuthoritativePatchConstructionDenial, AuthoritativeStateAdmissionDenial, BoundarySourceLocator,
    ContractValidationDenial,
};
use serde::{Deserialize, Serialize};

use crate::identity::data::KindId;
use crate::transactions::data::AspectFieldPatchTarget;

use super::AspectFieldTargetRejectionReason;

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
    UnsupportedAspectFieldTarget {
        target: AspectFieldPatchTarget,
        reason: AspectFieldTargetRejectionReason,
    },
    StructValueConstructionDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
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
