mod canonical_rows;
mod completeness;
mod evidence;
mod lane_builders;
mod model;
mod rejection_evidence;
mod rejection_rows;
mod row_catalog;

#[cfg(test)]
mod expectations;
#[cfg(test)]
mod tests;

use model::{MilestoneFivePointTwoPreviewCertificationArtifact, PreviewCertificationMatrix};
pub use model::{PreviewFailureClass, PreviewLaneEvaluationClass, PreviewPerturbationClass};
pub(crate) use row_catalog::{
    PREVIEW_REQUIRED_CANONICAL_ROW_NAMES, PREVIEW_REQUIRED_REJECTION_ROW_NAMES,
};

use canonical_rows::canonical_rows;
use evidence::build_preview_certification_evidence;
use rejection_rows::rejection_rows;

pub struct MilestoneFivePointTwoPreviewCertificationAdapter;

impl MilestoneFivePointTwoPreviewCertificationAdapter {
    pub fn preview_session_basis_and_promotion_parity_test() -> PreviewCertificationMatrix {
        let evidence = build_preview_certification_evidence();

        PreviewCertificationMatrix {
            suite_name: "Preview Session Basis And Promotion Parity Test",
            rows: canonical_rows(&evidence.lanes),
            rejection_rows: rejection_rows(&evidence.lanes, &evidence.rejections),
        }
    }

    pub fn preview_session_basis_and_promotion_parity_artifact(
    ) -> MilestoneFivePointTwoPreviewCertificationArtifact {
        Self::preview_session_basis_and_promotion_parity_test()
            .into_milestone_five_point_two_artifact()
    }
}
