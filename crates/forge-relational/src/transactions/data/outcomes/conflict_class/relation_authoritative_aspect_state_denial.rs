use forge_foundational::facade::{
    AspectKey, AuthoritativePatchApplicationDenial, AuthoritativePatchConstructionDenial,
    AuthoritativeStateAdmissionDenial, BoundarySourceLocator, ContractValidationDenial, FieldKey,
};
use serde::{Deserialize, Serialize};

use super::authoritative_aspect_source_locator::authoritative_aspect_field_source_locator;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationAuthoritativeAspectStateDenial {
    MissingAspectPlan {
        kind_id: crate::identity::data::KindId,
    },
    ContractValidationDenied {
        #[serde(
            with = "super::authoritative_aspect_source_locator::serde_boundary_source_locator"
        )]
        source_locator: BoundarySourceLocator,
        denial: ContractValidationDenial,
    },
    PatchConstructionDenied {
        denial: AuthoritativePatchConstructionDenial,
    },
    PatchApplicationDenied {
        denial: AuthoritativePatchApplicationDenial,
    },
    UnsupportedAspectValue {
        #[serde(
            with = "super::authoritative_aspect_source_locator::serde_boundary_source_locator"
        )]
        source_locator: BoundarySourceLocator,
        value_family: String,
    },
    StructValueConstructionDenied {
        #[serde(
            with = "super::authoritative_aspect_source_locator::serde_boundary_source_locator"
        )]
        source_locator: BoundarySourceLocator,
    },
    StructBindingShapeMismatch {
        #[serde(
            with = "super::authoritative_aspect_source_locator::serde_boundary_source_locator"
        )]
        source_locator: BoundarySourceLocator,
        shape: String,
    },
    StructContractValidationDenied {
        #[serde(
            with = "super::authoritative_aspect_source_locator::serde_boundary_source_locator"
        )]
        source_locator: BoundarySourceLocator,
        denial: ContractValidationDenial,
    },
    StateAdmissionDenied {
        denial: AuthoritativeStateAdmissionDenial,
    },
}

impl RelationAuthoritativeAspectStateDenial {
    pub(crate) fn contract_validation_denied(
        aspect_key: AspectKey,
        field: FieldKey,
        denial: ContractValidationDenial,
    ) -> Self {
        Self::ContractValidationDenied {
            source_locator: authoritative_aspect_field_source_locator(aspect_key, field),
            denial,
        }
    }
}
