use crate::graph_read_access_plan_adoption::{
    WorthGraphReadAccessPostureCapRow, WorthGraphReadAccessSpatialDensePostureProjection,
};

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPostureExport {
    posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    cap_rows: Vec<WorthGraphReadAccessPostureCapRow>,
    export_digest: String,
}

impl WorthGraphReadAccessPlanAdoptionPostureExport {
    pub(in crate::graph_read_access_plan_adoption::phase_eight_public_closeout) fn from_parts(
        posture_projections: &[WorthGraphReadAccessSpatialDensePostureProjection],
        cap_rows: &[WorthGraphReadAccessPostureCapRow],
    ) -> Self {
        let export_digest = stable_digest(
            &std::iter::once("worth_graph_read_access_plan_adoption_posture_export_v1".to_string())
                .chain(
                    posture_projections
                        .iter()
                        .map(|projection| format!("posture:{}", projection.projection_digest())),
                )
                .chain(
                    cap_rows
                        .iter()
                        .map(|row| format!("cap:{}", row.row_digest())),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            posture_projections: posture_projections.to_vec(),
            cap_rows: cap_rows.to_vec(),
            export_digest,
        }
    }

    pub fn posture_projections(&self) -> &[WorthGraphReadAccessSpatialDensePostureProjection] {
        &self.posture_projections
    }

    pub fn cap_rows(&self) -> &[WorthGraphReadAccessPostureCapRow] {
        &self.cap_rows
    }

    pub fn export_digest(&self) -> &str {
        &self.export_digest
    }
}
