const AUDITED_CONSTRUCTION_ROOT_FILES: [(&str, &str); 1] = [(
    "worth-kernel.construction-mod",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/mod.rs"
    )),
)];

const AUDITED_RESULT_SURFACE_FILES: [(&str, &str); 3] = [
    (
        "worth-kernel.result-surface-result",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/result_surface/result.rs"
        )),
    ),
    (
        "worth-kernel.result-evidence",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/result_surface/evidence.rs"
        )),
    ),
    (
        "worth-kernel.result-artifact",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/result_surface/artifact.rs"
        )),
    ),
];

const AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES: [(&str, &str); 1] = [(
    "worth-kernel.phase-chain-admitted-scaffold",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/admitted_scaffold/mod.rs"
    )),
)];

const AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES: [(&str, &str); 18] = [
    (
        "worth-kernel.admitted-scaffold-root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/mod.rs"
        )),
    ),
    (
        "worth-kernel.admitted-scaffold-birth-input",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/birth_input.rs"
        )),
    ),
    (
        "worth-kernel.admitted-result-input",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/result_input.rs"
        )),
    ),
    (
        "worth-kernel.admitted-scaffold-placement-admission",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/placement_admission.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-families-root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/mod.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-scalar-admission",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/scalar_admission.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-error-mapping",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/error_mapping.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-spatial-family-bridge",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/spatial_family_bridge.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-geometry",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/geometry.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-topology-counts",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/topology_counts.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-simplex-solid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/simplex_solid.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-birth-scaffold",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/birth_scaffold.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-orthotope",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/orthotope.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-regular-prism",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/regular_prism.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-regular-pyramid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/regular_pyramid.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-wire-body",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/wire_body.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-shell-with-hole",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/shell_with_hole.rs"
        )),
    ),
];

const AUDITED_RESULT_INPUT_FILE: [(&str, &str); 1] = [(
    "worth-kernel.admitted-result-input",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/admitted_scaffold/result_input.rs"
    )),
)];

const AUDITED_QUERY_BACKED_ENTRY_FILES: [(&str, &str); 5] = [
    (
        "worth-kernel.runtime-basis",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/runtime_basis.rs"
        )),
    ),
    (
        "worth-kernel.query-graph-composition-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/graph_composition_parity.rs"
        )),
    ),
    (
        "worth-kernel.query-inspection-parity",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/inspection_parity.rs"
        )),
    ),
    (
        "worth-kernel.query-projection-consumption-receipt",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/runtime_proof/query/projection_consumption_receipt.rs"
        )),
    ),
    (
        "worth-kernel.public-api-construction-contract",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/certification/public_facade_contracts/contracts/public_api_construction.rs"
        )),
    ),
];

const AUDITED_REQUEST_FILE: [(&str, &str); 1] = [(
    "worth-kernel.primitive-construction-request",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/request.rs"
    )),
)];

const AUDITED_BIRTH_INPUT_FILE: [(&str, &str); 1] = [(
    "worth-kernel.admitted-scaffold-birth-input",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/admitted_scaffold/birth_input.rs"
    )),
)];

const AUDITED_FAMILY_REALIZATION_SEAM_FILES: [(&str, &str); 8] = [
    (
        "worth-kernel.family-birth-input-root",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-scalar-admission",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/scalar_admission.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-orthotope",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/orthotope.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-regular-prism",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/regular_prism.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-regular-pyramid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/regular_pyramid.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-shell-with-hole",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/shell_with_hole.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-simplex-solid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/simplex_solid.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-wire-body",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/wire_body.rs"
        )),
    ),
];

const AUDITED_PLACEMENT_ADMISSION_FILE: [(&str, &str); 1] = [(
    "worth-kernel.admitted-scaffold-placement-admission",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/admitted_scaffold/placement_admission.rs"
    )),
)];

const AUDITED_FAMILY_BIRTH_INPUT_ERROR_MAPPING_FILE: [(&str, &str); 1] = [(
    "worth-kernel.family-birth-input-error-mapping",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/admitted_scaffold/family_birth_input/error_mapping.rs"
    )),
)];

const AUDITED_FAMILY_BIRTH_INPUT_SPATIAL_FAMILY_BRIDGE_FILE: [(&str, &str); 1] = [(
    "worth-kernel.family-birth-input-spatial-family-bridge",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/phase_chain/admitted_scaffold/family_birth_input/spatial_family_bridge.rs"
    )),
)];

