use forge_runtime_bridge::facade::{BridgeTruthViewEvaluationRequest, TruthBranchIdentity};

use crate::identity::hash_parts;
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeBoundaryEnvelope,
    ForgeQueryLowerRuntimeBoundaryExecutionReceipt, ForgeQueryLowerRuntimeCapabilityEligibility,
    ForgeQueryLowerRuntimeCapabilityRequest, ForgeQueryLowerRuntimeReadmissionReceipt,
    ForgeQueryLowerRuntimeRouteKind, ForgeQueryLowerRuntimeRoutePlan,
    ForgeQueryLowerRuntimeSeamKey,
};
use crate::query_basis_lifecycle::{
    admit_inspection_basis, admit_observation_basis, admit_subscription_activation_basis,
    admit_subscription_declaration_basis, evaluate_basis_eligibility, normalize_raw_basis,
    readmit_bridge_continuity_evidence, readmit_bridge_subscription_activation_evidence,
    readmit_bridge_subscription_declaration_evidence, readmit_bridge_truth_view_evidence,
    BasisOperationLaneRequest, RawBasisIntent,
};

use super::super::{ForgeQueryLowerRuntimeRepresentativeEvidenceSource, RepresentativeArtifacts};
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
                PHASE_SIX_MAIN_BRANCH,
                BasisOperationLaneRequest::Inspection,
            ))
            .expect("subscription continuity fixture should normalize inspection basis"),
        )
        .expect("subscription continuity fixture should admit inspection basis"),
    )
    .expect("subscription continuity fixture should wrap admitted inspection basis");
    let bound = readmit_bridge_continuity_evidence(capability, &continuity)
        .expect("subscription continuity fixture should readmit bridge continuity evidence");

    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        ForgeQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        ForgeQueryLowerRuntimeRouteKind::RoutePlanning,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        "Subscription continuity",
        hash_parts(&[
            "subscription_continuity_route_subject_v1".to_string(),
            format!(
                "route_identity:{}",
                bound
                    .evidence()
                    .route_identity()
                    .expect("continuity route identity should exist")
            ),
            format!(
                "continuity_identity:{}",
                bound
                    .evidence()
                    .continuity_identity()
                    .expect("continuity identity should exist")
            ),
        ]),
    );
    let eligibility = ForgeQueryLowerRuntimeCapabilityEligibility::admitted(
        request.clone(),
        bound.binding_digest(),
    );
    let route_plan = ForgeQueryLowerRuntimeRoutePlan::new(
        eligibility.clone(),
        bound
            .evidence()
            .continuity_resolution_digest()
            .expect("continuity resolution digest should exist"),
    );
    let retained_evidence = bound
        .evidence()
        .continuity_identity()
        .expect("continuity identity should exist");
    let boundary_receipt = ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_route_plan(
        &route_plan,
        retained_evidence,
    );
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_route_plan(
        ForgeQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        &route_plan,
        &boundary_receipt,
        retained_evidence,
    );
    RepresentativeArtifacts {
        seam_key: ForgeQueryLowerRuntimeSeamKey::SubscriptionContinuity,
        request,
        eligibility,
        route_plan: Some(route_plan),
        boundary_receipt,
        envelope,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}

pub(crate) fn representative_basis_truth_view_readmission_row() -> RepresentativeArtifacts {
    let runtime = observation_runtime();
    let evaluation = runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new(PHASE_SIX_MAIN_BRANCH),
        ))
        .expect("truth-view readmission fixture should evaluate branch head");
    let capability = admit_observation_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                PHASE_SIX_MAIN_BRANCH,
                BasisOperationLaneRequest::Observation,
            ))
            .expect("truth-view readmission fixture should normalize observation basis"),
        )
        .expect("truth-view readmission fixture should admit observation basis"),
    )
    .expect("truth-view readmission fixture should wrap admitted observation basis");
    let bound = readmit_bridge_truth_view_evidence(capability, &evaluation)
        .expect("truth-view readmission fixture should bind bridge evidence");

    readmission_row(
        ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromTruthViewEvidence,
        "Basis readmission from truth-view evidence",
        hash_parts(&[
            "basis_truth_view_readmission_subject_v1".to_string(),
            format!(
                "record_identity:{}",
                bound
                    .evidence()
                    .record_identity()
                    .expect("truth-view record identity should exist")
            ),
            format!(
                "selector_identity:{}",
                bound
                    .evidence()
                    .selector_identity()
                    .expect("truth-view selector identity should exist")
            ),
        ]),
        bound.binding_digest(),
        bound
            .evidence()
            .authority_digest()
            .expect("truth-view authority digest should exist"),
    )
}

pub(crate) fn representative_basis_subscription_readmission_row() -> RepresentativeArtifacts {
    let runtime = subscription_runtime();
    let admitted = detail_subscription(&runtime);
    let declaration = admit_subscription_declaration_basis(
        evaluate_basis_eligibility(
            normalize_raw_basis(RawBasisIntent::branch_head(
                PHASE_SIX_MAIN_BRANCH,
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
                PHASE_SIX_MAIN_BRANCH,
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

    readmission_row(
        ForgeQueryLowerRuntimeSeamKey::BasisReadmissionFromSubscriptionEvidence,
        "Basis readmission from subscription evidence",
        hash_parts(&[
            "basis_subscription_readmission_subject_v1".to_string(),
            format!(
                "admitted_subscription_identity:{}",
                declaration_bound
                    .evidence()
                    .admitted_subscription_identity()
                    .expect("admitted subscription identity should exist")
            ),
            format!(
                "strategy_identity:{}",
                declaration_bound
                    .evidence()
                    .strategy_identity()
                    .expect("subscription strategy identity should exist")
            ),
        ]),
        hash_parts(&[
            declaration_bound.binding_digest().to_string(),
            activation_bound.binding_digest().to_string(),
        ]),
        declaration_bound
            .evidence()
            .subscription_digest()
            .expect("subscription digest should exist"),
    )
}

fn readmission_row(
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    capability_label: &str,
    subject_digest: String,
    eligibility_detail: impl Into<String>,
    retained_evidence: &str,
) -> RepresentativeArtifacts {
    let request = ForgeQueryLowerRuntimeCapabilityRequest::new(
        seam_key,
        ForgeQueryLowerRuntimeRouteKind::ReadmissionHandoff,
        ForgeQueryLowerRuntimeAuthorityOwner::RuntimeBridge,
        capability_label,
        subject_digest,
    );
    let eligibility =
        ForgeQueryLowerRuntimeCapabilityEligibility::admitted(request.clone(), eligibility_detail);
    let readmission =
        ForgeQueryLowerRuntimeReadmissionReceipt::new(eligibility.clone(), retained_evidence);
    let boundary_receipt =
        ForgeQueryLowerRuntimeBoundaryExecutionReceipt::from_readmission_receipt(&readmission);
    let envelope = ForgeQueryLowerRuntimeBoundaryEnvelope::from_readmission_receipt(
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
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture,
    }
}
