use crate::data::proof::SingleConsumer;
use crate::logic::evaluation::PendingDependencySnapshot;
use crate::logic::planner::semantic::StageSemanticBatch;

/// Stage-lifetime workspace for lowered apply, snapshot deferral, and semantic finalize.
#[derive(Debug)]
pub(crate) struct StageScratch {
    pub(in crate::logic::planner) semantic_batch: SingleConsumer<StageSemanticBatch>,
    pub(in crate::logic::planner) pending_snapshots: Vec<PendingDependencySnapshot>,
}
