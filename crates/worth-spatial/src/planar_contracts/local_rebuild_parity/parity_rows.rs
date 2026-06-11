use super::PlanarLocalRebuildParityBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarLocalRebuildParityView {
    Live,
    Retained,
    ProjectionConsumed,
    Recovery,
    Replayed,
    MovementRotation,
    LocalRebuild,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalRebuildParityRow {
    view: PlanarLocalRebuildParityView,
    digest: String,
}

impl PlanarLocalRebuildParityRow {
    pub(crate) fn from_basis(basis: &PlanarLocalRebuildParityBasis) -> Vec<Self> {
        vec![
            row(
                PlanarLocalRebuildParityView::Live,
                basis.structural_identity().structural_identity_digest(),
            ),
            row(
                PlanarLocalRebuildParityView::Retained,
                basis.retained().retained_fact_digest(),
            ),
            row(
                PlanarLocalRebuildParityView::ProjectionConsumed,
                basis.projection_consumed().projection_consumption_digest(),
            ),
            row(
                PlanarLocalRebuildParityView::Recovery,
                basis.recovery().recovery_posture_digest(),
            ),
            row(
                PlanarLocalRebuildParityView::Replayed,
                basis.retained().retained_fact_digest(),
            ),
            row(
                PlanarLocalRebuildParityView::MovementRotation,
                basis.motion().retained_motion_digest(),
            ),
            row(
                PlanarLocalRebuildParityView::LocalRebuild,
                basis.neighborhood().fact_digest(),
            ),
        ]
    }

    pub fn view(&self) -> PlanarLocalRebuildParityView {
        self.view
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

fn row(view: PlanarLocalRebuildParityView, digest: impl ToString) -> PlanarLocalRebuildParityRow {
    PlanarLocalRebuildParityRow {
        view,
        digest: digest.to_string(),
    }
}
