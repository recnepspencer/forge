use worth_query::facade::foundation::{resolve_correspondence_evidence, CorrespondenceEvaluationError, CorrespondenceEvidenceResolved};
use worth_runtime_bridge::facade::ReducedStructuralMatchSet;

fn main() {
    let _: fn(
        ReducedStructuralMatchSet,
    ) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> =
        resolve_correspondence_evidence;
}
