#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PlanarBooleanLoopGuardProofSource {
    CompileFailFixture(&'static str),
    RuntimeAssertion(&'static str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlanarBooleanLoopGuardCoverage {
    guard_name: &'static str,
    proof_source: PlanarBooleanLoopGuardProofSource,
}

const LOOP_RECONSTRUCTION_GUARD_COVERAGE: &[PlanarBooleanLoopGuardCoverage] = &[
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "raw fragments",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/raw_fragment_set_not_boolean_evidence_row.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "raw continuation maps",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/raw_continuation_index_not_boolean_evidence_row.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "raw walk outcomes",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/raw_walk_outcome_set_not_boolean_evidence_row.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "manual role outcomes",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/manual_role_outcome_set_not_boolean_evidence_row.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "manual degeneracy rows",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/manual_degenerate_loop_outcome_set_not_boolean_evidence_row.rs",
    ),
    PlanarBooleanLoopGuardCoverage::runtime(
        "hand-filled loop evidence",
        "assert_loop_ledger_rejects_manual_or_counterless_evidence",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "synthetic loop ledger construction",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/synthetic_loop_ledger_not_boolean_evidence_row.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "raw split fragments are not boolean chain handoff",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/raw_split_fragments_not_boolean_chain_handoff.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "raw continuation maps are not boolean chain handoff",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/raw_continuation_index_not_boolean_chain_handoff.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "copied digests are not boolean chain handoff",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/copied_digest_not_boolean_chain_handoff.rs",
    ),
    PlanarBooleanLoopGuardCoverage::compile_fail(
        "local graph legality rows are not boolean chain handoff",
        "src/certification/public_facade_contracts/compile_fail/pb_loop_reconstruction/local_graph_legality_row_not_boolean_chain_handoff.rs",
    ),
];

impl PlanarBooleanLoopGuardCoverage {
    const fn compile_fail(guard_name: &'static str, fixture: &'static str) -> Self {
        Self {
            guard_name,
            proof_source: PlanarBooleanLoopGuardProofSource::CompileFailFixture(fixture),
        }
    }

    const fn runtime(guard_name: &'static str, assertion: &'static str) -> Self {
        Self {
            guard_name,
            proof_source: PlanarBooleanLoopGuardProofSource::RuntimeAssertion(assertion),
        }
    }

    pub(crate) fn guard_name(self) -> &'static str {
        self.guard_name
    }

    pub(crate) fn proof_source(self) -> PlanarBooleanLoopGuardProofSource {
        self.proof_source
    }
}

pub(crate) fn loop_reconstruction_guard_names() -> Vec<String> {
    LOOP_RECONSTRUCTION_GUARD_COVERAGE
        .iter()
        .map(|coverage| coverage.guard_name().to_string())
        .collect()
}

pub(crate) fn loop_reconstruction_compile_fail_fixtures() -> Vec<&'static str> {
    LOOP_RECONSTRUCTION_GUARD_COVERAGE
        .iter()
        .filter_map(|coverage| match coverage.proof_source() {
            PlanarBooleanLoopGuardProofSource::CompileFailFixture(fixture) => Some(fixture),
            PlanarBooleanLoopGuardProofSource::RuntimeAssertion(_) => None,
        })
        .collect()
}

pub(crate) fn assert_loop_reconstruction_guard_coverage_contract() {
    let guard_names = loop_reconstruction_guard_names();
    let compile_fail_fixtures = loop_reconstruction_compile_fail_fixtures();
    let runtime_assertions = LOOP_RECONSTRUCTION_GUARD_COVERAGE
        .iter()
        .filter_map(|coverage| match coverage.proof_source() {
            PlanarBooleanLoopGuardProofSource::RuntimeAssertion(assertion) => Some(assertion),
            PlanarBooleanLoopGuardProofSource::CompileFailFixture(_) => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(guard_names.len(), LOOP_RECONSTRUCTION_GUARD_COVERAGE.len());
    assert_eq!(
        compile_fail_fixtures.len() + runtime_assertions.len(),
        LOOP_RECONSTRUCTION_GUARD_COVERAGE.len()
    );
    assert_eq!(
        runtime_assertions,
        vec!["assert_loop_ledger_rejects_manual_or_counterless_evidence"]
    );
    assert!(
        compile_fail_fixtures
            .iter()
            .all(|fixture| fixture.contains("pb_loop_reconstruction/")),
        "loop reconstruction anti-theatre guards must stay tied to loop reconstruction compile-fail fixtures"
    );
}
