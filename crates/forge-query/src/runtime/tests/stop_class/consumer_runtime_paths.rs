use super::super::support::*;
use super::completeness_support::runtime_paths::{
    intent_commit_denied_error, intent_execution_routing_failed_error,
    preview_promotion_atomic_batch_unsupported_error, preview_promotion_rebinding_required_error,
    preview_promotion_stale_basis_error, preview_promotion_write_failed_error,
    read_domain_invariant_denied_error,
};
use super::consumer_support::routing::{route_consumer_stop_class, ConsumerStopRoute};
use super::consumer_support::runtime_errors::temporal_public_family_admission_error;

#[test]
fn consumer_router_handles_runtime_generated_stop_classes_without_string_matching() {
    let public_family_error = temporal_public_family_admission_error(
        "consumer-stop-class-public-family",
        "runtime-backed temporal family stays support-gated here",
    );
    assert_eq!(
        route_consumer_stop_class(&public_family_error),
        ConsumerStopRoute::FamilyAdmissionDenied {
            family: ForgeQueryRuntimeFacadeFamily::Temporal,
            status: ForgeQueryRuntimeFamilySupportStatus::Supported,
            teaching_posture: Some(ForgeQueryRuntimeFamilyTeachingPosture::SupportGateOnly),
        }
    );

    let intent_error = intent_commit_denied_error();
    assert_eq!(
        route_consumer_stop_class(&intent_error),
        ConsumerStopRoute::IntentCommitDenied
    );

    let preview_cases = [
        (
            preview_promotion_stale_basis_error(),
            ForgeQueryPreviewPromotionDenialKind::StaleBasis,
        ),
        (
            preview_promotion_atomic_batch_unsupported_error(),
            ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported,
        ),
        (
            preview_promotion_rebinding_required_error(),
            ForgeQueryPreviewPromotionDenialKind::RebindingRequired,
        ),
        (
            preview_promotion_write_failed_error(),
            ForgeQueryPreviewPromotionDenialKind::WriteFailed,
        ),
    ];
    for (preview_error, expected_kind) in preview_cases {
        assert_eq!(
            route_consumer_stop_class(&preview_error),
            ConsumerStopRoute::PreviewPromotionDenied(expected_kind)
        );
    }

    let read_error = read_domain_invariant_denied_error();
    assert_eq!(
        route_consumer_stop_class(&read_error),
        ConsumerStopRoute::ReadCompositionDomainInvariantDenied {
            hook_family: "domain_invariant_pack_hook",
            invariant_family: "no_traversal_reads",
        }
    );

    let routing_error = intent_execution_routing_failed_error();
    assert_eq!(
        route_consumer_stop_class(&routing_error),
        ConsumerStopRoute::IntentExecutionRoutingFailed(
            ForgeQueryRuntimeDeclarationFailureKind::LiveSubscriptionInstallation,
        )
    );
}
