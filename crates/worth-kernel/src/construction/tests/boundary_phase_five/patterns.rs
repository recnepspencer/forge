pub(super) const FORBIDDEN_PEER_SCAFFOLD_DECLARATION_PATTERNS: [&str; 4] = [
    "mod scaffold;",
    "mod scaffold_geometry;",
    "mod support;",
    "mod topology_counts;",
];

pub(super) const FORBIDDEN_RESULT_SURFACE_SCAFFOLD_DEPENDENCY_PATTERNS: [&str; 3] = [
    "PrimitiveConstructionScaffold",
    ".scaffold()",
    "crate::construction::scaffold",
];

pub(super) const FORBIDDEN_ADMITTED_SCAFFOLD_ROOT_ADMITTED_ARTIFACT_PATTERNS: [&str; 3] = [
    "struct PreparedPrimitiveConstructionAdmittedResultInput",
    "result_input.realization_report().clone()",
    "result_input.topology_query_admitted_handoff().clone()",
];

pub(super) const FORBIDDEN_ADMITTED_SCAFFOLD_WRAPPER_PATTERNS: [&str; 2] = [
    "PreparedPrimitiveConstructionAdmittedScaffold",
    "prepare_primitive_construction_admitted_scaffold(",
];

pub(super) const FORBIDDEN_TRANSIENT_ADMITTED_GEOMETRY_PATTERNS: [&str; 4] = [
    "AdmittedPrimitiveConstructionGeometry",
    "admit_primitive_construction_geometry(",
    "build_admitted_birth_input(request.family(), &intent_digest, &admitted_geometry)",
    "request.geometry().clone()",
];

pub(super) const FORBIDDEN_ADMITTED_SCAFFOLD_FAMILY_PARAMETER_PATTERNS: [&str; 3] = [
    "let family = request.family();",
    "build_wire_body_birth_input(family,",
    "family: PrimitiveConstructionFamily,\n    placement: &PrimitiveConstructionPlacement,",
];

pub(super) const FORBIDDEN_DUPLICATED_PLACEMENT_ADMISSION_PATTERNS: [&str; 2] = [
    "let placement = admit_placement(",
    "placement: &PrimitiveConstructionPlacement,",
];

pub(super) const FORBIDDEN_DUPLICATED_REALIZED_BIRTH_BRIDGE_PATTERNS: [&str; 2] = [
    "fn build_birth_input(",
    "PrimitiveConstructionBirthScaffoldInput::new_with_realization_facts(",
];

pub(super) const FORBIDDEN_OLD_BIRTH_SCAFFOLD_BRIDGE_ENTRY_PATTERNS: [&str; 2] = [
    "build_lower_layer_birth_scaffold_input(",
    "build_direct_planar_birth_scaffold_input(",
];

pub(super) const FORBIDDEN_FAMILY_HELPER_EMBEDDING_CHOREOGRAPHY_PATTERNS: [&str; 3] = [
    "apply_spatial_placement(",
    "build_direct_realization_report(",
    "RealizedPrimitiveConstructionBirth::new(",
];

pub(super) const FORBIDDEN_DELETED_BRIDGE_MODULE_PATTERNS: [&str; 4] = [
    "mod embedded_birth;",
    "mod realized_birth;",
    "phase_chain/admitted_scaffold/embedded_birth.rs",
    "phase_chain/admitted_scaffold/realized_birth.rs",
];

pub(super) const FORBIDDEN_OLD_REQUEST_ADMISSION_BUCKET_PATTERNS: [&str; 4] = [
    "mod request_admission;",
    "phase_chain/admitted_scaffold/request_admission.rs",
    "phase_chain/admitted_scaffold/admitted_family_request.rs",
    "phase_chain/admitted_scaffold/admitted_family_request/",
];

pub(super) const FORBIDDEN_QUERYLESS_RUNTIME_ENTRY_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_result(",
    "prepare_primitive_construction_outcome(",
];

pub(super) const FORBIDDEN_BROAD_BIRTH_INPUT_RETAINED_PATTERNS: [&str; 2] =
    ["PrimitiveConstructionBirthScaffoldInput", "fn birth_input("];

pub(super) const FORBIDDEN_LOCAL_DIGEST_SIDECAR_PATTERNS: [&str; 4] = [
    "request_digest: String",
    "intent_digest: String",
    "fn request_digest(",
    "fn intent_digest(",
];

