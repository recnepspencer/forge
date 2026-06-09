const AUDITED_LOCAL_PHASE_FILES: [(&str, &str); 2] = [
    (
        "worth-kernel.result-surface-result",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/result_surface/result.rs"
        )),
    ),
    (
        "worth-kernel.certification-runtime-truth",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/tests/support/runtime_truth.rs"
        )),
    ),
];

const AUDITED_ADMITTED_PROTOCOL_FILES: [(&str, &str); 5] = [
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
    (
        "worth-kernel.phase-chain-admitted-scaffold",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/mod.rs"
        )),
    ),
    (
        "worth-kernel.phase-chain-admitted-scaffold-family-birth-input",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
        )),
    ),
];

const AUDITED_ADMITTED_SUBTREE_BOUNDARY_FILES: [(&str, &str); 5] = [
    (
        "worth-kernel.construction-mod",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/mod.rs"
        )),
    ),
    (
        "worth-kernel.phase-chain-admitted-scaffold",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/mod.rs"
        )),
    ),
    (
        "worth-kernel.phase-chain-admitted-scaffold-family-birth-input",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
        )),
    ),
    (
        "worth-kernel.phase-chain-admitted-scaffold-family-birth-input",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/mod.rs"
        )),
    ),
    (
        "worth-kernel.phase-chain-admitted-scaffold-family-birth-input-simplex-solid",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/src/construction/phase_chain/admitted_scaffold/family_birth_input/families/simplex_solid.rs"
        )),
    ),
];

const AUDITED_PUBLIC_CONSTRUCTION_ENTRY_FILES: [(&str, &str); 2] = [
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

const AUDITED_PUBLIC_AUTHORING_CONTRACT_FILES: [(&str, &str); 1] = [(
    "worth-kernel.public-api-construction-contract",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/certification/public_facade_contracts/contracts/public_api_construction.rs"
    )),
)];

const FORBIDDEN_LOCAL_PHASE_PATTERNS: [&str; 3] = [
    ".build_scaffold(",
    "PreparedPrimitiveConstructionExecution::from_phase_chain(",
    "build_canonical_primitive_construction_artifact(",
];

const FORBIDDEN_ADMITTED_PHASE_PROTOCOL_PATTERNS: [&str; 2] = [
    "AdmittedPrimitiveConstructionIntent",
    "request.clone().admit()",
];

const FORBIDDEN_PEER_BIRTH_BRIDGE_PATTERNS: [&str; 4] = [
    "mod scaffold;",
    "mod topology_handoff;",
    "phase_chain/admitted_scaffold/scaffold.rs",
    "phase_chain/admitted_scaffold/topology_handoff.rs",
];

const FORBIDDEN_PEER_PHASE_DECLARATION_PATTERNS: [&str; 5] = [
    "phase_chain/admission.rs",
    "phase_chain/scaffold_realization.rs",
    "phase_chain/common_path.rs",
    "phase_chain/execution.rs",
    "phase_chain/phase_report.rs",
];

const AUDITED_RESULT_COMMON_PATH_FILES: [(&str, &str); 1] = [(
    "worth-kernel.result-surface-result",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/construction/result_surface/result.rs"
    )),
)];

const FORBIDDEN_RESULT_COMMON_PATH_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_phase_chain_common_path(",
    "PrimitiveConstructionPhaseChainCommonPathError",
];

const FORBIDDEN_OLD_ADMITTED_RESULT_INPUT_PATTERNS: [&str; 4] = [
    "PreparedPrimitiveConstructionAdmittedCommonPath",
    "prepare_primitive_construction_admitted_common_path(",
    "PrimitiveConstructionAdmittedCommonPathError",
    "from_admitted_common_path(",
];

const FORBIDDEN_OLD_EXECUTION_PHASE_PATTERNS: [&str; 2] = [
    "phase_chain/execution.rs",
    "PreparedPrimitiveConstructionExecution::from_phase_chain(",
];

const FORBIDDEN_EXECUTION_WRAPPER_RESIDUE_PATTERNS: [&str; 4] = [
    "PreparedPrimitiveConstructionExecution",
    "PrimitiveConstructionExecutionError",
    "mod execution;",
    "phase_chain/admitted_scaffold/execution.rs",
];

const FORBIDDEN_OLD_PHASE_REPORT_PATTERNS: [&str; 2] = [
    "phase_chain/phase_report.rs",
    "PrimitiveConstructionPhaseChainReport",
];

const FORBIDDEN_KERNEL_LOCAL_TOPOLOGY_ENVELOPE_PATTERNS: [&str; 4] = [
    "PrimitiveConstructionPhaseError::TopologyQueryEnvelope",
    "TopologyConstructionQueryEnvelopeError",
    "TopologyConstructionQueryReceiptError",
    "TopologyConstructionQueryMutationSurface::ComposeGraph",
];

const FORBIDDEN_PUBLIC_QUERYLESS_ENTRY_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_result",
    "prepare_primitive_construction_outcome",
];

