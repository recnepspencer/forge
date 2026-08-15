use super::{ExpectedSealedOriginBinding, ExpectedTrace, ExpectedWorkRecord};
use crate::tests::domains::fintech::world::FinancialStructuralMutation;

pub(super) fn apply_structural_trace(
    structural: FinancialStructuralMutation,
    trace: &mut ExpectedTrace,
) {
    trace.evaluations.insert(structural.target);
    trace.evaluation_occurrences += 1;
    let readiness_epoch = trace.allocate_readiness_epoch();
    trace.work_records.push(ExpectedWorkRecord {
        target: structural.target,
        dependency_revision: structural.resulting_dependency_revision,
        readiness_epoch,
        stage_order: 0,
        sealed_origin: ExpectedSealedOriginBinding::StructuralRecompute {
            structural_generation: structural.resulting_dependency_revision,
        },
    });
}
