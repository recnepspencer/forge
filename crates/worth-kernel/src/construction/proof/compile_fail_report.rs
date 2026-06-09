pub(crate) const PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/reports/public_verified_intent_arbitration_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_certification_bucket_not_exported.rs",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionProofBoundaryCompileFailFixture {
    path: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionProofBoundaryCompileFailReport {
    fixtures: Vec<PrimitiveConstructionProofBoundaryCompileFailFixture>,
}

impl PrimitiveConstructionProofBoundaryCompileFailReport {
    pub fn fixtures(&self) -> &[PrimitiveConstructionProofBoundaryCompileFailFixture] {
        &self.fixtures
    }
}

pub fn prepare_primitive_construction_proof_boundary_compile_fail_report(
) -> PrimitiveConstructionProofBoundaryCompileFailReport {
    let fixtures = PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES
        .iter()
        .map(|path| PrimitiveConstructionProofBoundaryCompileFailFixture { path })
        .collect::<Vec<_>>();
    PrimitiveConstructionProofBoundaryCompileFailReport { fixtures }
}
