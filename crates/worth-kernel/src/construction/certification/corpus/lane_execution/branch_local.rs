use forge_query::facade::{
    ForgeQueryBranchOptions, ForgeQueryPreviewOptions, ForgeQueryRuntimeError,
    ForgeQueryRuntimeFacadeFamily, ForgeQuerySessionLabel, ForgeQueryWorkspace,
};

use crate::construction::intent::PrimitiveConstructionIntent;

use super::{PrimitiveConstructionCorpusBranchLocalLane, PrimitiveConstructionCorpusLaneGap};

#[derive(Debug)]
pub(crate) enum PrimitiveConstructionCorpusBranchLocalLaneError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionCorpusBranchLocalLaneError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionCorpusBranchLocalLaneError {}

pub(crate) fn prepare_branch_local_lane(
    workspace: &mut ForgeQueryWorkspace,
    lane_label: &str,
    scenario_id: &str,
    intent: &PrimitiveConstructionIntent,
) -> Result<
    PrimitiveConstructionCorpusBranchLocalLane,
    PrimitiveConstructionCorpusBranchLocalLaneError,
> {
    let family = intent.family();
    let branch_preview_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
        .map_err(PrimitiveConstructionCorpusBranchLocalLaneError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let preview_admission_digest = {
        let preview = workspace
            .preview_with_options(
                ForgeQuerySessionLabel::scoped_strs(
                    "worth-kernel",
                    [family.as_str(), lane_label, scenario_id, "preview"],
                )
                .expect("preview label"),
                ForgeQueryPreviewOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionCorpusBranchLocalLaneError::QueryRuntime)?;
        preview
            .basis_admission()
            .admission_identity()
            .terminal_projection_for_reporting()
            .to_string()
    };
    let branch_admission_digest = {
        let branch = workspace
            .branch_with_options(
                ForgeQuerySessionLabel::scoped_strs(
                    "worth-kernel",
                    [family.as_str(), lane_label, scenario_id, "branch"],
                )
                .expect("branch label"),
                ForgeQueryBranchOptions::sandboxed_write_intent(),
            )
            .map_err(PrimitiveConstructionCorpusBranchLocalLaneError::QueryRuntime)?;
        branch
            .basis_admission()
            .admission_identity()
            .terminal_projection_for_reporting()
            .to_string()
    };
    let execution_gap = PrimitiveConstructionCorpusLaneGap::new(
        "branch_local_execution_surface_missing",
        format!(
            "primitive construction for {} does not yet lower to a branch-local Query intent receipt; only BranchPreview basis admission is currently runtime-backed",
            family.as_str()
        ),
    );
    Ok(PrimitiveConstructionCorpusBranchLocalLane::new(
        branch_preview_contract_digest,
        preview_admission_digest,
        branch_admission_digest,
        execution_gap,
    ))
}
