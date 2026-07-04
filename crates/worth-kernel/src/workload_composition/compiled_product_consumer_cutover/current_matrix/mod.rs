mod public_closeout_consumers;
mod query_boundary_consumers;
mod replay_undo_consumers;
mod spatial_consumers;
mod topology_consumers;

use super::coverage_target::KernelCompiledProductConsumerCoverageTarget;
use super::error::KernelCompiledProductConsumerDependencyError;

pub(crate) fn current_coverage_targets() -> Result<
    Vec<KernelCompiledProductConsumerCoverageTarget>,
    KernelCompiledProductConsumerDependencyError,
> {
    let mut rows = spatial_consumers::current_spatial_consumer_rows()?;
    rows.extend(topology_consumers::current_topology_consumer_rows()?);
    rows.extend(replay_undo_consumers::current_replay_undo_consumer_rows()?);
    rows.extend(public_closeout_consumers::current_public_closeout_consumer_rows()?);
    rows.extend(query_boundary_consumers::current_query_boundary_consumer_rows()?);
    Ok(rows)
}
