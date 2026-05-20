use super::acceptance_checks::{
    required_concrete_seam_coverage_digest, synthetic_tail_exactness_digest,
};
use super::{
    forge_query_lower_runtime_representative_surface,
    ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
};
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey;

#[test]
fn required_phase_six_concrete_seams_are_enforced_hostilely() {
    let surface = forge_query_lower_runtime_representative_surface().with_evidence_source_override(
        ForgeQueryLowerRuntimeSeamKey::SubscriptionActivation,
        ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
    );

    let panic = std::panic::catch_unwind(|| required_concrete_seam_coverage_digest(&surface))
        .expect_err("required concrete seam fallback must fail acceptance");
    let message = panic_message(panic);

    assert!(message.contains("subscription-activation"));
}

#[test]
fn synthetic_tail_exactness_rejects_unexpected_runtime_backing_drift() {
    let surface = forge_query_lower_runtime_representative_surface().with_evidence_source_override(
        ForgeQueryLowerRuntimeSeamKey::ComposeRead,
        ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
    );

    let panic = std::panic::catch_unwind(|| synthetic_tail_exactness_digest(&surface))
        .expect_err("synthetic tail drift must fail acceptance");
    let message = panic_message(panic);

    assert!(message.contains("synthetic surface width drifted"));
}

fn panic_message(payload: Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    if let Some(message) = payload.downcast_ref::<&str>() {
        return (*message).to_string();
    }
    "non-string panic payload".to_string()
}
