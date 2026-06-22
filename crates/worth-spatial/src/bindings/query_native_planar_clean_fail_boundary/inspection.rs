use crate::planar_contracts::clean_fail_boundary::PlanarCleanFailBoundaryBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarCleanFailBoundaryInspectionKind {
    CleanFailSource,
    CleanFailSourceDetail,
    Admission,
    TransformPosture,
    Recovery,
    Diagnostics,
    NoRepairPolicy,
    NoBoundedConversionPolicy,
    TruthEffect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryInspectionRow {
    kind: PlanarCleanFailBoundaryInspectionKind,
    locus: String,
    value: String,
}

impl PlanarCleanFailBoundaryInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarCleanFailBoundaryBasis) -> Vec<Self> {
        vec![
            row(
                PlanarCleanFailBoundaryInspectionKind::CleanFailSource,
                "planar_clean_fail.source",
                basis.input().source_digest(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::CleanFailSourceDetail,
                "planar_clean_fail.source_detail",
                basis.input().source_detail(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::Admission,
                "planar_clean_fail.admission",
                basis
                    .input()
                    .admission_row()
                    .expect("validated admission")
                    .row_digest(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::TransformPosture,
                "planar_clean_fail.transform_posture",
                basis
                    .input()
                    .transform_posture_digest()
                    .expect("validated transform posture"),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::Recovery,
                "planar_clean_fail.recovery",
                basis.recovery().recovery_posture_digest(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::Diagnostics,
                "planar_clean_fail.diagnostics",
                basis.diagnostics().diagnostic_bundle_digest(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::NoRepairPolicy,
                "planar_clean_fail.no_repair",
                basis.repair_attempt().as_str(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::NoBoundedConversionPolicy,
                "planar_clean_fail.no_bounded_conversion",
                basis.bounded_conversion().as_str(),
            ),
            row(
                PlanarCleanFailBoundaryInspectionKind::TruthEffect,
                "planar_clean_fail.truth_effect",
                basis.truth_effect().as_str(),
            ),
        ]
    }
}

fn row(
    kind: PlanarCleanFailBoundaryInspectionKind,
    locus: impl Into<String>,
    value: impl ToString,
) -> PlanarCleanFailBoundaryInspectionRow {
    PlanarCleanFailBoundaryInspectionRow {
        kind,
        locus: locus.into(),
        value: value.to_string(),
    }
}
