use forge_query::facade::{
    ForgeQueryBranchOptions, ForgeQueryPreviewOptions, ForgeQueryRuntimeError,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::{
    prepare_primitive_construction_preview_surface_report, PrimitiveConstructionPreviewCase,
    PrimitiveConstructionPreviewRow, PrimitiveConstructionPreviewSurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisLaneReport;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPreviewBranchPreviewRuntimeReport {
    case: PrimitiveConstructionPreviewCase,
    preview_row: PrimitiveConstructionPreviewRow,
    branch_preview_contract_digest: String,
    preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    report_digest: String,
}

impl PrimitiveConstructionPreviewBranchPreviewRuntimeReport {
    fn new(
        case: PrimitiveConstructionPreviewCase,
        preview_row: PrimitiveConstructionPreviewRow,
        branch_preview_contract_digest: String,
        preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
        branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    ) -> Self {
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            preview_row.row_digest().to_string(),
            branch_preview_contract_digest.clone(),
            preview_lane.admission_digest().to_string(),
            branch_lane.admission_digest().to_string(),
        ]);
        Self {
            case,
            preview_row,
            branch_preview_contract_digest,
            preview_lane,
            branch_lane,
            report_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionPreviewCase {
        self.case
    }

    pub fn preview_row(&self) -> &PrimitiveConstructionPreviewRow {
        &self.preview_row
    }

    pub fn branch_preview_contract_digest(&self) -> &str {
        &self.branch_preview_contract_digest
    }

    pub fn preview_lane(&self) -> &PrimitiveConstructionRuntimeBasisLaneReport {
        &self.preview_lane
    }

    pub fn branch_lane(&self) -> &PrimitiveConstructionRuntimeBasisLaneReport {
        &self.branch_lane
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionPreviewBranchPreviewRuntimeError {
    PreviewReport(PrimitiveConstructionPreviewSurfaceReportError),
    MissingRow(PrimitiveConstructionPreviewCase),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionPreviewBranchPreviewRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreviewReport(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing preview row for {case:?}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPreviewBranchPreviewRuntimeError {}

pub fn prepare_primitive_construction_preview_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPreviewCase,
) -> Result<
    PrimitiveConstructionPreviewBranchPreviewRuntimeReport,
    PrimitiveConstructionPreviewBranchPreviewRuntimeError,
> {
    let report = prepare_primitive_construction_preview_surface_report()
        .map_err(PrimitiveConstructionPreviewBranchPreviewRuntimeError::PreviewReport)?;
    let preview_row = report
        .row(case)
        .ok_or(PrimitiveConstructionPreviewBranchPreviewRuntimeError::MissingRow(case))?
        .clone();
    let branch_preview_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .map_err(PrimitiveConstructionPreviewBranchPreviewRuntimeError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let preview_lane = {
        let preview = workspace
            .preview_with_options(
                format!("worth-kernel.preview.{case:?}.preview"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionPreviewBranchPreviewRuntimeError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_preview(preview.basis_admission())
    };
    let branch_lane = {
        let branch = workspace
            .branch_with_options(
                format!("worth-kernel.preview.{case:?}.branch"),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionPreviewBranchPreviewRuntimeError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_branch(branch.basis_admission())
    };
    Ok(PrimitiveConstructionPreviewBranchPreviewRuntimeReport::new(
        case,
        preview_row,
        branch_preview_contract_digest,
        preview_lane,
        branch_lane,
    ))
}

#[cfg(test)]
mod tests {
    use forge_query::facade::ForgeQueryAuthorityLane;

    use super::prepare_primitive_construction_preview_branch_preview_runtime_report;
    use crate::construction::PrimitiveConstructionPreviewCase;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn preview_branch_runtime_preserves_preview_truth_and_runtime_lanes() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.preview-branch-runtime".to_string(),
        )
        .expect("workspace");

        let report = prepare_primitive_construction_preview_branch_preview_runtime_report(
            &mut workspace,
            PrimitiveConstructionPreviewCase::HostFaceBimAttach,
        )
        .expect("report");

        assert_eq!(
            report.preview_row().commit_disposition(),
            worth_spatial::facade::arbitration::SpatialIntentPreviewCommitDisposition::WouldAutoResolve(
                worth_spatial::facade::arbitration::SpatialIntentCandidate::AttachRelationally
            )
        );
        assert_eq!(
            report.preview_lane().authority_lane(),
            ForgeQueryAuthorityLane::PreviewTruth
        );
        assert_eq!(
            report.branch_lane().authority_lane(),
            ForgeQueryAuthorityLane::BranchLocalTruth
        );
        assert!(!report.branch_preview_contract_digest().is_empty());
    }
}
