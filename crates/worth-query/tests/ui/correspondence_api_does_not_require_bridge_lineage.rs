use worth_query::facade::foundation::{resolve_correspondence_evidence, CorrespondenceEvaluationError, CorrespondenceEvidenceResolved};
use worth_runtime_bridge::facade::BridgeHistoricalLineageAuthority;

fn main() {
    let _: fn(
        BridgeHistoricalLineageAuthority,
    ) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> =
        resolve_correspondence_evidence;
}
