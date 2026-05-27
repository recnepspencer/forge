use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationRoutePlan, ForgeQueryDomainEntryMarker,
};
use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope,
};

use super::ForgeQueryBindingTargetSemantics;

impl ForgeQueryBindingTargetSemantics {
    pub(crate) fn for_intent_declaration(declaration: &ForgeQueryIntentDeclaration) -> Self {
        Self::IntentDeclaration {
            name: declaration.name().to_string(),
            strategy_name: declaration.strategy_name().to_string(),
            strategy_version: declaration.strategy_version().to_string(),
            input_contract: declaration.input_contract().to_string(),
            source_lane: declaration.source_lane(),
            target_lane: declaration.target_lane(),
        }
    }

    pub(crate) fn for_admitted_intent_plan(plan: &ForgeQueryAdmittedIntentPlan) -> Self {
        Self::AdmittedIntentPlan {
            family: plan.family(),
            entrypoint: plan.entrypoint(),
            request_digest: plan.request_digest().to_string(),
            eligibility_digest: plan.eligibility_digest().to_string(),
            decision_digest: plan.decision_digest().to_string(),
        }
    }

    pub(crate) fn for_lower_runtime_boundary_envelope(
        envelope: &ForgeQueryLowerRuntimeBoundaryEnvelope,
    ) -> Self {
        Self::LowerRuntimeBoundaryEnvelope {
            seam_key: envelope.seam_key(),
            capability_label: envelope.capability_label(),
            crossing_classification: envelope.crossing_classification(),
            route_kind: envelope.route_kind(),
            support_posture: envelope.support_posture(),
            envelope_digest: envelope.envelope_digest().to_string(),
        }
    }

    pub(crate) fn for_admitted_declaration_progression(
        handle_identity_digest: String,
        operating_context_identity_digest: String,
        declaration_digest: String,
        progression_digest: String,
        declaration_family_label: &'static str,
        aspect_contract: ForgeQueryDeclarationAspectContract,
        reviewed_aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    ) -> Self {
        Self::AdmittedDeclarationProgression {
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_digest,
            progression_digest,
            declaration_family_label,
            aspect_contract,
            reviewed_aspect_coverage,
        }
    }

    pub(crate) fn for_progressed_declaration<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        progressed: &ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Self {
        Self::for_admitted_declaration_progression(
            progressed
                .canonical_declaration()
                .handle_identity_digest()
                .to_string(),
            progressed.operating_context_identity_digest().to_string(),
            format!(
                "{:?}",
                progressed.canonical_declaration().declaration_digest()
            ),
            progressed.progression_digest().to_string(),
            progressed.declaration_family_key(),
            progressed.aspect_contract().clone(),
            progressed.reviewed_aspect_coverage().clone(),
        )
    }

    pub(crate) fn for_declaration_route_plan<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        route_plan: &ForgeQueryDeclarationRoutePlan<D, I>,
    ) -> Self {
        Self::DeclarationRoutePlan {
            handle_identity_digest: route_plan.handle_identity_digest().to_string(),
            declaration_digest: route_plan.declaration_digest().to_string(),
            route_plan_digest: route_plan.route_plan_digest().to_string(),
            route_contract_family_label: route_plan.declaration_family_key(),
            route_aspect_contract: route_plan.aspect_contract().clone(),
            route_aspect_fit: route_plan.aspect_fit(),
            route_aspect_publication: route_plan.aspect_publication().clone(),
        }
    }

    pub(crate) fn for_declaration_receipt<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        receipt: &ForgeQueryDeclarationReceipt<D, I>,
    ) -> Self {
        Self::DeclarationReceipt {
            declaration_digest: receipt.declaration_digest().to_string(),
            route_plan_digest: receipt.route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", receipt.receipt_digest()),
            receipt_posture_class: match receipt.class() {
                crate::application::ForgeQueryDeclarationReceiptClass::CoveredCrossing => "covered",
                crate::application::ForgeQueryDeclarationReceiptClass::DeferredCrossing => {
                    "deferred"
                }
                crate::application::ForgeQueryDeclarationReceiptClass::DeniedCrossing => "denied",
                crate::application::ForgeQueryDeclarationReceiptClass::FailedCrossing => "failed",
            },
            crossing_aspect_contract: receipt.aspect_contract().clone(),
            crossing_aspect_coverage: receipt.aspect_coverage().clone(),
            crossing_aspect_publication: receipt.aspect_publication().clone(),
        }
    }

    pub(crate) fn for_declaration_envelope<
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    >(
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self::DeclarationEnvelope {
            declaration_digest: envelope.declaration_digest().to_string(),
            route_plan_digest: envelope.route_plan_digest().map(ToOwned::to_owned),
            receipt_digest: format!("{:?}", envelope.receipt_digest()),
            envelope_digest: format!("{:?}", envelope.envelope_digest()),
            published_aspect_contract: envelope.aspect_contract().clone(),
            published_aspect_publication: envelope.aspect_publication().clone(),
        }
    }
}