const AUDITED_FAMILY_REALIZATION_FILES: [(&str, &str); 6] = [
    (
        "worth-kernel.family-birth-input-simplex-solid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/simplex_solid.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-orthotope",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/orthotope.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-regular-prism",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/regular_prism.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-regular-pyramid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/regular_pyramid.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-wire-body",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/wire_body.rs"
        )),
    ),
    (
        "worth-kernel.family-birth-input-shell-with-hole",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/shell_with_hole.rs"
        )),
    ),
];

const FORBIDDEN_PEER_SCAFFOLD_DECLARATION_PATTERNS: [&str; 4] = [
    "mod scaffold;",
    "mod scaffold_geometry;",
    "mod support;",
    "mod topology_counts;",
];

const FORBIDDEN_RESULT_SURFACE_SCAFFOLD_DEPENDENCY_PATTERNS: [&str; 3] = [
    "PrimitiveConstructionScaffold",
    ".scaffold()",
    "crate::construction::scaffold",
];

const FORBIDDEN_ADMITTED_SCAFFOLD_ROOT_RESULT_INPUT_PATTERNS: [&str; 3] = [
    "struct PreparedPrimitiveConstructionAdmittedResultInput",
    "result_input.realization_report().clone()",
    "result_input.topology_query_admitted_handoff().clone()",
];

const FORBIDDEN_ADMITTED_SCAFFOLD_WRAPPER_PATTERNS: [&str; 2] = [
    "PreparedPrimitiveConstructionAdmittedScaffold",
    "prepare_primitive_construction_admitted_scaffold(",
];

const FORBIDDEN_TRANSIENT_ADMITTED_GEOMETRY_PATTERNS: [&str; 4] = [
    "AdmittedPrimitiveConstructionGeometry",
    "admit_primitive_construction_geometry(",
    "build_admitted_birth_input(request.family(), &intent_digest, &admitted_geometry)",
    "request.geometry().clone()",
];

const FORBIDDEN_ADMITTED_SCAFFOLD_FAMILY_PARAMETER_PATTERNS: [&str; 3] = [
    "let family = request.family();",
    "build_wire_body_birth_input(family,",
    "family: PrimitiveConstructionFamily,\n    placement: &PrimitiveConstructionPlacement,",
];

const FORBIDDEN_DUPLICATED_PLACEMENT_ADMISSION_PATTERNS: [&str; 2] = [
    "let placement = admit_placement(",
    "placement: &PrimitiveConstructionPlacement,",
];

const FORBIDDEN_DUPLICATED_REALIZED_BIRTH_BRIDGE_PATTERNS: [&str; 2] = [
    "fn build_birth_input(",
    "PrimitiveConstructionBirthScaffoldInput::new_with_realization(",
];

const FORBIDDEN_OLD_BIRTH_SCAFFOLD_BRIDGE_ENTRY_PATTERNS: [&str; 2] = [
    "build_lower_layer_birth_scaffold_input(",
    "build_direct_planar_birth_scaffold_input(",
];

const FORBIDDEN_FAMILY_HELPER_EMBEDDING_CHOREOGRAPHY_PATTERNS: [&str; 3] = [
    "apply_spatial_placement(",
    "build_direct_realization_report(",
    "RealizedPrimitiveConstructionBirth::new(",
];

const FORBIDDEN_DELETED_BRIDGE_MODULE_PATTERNS: [&str; 4] = [
    "mod embedded_birth;",
    "mod realized_birth;",
    "phase_chain/admitted_scaffold/embedded_birth.rs",
    "phase_chain/admitted_scaffold/realized_birth.rs",
];

const FORBIDDEN_OLD_REQUEST_ADMISSION_BUCKET_PATTERNS: [&str; 4] = [
    "mod request_admission;",
    "phase_chain/admitted_scaffold/request_admission.rs",
    "phase_chain/admitted_scaffold/admitted_family_request.rs",
    "phase_chain/admitted_scaffold/admitted_family_request/",
];

const FORBIDDEN_QUERYLESS_RUNTIME_ENTRY_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_result(",
    "prepare_primitive_construction_outcome(",
];

const FORBIDDEN_BROAD_BIRTH_INPUT_RETAINED_PATTERNS: [&str; 2] =
    ["PrimitiveConstructionBirthScaffoldInput", "fn birth_input("];

const FORBIDDEN_LOCAL_DIGEST_SIDECAR_PATTERNS: [&str; 4] = [
    "request_digest: String",
    "intent_digest: String",
    "fn request_digest(",
    "fn intent_digest(",
];

