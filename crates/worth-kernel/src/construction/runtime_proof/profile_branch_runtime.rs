use forge_query::facade::{
    ForgeQueryBranchOptions, ForgeQueryPreviewOptions, ForgeQueryRuntimeError,
    ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};

use crate::construction::certification::{
    prepare_primitive_construction_policy_profile_report, PrimitiveConstructionPolicyProfileCase,
    PrimitiveConstructionPolicyProfileRow,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::runtime_basis::PrimitiveConstructionRuntimeBasisLaneReport;

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport {
    profile_row: PrimitiveConstructionPolicyProfileRow,
    branch_preview_contract_digest: String,
    preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    report_digest: String,
}

impl PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport {
    fn new(
        profile_row: PrimitiveConstructionPolicyProfileRow,
        branch_preview_contract_digest: String,
        preview_lane: PrimitiveConstructionRuntimeBasisLaneReport,
        branch_lane: PrimitiveConstructionRuntimeBasisLaneReport,
    ) -> Self {
        let report_digest = digest_owned_parts(&[
            profile_row.row_digest().to_string(),
            branch_preview_contract_digest.clone(),
            preview_lane.admission_digest().to_string(),
            branch_lane.admission_digest().to_string(),
        ]);
        Self {
            profile_row,
            branch_preview_contract_digest,
            preview_lane,
            branch_lane,
            report_digest,
        }
    }

    pub fn profile_row(&self) -> &PrimitiveConstructionPolicyProfileRow {
        &self.profile_row
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
pub enum PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError {
    MissingRow(PrimitiveConstructionPolicyProfileCase),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRow(case) => write!(f, "missing policy profile row for {case:?}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError {}

pub fn prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
    workspace: &mut ForgeQueryWorkspace,
    case: PrimitiveConstructionPolicyProfileCase,
) -> Result<
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport,
    PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError,
> {
    let report = prepare_primitive_construction_policy_profile_report();
    let profile_row = report
        .row(case)
        .ok_or(PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError::MissingRow(case))?
        .clone();
    let branch_preview_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .map_err(PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let preview_lane = {
        let preview = workspace
            .preview_with_options(
                format!("worth-kernel.profile.{case:?}.preview"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_preview(preview.basis_admission())
    };
    let branch_lane = {
        let branch = workspace
            .branch_with_options(
                format!("worth-kernel.profile.{case:?}.branch"),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionPolicyProfileBranchPreviewRuntimeError::QueryRuntime)?;
        PrimitiveConstructionRuntimeBasisLaneReport::from_branch(branch.basis_admission())
    };
    Ok(
        PrimitiveConstructionPolicyProfileBranchPreviewRuntimeReport::new(
            profile_row,
            branch_preview_contract_digest,
            preview_lane,
            branch_lane,
        ),
    )
}

#[cfg(test)]
mod tests {
    use forge_query::facade::ForgeQueryAuthorityLane;

    use super::prepare_primitive_construction_policy_profile_branch_preview_runtime_report;
    use crate::construction::PrimitiveConstructionPolicyProfileCase;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };
    use worth_spatial::facade::arbitration::SpatialPreviewRichness;

    #[test]
    fn policy_profile_branch_runtime_preserves_profile_truth_and_runtime_lanes() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.profile-branch-runtime".to_string(),
        )
        .expect("workspace");

        let report = prepare_primitive_construction_policy_profile_branch_preview_runtime_report(
            &mut workspace,
            PrimitiveConstructionPolicyProfileCase::HighFidelityPreview,
        )
        .expect("report");

        assert_eq!(
            report.profile_row().preview_richness(),
            SpatialPreviewRichness::HighFidelity
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
