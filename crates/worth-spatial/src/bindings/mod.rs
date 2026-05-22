mod authority;
mod primitive_birth;
mod primitive_birth_completeness;
mod primitive_birth_contract;
mod primitive_birth_mapping;
mod primitive_birth_rejection;
mod primitive_birth_validation;

pub use authority::{construction_birth_authority, SpatialConstructionBirthAuthority};
pub use primitive_birth::{
    plan_primitive_construction_birth, PrimitiveConstructionBirthFamily,
    PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthError,
    SpatialConstructionBirthPlan,
};
pub use primitive_birth_completeness::{
    certify_primitive_construction_birth_completeness,
    impossible_primitive_construction_birth_attachment, SpatialConstructionBirthCompletenessReport,
};
pub use primitive_birth_contract::{
    primitive_birth_contract_matches_counts, primitive_birth_contract_matches_support_planes,
    PrimitiveConstructionBirthContractCounts,
};
pub use primitive_birth_mapping::{
    build_primitive_construction_birth_mapping_report, SpatialConstructionBirthMappingKind,
    SpatialConstructionBirthMappingReport, SpatialConstructionBirthMappingRow,
};
pub use primitive_birth_rejection::{
    SpatialConstructionBirthRejectionKind, SpatialConstructionBirthRejectionRow,
};
