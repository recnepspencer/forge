use super::super::types::{EvaluationCursor, EvaluationPlan, ExecutionStage};

pub(super) fn materialize_plan_from_cursor(cursor: EvaluationCursor) -> EvaluationPlan {
    let EvaluationCursor {
        request_mode,
        targets,
        tasks,
        stages: stage_cursors,
        summary,
    } = cursor;
    let mut remaining_tasks = tasks.into_iter();
    let mut consumed = 0usize;
    let mut stages = Vec::with_capacity(stage_cursors.len());
    for stage in &stage_cursors {
        debug_assert_eq!(stage.start, consumed);
        let stage_len = stage.end.saturating_sub(stage.start);
        stages.push(ExecutionStage {
            index: stage.index,
            tasks: remaining_tasks.by_ref().take(stage_len).collect(),
            barrier: stage.barrier,
        });
        consumed = stage.end;
    }
    EvaluationPlan {
        request_mode,
        targets,
        stages,
        summary,
    }
}
