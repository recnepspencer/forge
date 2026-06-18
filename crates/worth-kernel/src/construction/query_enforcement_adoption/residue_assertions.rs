use forge_query::facade::consumer_kit::query_test_backend_residue_audit;

const AUTHORING_RS: &str = include_str!("../authoring.rs");
const QUERY_SUPPORT_PINS_RS: &str = include_str!("../query_support_pins.rs");
const QUERY_SUPPORT_PINS_JSON: &str = include_str!("../query_support_pins.json");
const PHASE_EIGHT_MINIMIZATION_RS: &str = include_str!("../tests/phase_eight_minimization.rs");
const BOUNDARY_RS: &str = include_str!("../tests/boundary.rs");
const PHASE_FIVE_CLOSEOUT_RS: &str =
    include_str!("../certification/phase_five_boundary_closeout_tests.rs");

const QUERY_ENFORCEMENT_FOLKLORE_PATTERNS: &[(&str, &str, &str)] = &[
    (
        "authoring.rs",
        "REQUIRED_QUERY_FAMILIES",
        "support posture must be pinned through Query support-pinning contract",
    ),
    (
        "authoring.rs",
        "REPORTED_QUERY_FAMILIES",
        "support posture must be pinned through Query support-pinning contract",
    ),
    (
        "authoring.rs",
        "PrimitiveConstructionQueryGapRow",
        "gap rows are owned by Query support-pinning findings",
    ),
    (
        "authoring.rs",
        "support_pinning_contract(\"worth-kernel\")",
        "durable pin documents must load through the Query document loader",
    ),
    (
        "phase_five_boundary_closeout_tests.rs",
        "FORBIDDEN_RUNTIME_PATTERNS",
        "Query hard prohibitions must be enforced by the shipped boundary audit",
    ),
    (
        "phase_five_boundary_closeout_tests.rs",
        "query_runtime_violation_count",
        "Query hard prohibitions must be enforced by the shipped boundary audit",
    ),
];

pub(crate) fn assert_no_query_enforcement_folklore_residue() {
    let violations = query_enforcement_folklore_violations();

    assert_eq!(
        violations,
        Vec::<String>::new(),
        "worth-kernel still carries Query enforcement folklore now owned by Query kit: {violations:?}"
    );
}

pub(crate) fn query_enforcement_folklore_violation_count() -> usize {
    query_enforcement_folklore_violations().len()
}

fn query_enforcement_folklore_violations() -> Vec<String> {
    let inspected_sources = [
        ("authoring.rs", AUTHORING_RS),
        ("query_support_pins.rs", QUERY_SUPPORT_PINS_RS),
        ("query_support_pins.json", QUERY_SUPPORT_PINS_JSON),
        ("phase_eight_minimization.rs", PHASE_EIGHT_MINIMIZATION_RS),
        ("boundary.rs", BOUNDARY_RS),
        (
            "phase_five_boundary_closeout_tests.rs",
            PHASE_FIVE_CLOSEOUT_RS,
        ),
    ];
    QUERY_ENFORCEMENT_FOLKLORE_PATTERNS
        .iter()
        .filter_map(|(path, pattern, reason)| {
            inspected_sources
                .iter()
                .find(|(source_path, _)| source_path == path)
                .and_then(|(_, source)| {
                    source
                        .contains(pattern)
                        .then_some((*path, *pattern, *reason))
                })
        })
        .map(|(path, pattern, reason)| format!("{path}:{pattern}:{reason}"))
        .collect::<Vec<_>>()
}

pub(crate) fn assert_no_hand_assembled_test_backend_residue() {
    let report = query_test_backend_residue_audit("worth-kernel")
        .required_root(format!("{}/src/construction", env!("CARGO_MANIFEST_DIR")))
        .evaluate()
        .expect("worth-kernel construction residue audit evaluates");
    report.assert_clean();
}

pub(crate) fn remaining_worth_domain_hygiene_audit_labels() -> Vec<&'static str> {
    vec![
        "phase-eight kernel-minimization topology hygiene",
        "phase-five construction-boundary legacy-deletion hygiene",
        "phase-five boundary pattern inventory hygiene",
    ]
}
