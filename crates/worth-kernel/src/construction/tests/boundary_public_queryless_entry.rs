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

const FORBIDDEN_PUBLIC_QUERYLESS_ENTRY_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_result",
    "prepare_primitive_construction_outcome",
];

const FORBIDDEN_PUBLIC_AUTHORING_QUERYLESS_ENTRY_PATTERNS: [&str; 2] = [
    "prepare_primitive_construction_result(",
    "prepare_primitive_construction_outcome(",
];

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