const FORBIDDEN_PUBLIC_AUTHORING_QUERYLESS_ENTRY_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_result(",
    "prepare_primitive_construction_outcome(",
];

const FORBIDDEN_DEAD_FAILURE_LANE_PATTERNS: [&str; 4] = [
    "PrimitiveConstructionPhaseError::SpatialBirth",
    "PrimitiveConstructionArtifactError",
    "PrimitiveConstructionResultError::Artifact",
    "PrimitiveConstructionRejectionLocality::Artifact",
];

#[test]
fn phase_five_internal_scaffold_spread_stays_quarantined() {
    let violations = AUDITED_LOCAL_PHASE_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_LOCAL_PHASE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "non-test production files reintroduced local scaffold-phase planning spread: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_intent_protocol_stays_quarantined() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_ADMITTED_PHASE_PROTOCOL_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "non-test production files reintroduced admitted intent protocol spread: {violations:?}"
    );
}

#[test]
fn phase_five_deleted_scaffold_bridge_helpers_stay_deleted() {
    let violations = AUDITED_ADMITTED_SUBTREE_BOUNDARY_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PEER_BIRTH_BRIDGE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "construction boundary files reintroduced the deleted scaffold or topology-handoff helper files instead of keeping the birth bridge on the realized lower-layer input seam: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_subtree_owns_helper_modules() {
    let violations = AUDITED_ADMITTED_SUBTREE_BOUNDARY_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PEER_PHASE_DECLARATION_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "construction root still teaches admitted helper phases as peers instead of subtree helpers: {violations:?}"
    );
}

#[test]
fn phase_five_result_surface_does_not_reintroduce_common_path_wrapper() {
    let violations = AUDITED_RESULT_COMMON_PATH_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_RESULT_COMMON_PATH_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "result surface reintroduced the deleted common-path wrapper lane: {violations:?}"
    );
}

#[test]
fn phase_five_result_assembly_boundary_no_longer_teaches_admitted_common_path() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_OLD_ADMITTED_RESULT_INPUT_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "live admitted result-assembly files reintroduced the deleted admitted common-path vocabulary: {violations:?}"
    );
}

#[test]
fn phase_five_execution_boundary_no_longer_teaches_phase_chain_entry() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_OLD_EXECUTION_PHASE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "live admitted-scaffold construction files reintroduced the deleted execution peer phase or old phase-chain entry: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_artifact_boundary_no_longer_teaches_execution_wrapper() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .chain(AUDITED_ADMITTED_SUBTREE_BOUNDARY_FILES.iter())
        .flat_map(|(label, source)| {
            FORBIDDEN_EXECUTION_WRAPPER_RESIDUE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "live admitted-scaffold/result files reintroduced the deleted execution wrapper seam: {violations:?}"
    );
}

#[test]
fn phase_five_result_assembly_boundary_no_longer_teaches_phase_chain_report() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_OLD_PHASE_REPORT_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "live result-assembly construction files reintroduced the deleted phase-chain report lane: {violations:?}"
    );
}

#[test]
fn phase_five_admitted_scaffold_boundary_no_longer_teaches_kernel_local_topology_envelope_checks() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_KERNEL_LOCAL_TOPOLOGY_ENVELOPE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "live admitted-scaffold/result files reintroduced kernel-local topology envelope posture checking instead of relying on the topology admitted-handoff boundary: {violations:?}"
    );
}

#[test]
fn phase_five_public_construction_entry_no_longer_teaches_queryless_happy_path_helpers() {
    let violations = AUDITED_PUBLIC_CONSTRUCTION_ENTRY_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PUBLIC_QUERYLESS_ENTRY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "public facade files reintroduced the queryless construction happy-path entry helpers instead of keeping query-backed authoring as the sanctioned front door: {violations:?}"
    );
}

#[test]
fn phase_five_public_authoring_session_no_longer_teaches_queryless_entry_bypass() {
    let violations = AUDITED_PUBLIC_AUTHORING_CONTRACT_FILES
        .iter()
        .flat_map(|(label, source)| {
            FORBIDDEN_PUBLIC_AUTHORING_QUERYLESS_ENTRY_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "public construction contract surfaces still call direct local preparation helpers instead of proving the query-backed declaration-authoring entry lane: {violations:?}"
    );
}

#[test]
fn phase_five_dead_local_failure_lanes_stay_deleted() {
    let violations = AUDITED_ADMITTED_PROTOCOL_FILES
        .iter()
        .chain(AUDITED_RESULT_COMMON_PATH_FILES.iter())
        .flat_map(|(label, source)| {
            FORBIDDEN_DEAD_FAILURE_LANE_PATTERNS
                .iter()
                .filter(move |pattern| source.contains(**pattern))
                .map(move |pattern| format!("{label}:{pattern}"))
        })
        .collect::<Vec<_>>();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "live construction production files reintroduced deleted local spatial-birth or artifact failure lanes instead of using the surviving admitted-handoff failure story: {violations:?}"
    );
}
