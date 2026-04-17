use forge_query::facade::{
    resolve_correspondence_evidence, CorrespondenceEvaluationError, CorrespondenceEvidenceResolved,
};
use forge_runtime_bridge::facade::BridgeHistoricalLineageAuthority;

fn main() {
    let _: fn(
        BridgeHistoricalLineageAuthority,
    ) -> Result<CorrespondenceEvidenceResolved, CorrespondenceEvaluationError> =
        resolve_correspondence_evidence;
}
