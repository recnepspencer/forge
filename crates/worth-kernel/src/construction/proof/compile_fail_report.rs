use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

pub(crate) const PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES: &[&str] = &[
    "src/certification/public_facade_contracts/compile_fail/reports/public_verified_motion_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_verified_intent_arbitration_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_compound_parity_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_policy_pressure_report_bundle_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_phase_five_six_closeout_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_milestone_four_kernel_closeout_report_constructor_not_exported.rs",
    "src/certification/public_facade_contracts/compile_fail/reports/public_proof_substrate_closeout_report_constructor_not_exported.rs",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionProofBoundaryCompileFailFixture {
    path: &'static str,
}

impl PrimitiveConstructionProofBoundaryCompileFailFixture {
    pub fn path(&self) -> &'static str {
        self.path
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionProofBoundaryCompileFailReport {
    fixtures: Vec<PrimitiveConstructionProofBoundaryCompileFailFixture>,
    report_digest: String,
}

impl PrimitiveConstructionProofBoundaryCompileFailReport {
    pub fn fixtures(&self) -> &[PrimitiveConstructionProofBoundaryCompileFailFixture] {
        &self.fixtures
    }
    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_proof_boundary_compile_fail_report(
) -> PrimitiveConstructionProofBoundaryCompileFailReport {
    let fixtures = PROOF_BOUNDARY_COMPILE_FAIL_FIXTURES
        .iter()
        .map(|path| PrimitiveConstructionProofBoundaryCompileFailFixture { path })
        .collect::<Vec<_>>();
    let report_digest = digest_owned_parts_with_scope(
        ConstructionDigestScope::ArtifactIdentity,
        &fixtures
            .iter()
            .map(|fixture| fixture.path.to_string())
            .collect::<Vec<_>>(),
    );
    PrimitiveConstructionProofBoundaryCompileFailReport {
        fixtures,
        report_digest,
    }
}
