use worth_runtime_bridge::facade::{BridgeTruthViewEvaluationRequest, TruthBranchIdentity};

use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::lower_runtime_routing::{
    WorthQueryLowerRuntimeAuthorityOwner, WorthQueryLowerRuntimeBoundaryEnvelope,
    WorthQueryLowerRuntimeBoundaryExecutionReceipt, WorthQueryLowerRuntimeCapabilityEligibility,
    WorthQueryLowerRuntimeCapabilityRequest, WorthQueryLowerRuntimeReadmissionReceipt,
    WorthQueryLowerRuntimeRouteKind, WorthQueryLowerRuntimeRoutePlan,
    WorthQueryLowerRuntimeSeamKey,
};
use crate::query_basis_lifecycle::{
    admit_inspection_basis, admit_observation_basis, admit_subscription_activation_basis,
    admit_subscription_declaration_basis, evaluate_basis_eligibility, normalize_raw_basis,
    readmit_bridge_continuity_evidence, readmit_bridge_subscription_activation_evidence,
    readmit_bridge_subscription_declaration_evidence, readmit_bridge_truth_view_evidence,
    BasisOperationLaneRequest, RawBasisIntent,
};

use super::super::{RepresentativeArtifacts, WorthQueryLowerRuntimeRepresentativeEvidenceSource};
use super::readmission_support::{
    continuity_runtime, delivered_continuity, detail_subscription, observation_runtime,
    subscription_runtime, PHASE_SIX_MAIN_BRANCH,
};

pub(crate) fn representative_subscription_continuity_row() -> RepresentativeArtifacts {
    let runtime = continuity_runtime();
    let continuity = delivered_continuity(&runtime);
    let capability = admit_inspection_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                phase_six_branch_identity(),
                BasisOperationLaneRequest::Inspection,
            ))
            .expect("subscription continuity fixture should normalize inspection basis"),
        )
        .expect("subscription continuity fixture should admit inspection basis"),
    )
    .expect("subscription continuity fixture should wrap admitted inspection basis");
    let bound = readmit_bridge_continuity_evidence(capability, &continuity)
        .expect("subscription continuity fixture should readmit bridge continuity evidence");
    let continuity_evidence =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                WorthQueryEvidenceTag::new("route_identity"),
                bound
                    .evidence()
                    .route_identity()
                    .expect("continuity route identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("continuity_identity"),
                bound
                    .evidence()
                    .continuity_identity()
                    .expect("continuity identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("continuity_resolution"),
                bound
                    .evidence()
                    .continuity_resolution_digest()
                    .expect("continuity resolution digest should exist"),
            )
            .seal();

    let request = WorthQueryLowerRuntimeCapabilityRequest::new(
        WorthQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        WorthQueryLowerRuntimeRouteKind::RoutePlanning,
        WorthQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Subscription continuity",
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSubjectIdentity::compose(
            "subscription-continuity-route-subject",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("continuity"),
            &continuity_evidence,
        )
        .seal(),
    );
    let eligibility = WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &continuity_evidence,
    );
    let route_plan = WorthQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        crate::lower_runtime_routing::WorthQueryLowerRuntimeRouteSubjectIdentity::from_evidence_identity(
            "subscription-continuity-route",
            &continuity_evidence,
        ),
    );
    let retained_evidence =
        crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
            "phase-six-continuity-route",
            &continuity_evidence,
        );
    let boundary_receipt = WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        &retained_evidence,
    );
    let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        WorthQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        &route_plan,
        &boundary_receipt,
        &retained_evidence,
    );
    RepresentativeArtifacts {
        seam_key: WorthQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_basis_truth_view_readmission_row() -> RepresentativeArtifacts {
    let runtime = observation_runtime();
    let evaluation = runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::from_relational_branch_id(PHASE_SIX_MAIN_BRANCH),
        ))
        .expect("truth-view readmission fixture should evaluate branch head");
    let capability = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                phase_six_branch_identity(),
                BasisOperationLaneRequest::Observation,
            ))
            .expect("truth-view readmission fixture should normalize observation basis"),
        )
        .expect("truth-view readmission fixture should admit observation basis"),
    )
    .expect("truth-view readmission fixture should wrap admitted observation basis");
    let bound = readmit_bridge_truth_view_evidence(capability, &evaluation)
        .expect("truth-view readmission fixture should bind bridge evidence");
    let truth_view_evidence =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                WorthQueryEvidenceTag::new("record_identity"),
                bound
                    .evidence()
                    .record_identity()
                    .expect("truth-view record identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("selector_identity"),
                bound
                    .evidence()
                    .selector_identity()
                    .expect("truth-view selector identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("authority"),
                bound
                    .evidence()
                    .authority_digest()
                    .expect("truth-view authority digest should exist"),
            )
            .seal();

    readmission_row(
        WorthQueryLowerRuntimeSeamKey::BasisReadmissionFromTruthViewEvidence,
        "Basis readmission from truth-view evidence",
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSubjectIdentity::compose(
            "basis-truth-view-readmission-subject",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("truth_view"),
            &truth_view_evidence,
        )
        .seal(),
        truth_view_evidence.clone(),
        truth_view_evidence,
    )
}

