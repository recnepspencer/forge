use crate::certification::error::TopologyCertificationError;

use super::super::support::{closed_row, collect_rs_sources, ensure, source_text};
use super::super::TopologyQueryBoundaryCleanupArea;

pub(crate) fn certify_basis_adapter_row(
) -> Result<super::super::TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let basis_context =
        source_text("src/projection/runtime_boundary/read_execution/basis_context.rs")?;
    let family_execution =
        source_text("src/projection/runtime_boundary/read_execution/family_execution.rs")?;
    let handle_reads = source_text("src/projection/read_views/domain/handle_reads.rs")?;
    let contracts = source_text("src/projection/runtime_boundary/query_runtime/contracts.rs")?;
    let read_proof_harness = source_text("src/certification/support/read_proof_harness.rs")?;
    let query_runtime_sources =
        collect_rs_sources("src/projection/runtime_boundary/query_runtime")?;

    ensure(basis_context.contains("pub(crate) enum TopologyReadExecutionTarget"))?;
    ensure(basis_context.contains("HistoricalSnapshot"))?;
    ensure(basis_context.contains("snapshot_identity: ForgeQuerySnapshotIdentity"))?;
    ensure(family_execution.contains("execution_target.execute_family("))?;
    ensure(handle_reads.contains("TopologyReadExecutionTarget::current_head()"))?;
    ensure(handle_reads.contains("TopologyReadExecutionTarget::historical_snapshot("))?;
    ensure(!contracts.contains("workspace_requires_historical_basis_context"))?;
    ensure(read_proof_harness.contains("historical_from_workspace_token"))?;
    ensure(!read_proof_harness.contains("SNAPSHOT_HISTORICAL_BASIS_EVIDENCE"))?;
    ensure(!read_proof_harness.contains("public_api_contract()"))?;

    let public_api_contract_mentions = query_runtime_sources
        .iter()
        .filter(|(_, source)| source.contains("public_api_contract()"))
        .map(|(path, _)| path.clone())
        .collect::<Vec<_>>();
    ensure(public_api_contract_mentions.is_empty())?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::BasisAdapter,
        "configured read handles now choose current-vs-historical execution explicitly and read execution no longer infers historical posture by inspecting workspace runtime contracts",
        Some("src/projection/read_views/domain/handle_reads.rs"),
        [
            "src/projection/runtime_boundary/read_execution/basis_context.rs",
            "src/projection/runtime_boundary/read_execution/family_execution.rs",
            "src/projection/read_views/domain/handle_reads.rs",
        ],
    )
}