const FORBIDDEN_RESULT_SURFACE_LOCAL_DIGEST_DEPENDENCY_PATTERNS: [&str; 2] =
    [".request_digest()", ".intent_digest()"];

const FORBIDDEN_LOCAL_BIRTH_SIDE_SIDECAR_PATTERNS: [&str; 4] = [
    "family: PrimitiveConstructionFamily",
    "scaffold_digest: String",
    "self.family",
    "&self.scaffold_digest",
];

const FORBIDDEN_REQUEST_FAMILY_SIDECAR_PATTERNS: [&str; 3] = [
    "pub struct PrimitiveConstructionRequest {\n    family:",
    "Self {\n            family,",
    "request_digest_parts(self.family, &geometry)",
];

const FORBIDDEN_ROOT_CATEGORY_BUCKET_PATTERNS: [&str; 2] =
    ["mod closed_solids;", "mod planar_constructions;"];

const FORBIDDEN_BIRTH_INPUT_LOCAL_REQUEST_ADMISSION_HELPER_PATTERNS: [&str; 8] = [
    "fn admit_request_placement(",
    "fn admit_simplex_parameters(",
    "fn admit_orthotope_half_extents(",
    "fn admit_prism_parameters(",
    "fn admit_pyramid_parameters(",
    "fn admit_wire_body_edge_count(",
    "fn admit_shell_outer_loop_edge_count(",
    "fn admit_shell_hole_loop_edge_counts(",
];

const FORBIDDEN_SUPPORT_REQUEST_ADMISSION_PATTERNS: [&str; 5] = [
    "reject_non_positive_scalar(",
    "reject_non_negative_scalar(",
    "reject_minimum_sides(",
    "decode_triplet(",
    "placement_error_reason(",
];

const FORBIDDEN_BIRTH_INPUT_FAMILY_STRATEGY_PATTERNS: [&str; 2] = [
    "PrimitiveConstructionGeometry::",
    "match request.geometry()",
];

const FORBIDDEN_SPLIT_FAMILY_AUTHORITY_PATTERNS: [&str; 3] = [
    "AdmittedPrimitiveConstructionFamilyRequest",
    "admit_family_realization_request(",
    "realize_admitted_family_request(",
];

const FORBIDDEN_OLD_FAMILY_PARAMETER_BUCKET_PATTERNS: [&str; 2] = [
    "mod parameter_admission;",
    "family_birth_input/parameter_admission.rs",
];

const FORBIDDEN_SUPPORT_BUCKET_PATTERNS: [&str; 2] =
    ["mod support;", "family_birth_input/support.rs"];

const FORBIDDEN_FLAT_FAMILY_CASE_PATTERNS: [&str; 7] = [
    "mod simplex_solid;",
    "mod orthotope;",
    "mod regular_prism;",
    "mod regular_pyramid;",
    "mod wire_body;",
    "mod shell_with_hole;",
    "family_birth_input/simplex_solid.rs",
];

const FORBIDDEN_RAW_FAMILY_PARAMETER_DECODING_PATTERNS: [&str; 2] =
    ["f64::from_bits(", "decode_triplet("];

const FORBIDDEN_SPATIAL_FAMILY_BRIDGE_ERROR_MAPPING_PATTERNS: [&str; 3] = [
    "map_realization_geometry(",
    "map_support_plane(",
    "map_placement_geometry(",
];

const FORBIDDEN_KERNEL_TOPOLOGY_HANDOFF_SEQUENCING_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_query_handoff(",
    "TopologyPrimitiveConstructionQueryHandoff",
];

const FORBIDDEN_PLACEMENT_ADMISSION_FAMILY_PARAMETER_PATTERNS: [&str; 6] = [
    "reject_non_positive_scalar(",
    "reject_non_negative_scalar(",
    "reject_minimum_sides(",
    "decode_triplet(",
    "PrimitiveConstructionGeometry::",
    "hole_loop_edge_counts.is_empty()",
];

#[test]
fn phase_five_admitted_scaffold_subtree_owns_scaffold_only_modules() {
    let violations = AUDITED_CONSTRUCTION_ROOT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PEER_SCAFFOLD_DECLARATION_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "construction root still teaches scaffold-only modules as peer phases instead of admitted-scaffold subtree details: {violations:?}"
    );
}

