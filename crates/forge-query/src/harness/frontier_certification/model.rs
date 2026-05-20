use super::super::certification::{digest_parts, CertificationMatrix};
use crate::planning::{FrontierCounterSnapshot, FrontierParityBundle, PlannedRouteFamily};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum FrontierPerturbationClass {
    SerialControlParity,
    ParallelAdmittedParity,
    SerialFallbackParity,
    PredictedRealizedBreadth,
    BundleRoutePostureParity,
    ExactBasisBundleParity,
    WorkAvoidedCounterParity,
    UnsupportedFrontierFamilyRejection,
    UnsupportedBundleCompositionRejection,
    MixedBasisBundleRejection,
    ExecutorSpeculativeAdmissionRejection,
    HiddenSerialFallbackRejection,
    RoutePostureOverrideRejection,
    SerialRouteOnParallelEntrypointRejection,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontierRouteClass {
    SerialControl,
    ParallelAdmitted,
    ParallelAdmittedBundle,
    SerialFallback,
    SerialFallbackBundle,
}

impl FrontierRouteClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SerialControl => "serial_control",
            Self::ParallelAdmitted => "parallel_admitted",
            Self::ParallelAdmittedBundle => "parallel_admitted_bundle",
            Self::SerialFallback => "serial_fallback",
            Self::SerialFallbackBundle => "serial_fallback_bundle",
        }
    }
}

