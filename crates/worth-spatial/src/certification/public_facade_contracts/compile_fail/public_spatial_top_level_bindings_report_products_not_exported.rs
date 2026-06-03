use worth_spatial::facade::{
    build_primitive_construction_birth_mapping_report,
    certify_primitive_construction_birth_completeness, construction_birth_authority,
    impossible_primitive_construction_birth_attachment, PrimitiveConstructionBirthContractCounts,
    SpatialConstructionBirthAuthority, SpatialConstructionBirthCompletenessReport,
    SpatialConstructionBirthMappingReport, SpatialConstructionBirthRejectionRow,
};

fn main() {
    let _ = build_primitive_construction_birth_mapping_report;
    let _ = certify_primitive_construction_birth_completeness;
    let _ = construction_birth_authority;
    let _ = impossible_primitive_construction_birth_attachment;
    let _ = std::mem::size_of::<PrimitiveConstructionBirthContractCounts>();
    let _ = std::mem::size_of::<SpatialConstructionBirthAuthority>();
    let _ = std::mem::size_of::<SpatialConstructionBirthCompletenessReport>();
    let _ = std::mem::size_of::<SpatialConstructionBirthMappingReport>();
    let _ = std::mem::size_of::<SpatialConstructionBirthRejectionRow>();
}