#[test]
fn phase_five_result_surfaces_no_longer_depend_on_broad_scaffold_type() {
    let violations = AUDITED_RESULT_SURFACE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_RESULT_SURFACE_SCAFFOLD_DEPENDENCY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "result/evidence/artifact production files still depend on the broad scaffold type instead of the narrowed admitted-result input seam: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_inlines_result_input_cloning() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_ADMITTED_SCAFFOLD_ROOT_RESULT_INPUT_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still owns the result-input struct or re-clones result-input truth instead of delegating that phase boundary honestly: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_teaches_transient_wrapper_phase() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_ADMITTED_SCAFFOLD_WRAPPER_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still teaches a transient admitted-scaffold wrapper phase instead of producing admitted result input as its one honest output: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_no_longer_teaches_transient_admitted_geometry_phase() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_TRANSIENT_ADMITTED_GEOMETRY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold subtree still teaches a transient admitted-geometry phase instead of one direct request-to-birth-input bridge: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_no_longer_teaches_deleted_split_bridge_modules() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_DELETED_BRIDGE_MODULE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold subtree still teaches the deleted embedded-birth or realized-birth split instead of one explicit birth-scaffold bridge seam: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_no_longer_teaches_old_mixed_request_buckets() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_OLD_REQUEST_ADMISSION_BUCKET_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold subtree still teaches the deleted mixed request buckets or the removed admitted-family-request subtree instead of one placement seam plus one family-owned realization lane: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_birth_input_lane_no_longer_passes_parallel_family_parameter() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_ADMITTED_SCAFFOLD_FAMILY_PARAMETER_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold birth-input helpers still pass a parallel family parameter instead of deriving family from the geometry-specific builder role itself: {violations:?}"
    );
}

#[test]
fn phase_five_placement_admission_seam_no_longer_owns_family_parameter_admission() {
    let violations = AUDITED_PLACEMENT_ADMISSION_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PLACEMENT_ADMISSION_FAMILY_PARAMETER_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "placement-admission seam still owns family parameter admission instead of leaving that work to the admitted-family-request subtree: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_family_helpers_no_longer_own_request_level_placement_admission() {
    let violations = AUDITED_FAMILY_REALIZATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_DUPLICATED_PLACEMENT_ADMISSION_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold family helpers still own request-level placement admission instead of receiving one shared admitted placement from the dispatcher: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_sibling_helpers_no_longer_duplicate_realized_birth_bridge() {
    let violations = AUDITED_FAMILY_REALIZATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_DUPLICATED_REALIZED_BIRTH_BRIDGE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold sibling helpers still duplicate the shared realized-birth bridge instead of lowering through one explicit module: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_subtree_no_longer_teaches_dual_bridge_entry_protocol() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_OLD_BIRTH_SCAFFOLD_BRIDGE_ENTRY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input subtree still teaches the deleted dual bridge entry protocol instead of one explicit family birth-scaffold plan lane: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_family_helpers_no_longer_own_embedding_choreography() {
    let violations = AUDITED_FAMILY_REALIZATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_FAMILY_HELPER_EMBEDDING_CHOREOGRAPHY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold family helpers still own embedding choreography instead of lowering through one shared embedding seam: {violations:?}"
    );
}

#[test]
fn phase_five_query_backed_entry_surfaces_no_longer_call_direct_local_preparation() {
    let violations = AUDITED_QUERY_BACKED_ENTRY_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_QUERYLESS_RUNTIME_ENTRY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "query-backed runtime/public construction entry surfaces still call direct local preparation helpers instead of crossing the authoring-session query front door: {violations:?}"
    );
}

#[test]
fn phase_five_result_input_no_longer_retains_broad_birth_input_bag() {
    let violations = AUDITED_RESULT_INPUT_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_BROAD_BIRTH_INPUT_RETAINED_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted result-input seam still retains the full lower-layer birth-input bag instead of only the birth-side facts the post-handoff kernel result lane genuinely needs: {violations:?}"
    );
}

#[test]
fn phase_five_result_input_no_longer_retains_local_request_or_intent_digest_sidecars() {
    let violations = AUDITED_RESULT_INPUT_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_LOCAL_DIGEST_SIDECAR_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted result-input seam still retains local request or intent digest sidecars after topology admitted handoff instead of keeping only the remaining post-handoff birth witness facts: {violations:?}"
    );
}

