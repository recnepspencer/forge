use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::{
    prepare_primitive_construction_continuity_surface_report, PrimitiveConstructionContinuityCase,
    PrimitiveConstructionContinuityRow, PrimitiveConstructionContinuitySurfaceReportError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisLaneReport;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionContinuityBranchPreviewRuntimeReport {
    case: PrimitiveConstructionContinuityCase,
    continuity_row: PrimitiveConstructionContinuityRow,
    branch_preview_contract_digest: String,
    preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    report_digest: String,
}

impl PrimitiveConstructionContinuityBranchPreviewRuntimeReport {
    fn new(
        case: PrimitiveConstructionContinuityCase,
        continuity_row: PrimitiveConstructionContinuityRow,
        branch_preview_contract_digest: String,
        preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
        branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    ) -> Self {
        let report_digest = digest_owned_parts(&[
            format!("{case:?}"),
            continuity_row.row_digest().to_string(),
            branch_preview_contract_digest.clone(),
            preview_lane.admission_digest().to_string(),
            branch_lane.admission_digest().to_string(),
        ]);
        Self {
            case,
            continuity_row,
            branch_preview_contract_digest,
            preview_lane,
            branch_lane,
            report_digest,
        }
    }

    pub fn case(&self) -> PrimitiveConstructionContinuityCase {
        self.case
    }

    pub fn continuity_row(&self) -> &PrimitiveConstructionContinuityRow {
        &self.continuity_row
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
pub enum PrimitiveConstructionContinuityBranchPreviewRuntimeError {
    ContinuityReport(PrimitiveConstructionContinuitySurfaceReportError),
    MissingRow(PrimitiveConstructionContinuityCase),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionContinuityBranchPreviewRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ContinuityReport(error) => write!(f, "{error}"),
            Self::MissingRow(case) => write!(f, "missing continuity row for {case:?}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionContinuityBranchPreviewRuntimeError {}

pub fn prepare_primitive_construction_continuity_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionContinuityCase,
) -> Result<
    PrimitiveConstructionContinuityBranchPreviewRuntimeReport,
    PrimitiveConstructionContinuityBranchPreviewRuntimeError,
> {
    let report = prepare_primitive_construction_continuity_surface_report()
        .map_err(PrimitiveConstructionContinuityBranchPreviewRuntimeError::ContinuityReport)?;
    let continuity_row = report
        .row(case)
        .ok_or(PrimitiveConstructionContinuityBranchPreviewRuntimeError::MissingRow(case))?
        .clone();
    let branch_preview_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .map_err(PrimitiveConstructionContinuityBranchPreviewRuntimeError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let preview_lane = {
        let preview = workspace
            .preview_with_options(
                format!("worth-kernel.continuity.{case:?}.preview"),
                forge_query::facade::ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionContinuityBranchPreviewRuntimeError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_preview(preview.basis_admission())
    };
    let branch_lane = {
        let branch = workspace
            .branch_with_options(
                format!("worth-kernel.continuity.{case:?}.branch"),
                forge_query::facade::ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionContinuityBranchPreviewRuntimeError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_branch(branch.basis_admission())
    };
    Ok(
        PrimitiveConstructionContinuityBranchPreviewRuntimeReport::new(
            case,
            continuity_row,
            branch_preview_contract_digest,
            preview_lane,
            branch_lane,
        ),
    )
}

#[cfg(test)]
mod tests {
    use forge_query::facade::ForgeQueryAuthorityLane;

    use super::prepare_primitive_construction_continuity_branch_preview_runtime_report;
    use crate::construction::PrimitiveConstructionContinuityCase;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn continuity_branch_preview_runtime_report_preserves_identity_truth_with_runtime_lanes() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.continuity-branch-runtime".to_string(),
        )
        .expect("workspace");

        let report = prepare_primitive_construction_continuity_branch_preview_runtime_report(
            &mut workspace,
            PrimitiveConstructionContinuityCase::HostAttachReinterpreted,
        )
        .expect("report");

        assert_eq!(
            report.continuity_row().continuity_class(),
            worth_spatial::facade::arbitration::SpatialIdentityContinuityClass::IdentityReinterpreted
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
        assert!(!report.report_digest().is_empty());
    }
}
