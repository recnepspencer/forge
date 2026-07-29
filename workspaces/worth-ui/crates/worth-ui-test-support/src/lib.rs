//! SUPPORT AUTHORITY — synthetic fixtures for falsifying production surfaces.
//!
//! Sole public home for cross-crate certification fixtures. Do not import
//! `worth_ui_runtime::certification_support` from product code.

pub use worth_ui_runtime::certification_support::{
    identity_overlay_projection_for_certification, launch_empty_runtime_for_certification,
    planning_pair_for_certification_suite, runtime_origin_fixture,
    with_activation_precommit_interruption, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementMode, UiGraphFactConsumerIdentity, UiGraphFactIndexBasis,
    UiGraphFactIndexEntry, UiGraphFactLookupCost, UiGraphFactLookupDenial,
    UiGraphFactLookupReceipt, UiIdentityOverlayProjectionCertificationMutation,
    UiMeasurementAdmissionPosture, UiMeasurementCapabilityGateReason,
    UiMeasurementUnsupportedReason, UiMountedVisualOverlayLeaseCertificationReceipt,
    UiQueryMeasurementEligibilityPosture, UiQueryMeasurementSourceIdentity,
    UiQueryMeasurementUnsupportedQueryReason, UiRebindPlanningBasisMutation,
    UiRepeatedInstanceIdentityCertificationRow, UiResolvedIdentityLifecycleCertificationExt,
    WorthUiActivationPrecommitStage, WorthUiActiveSessionCertificationExt,
    WorthUiApplicationBuilderCertificationExt, WorthUiApplicationGraphCertificationExt,
    WorthUiApplicationReplacementCertificationExt, WorthUiFrameworkTurnCertificationExt,
    WorthUiMountedAllocationCertificationExt, WorthUiMountedAllocationInspectionCertificationExt,
    WorthUiMountedFrameExecutionCertificationExt, WorthUiMountedIdentityCertificationExt,
    WorthUiMountedPublicationCertificationExt, WorthUiTouchOriginCertificationFixture,
    WorthUiTouchOriginFixtureVariant,
};
