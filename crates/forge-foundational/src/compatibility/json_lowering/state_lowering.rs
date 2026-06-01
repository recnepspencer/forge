use forge_proof::TransitionOutcome;

use crate::aspects::admit_authoritative_record_aspect_state;

use super::super::{JsonCompatibilityAspectInput, JsonCompatibilityLoweringDenial};
use super::{lower_json_aspect_value, JsonCompatibilityLoweringOutcome};

pub fn lower_json_record_aspect_state(
    inputs: impl IntoIterator<Item = JsonCompatibilityAspectInput>,
) -> JsonCompatibilityLoweringOutcome<crate::aspects::AuthoritativeRecordAspectStateArtifact> {
    let mut lowered_entries = Vec::new();

    for input in inputs {
        match lower_json_aspect_value(input.contract(), input.source().clone(), input.value()) {
            TransitionOutcome::Success(artifact) => lowered_entries.push(artifact),
            TransitionOutcome::Denied(denial) => return TransitionOutcome::denied(denial),
            TransitionOutcome::Deferred(deferred) => return TransitionOutcome::deferred(deferred),
            TransitionOutcome::Stale(stale) => return TransitionOutcome::stale(stale),
            TransitionOutcome::RebindRequired(rebind) => {
                return TransitionOutcome::rebind_required(rebind);
            }
            TransitionOutcome::Failed(failure) => return TransitionOutcome::failed(failure),
        }
    }

    match admit_authoritative_record_aspect_state(lowered_entries) {
        TransitionOutcome::Success(state) => TransitionOutcome::success(state),
        TransitionOutcome::Denied(denial) => TransitionOutcome::denied(
            JsonCompatibilityLoweringDenial::StateAdmissionDenied(denial),
        ),
        TransitionOutcome::Deferred(_)
        | TransitionOutcome::Stale(_)
        | TransitionOutcome::RebindRequired(_)
        | TransitionOutcome::Failed(_) => unreachable!("state admission uses only denied"),
    }
}