impl From<&PlannedRouteFamily> for FrontierRouteClass {
    fn from(value: &PlannedRouteFamily) -> Self {
        match value {
            PlannedRouteFamily::FrontierSerialControl => Self::SerialControl,
            PlannedRouteFamily::FrontierParallelAdmitted => Self::ParallelAdmitted,
            PlannedRouteFamily::FrontierParallelAdmittedBundle => Self::ParallelAdmittedBundle,
            PlannedRouteFamily::FrontierSerialFallback => Self::SerialFallback,
            PlannedRouteFamily::FrontierSerialFallbackBundle => Self::SerialFallbackBundle,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontierFailureClass {
    UnsupportedFrontierFamily,
    UnsupportedBundleComposition,
    MixedBasisBundleDenied,
    HiddenSerialFallbackDenied,
    CompileFail,
}

impl FrontierFailureClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::UnsupportedFrontierFamily => "unsupported-frontier-family",
            Self::UnsupportedBundleComposition => "unsupported-bundle-composition",
            Self::MixedBasisBundleDenied => "mixed-basis-bundle-denied",
            Self::HiddenSerialFallbackDenied => "hidden-serial-fallback-denied",
            Self::CompileFail => "compile_fail",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierCertificationLane {
    pub parity_bundle: FrontierParityBundle,
}

impl FrontierCertificationLane {
    pub fn route_class(&self) -> FrontierRouteClass {
        FrontierRouteClass::from(self.parity_bundle.route_family())
    }

    pub fn counter_snapshot(&self) -> &FrontierCounterSnapshot {
        self.parity_bundle.counter_snapshot()
    }

    pub fn has_required_outputs(&self) -> bool {
        !self.parity_bundle.query_digest().as_str().is_empty()
            && !self.parity_bundle.plan_digest().as_str().is_empty()
            && !self.parity_bundle.result_digest().as_str().is_empty()
            && !self.parity_bundle.basis_digest().is_empty()
            && !self
                .parity_bundle
                .route_posture_digest()
                .as_str()
                .is_empty()
            && self
                .parity_bundle
                .counter_snapshot()
                .frontier_lookup_count()
                > 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierCertificationRejection {
    pub failure_class: FrontierFailureClass,
    pub failure_digest: String,
    pub counter_snapshot: FrontierCounterSnapshot,
    pub compile_fail_case: Option<&'static str>,
}

impl FrontierCertificationRejection {
    pub fn has_required_outputs(&self) -> bool {
        !self.failure_digest.is_empty()
            && (self.counter_snapshot.frontier_lookup_count() > 0
                || self.compile_fail_case.is_some())
    }
}

pub type FrontierCertificationMatrix = CertificationMatrix<
    FrontierPerturbationClass,
    FrontierCertificationLane,
    FrontierCertificationRejection,
>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFivePointThreeFrontierCertificationArtifact {
    pub suite_name: &'static str,
    pub certification_bundle_digest: String,
    pub coverage_matrix_digest: String,
    pub counter_snapshot: FrontierCounterSnapshot,
    pub matrix: FrontierCertificationMatrix,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FrontierCloseoutStatus {
    Satisfied,
}

impl FrontierCloseoutStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierCloseoutRequirement {
    pub requirement_name: &'static str,
    pub status: FrontierCloseoutStatus,
    pub production_artifacts: &'static [&'static str],
    pub certification_rows: &'static [&'static str],
    pub compile_fail_cases: &'static [&'static str],
    pub notes: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFivePointThreeFrontierCloseoutArtifact {
    pub suite_name: &'static str,
    pub closeout_matrix_digest: String,
    pub certification_bundle_digest: String,
    pub must_ship: Vec<FrontierCloseoutRequirement>,
    pub must_preserve: Vec<FrontierCloseoutRequirement>,
    pub proof_obligations: Vec<FrontierCloseoutRequirement>,
    pub acceptance_evidence: Vec<FrontierCloseoutRequirement>,
}

impl MilestoneFivePointThreeFrontierCloseoutArtifact {
    pub fn is_full_spec_ready(&self) -> bool {
        self.must_ship
            .iter()
            .chain(self.must_preserve.iter())
            .chain(self.proof_obligations.iter())
            .chain(self.acceptance_evidence.iter())
            .all(|requirement| requirement.status == FrontierCloseoutStatus::Satisfied)
    }
}

impl FrontierCertificationMatrix {
    pub fn into_milestone_five_point_three_artifact(
        self,
    ) -> MilestoneFivePointThreeFrontierCertificationArtifact {
        let certification_bundle_digest = digest_parts(&bundle_digest_parts(&self));
        let coverage_matrix_digest = digest_parts(&coverage_digest_parts(&self));
        let counter_snapshot = self.aggregate_counters();

        MilestoneFivePointThreeFrontierCertificationArtifact {
            suite_name: self.suite_name,
            certification_bundle_digest,
            coverage_matrix_digest,
            counter_snapshot,
            matrix: self,
        }
    }

    fn aggregate_counters(&self) -> FrontierCounterSnapshot {
        let mut aggregate = FrontierCounterSnapshot::default();
        for row in &self.rows {
            aggregate.absorb(row.control_lane.counter_snapshot());
            aggregate.absorb(row.hostile_lane.counter_snapshot());
            aggregate.absorb(row.parity_lane.counter_snapshot());
        }
        for row in &self.rejection_rows {
            aggregate.absorb(row.control_lane.counter_snapshot());
            aggregate.absorb(&row.hostile_lane.counter_snapshot);
            aggregate.absorb(row.parity_lane.counter_snapshot());
        }
        aggregate
    }
}

fn bundle_digest_parts(matrix: &FrontierCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("canonical:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(lane_digest_parts(&row.hostile_lane, "hostile"));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
        parts.extend(lane_digest_parts(&row.control_lane, "control"));
        parts.extend(rejection_digest_parts(
            &row.hostile_lane,
            "hostile_rejection",
        ));
        parts.extend(lane_digest_parts(&row.parity_lane, "parity"));
    }
    parts
}

fn coverage_digest_parts(matrix: &FrontierCertificationMatrix) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("canonical:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}

fn lane_digest_parts(bundle: &FrontierCertificationLane, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!(
            "{label}.query_digest:{}",
            bundle.parity_bundle.query_digest().as_str()
        ),
        format!(
            "{label}.plan_digest:{}",
            bundle.parity_bundle.plan_digest().as_str()
        ),
        format!(
            "{label}.result_digest:{}",
            bundle.parity_bundle.result_digest().as_str()
        ),
        format!(
            "{label}.basis_digest:{}",
            bundle.parity_bundle.basis_digest()
        ),
        format!("{label}.route_class:{}", bundle.route_class().as_str()),
        format!(
            "{label}.route_posture_digest:{}",
            bundle.parity_bundle.route_posture_digest().as_str()
        ),
        format!(
            "{label}.predicted_breadth:{}",
            bundle.parity_bundle.predicted_breadth().value()
        ),
        format!(
            "{label}.realized_breadth:{}",
            bundle.parity_bundle.realized_breadth()
        ),
    ];
    parts.extend(bundle.parity_bundle.counter_snapshot().digest_parts(label));
    parts
}

fn rejection_digest_parts(bundle: &FrontierCertificationRejection, label: &str) -> Vec<String> {
    let mut parts = vec![
        format!("{label}.failure_class:{}", bundle.failure_class.as_str()),
        format!("{label}.failure_digest:{}", bundle.failure_digest),
    ];
    if let Some(case) = bundle.compile_fail_case {
        parts.push(format!("{label}.compile_fail_case:{case}"));
    }
    parts.extend(bundle.counter_snapshot.digest_parts(label));
    parts
}

pub fn closeout_matrix_digest_parts(
    sections: &[(&str, &[FrontierCloseoutRequirement])],
    certification_bundle_digest: &str,
) -> Vec<String> {
    let mut parts = vec![format!(
        "certification_bundle_digest:{certification_bundle_digest}"
    )];
    for (section_name, requirements) in sections {
        parts.push(format!("section:{section_name}"));
        for requirement in *requirements {
            parts.push(format!("requirement:{}", requirement.requirement_name));
            parts.push(format!("status:{}", requirement.status.as_str()));
            for artifact in requirement.production_artifacts {
                parts.push(format!("artifact:{artifact}"));
            }
            for row in requirement.certification_rows {
                parts.push(format!("row:{row}"));
            }
            for case in requirement.compile_fail_cases {
                parts.push(format!("compile_fail:{case}"));
            }
            parts.push(format!("notes:{}", requirement.notes));
        }
    }
    parts
}
