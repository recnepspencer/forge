use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, AuthoritativePatchApplicationDenial,
    AuthoritativePatchConstructionDenial, ContractValidationDenial,
};
use serde::{Deserialize, Serialize};

use crate::identity::data::KindId;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityFieldAspectPatchDenial {
    MissingAspectPlan {
        kind_id: KindId,
    },
    UndeclaredEntityAspectTarget {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
    },
    EntityAspectFieldPathMismatch {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
    },
    UnsupportedNestedEntityFieldPath {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
    },
    ContractValidationDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
        denial: ContractValidationDenial,
    },
    PatchConstructionDenied {
        #[serde(with = "crate::aspect_wire::serde_optional_canonical_aspect_field_locator")]
        field_locator: Option<AspectFieldLocator>,
        denial: AuthoritativePatchConstructionDenial,
    },
    FieldPatchApplicationDenied {
        #[serde(with = "crate::aspect_wire::serde_canonical_aspect_field_locator")]
        field_locator: AspectFieldLocator,
        denial: AuthoritativePatchApplicationDenial,
    },
    WholeAspectPatchApplicationDenied {
        aspect_key: AspectKey,
        denial: AuthoritativePatchApplicationDenial,
    },
    MissingAuthoritativeAspectState {
        aspect_key: Option<AspectKey>,
    },
    EmptyAuthoritativePatchPlan,
}
