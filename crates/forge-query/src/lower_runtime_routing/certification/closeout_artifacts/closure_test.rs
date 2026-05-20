use crate::identity::hash_parts;

use super::{
    certify_lower_runtime_routing, ForgeQueryLowerRuntimeCertificationBundle,
    ForgeQueryLowerRuntimeCertificationLane,
};
use crate::lower_runtime_routing::certification::boundary_certification::{
    forge_query_lower_runtime_boundary_reconciliation_report,
    forge_query_lower_runtime_proof_shape_audit,
    ForgeQueryLowerRuntimeBoundaryReconciliationReport, ForgeQueryLowerRuntimeProofShapeAudit,
};
use crate::lower_runtime_routing::certification::surface::{
    forge_query_lower_runtime_acceptance_suite, ForgeQueryLowerRuntimeAcceptanceLane,
    ForgeQueryLowerRuntimeAcceptanceSuite,
};

pub const LOWER_RUNTIME_CLOSURE_TEST_NAME: &str =
    "9.3.6. Lower-Runtime Capability Routing And Boundary Envelope Closure Test";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryLowerRuntimeClosureTestLane {
    Control,
    Hostile,
    Parity,
    DownstreamBoundary,
    CompileBoundary,
}

impl ForgeQueryLowerRuntimeClosureTestLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Control => "control-lane",
            Self::Hostile => "hostile-lane",
            Self::Parity => "parity-lane",
            Self::DownstreamBoundary => "downstream-boundary-lane",
            Self::CompileBoundary => "compile-boundary-lane",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeClosureTestRow {
    lane: ForgeQueryLowerRuntimeClosureTestLane,
    digest: String,
    detail: String,
}

