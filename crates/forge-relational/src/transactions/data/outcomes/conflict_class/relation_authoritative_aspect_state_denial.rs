use forge_foundational::facade::{
    AuthoritativePatchApplicationDenial, AuthoritativePatchConstructionDenial,
    AuthoritativeStateAdmissionDenial, BoundarySourceLocator, ContractValidationDenial,
};
use serde::{Deserialize, Serialize};

use super::AspectFieldTargetRejectionReason;
use crate::transactions::data::AspectFieldPatchTarget;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationAuthoritativeAspectStateDenial {
    MissingAspectPlan {
        kind_id: crate::identity::data::KindId,
    },
    ContractValidationDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_boundary_source_locator")]
        source_locator: BoundarySourceLocator,
        denial: ContractValidationDenial,
    },
    PatchConstructionDenied {
        denial: AuthoritativePatchConstructionDenial,
    },
    PatchApplicationDenied {
        denial: AuthoritativePatchApplicationDenial,
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
