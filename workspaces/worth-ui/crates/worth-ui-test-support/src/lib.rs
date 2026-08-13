//! SUPPORT AUTHORITY — synthetic fixtures for falsifying production surfaces.
//!
//! Sole public home for cross-crate certification fixtures. Do not import
//! `worth_ui_runtime::certification_support` from product code.

pub use worth_ui_runtime::certification_support::{
    classify_intent_operability_for_certification, draft_recipient_contract_for_certification,
    empty_projection_for_certification, identity_overlay_projection_for_certification,
    initial_presentation_mechanics_for_certification, launch_empty_runtime_for_certification,
    planning_pair_for_certification_suite, runtime_origin_fixture,
    semantic_text_layout_resolver_for_certification, semantic_text_projection_for_certification,
    semantic_text_projection_for_certification_with_capability,
    with_activation_precommit_interruption, UiCertificationQualifiedTextResolver,
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementMode, UiGraphFactConsumerIdentity,
    UiGraphFactIndexBasis, UiGraphFactIndexEntry, UiGraphFactLookupCost, UiGraphFactLookupDenial,
    UiGraphFactLookupReceipt, UiIdentityOverlayProjectionCertificationMutation,
    UiIntentOccupancyReleasePosture, UiIntentOccupancyReservation,
    UiIntentOccupancyReservationDenial, UiIntentOperabilityDecisionCertificationInput,
    UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason, UiMountedVisualOverlayLeaseCertificationReceipt,
    UiQueryMeasurementEligibilityPosture, UiQueryMeasurementSourceIdentity,
    UiQueryMeasurementUnsupportedQueryReason, UiRebindPlanningBasisMutation,
    UiRepeatedInstanceIdentityCertificationRow, UiResolvedIdentityLifecycleCertificationExt,
    UiSemanticTextProjectionCertificationMutation, WorthUiActivationPrecommitStage,
    WorthUiActiveSessionCertificationExt, WorthUiApplicationBuilderCertificationExt,
    WorthUiApplicationGraphCertificationExt, WorthUiApplicationReplacementCertificationExt,
    WorthUiFrameworkTurnCertificationExt, WorthUiIntentOccupancyCertificationExt,
    WorthUiLocalInputRecipientCertificationExt, WorthUiMountedAllocationCertificationExt,
    WorthUiMountedAllocationInspectionCertificationExt,
    WorthUiMountedFrameExecutionCertificationExt, WorthUiMountedIdentityCertificationExt,
    WorthUiMountedInteractionLifecycleCertificationExt, WorthUiMountedPublicationCertificationExt,
    WorthUiTouchOriginCertificationFixture, WorthUiTouchOriginFixtureVariant,
};
