use crate::planar_contracts::local_rebuild_parity::PlanarLocalRebuildParityBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarLocalRebuildParityInspectionKind {
    LocalNeighborhood,
    RebindingContinuity,
    PlanarBasisReceipt,
    ParityView,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalRebuildParityInspectionRow {
    kind: PlanarLocalRebuildParityInspectionKind,
    locus: String,
    value: String,
}

impl PlanarLocalRebuildParityInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarLocalRebuildParityBasis) -> Vec<Self> {
        vec![
            row(
                PlanarLocalRebuildParityInspectionKind::LocalNeighborhood,
                "planar_local_rebuild.neighborhood",
                basis.neighborhood().fact_digest(),
            ),
            row(
                PlanarLocalRebuildParityInspectionKind::RebindingContinuity,
                "planar_local_rebuild.rebinding",
                basis.rebinding().continuity_digest(),
            ),
            row(
                PlanarLocalRebuildParityInspectionKind::PlanarBasisReceipt,
                "planar_local_rebuild.retained",
                basis.retained().retained_fact_digest(),
            ),
            row(
                PlanarLocalRebuildParityInspectionKind::PlanarBasisReceipt,
                "planar_local_rebuild.projection_consumed",
                basis.projection_consumed().projection_consumption_digest(),
            ),
            row(
                PlanarLocalRebuildParityInspectionKind::ParityView,
                "planar_local_rebuild.motion",
                basis.motion().retained_motion_digest(),
            ),
            row(
                PlanarLocalRebuildParityInspectionKind::ParityView,
                "planar_local_rebuild.diagnostics",
                basis.diagnostics().diagnostic_bundle_digest(),
            ),
        ]
    }
}

fn row(
    kind: PlanarLocalRebuildParityInspectionKind,
    locus: impl Into<String>,
    value: impl ToString,
) -> PlanarLocalRebuildParityInspectionRow {
    PlanarLocalRebuildParityInspectionRow {
        kind,
        locus: locus.into(),
        value: value.to_string(),
    }
}