pub(super) const FORBIDDEN_RESULT_SURFACE_LOCAL_DIGEST_DEPENDENCY_PATTERNS: [&str; 2] =
    [".request_digest()", ".intent_digest()"];

pub(super) const FORBIDDEN_LOCAL_BIRTH_SIDE_SIDECAR_PATTERNS: [&str; 4] = [
    "family: PrimitiveConstructionFamily",
    "scaffold_digest: String",
    "self.family",
    "&self.scaffold_digest",
];

pub(super) const FORBIDDEN_REQUEST_FAMILY_SIDECAR_PATTERNS: [&str; 3] = [
    "pub struct PrimitiveConstructionRequest {\n    family:",
    "Self {\n            family,",
    "request_digest_parts(self.family, &geometry)",
];

pub(super) const FORBIDDEN_ROOT_CATEGORY_BUCKET_PATTERNS: [&str; 2] =
    ["mod closed_solids;", "mod planar_constructions;"];

pub(super) const FORBIDDEN_BIRTH_INPUT_LOCAL_REQUEST_ADMISSION_HELPER_PATTERNS: [&str; 8] = [
    "fn admit_request_placement(",
    "fn admit_simplex_parameters(",
    "fn admit_orthotope_half_extents(",
    "fn admit_prism_parameters(",
    "fn admit_pyramid_parameters(",
    "fn admit_wire_body_edge_count(",
    "fn admit_shell_outer_loop_edge_count(",
    "fn admit_shell_hole_loop_edge_counts(",
];

pub(super) const FORBIDDEN_SUPPORT_REQUEST_ADMISSION_PATTERNS: [&str; 5] = [
    "reject_non_positive_scalar(",
    "reject_non_negative_scalar(",
    "reject_minimum_sides(",
    "decode_triplet(",
    "placement_error_reason(",
];

pub(super) const FORBIDDEN_BIRTH_INPUT_FAMILY_STRATEGY_PATTERNS: [&str; 2] = [
    "PrimitiveConstructionGeometry::",
    "match request.geometry()",
];

pub(super) const FORBIDDEN_SPLIT_FAMILY_AUTHORITY_PATTERNS: [&str; 3] = [
    "AdmittedPrimitiveConstructionFamilyRequest",
    "admit_family_realization_request(",
    "realize_admitted_family_request(",
];

pub(super) const FORBIDDEN_OLD_FAMILY_PARAMETER_BUCKET_PATTERNS: [&str; 2] = [
    "mod parameter_admission;",
    "family_birth_input/parameter_admission.rs",
];

pub(super) const FORBIDDEN_SUPPORT_BUCKET_PATTERNS: [&str; 2] =
    ["mod support;", "family_birth_input/support.rs"];

pub(super) const FORBIDDEN_FLAT_FAMILY_CASE_PATTERNS: [&str; 7] = [
    "mod simplex_solid;",
    "mod orthotope;",
    "mod regular_prism;",
    "mod regular_pyramid;",
    "mod wire_body;",
    "mod shell_with_hole;",
    "family_birth_input/simplex_solid.rs",
];

pub(super) const FORBIDDEN_RAW_FAMILY_PARAMETER_DECODING_PATTERNS: [&str; 2] =
    ["f64::from_bits(", "decode_triplet("];

pub(super) const FORBIDDEN_SPATIAL_FAMILY_BRIDGE_ERROR_MAPPING_PATTERNS: [&str; 3] = [
    "map_realization_geometry(",
    "map_support_plane(",
    "map_placement_geometry(",
];

pub(super) const FORBIDDEN_KERNEL_TOPOLOGY_HANDOFF_SEQUENCING_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_query_handoff(",
    "TopologyPrimitiveConstructionQueryHandoff",
];

pub(super) const FORBIDDEN_PLACEMENT_ADMISSION_FAMILY_PARAMETER_PATTERNS: [&str; 6] = [
    "reject_non_positive_scalar(",
    "reject_non_negative_scalar(",
    "reject_minimum_sides(",
    "decode_triplet(",
    "PrimitiveConstructionGeometry::",
    "hole_loop_edge_counts.is_empty()",
];

pub(super) fn collect_violations(
    audited_files: &[(&str, &str)],
    forbidden_patterns: &[&str],
) -> Vec<String> {
    audited_files
        .iter()
        .flat_map(|(label, source)| {
            forbidden_patterns
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>()
}