pub(crate) fn representative_basis_subscription_readmission_row() -> RepresentativeArtifacts {
    let runtime = subscription_runtime();
    let admitted = detail_subscription(&runtime);
    let declaration = admit_subscription_declaration_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                phase_six_branch_identity(),
                BasisOperationLaneRequest::SubscriptionDeclaration,
            ))
            .expect("subscription readmission fixture should normalize declaration basis"),
        )
        .expect("subscription readmission fixture should admit declaration basis"),
    )
    .expect("subscription readmission fixture should wrap admitted declaration basis");
    let activation = admit_subscription_activation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                phase_six_branch_identity(),
                BasisOperationLaneRequest::SubscriptionActivation,
            ))
            .expect("subscription readmission fixture should normalize activation basis"),
        )
        .expect("subscription readmission fixture should admit activation basis"),
    )
    .expect("subscription readmission fixture should wrap admitted activation basis");
    let declaration_bound =
        readmit_bridge_subscription_declaration_evidence(declaration, &admitted)
            .expect("subscription declaration readmission should bind bridge evidence");
    let activation_bound = readmit_bridge_subscription_activation_evidence(activation, &admitted)
        .expect("subscription activation readmission should bind bridge evidence");
    let subscription_evidence =
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
            .field_value(
                WorthQueryEvidenceTag::new("declaration_subscription"),
                declaration_bound
                    .evidence()
                    .admitted_subscription_identity()
                    .expect("admitted subscription identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("declaration_strategy"),
                declaration_bound
                    .evidence()
                    .strategy_identity()
                    .expect("subscription strategy identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("activation_subscription"),
                activation_bound
                    .evidence()
                    .admitted_subscription_identity()
                    .expect("admitted subscription identity should exist"),
            )
            .field_value(
                WorthQueryEvidenceTag::new("activation_strategy"),
                activation_bound
                    .evidence()
                    .strategy_identity()
                    .expect("subscription strategy identity should exist"),
            )
            .seal();

    readmission_row(
        WorthQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence,
        "Basis readmission from subscription evidence",
        crate::lower_runtime_routing::WorthQueryLowerRuntimeSubjectIdentity::compose(
            "basis-subscription-readmission-subject",
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("subscription"),
            &subscription_evidence,
        )
        .seal(),
        subscription_evidence.clone(),
        subscription_evidence,
    )
}

fn phase_six_branch_identity() -> worth_runtime_bridge::facade::BridgeIdentityEvidence {
    TruthBranchIdentity::from_relational_branch_id(PHASE_SIX_MAIN_BRANCH)
        .bridge_admission_evidence()
}

fn readmission_row(
    seam_key: WorthQueryLowerRuntimeSeamKey,
    capability_label: &str,
    subject_identity: crate::lower_runtime_routing::WorthQueryLowerRuntimeSubjectIdentity,
    eligibility_evidence: WorthQueryEvidenceIdentity,
    retained_evidence: crate::evidence_identity::WorthQueryEvidenceIdentity,
) -> RepresentativeArtifacts {
    let request = WorthQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        WorthQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        WorthQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        capability_label,
        subject_identity,
    );
    let eligibility = WorthQueryLowerRuntimeCapabilityEligibility::admitted_with_evidence_identity(
        request.clone(),
        &eligibility_evidence,
    );
    let retained_evidence_identity =
        crate::lower_runtime_routing::worth_query_lower_runtime_retained_evidence_identity(
            "phase-six-readmission",
            &retained_evidence,
        );
    let readmission = WorthQueryLowerRuntimeReadmissionReceipt::new(
        eligibility.clone(),
        &retained_evidence_identity,
    );
    let boundary_receipt =
        WorthQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&readmission);
    let envelope = WorthQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
        seam_key,
        &readmission,
        &boundary_receipt,
    );
    RepresentativeArtifacts {
        seam_key,
        request,
        eligibility,
        route_plan: None,
        boundary_receipt,
        envelope,
        evidence_source: WorthQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}