impl ForgeQueryLowerRuntimeClosureTestRow {
    fn new(
        lane: ForgeQueryLowerRuntimeClosureTestLane,
        digest: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            lane,
            digest: digest.into(),
            detail: detail.into(),
        }
    }

    pub fn lane(&self) -> ForgeQueryLowerRuntimeClosureTestLane {
        self.lane
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeClosureTest {
    name: &'static str,
    certification_bundle: ForgeQueryLowerRuntimeCertificationBundle,
    acceptance_suite: ForgeQueryLowerRuntimeAcceptanceSuite,
    boundary_reconciliation: ForgeQueryLowerRuntimeBoundaryReconciliationReport,
    proof_shape_audit: ForgeQueryLowerRuntimeProofShapeAudit,
    rows: Vec<ForgeQueryLowerRuntimeClosureTestRow>,
    suite_digest: String,
}

impl ForgeQueryLowerRuntimeClosureTest {
    pub(crate) fn new(
        certification_bundle: ForgeQueryLowerRuntimeCertificationBundle,
        acceptance_suite: ForgeQueryLowerRuntimeAcceptanceSuite,
        boundary_reconciliation: ForgeQueryLowerRuntimeBoundaryReconciliationReport,
        proof_shape_audit: ForgeQueryLowerRuntimeProofShapeAudit,
        rows: Vec<ForgeQueryLowerRuntimeClosureTestRow>,
    ) -> Self {
        let suite_digest = hash_parts(&[
            LOWER_RUNTIME_CLOSURE_TEST_NAME.to_string(),
            certification_bundle
                .certification_bundle_digest()
                .to_string(),
            acceptance_suite.suite_digest().to_string(),
            boundary_reconciliation.report_digest().to_string(),
            proof_shape_audit.proof_shape_digest().to_string(),
            hash_parts(
                &rows
                    .iter()
                    .map(|row| format!("{}|{}|{}", row.lane().as_str(), row.digest(), row.detail()))
                    .collect::<Vec<_>>(),
            ),
        ]);
        Self {
            name: LOWER_RUNTIME_CLOSURE_TEST_NAME,
            certification_bundle,
            acceptance_suite,
            boundary_reconciliation,
            proof_shape_audit,
            rows,
            suite_digest,
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn certification_bundle(&self) -> &ForgeQueryLowerRuntimeCertificationBundle {
        &self.certification_bundle
    }

    pub fn acceptance_suite(&self) -> &ForgeQueryLowerRuntimeAcceptanceSuite {
        &self.acceptance_suite
    }

    pub fn boundary_reconciliation(&self) -> &ForgeQueryLowerRuntimeBoundaryReconciliationReport {
        &self.boundary_reconciliation
    }

    pub fn proof_shape_audit(&self) -> &ForgeQueryLowerRuntimeProofShapeAudit {
        &self.proof_shape_audit
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimeClosureTestRow] {
        &self.rows
    }

    pub fn suite_digest(&self) -> &str {
        &self.suite_digest
    }

    pub fn lane(
        &self,
        lane: ForgeQueryLowerRuntimeClosureTestLane,
    ) -> &ForgeQueryLowerRuntimeClosureTestRow {
        self.rows
            .iter()
            .find(|row| row.lane() == lane)
            .unwrap_or_else(|| panic!("missing closure test lane {}", lane.as_str()))
    }
}

pub fn forge_query_lower_runtime_closure_test() -> ForgeQueryLowerRuntimeClosureTest {
    let certification_bundle = certify_lower_runtime_routing();
    let acceptance_suite = forge_query_lower_runtime_acceptance_suite();
    let boundary_reconciliation = forge_query_lower_runtime_boundary_reconciliation_report();
    let proof_shape_audit = forge_query_lower_runtime_proof_shape_audit();

    let control = acceptance_suite
        .lane(ForgeQueryLowerRuntimeAcceptanceLane::Control)
        .clone();
    let hostile = acceptance_suite
        .lane(ForgeQueryLowerRuntimeAcceptanceLane::Hostile)
        .clone();
    let parity = acceptance_suite
        .lane(ForgeQueryLowerRuntimeAcceptanceLane::Parity)
        .clone();
    let former_specialist = certification_bundle
        .rows()
        .iter()
        .find(|row| {
            row.lane() == ForgeQueryLowerRuntimeCertificationLane::FormerSpecialistSeamClosure
        })
        .expect("named closure test requires former specialist seam closure lane")
        .clone();
    let deferred_neighbor = certification_bundle
        .rows()
        .iter()
        .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::DeferredNeighborDenial)
        .expect("named closure test requires deferred neighbor denial lane")
        .clone();
    let downstream_boundary = certification_bundle
        .rows()
        .iter()
        .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::DownstreamBoundaryAudit)
        .expect("named closure test requires downstream boundary audit lane")
        .clone();
    let compile_boundary = certification_bundle
        .rows()
        .iter()
        .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::CompileFailBoundary)
        .expect("named closure test requires compile-boundary lane")
        .clone();

    assert_eq!(
        certification_bundle
            .rows()
            .iter()
            .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::AcceptanceEvidence)
            .map(|row| row.artifact_digest()),
        Some(acceptance_suite.suite_digest()),
        "named closure test requires bundle parity with acceptance evidence"
    );
    assert_eq!(
        certification_bundle
            .rows()
            .iter()
            .find(|row| row.lane() == ForgeQueryLowerRuntimeCertificationLane::AcceptanceEvidence)
            .and_then(|row| row.failure_digest()),
        Some(hostile.digest()),
        "named closure test requires hostile acceptance parity"
    );
    assert_eq!(
        certification_bundle.output_digest("route_boundary_reconciliation_digest"),
        Some(boundary_reconciliation.report_digest()),
        "named closure test requires bundle parity with boundary reconciliation"
    );
    assert_eq!(
        certification_bundle.output_digest("route_proof_shape_digest"),
        Some(proof_shape_audit.proof_shape_digest()),
        "named closure test requires bundle parity with proof-shape audit"
    );

    ForgeQueryLowerRuntimeClosureTest::new(
        certification_bundle,
        acceptance_suite,
        boundary_reconciliation,
        proof_shape_audit,
        vec![
            ForgeQueryLowerRuntimeClosureTestRow::new(
                ForgeQueryLowerRuntimeClosureTestLane::Control,
                control.digest(),
                control.detail(),
            ),
            ForgeQueryLowerRuntimeClosureTestRow::new(
                ForgeQueryLowerRuntimeClosureTestLane::Hostile,
                hash_parts(&[
                    hostile.digest().to_string(),
                    former_specialist.artifact_digest().to_string(),
                    deferred_neighbor
                        .failure_digest()
                        .expect("deferred neighbor lane should expose hostile digest")
                        .to_string(),
                ]),
                "hostile closure certifies deleted seam survival, former specialist seam survival, and deferred-neighbor widening remain forbidden".to_string(),
            ),
            ForgeQueryLowerRuntimeClosureTestRow::new(
                ForgeQueryLowerRuntimeClosureTestLane::Parity,
                parity.digest(),
                parity.detail(),
            ),
            ForgeQueryLowerRuntimeClosureTestRow::new(
                ForgeQueryLowerRuntimeClosureTestLane::DownstreamBoundary,
                downstream_boundary.artifact_digest(),
                downstream_boundary.detail(),
            ),
            ForgeQueryLowerRuntimeClosureTestRow::new(
                ForgeQueryLowerRuntimeClosureTestLane::CompileBoundary,
                compile_boundary.artifact_digest(),
                compile_boundary.detail(),
            ),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closure_test_has_named_control_hostile_and_parity_lanes() {
        let suite = forge_query_lower_runtime_closure_test();

        assert_eq!(suite.name(), LOWER_RUNTIME_CLOSURE_TEST_NAME);
        assert_eq!(suite.rows().len(), 5);
        assert!(!suite.suite_digest().is_empty());
        assert_eq!(
            suite
                .lane(ForgeQueryLowerRuntimeClosureTestLane::Control)
                .digest(),
            suite
                .acceptance_suite()
                .lane(ForgeQueryLowerRuntimeAcceptanceLane::Control)
                .digest()
        );
        assert_eq!(
            suite.boundary_reconciliation().report_digest(),
            suite
                .certification_bundle()
                .output_digest("route_boundary_reconciliation_digest")
                .expect("boundary reconciliation output should exist")
        );
        assert_eq!(
            suite.proof_shape_audit().proof_shape_digest(),
            suite
                .certification_bundle()
                .output_digest("route_proof_shape_digest")
                .expect("proof-shape output should exist")
        );
    }

    #[test]
    fn closure_test_binds_boundary_and_compile_lanes_to_certified_rows() {
        let suite = forge_query_lower_runtime_closure_test();

        assert_eq!(
            suite
                .lane(ForgeQueryLowerRuntimeClosureTestLane::DownstreamBoundary)
                .digest(),
            suite
                .certification_bundle()
                .rows()
                .iter()
                .find(|row| row.lane()
                    == ForgeQueryLowerRuntimeCertificationLane::DownstreamBoundaryAudit)
                .expect("downstream boundary row")
                .artifact_digest()
        );
        assert_eq!(
            suite
                .lane(ForgeQueryLowerRuntimeClosureTestLane::CompileBoundary)
                .digest(),
            suite
                .certification_bundle()
                .rows()
                .iter()
                .find(|row| row.lane()
                    == ForgeQueryLowerRuntimeCertificationLane::CompileFailBoundary)
                .expect("compile fail row")
                .artifact_digest()
        );
    }

    #[test]
    fn closure_test_hostile_lane_aggregates_phase_seven_hostile_obligations() {
        let suite = forge_query_lower_runtime_closure_test();
        let former_specialist = suite
            .certification_bundle()
            .rows()
            .iter()
            .find(|row| {
                row.lane() == ForgeQueryLowerRuntimeCertificationLane::FormerSpecialistSeamClosure
            })
            .expect("former specialist seam closure row");
        let deferred_neighbor = suite
            .certification_bundle()
            .rows()
            .iter()
            .find(|row| {
                row.lane() == ForgeQueryLowerRuntimeCertificationLane::DeferredNeighborDenial
            })
            .expect("deferred neighbor denial row");
        let expected = hash_parts(&[
            suite
                .acceptance_suite()
                .lane(ForgeQueryLowerRuntimeAcceptanceLane::Hostile)
                .digest()
                .to_string(),
            former_specialist.artifact_digest().to_string(),
            deferred_neighbor
                .failure_digest()
                .expect("deferred neighbor lane should expose hostile digest")
                .to_string(),
        ]);

        assert_eq!(
            suite
                .lane(ForgeQueryLowerRuntimeClosureTestLane::Hostile)
                .digest(),
            expected
        );
        assert_eq!(
            suite
                .lane(ForgeQueryLowerRuntimeClosureTestLane::Parity)
                .digest(),
            suite
                .acceptance_suite()
                .lane(ForgeQueryLowerRuntimeAcceptanceLane::Parity)
                .digest()
        );
    }
}
