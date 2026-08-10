use super::activation_world::activation_for;
use super::preview_certification_world::preview_certification;
use super::preview_discard_world::preview_discard_certification_artifacts;
use super::preview_promotion_world::preview_promotion_certification_artifacts;
use super::*;
use crate::live::LiveQueryFamily;
use crate::view_shape_live::LiveViewShapeFamily;

#[test]
fn lifecycle_certification_emits_preview_discard_and_support_evidence() {
    let artifacts = preview_discard_certification_artifacts();

    let bundle = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        preview_certification(&artifacts.preview),
        &artifacts.closeout,
    )
    .unwrap();

    assert_ne!(bundle.preview_isolation_projection().label(), "none");
    assert_ne!(bundle.preview_residue_projection().label(), "none");
    assert!(
        !bundle.counter_sequence_identity().as_str().is_empty(),
        "preview discard certification should include typed counter sequence identity"
    );
    assert!(!bundle.support_matrix_projection().label().is_empty());
}

#[test]
fn lifecycle_certification_emits_preview_promotion_boundary_evidence() {
    let artifacts = preview_promotion_certification_artifacts();

    let bundle = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        preview_certification(&artifacts.preview),
        &artifacts.closeout,
    )
    .unwrap();

    assert_ne!(bundle.preview_isolation_projection().label(), "none");
    assert_ne!(bundle.preview_residue_projection().label(), "none");
    assert!(
        !bundle.counter_sequence_identity().as_str().is_empty(),
        "preview promotion certification should include typed counter sequence identity"
    );
}

#[test]
fn lifecycle_certification_denies_preview_promotion_with_foreign_handoff_source() {
    let artifacts = preview_promotion_certification_artifacts();
    let SubscriptionLifecyclePreviewCertificationArtifacts::Promotion {
        isolation,
        residue_report,
        ..
    } = &artifacts.preview
    else {
        panic!("expected preview promotion artifacts");
    };
    let foreign_residue_report = measure_preview_subscription_residue(
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(0),
        PreviewResidueWidth::measured(1),
    );
    let mut foreign_runtime = ActiveSubscriptionRuntime::new();
    let foreign_authoritative_admission = admit_active_subscription_lane(
        activation_for(
            LiveQueryFamily::OrderedCollection,
            Some(LiveViewShapeFamily::Table),
        ),
        ActiveSubscriptionWorkBudget::admitted(
            ActiveRegistryLookupWidth::measured(1),
            ActiveFanoutWidth::measured(1),
            ActiveAllocationScopeWidth::measured(1),
            ActiveSubscriptionAllocationPosture::LifecycleArena,
        ),
    )
    .unwrap();
    let foreign_authoritative_handle =
        open_active_subscription_lane(&mut foreign_runtime, foreign_authoritative_admission)
            .unwrap();
    let foreign_handoff = promote_preview_subscription(
        isolation.clone(),
        &foreign_residue_report,
        &foreign_authoritative_handle,
        "foreign-authority",
    )
    .unwrap();

    let error = certify_subscription_lifecycle(
        artifacts.context,
        &artifacts.admission,
        &artifacts.activation,
        &artifacts.scale_report,
        &artifacts.active_admission,
        &artifacts.handle,
        &artifacts.attachment,
        artifacts.delivery_batch.delivery_window_identity(),
        &artifacts.delta,
        &artifacts.lowering_report,
        &artifacts.work_packet,
        &artifacts.delivery_batch,
        &artifacts.acknowledged_attachment,
        artifacts.continuation_report.as_ref(),
        SubscriptionLifecyclePreviewCertification::Promotion {
            isolation,
            residue_report,
            promotion_handoff: &foreign_handoff,
        },
        &artifacts.closeout,
    )
    .unwrap_err();

    assert_eq!(
        error.denial_kind(),
        &SubscriptionLifecycleCertificationDenialKind::PreviewSourceMismatch
    );
}
