const AUDITED_KERNEL_PHASE_EIGHT_FILES: [(&str, &str); 3] = [
    (
        "worth-kernel.construction-authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/authoring.rs"
        )),
    ),
    (
        "worth-kernel.construction-mod",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/mod.rs"
        )),
    ),
    (
        "worth-kernel.lib-root",
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")),
    ),
];

const FORBIDDEN_KERNEL_LOCAL_RUNTIME_CARRIER_PATTERNS: [&str; 6] = [
    "PrimitiveConstructionAuthorityChainReport",
    "authority_chain_report(",
    "PrimitiveConstructionQueryGapRow",
    "family_coverage",
    "rejection_locality",
    "runtime_proof",
];

const FORBIDDEN_KERNEL_OWNED_QUERY_RUNTIME_SEAMS: [&str; 12] = [
    "PrimitiveConstructionQueryDomain",
    "PrimitiveConstructionDeclarationFamily",
    "HistoricalGeometryInspectionDeclarationFamily",
    "BranchLocalGeometryInspectionDeclarationFamily",
    "GeometryReplayParityDeclarationFamily",
    "PrimitiveConstructionAuthoringEntry",
    "primitive_construction_contribution_workflow",
    "policy_pressure",
    "policy_profile",
    "arbitration_policy",
    "query_native_policy_",
    "query_native_arbitration_",
];

const FORBIDDEN_NEW_CAPABILITY_SHORTCUT_PATTERNS: [&str; 8] = [
    "pub use crate::construction::certification::",
    "diagnostics",
    "helper",
    "runtime_proof",
    "family_coverage",
    "rejection_locality",
    "representative_evidence",
    "closeout",
];

const FORBIDDEN_DECLARATION_AUTHORING_SESSION_PATTERNS: [&str; 5] = [
    "PrimitiveConstructionAuthoringSession",
    "primitive_construction_authoring(",
    "query_front_door(",
    "workspace_name(",
    "admit_query_family(",
];

const FORBIDDEN_PUBLIC_CONSTRUCTION_QUERY_RUNTIME_EXPORT_PATTERNS: [&str; 10] = [
    "author_primitive_construction_declaration",
    "PrimitiveConstructionAuthoringEntry",
    "primitive_construction_contribution_workflow",
    "PrimitiveConstructionDeclarationFamily",
    "PrimitiveConstructionQueryDomain",
    "PrimitiveConstructionQueryWorld",
    "historical_geometry_inspection_entry_from_construction_declaration",
    "branch_local_geometry_inspection_entry_from_construction_declaration",
    "geometry_replay_parity_entry_from_retained_facts",
    "GeometryReplayRetainedSourceFact",
];

#[test]
fn phase_eight_surviving_kernel_production_path_needs_query_native_family_workflow() {
    let violations = AUDITED_KERNEL_PHASE_EIGHT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_KERNEL_LOCAL_RUNTIME_CARRIER_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "kernel production path still teaches local runtime carriers after the Query-native collapse: {violations:?}"
    );
}

#[test]
fn phase_eight_kernel_construction_topology_no_longer_teaches_a_hidden_query_runtime_forest() {
    let violations = AUDITED_KERNEL_PHASE_EIGHT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_KERNEL_OWNED_QUERY_RUNTIME_SEAMS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .chain(
            AUDITED_KERNEL_PHASE_EIGHT_FILES
                .iter()
                .flat_map(|(label, source)| {
                    FORBIDDEN_NEW_CAPABILITY_SHORTCUT_PATTERNS
                        .iter()
                        .filter(move |pattern| source.contains(**pattern))
                        .map(move |pattern| format!("{label}:{pattern}"))
                }),
        )
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "kernel construction topology still teaches a hidden query-runtime forest or helper bucket instead of pure DX/certification boundaries: {violations:?}"
    );
}

#[test]
fn phase_eight_construction_authoring_no_longer_teaches_session_shaped_query_runtime_shell() {
    let violations = AUDITED_KERNEL_PHASE_EIGHT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_DECLARATION_AUTHORING_SESSION_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "kernel construction production topology reintroduced the deleted session-shaped query front door instead of the direct declaration-authoring seam: {violations:?}"
    );
}

#[test]
fn phase_eight_public_construction_facade_no_longer_teaches_kernel_owned_query_runtime_lane() {
    let facade_source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs"));
    let violations = FORBIDDEN_PUBLIC_CONSTRUCTION_QUERY_RUNTIME_EXPORT_PATTERNS
        .iter()
        .filter(|pattern| facade_source.contains(**pattern))
        .map(|pattern| format!("worth-kernel.lib-root:{pattern}"))
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "kernel public construction facade still teaches a deleted kernel-owned construction query-runtime lane: {violations:?}"
    );
}
