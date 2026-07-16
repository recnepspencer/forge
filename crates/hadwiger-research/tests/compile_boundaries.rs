#[test]
#[ignore = "expensive trybuild public-boundary suite; run explicitly for closeout QA"]
fn public_compile_boundaries_hold() {
    let t = trybuild::TestCases::new();
    let scopes = requested_scopes();

    if scopes.is_empty() {
        run_all_scopes(&t);
    } else {
        for scope in scopes {
            run_scope(&t, scope.as_str());
        }
    }
}

fn requested_scopes() -> Vec<String> {
    std::env::var("HADWIGER_TRYBUILD_SCOPE")
        .ok()
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|scope| !scope.is_empty())
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn run_all_scopes(t: &trybuild::TestCases) {
    for scope in ALL_SCOPES {
        run_scope(t, scope);
    }
}

const ALL_SCOPES: &[&str] = &[
    "phase1_query_entry",
    "canonical_artifacts",
    "aspect_authority",
    "proof_claim_admission",
    "explanations",
    "discovery_loop",
    "research_graph_invariants",
    "agent_advisory",
    "research_cockpit",
    "motif_language",
    "tiling_geometry",
    "periodic_patterns",
    "conflict_graph_extraction",
    "tiling_equivalence",
    "tiling_iteration",
];

fn run_scope(t: &trybuild::TestCases, scope: &str) {
    match scope {
        "phase1_query_entry" => phase1_query_entry(t),
        "canonical_artifacts" => canonical_artifacts(t),
        "aspect_authority" => aspect_authority(t),
        "proof_claim_admission" => proof_claim_admission(t),
        "explanations" => explanations(t),
        "discovery_loop" => discovery_loop(t),
        "research_graph_invariants" => research_graph_invariants(t),
        "agent_advisory" => agent_advisory(t),
        "research_cockpit" => research_cockpit(t),
        "motif_language" => motif_language(t),
        "tiling_geometry" => tiling_geometry(t),
        "periodic_patterns" => periodic_patterns(t),
        "conflict_graph_extraction" => conflict_graph_extraction(t),
        "tiling_equivalence" => tiling_equivalence(t),
        "tiling_iteration" => tiling_iteration(t),
        unknown => panic!(
            "unknown HADWIGER_TRYBUILD_SCOPE `{unknown}`; valid scopes: {}",
            ALL_SCOPES.join(", ")
        ),
    }
}

fn phase1_query_entry(t: &trybuild::TestCases) {
    t.pass("tests/ui/phase1_query_entry/golden/*.rs");
    t.compile_fail("tests/ui/phase1_query_entry/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/phase1_query_entry/boundaries/facade_seal/*.rs");
    t.compile_fail("tests/ui/phase1_query_entry/boundaries/topology/*.rs");
}

fn canonical_artifacts(t: &trybuild::TestCases) {
    t.pass("tests/ui/canonical_artifacts/golden/*.rs");
    t.compile_fail("tests/ui/canonical_artifacts/boundaries/authority/*/*.rs");
    t.compile_fail("tests/ui/canonical_artifacts/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/canonical_artifacts/boundaries/topology/*.rs");
}

fn aspect_authority(t: &trybuild::TestCases) {
    t.pass("tests/ui/aspect_authority/golden/*.rs");
    t.compile_fail("tests/ui/aspect_authority/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/aspect_authority/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/aspect_authority/boundaries/topology/*.rs");
}

fn proof_claim_admission(t: &trybuild::TestCases) {
    t.pass("tests/ui/proof_claim_admission/golden/*.rs");
    t.compile_fail("tests/ui/proof_claim_admission/boundaries/*.rs");
}

fn explanations(t: &trybuild::TestCases) {
    t.pass("tests/ui/explanations/golden/*.rs");
    t.compile_fail("tests/ui/explanations/boundaries/*.rs");
}

fn discovery_loop(t: &trybuild::TestCases) {
    t.pass("tests/ui/discovery_loop/golden/*.rs");
    t.compile_fail("tests/ui/discovery_loop/boundaries/*.rs");
}

fn research_graph_invariants(t: &trybuild::TestCases) {
    t.pass("tests/ui/research_graph_invariants/golden/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/boundary_source/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/topology/*.rs");
}

fn agent_advisory(t: &trybuild::TestCases) {
    t.pass("tests/ui/agent_advisory/golden/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/topology/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/typing/*.rs");
}

fn research_cockpit(t: &trybuild::TestCases) {
    t.compile_fail("tests/ui/research_cockpit/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/research_cockpit/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/research_cockpit/boundaries/topology/*.rs");
    t.compile_fail("tests/ui/research_cockpit/boundaries/typing/*.rs");
}

fn motif_language(t: &trybuild::TestCases) {
    t.pass("tests/ui/motif_language/golden/*.rs");
    t.compile_fail("tests/ui/motif_language/boundaries/*.rs");
}

fn tiling_geometry(t: &trybuild::TestCases) {
    t.pass("tests/ui/tiling_geometry/golden/*.rs");
    t.compile_fail("tests/ui/tiling_geometry/boundaries/*.rs");
}

fn periodic_patterns(t: &trybuild::TestCases) {
    t.pass("tests/ui/periodic_patterns/golden/*.rs");
    t.compile_fail("tests/ui/periodic_patterns/boundaries/*.rs");
}

fn conflict_graph_extraction(t: &trybuild::TestCases) {
    t.pass("tests/ui/conflict_graph_extraction/golden/*.rs");
    t.compile_fail("tests/ui/conflict_graph_extraction/boundaries/*.rs");
}

fn tiling_equivalence(t: &trybuild::TestCases) {
    t.pass("tests/ui/tiling_equivalence/golden/*.rs");
    t.compile_fail("tests/ui/tiling_equivalence/boundaries/*.rs");
}

fn tiling_iteration(t: &trybuild::TestCases) {
    t.pass("tests/ui/tiling_iteration/golden/*.rs");
    t.compile_fail("tests/ui/tiling_iteration/boundaries/*.rs");
}