#[test]
fn phase_five_result_input_no_longer_retains_local_family_or_scaffold_sidecars() {
    let violations = AUDITED_RESULT_INPUT_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_LOCAL_BIRTH_SIDE_SIDECAR_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted result-input seam still retains local family or scaffold sidecars even though topology admitted handoff already carries birth-plan truth: {violations:?}"
    );
}

#[test]
fn phase_five_result_surfaces_no_longer_depend_on_result_input_request_or_intent_digest() {
    let violations = AUDITED_RESULT_SURFACE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_RESULT_SURFACE_LOCAL_DIGEST_DEPENDENCY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "result/evidence/artifact production files still depend on result-input request or intent digest sidecars instead of the remaining post-handoff semantic seam: {violations:?}"
    );
}

#[test]
fn phase_five_request_no_longer_retains_parallel_family_sidecar() {
    let violations = AUDITED_REQUEST_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_REQUEST_FAMILY_SIDECAR_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "primitive construction request still retains a parallel family sidecar instead of deriving family truth from geometry itself: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_teaches_category_bucket_family_helpers() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_ROOT_CATEGORY_BUCKET_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still teaches broad category-bucket family helper files instead of the explicit family-birth-input subtree: {violations:?}"
    );
}

#[test]
fn phase_five_birth_input_dispatcher_no_longer_owns_local_request_admission_helpers() {
    let violations = AUDITED_BIRTH_INPUT_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_BIRTH_INPUT_LOCAL_REQUEST_ADMISSION_HELPER_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "birth-input dispatcher still defines request-admission helpers locally instead of lowering through one explicit request-admission seam: {violations:?}"
    );
}

#[test]
fn phase_five_birth_input_dispatcher_no_longer_owns_family_strategy_match() {
    let violations = AUDITED_BIRTH_INPUT_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_BIRTH_INPUT_FAMILY_STRATEGY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "birth-input dispatcher still owns the raw geometry family strategy table instead of lowering through the admitted family-request seam: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_no_longer_teaches_support_bucket() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_SUPPORT_BUCKET_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input subtree still teaches a generic support bucket instead of explicit error-mapping and spatial-family bridge seams: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_root_no_longer_mixes_family_cases_with_shared_helpers() {
    let violations = AUDITED_FAMILY_REALIZATION_SEAM_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_FLAT_FAMILY_CASE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input boundary still teaches a flat mixed directory instead of separating per-family cases from shared helper seams: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_error_mapping_seam_no_longer_owns_request_admission_helpers() {
    let violations = AUDITED_FAMILY_BIRTH_INPUT_ERROR_MAPPING_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_SUPPORT_REQUEST_ADMISSION_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input error-mapping seam still mixes request-admission helpers with geometry lowering instead of leaving request policing on scalar or family-local admission seams: {violations:?}"
    );
}

#[test]
fn phase_five_spatial_family_bridge_seam_no_longer_mixes_error_mapping_helpers() {
    let violations = AUDITED_FAMILY_BIRTH_INPUT_SPATIAL_FAMILY_BRIDGE_FILE
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_SPATIAL_FAMILY_BRIDGE_ERROR_MAPPING_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input spatial-family bridge seam still mixes geometry error lowering instead of owning only lower-layer family translation: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_root_no_longer_owns_topology_handoff_sequencing() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_ROOT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_KERNEL_TOPOLOGY_HANDOFF_SEQUENCING_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "admitted-scaffold root still sequences topology handoff creation locally instead of crossing the topology-owned synopsis-to-admitted-handoff seam: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_boundary_no_longer_splits_family_authority() {
    let violations = AUDITED_FAMILY_REALIZATION_SEAM_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_SPLIT_FAMILY_AUTHORITY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family-local construction authority is still split across parallel admitted-request and realization seams instead of one family-owned birth-input lane: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_boundary_no_longer_teaches_old_parameter_bucket() {
    let violations = AUDITED_ADMITTED_SCAFFOLD_SUBTREE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_OLD_FAMILY_PARAMETER_BUCKET_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input subtree still teaches the deleted generic parameter bucket instead of the narrower scalar-admission seam plus family-local admitted parameter steps: {violations:?}"
    );
}

#[test]
fn phase_five_family_birth_input_helpers_no_longer_inline_raw_parameter_decoding() {
    let violations = AUDITED_FAMILY_REALIZATION_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_RAW_FAMILY_PARAMETER_DECODING_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "family birth-input helpers still inline raw parameter decoding instead of lowering through named admitted-parameter steps: {violations:?}"
    );
}
