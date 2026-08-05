use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationBundle;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationMatrix;
use crate::harness::milestone_nine_certification::classifications::MilestoneNineFailureClass;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MilestoneNinePhaseFourSupportSurface {
    EmployeeRecordFixture,
    HiddenInfluenceExhaustiveness,
    PlaceholderMaskingDenial,
    LiveDriftReadmission,
    DeliveryWidthClass,
    PolicyScaleSlope,
    PolicyCompositionParity,
    StoreBackedPolicyExecution,
    DurablePolicyArtifacts,
}

impl MilestoneNinePhaseFourSupportSurface {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmployeeRecordFixture => "employee_record_fixture",
            Self::HiddenInfluenceExhaustiveness => "hidden_influence_exhaustiveness",
            Self::PlaceholderMaskingDenial => "placeholder_masking_denial",
            Self::LiveDriftReadmission => "live_drift_readmission",
            Self::DeliveryWidthClass => "delivery_width_class",
            Self::PolicyScaleSlope => "policy_scale_slope",
            Self::PolicyCompositionParity => "policy_composition_parity",
            Self::StoreBackedPolicyExecution => "store_backed_policy_execution",
            Self::DurablePolicyArtifacts => "durable_policy_artifacts",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MilestoneNinePhaseFourSupportStatus {
    Verified,
    Deferred,
}

impl MilestoneNinePhaseFourSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNinePhaseFourDiagnostic {
    surface: MilestoneNinePhaseFourSupportSurface,
    status: MilestoneNinePhaseFourSupportStatus,
    row_name: &'static str,
}

impl MilestoneNinePhaseFourDiagnostic {
    fn new(
        surface: MilestoneNinePhaseFourSupportSurface,
        status: MilestoneNinePhaseFourSupportStatus,
        row_name: &'static str,
    ) -> Self {
        Self {
            surface,
            status,
            row_name,
        }
    }

    pub fn surface(&self) -> MilestoneNinePhaseFourSupportSurface {
        self.surface
    }

    pub fn status(&self) -> MilestoneNinePhaseFourSupportStatus {
        self.status
    }

