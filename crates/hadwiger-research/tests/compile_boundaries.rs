#[test]
#[ignore = "expensive trybuild public-boundary suite; run explicitly for closeout QA"]
fn public_compile_boundaries_hold() {
    let t = trybuild::TestCases::new();

    t.pass("tests/ui/phase1_query_entry/golden/*.rs");
    t.compile_fail("tests/ui/phase1_query_entry/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/phase1_query_entry/boundaries/facade_seal/*.rs");
    t.compile_fail("tests/ui/phase1_query_entry/boundaries/topology/*.rs");

    t.pass("tests/ui/canonical_artifacts/golden/*.rs");
    t.compile_fail("tests/ui/canonical_artifacts/boundaries/authority/*/*.rs");
    t.compile_fail("tests/ui/canonical_artifacts/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/canonical_artifacts/boundaries/topology/*.rs");

    t.pass("tests/ui/aspect_authority/golden/*.rs");
    t.compile_fail("tests/ui/aspect_authority/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/aspect_authority/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/aspect_authority/boundaries/topology/*.rs");

    t.pass("tests/ui/proof_claim_admission/golden/*.rs");
    t.compile_fail("tests/ui/proof_claim_admission/boundaries/*.rs");

    t.pass("tests/ui/explanations/golden/*.rs");
    t.compile_fail("tests/ui/explanations/boundaries/*.rs");

    t.pass("tests/ui/discovery_loop/golden/*.rs");
    t.compile_fail("tests/ui/discovery_loop/boundaries/*.rs");

    t.pass("tests/ui/research_graph_invariants/golden/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/boundary_source/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/registration/*.rs");
    t.compile_fail("tests/ui/research_graph_invariants/boundaries/topology/*.rs");

    t.pass("tests/ui/agent_advisory/golden/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/topology/*.rs");
    t.compile_fail("tests/ui/agent_advisory/boundaries/typing/*.rs");

    t.compile_fail("tests/ui/research_cockpit/boundaries/authority/*.rs");
    t.compile_fail("tests/ui/research_cockpit/boundaries/immutability/*.rs");
    t.compile_fail("tests/ui/research_cockpit/boundaries/topology/*.rs");
    t.compile_fail("tests/ui/research_cockpit/boundaries/typing/*.rs");
}
