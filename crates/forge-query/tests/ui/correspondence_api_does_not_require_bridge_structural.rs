use forge_query::facade::{
    resolve_correspondence_evidence, CorrespondenceEvaluationError, CorrespondenceEvidenceResolved,
};
use forge_runtime_bridge::facade::ReducedStructuralMatchSet;

fn main() {
    let _: fn(
        ReducedStructuralMatchSet,
    ) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> =
        resolve_correspondence_evidence;
}
