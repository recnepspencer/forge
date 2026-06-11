const AUDITED_GEOMETRY_RUNTIME_FILES: [(&str, &str); 7] = [
    (
        "worth-spatial.binding-authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/bindings/query_native_binding_authoring.rs"
        )),
    ),
    (
        "worth-spatial.anchor-binding-authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/bindings/query_native_anchor_binding_authoring.rs"
        )),
    ),
    (
        "worth-spatial.rebinding-authoring",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/bindings/query_native_rebinding_authoring.rs"
        )),
    ),
    (
        "worth-spatial.rebinding-projection",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/bindings/query_native_rebinding_projection.rs"
        )),
    ),
    (
        "worth-spatial.public-bindings-facade",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../worth-spatial/src/facade/bindings.rs"
        )),
    ),
    (
        "worth-kernel.lib-root",
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")),
    ),
    (
        "worth-kernel.public-api-construction-contract",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/certification/public_facade_contracts/contracts/public_api_construction.rs"
        )),
    ),
];

const FORBIDDEN_GEOMETRY_SHADOW_RUNTIME_PATTERNS: [&str; 8] = [
    "fn admit_intent(",
    "SpatialAdmittedPrimitiveBinding",
    "AdmittedRebindingDecision",
    "primitive_rebinding_prior_binding_fact_from_binding",
    "primitive_rebinding_candidate_fact_from_binding",
    "certification_support",
    "PrimitiveConstructionAuthoringSession",
    "prepare_primitive_construction_result(",
];

#[test]
fn phase_nine_deleted_geometry_entrypoints_and_shadow_runtime_carriers_stay_gone() {
    let violations = AUDITED_GEOMETRY_RUNTIME_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_GEOMETRY_SHADOW_RUNTIME_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "phase-nine deletion proof failed because legacy geometry entrypoints or shadow runtime carriers reappeared: {violations:?}"
    );

    let deleted_kernel_workflow_boundary_paths = [
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/binding/workflow_boundary/canonical_artifacts.rs"
        ),
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/binding/workflow_boundary/summaries.rs"
        ),
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/binding/workflow_boundary/mod.rs"
        ),
    ];

    let resurrected = deleted_kernel_workflow_boundary_paths
        .iter()
        .filter(|path| std::path::Path::new(path).exists())
        .copied()
        .collect::<Vec<_>>();

    assert_eq!(
        resurrected,
        Vec::<&str>::new(),
        "phase-nine deletion proof failed because workflow-boundary kernel shelves reappeared under src/binding: {resurrected:?}"
    );
}