    pub fn row_name(&self) -> &'static str {
        self.row_name
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneNinePhaseFourSupportReport {
    diagnostics: Vec<MilestoneNinePhaseFourDiagnostic>,
    report_digest: String,
}

fn phase_four_value_verified(value: &str) -> bool {
    !value.is_empty() && !value.contains("deferred")
}

fn canonical_verified(
    matrix: &MilestoneNineCertificationMatrix,
    row_name: &str,
    evidence: impl Fn(&MilestoneNineCertificationBundle) -> bool,
) -> bool {
    matrix
        .rows
        .iter()
        .find(|row| row.row_name == row_name)
        .is_some_and(|row| {
            row.control_lane.has_required_outputs()
                && row.hostile_lane.has_required_outputs()
                && row.parity_lane.has_required_outputs()
                && evidence(&row.control_lane)
                && evidence(&row.hostile_lane)
                && evidence(&row.parity_lane)
        })
}

fn rejection_verified(
    matrix: &MilestoneNineCertificationMatrix,
    row_name: &str,
    expected_failure: MilestoneNineFailureClass,
) -> bool {
    matrix
        .rejection_rows
        .iter()
        .find(|row| row.row_name == row_name)
        .is_some_and(|row| {
            row.control_lane.has_required_outputs()
                && row.parity_lane.has_required_outputs()
                && row.hostile_lane.failure_class == expected_failure
                && !row.hostile_lane.failure_digest.is_empty()
                && !row.hostile_lane.counter_snapshot_digest.is_empty()
        })
}

type PhaseFourSupportCandidate = (
    MilestoneNinePhaseFourSupportSurface,
    MilestoneNinePhaseFourSupportStatus,
    &'static str,
    bool,
);

fn phase_four_support_candidates(
    matrix: &MilestoneNineCertificationMatrix,
) -> [PhaseFourSupportCandidate; 9] {
    [
        (
            MilestoneNinePhaseFourSupportSurface::EmployeeRecordFixture,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "employee-record-fixture-policy-basis",
            canonical_verified(matrix, "employee-record-fixture-policy-basis", |bundle| {
                phase_four_value_verified(&bundle.employee_fixture_digest)
            }),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::HiddenInfluenceExhaustiveness,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "masked-view-membership-without-witness-forbidden",
            rejection_verified(
                matrix,
                "masked-aggregation-without-witness-forbidden",
                MilestoneNineFailureClass::PolicyNarrowingDenied,
            ) && rejection_verified(
                matrix,
                "masked-cursor-without-witness-forbidden",
                MilestoneNineFailureClass::PolicyNarrowingDenied,
            ) && rejection_verified(
                matrix,
                "masked-view-membership-without-witness-forbidden",
                MilestoneNineFailureClass::PolicyNarrowingDenied,
            ),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::PlaceholderMaskingDenial,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "masked-placeholder-shape-forbidden",
            rejection_verified(
                matrix,
                "masked-placeholder-shape-forbidden",
                MilestoneNineFailureClass::PolicyExecutionSeamDenied,
            ),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::LiveDriftReadmission,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "live-policy-epoch-drift-readmission",
            canonical_verified(matrix, "live-policy-epoch-drift-readmission", |bundle| {
                phase_four_value_verified(&bundle.live_drift_evidence_digest)
            }) && canonical_verified(matrix, "live-policy-density-posture-honesty", |bundle| {
                phase_four_value_verified(&bundle.live_drift_evidence_digest)
            }),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::DeliveryWidthClass,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "delivery-width-class-honesty",
            canonical_verified(matrix, "delivery-width-class-honesty", |bundle| {
                phase_four_value_verified(&bundle.delivery_width_class_digest)
            }),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::PolicyScaleSlope,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "policy-scale-slope-honesty",
            canonical_verified(matrix, "policy-scale-slope-honesty", |bundle| {
                phase_four_value_verified(&bundle.policy_scale_counter_slope_digest)
            }),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::PolicyCompositionParity,
            MilestoneNinePhaseFourSupportStatus::Verified,
            "policy-direct-scope-template-saved-parity",
            canonical_verified(
                matrix,
                "policy-direct-scope-template-saved-parity",
                |bundle| phase_four_value_verified(&bundle.composition_policy_parity_digest),
            ) && canonical_verified(matrix, "policy-view-shape-delivery-parity", |bundle| {
                phase_four_value_verified(&bundle.view_shape_policy_parity_digest)
            }) && canonical_verified(matrix, "policy-identity-aware-inspector-parity", |bundle| {
                phase_four_value_verified(&bundle.view_shape_policy_parity_digest)
            }),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::StoreBackedPolicyExecution,
            MilestoneNinePhaseFourSupportStatus::Deferred,
            "store-backed-policy-execution-deferred",
            rejection_verified(
                matrix,
                "store-backed-policy-execution-deferred",
                MilestoneNineFailureClass::PolicyExecutionSeamDenied,
            ),
        ),
        (
            MilestoneNinePhaseFourSupportSurface::DurablePolicyArtifacts,
            MilestoneNinePhaseFourSupportStatus::Deferred,
            "durable-policy-artifact-reload-deferred",
            rejection_verified(
                matrix,
                "durable-policy-cursor-deferred",
                MilestoneNineFailureClass::PolicyExecutionSeamDenied,
            ) && rejection_verified(
                matrix,
                "durable-policy-artifact-reload-deferred",
                MilestoneNineFailureClass::PolicyExecutionSeamDenied,
            ) && rejection_verified(
                matrix,
                "durable-policy-delivery-metadata-deferred",
                MilestoneNineFailureClass::PolicyExecutionSeamDenied,
            ),
        ),
    ]
}

fn phase_four_support_diagnostics(
    candidates: [PhaseFourSupportCandidate; 9],
) -> Vec<MilestoneNinePhaseFourDiagnostic> {
    candidates
        .into_iter()
        .filter(|(_, _, _, present)| *present)
        .map(|(surface, status, row_name, _)| {
            MilestoneNinePhaseFourDiagnostic::new(surface, status, row_name)
        })
        .collect::<Vec<_>>()
}

fn phase_four_support_digest(diagnostics: &[MilestoneNinePhaseFourDiagnostic]) -> String {
    digest_parts(
        &diagnostics
            .iter()
            .map(|diagnostic| {
                format!(
                    "{}:{}:{}",
                    diagnostic.surface().as_str(),
                    diagnostic.status().as_str(),
                    diagnostic.row_name()
                )
            })
            .collect::<Vec<_>>(),
    )
}

impl MilestoneNinePhaseFourSupportReport {
    pub(in crate::harness::milestone_nine_certification) fn new(
        matrix: &MilestoneNineCertificationMatrix,
    ) -> Self {
        let candidates = phase_four_support_candidates(matrix);
        let diagnostics = phase_four_support_diagnostics(candidates);
        let report_digest = phase_four_support_digest(&diagnostics);
        Self {
            diagnostics,
            report_digest,
        }
    }

    pub fn diagnostics(&self) -> &[MilestoneNinePhaseFourDiagnostic] {
        &self.diagnostics
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn verified_surface_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.status() == MilestoneNinePhaseFourSupportStatus::Verified
            })
            .count()
    }

    pub fn deferred_surface_count(&self) -> usize {
        self.diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.status() == MilestoneNinePhaseFourSupportStatus::Deferred
            })
            .count()
    }
}
