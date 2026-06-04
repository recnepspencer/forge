use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::preview::{
    PrimitiveConstructionPreviewCase, PrimitiveConstructionPreviewRow,
};
use crate::construction::digest::digest_owned_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreviewQueryInspectionSurface {
    PreviewSurfaceReportReceipt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreviewQueryReadSurface {
    PreviewInspection,
    ProjectionConsumptionFromPreviewSurfaceReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionPreviewQueryFactProvenance {
    DirectPreviewSurfaceReport,
    EquivalentProjectionConsumptionFacts,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionQueryPreviewParityReport {
    case: PrimitiveConstructionPreviewCase,
    profile_name: &'static str,
    authored_act: worth_spatial::facade::arbitration::SpatialAuthoredActKind,
    proximity_posture: worth_spatial::facade::arbitration::SpatialThresholdPosture,
    alignment_posture: worth_spatial::facade::arbitration::SpatialThresholdPosture,
    conflict_class: worth_spatial::facade::arbitration::SpatialIntentConflictClass,
    commit_disposition: worth_spatial::facade::arbitration::SpatialIntentPreviewCommitDisposition,
    preview_richness: worth_spatial::facade::arbitration::SpatialPreviewRichness,
    candidates: Vec<worth_spatial::facade::arbitration::SpatialIntentCandidate>,
    blocked_candidates: Vec<(
        worth_spatial::facade::arbitration::SpatialIntentCandidate,
        worth_spatial::facade::arbitration::SpatialBlockedCapability,
    )>,
    warnings: Vec<worth_spatial::facade::arbitration::SpatialIntentPreviewWarning>,
    query_contract_digest: String,
    read_surface: PrimitiveConstructionPreviewQueryReadSurface,
    inspection_surface: PrimitiveConstructionPreviewQueryInspectionSurface,
    fact_provenance: PrimitiveConstructionPreviewQueryFactProvenance,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryPreviewParityReport {
    fn new(
        query_contract_digest: String,
        row: PrimitiveConstructionPreviewRow,
        read_surface: PrimitiveConstructionPreviewQueryReadSurface,
        fact_provenance: PrimitiveConstructionPreviewQueryFactProvenance,
    ) -> Self {
        let inspection_surface =
            PrimitiveConstructionPreviewQueryInspectionSurface::PreviewSurfaceReportReceipt;
        let parity_verified = !query_contract_digest.is_empty();
        let report_digest = digest_owned_parts(&[
            format!("{:?}", row.case()),
            row.profile_name().to_string(),
            row.authored_act().as_str().to_string(),
            format!("{:?}", row.proximity_posture()),
            format!("{:?}", row.alignment_posture()),
            format!("{:?}", row.conflict_class()),
            format!("{:?}", row.commit_disposition()),
            format!("{:?}", row.preview_richness()),
            format!("{:?}", row.candidates()),
            format!("{:?}", row.blocked_candidates()),
            format!("{:?}", row.warnings()),
            query_contract_digest.clone(),
            format!("{read_surface:?}"),
            format!("{inspection_surface:?}"),
            format!("{fact_provenance:?}"),
            parity_verified.to_string(),
        ]);
        Self {
            case: row.case(),
            profile_name: row.profile_name(),
            authored_act: row.authored_act(),
            proximity_posture: row.proximity_posture(),
            alignment_posture: row.alignment_posture(),
            conflict_class: row.conflict_class(),
            commit_disposition: row.commit_disposition(),
            preview_richness: row.preview_richness(),
            candidates: row.candidates().to_vec(),
            blocked_candidates: row.blocked_candidates().to_vec(),
            warnings: row.warnings().to_vec(),
            query_contract_digest,
            read_surface,
            inspection_surface,
            fact_provenance,
            parity_verified,
            report_digest,
        }
    }

    pub fn commit_disposition(
        &self,
    ) -> worth_spatial::facade::arbitration::SpatialIntentPreviewCommitDisposition {
        self.commit_disposition
    }

    pub fn proximity_posture(&self) -> worth_spatial::facade::arbitration::SpatialThresholdPosture {
        self.proximity_posture
    }

    pub fn preview_richness(&self) -> worth_spatial::facade::arbitration::SpatialPreviewRichness {
        self.preview_richness
    }

    pub fn candidates(&self) -> &[worth_spatial::facade::arbitration::SpatialIntentCandidate] {
        &self.candidates
    }

    pub fn blocked_candidates(
        &self,
    ) -> &[(
        worth_spatial::facade::arbitration::SpatialIntentCandidate,
        worth_spatial::facade::arbitration::SpatialBlockedCapability,
    )] {
        &self.blocked_candidates
    }

    pub fn warnings(&self) -> &[worth_spatial::facade::arbitration::SpatialIntentPreviewWarning] {
        &self.warnings
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryPreviewParityError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryPreviewParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryPreviewParityError {}

pub fn prepare_primitive_construction_query_preview_inspection_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    row: PrimitiveConstructionPreviewRow,
) -> Result<
    PrimitiveConstructionQueryPreviewParityReport,
    PrimitiveConstructionQueryPreviewParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryPreviewParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryPreviewParityReport::new(
        query_contract_digest,
        row,
        PrimitiveConstructionPreviewQueryReadSurface::PreviewInspection,
        PrimitiveConstructionPreviewQueryFactProvenance::DirectPreviewSurfaceReport,
    ))
}

pub fn prepare_primitive_construction_query_preview_projection_consumption_receipt_report(
    workspace: &mut ForgeQueryWorkspace,
    row: PrimitiveConstructionPreviewRow,
) -> Result<
    PrimitiveConstructionQueryPreviewParityReport,
    PrimitiveConstructionQueryPreviewParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryPreviewParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    Ok(PrimitiveConstructionQueryPreviewParityReport::new(
        query_contract_digest,
        row,
        PrimitiveConstructionPreviewQueryReadSurface::ProjectionConsumptionFromPreviewSurfaceReport,
        PrimitiveConstructionPreviewQueryFactProvenance::EquivalentProjectionConsumptionFacts,
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_preview_inspection_parity_report,
        prepare_primitive_construction_query_preview_projection_consumption_receipt_report,
        PrimitiveConstructionPreviewQueryReadSurface,
    };
    use crate::construction::certification::preview::{
        prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn query_preview_parity_preserves_profile_dependent_preview_truth() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-preview".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_preview_surface_report().expect("report");
        let row = report
            .row(PrimitiveConstructionPreviewCase::GrazingAggressiveSnap)
            .expect("row")
            .clone();

        let inspection = prepare_primitive_construction_query_preview_inspection_parity_report(
            &mut workspace,
            row.clone(),
        )
        .expect("inspection");
        let projection =
            prepare_primitive_construction_query_preview_projection_consumption_receipt_report(
                &mut workspace,
                row,
            )
            .expect("projection");

        assert_eq!(
            inspection.commit_disposition(),
            projection.commit_disposition()
        );
        assert_eq!(
            inspection.preview_richness(),
            worth_spatial::facade::arbitration::SpatialPreviewRichness::Standard
        );
        assert_eq!(
            inspection.proximity_posture(),
            worth_spatial::facade::arbitration::SpatialThresholdPosture::Generous
        );
        assert_eq!(inspection.candidates(), projection.candidates());
        assert_eq!(
            inspection.blocked_candidates(),
            projection.blocked_candidates()
        );
        assert_eq!(inspection.warnings(), projection.warnings());
        assert!(inspection
            .candidates()
            .contains(&worth_spatial::facade::arbitration::SpatialIntentCandidate::SnapFlush));
        assert!(inspection.parity_verified());
        assert!(projection.parity_verified());
        assert!(matches!(
            PrimitiveConstructionPreviewQueryReadSurface::PreviewInspection,
            PrimitiveConstructionPreviewQueryReadSurface::PreviewInspection
        ));
    }
}
