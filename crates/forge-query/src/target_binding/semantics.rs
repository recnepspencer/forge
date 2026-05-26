use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationEnvelope,
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceipt, ForgeQueryDeclarationRoutePlan,
    ForgeQueryDomainEntryMarker,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionFamily,
};
use crate::lower_runtime_routing::{
    ForgeQueryLowerRuntimeCrossingClassification, ForgeQueryLowerRuntimeRouteKind,
    ForgeQueryLowerRuntimeSeamKey, ForgeQueryLowerRuntimeSupportPosture,
};
use crate::runtime::{
    ForgeQueryAdmittedIntentPlan, ForgeQueryIntentDeclaration,
    ForgeQueryLowerRuntimeBoundaryEnvelope,
};
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryBindingTargetSemantics {
    IntentDeclaration {
        name: String,
        strategy_name: String,
        strategy_version: String,
        input_contract: String,
        source_lane: crate::runtime::ForgeQueryIntentSourceLane,
        target_lane: crate::runtime::ForgeQueryAuthorityLane,
    },
    AdmittedIntentPlan {
        family: ForgeQueryIntentAdmissionFamily,
        entrypoint: ForgeQueryIntentAdmissionCoveredEntrypoint,
        request_digest: String,
        eligibility_digest: String,
        decision_digest: String,
    },
    LowerRuntimeBoundaryEnvelope {
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        capability_label: &'static str,
        crossing_classification: ForgeQueryLowerRuntimeCrossingClassification,
        route_kind: ForgeQueryLowerRuntimeRouteKind,
        support_posture: ForgeQueryLowerRuntimeSupportPosture,
        envelope_digest: String,
    },
    AdmittedDeclarationProgression {
        handle_identity_digest: String,
        operating_context_identity_digest: String,
        declaration_digest: String,
        progression_digest: String,
        declaration_family_label: &'static str,
    },
    DeclarationRoutePlan {
        handle_identity_digest: String,
        declaration_digest: String,
        route_plan_digest: String,
        route_contract_family_label: &'static str,
    },
    DeclarationReceipt {
        declaration_digest: String,
        route_plan_digest: Option<String>,
        receipt_digest: String,
        receipt_posture_class: &'static str,
    },
    DeclarationEnvelope {
        declaration_digest: String,
        route_plan_digest: Option<String>,
        receipt_digest: String,
        envelope_digest: String,
    },
}

impl ForgeQueryBindingTargetSemantics {
    pub fn intent_declaration(
        &self,
    ) -> Option<(
        &str,
        &str,
        &str,
        &str,
        crate::runtime::ForgeQueryIntentSourceLane,
        crate::runtime::ForgeQueryAuthorityLane,
    )> {
        match self {
            Self::IntentDeclaration {
                name,
                strategy_name,
                strategy_version,
                input_contract,
                source_lane,
                target_lane,
            } => Some((
                name.as_str(),
                strategy_name.as_str(),
                strategy_version.as_str(),
                input_contract.as_str(),
                *source_lane,
                *target_lane,
            )),
            _ => None,
        }
    }

    pub fn admitted_intent_plan(
        &self,
    ) -> Option<(
        ForgeQueryIntentAdmissionFamily,
        ForgeQueryIntentAdmissionCoveredEntrypoint,
        &str,
        &str,
        &str,
    )> {
        match self {
            Self::AdmittedIntentPlan {
                family,
                entrypoint,
                request_digest,
                eligibility_digest,
                decision_digest,
            } => Some((
                *family,
                *entrypoint,
                request_digest.as_str(),
                eligibility_digest.as_str(),
                decision_digest.as_str(),
            )),
            _ => None,
        }
    }

    pub fn lower_runtime_boundary(
        &self,
    ) -> Option<(
        ForgeQueryLowerRuntimeSeamKey,
        &'static str,
        ForgeQueryLowerRuntimeCrossingClassification,
        ForgeQueryLowerRuntimeRouteKind,
        ForgeQueryLowerRuntimeSupportPosture,
        &str,
    )> {
        match self {
            Self::LowerRuntimeBoundaryEnvelope {
                seam_key,
                capability_label,
                crossing_classification,
                route_kind,
                support_posture,
                envelope_digest,
            } => Some((
                *seam_key,
                *capability_label,
                *crossing_classification,
                *route_kind,
                *support_posture,
                envelope_digest.as_str(),
            )),
            _ => None,
        }
    }

    pub fn admitted_declaration_progression(
        &self,
    ) -> Option<(&str, &str, &str, &str, &'static str)> {
        match self {
            Self::AdmittedDeclarationProgression {
                handle_identity_digest,
                operating_context_identity_digest,
                declaration_digest,
                progression_digest,
                declaration_family_label,
            } => Some((
                handle_identity_digest.as_str(),
                operating_context_identity_digest.as_str(),
                declaration_digest.as_str(),
                progression_digest.as_str(),
                *declaration_family_label,
            )),
            _ => None,
        }
    }

    pub fn declaration_route_plan(&self) -> Option<(&str, &str, &str, &'static str)> {
        match self {
            Self::DeclarationRoutePlan {
                handle_identity_digest,
                declaration_digest,
                route_plan_digest,
                route_contract_family_label,
            } => Some((
                handle_identity_digest.as_str(),
                declaration_digest.as_str(),
                route_plan_digest.as_str(),
                *route_contract_family_label,
            )),
            _ => None,
        }
    }

    pub fn declaration_receipt(&self) -> Option<(&str, Option<&str>, &str, &'static str)> {
        match self {
            Self::DeclarationReceipt {
                declaration_digest,
                route_plan_digest,
                receipt_digest,
                receipt_posture_class,
            } => Some((
                declaration_digest.as_str(),
                route_plan_digest.as_deref(),
                receipt_digest.as_str(),
                *receipt_posture_class,
            )),
            _ => None,
        }
    }

    pub fn declaration_envelope(&self) -> Option<(&str, Option<&str>, &str, &str)> {
        match self {
            Self::DeclarationEnvelope {
                declaration_digest,
                route_plan_digest,
                receipt_digest,
                envelope_digest,
            } => Some((
                declaration_digest.as_str(),
                route_plan_digest.as_deref(),
                receipt_digest.as_str(),
                envelope_digest.as_str(),
            )),
            _ => None,
        }
    }

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
    ) -> Self {
        Self::AdmittedDeclarationProgression {
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_digest,
            progression_digest,
            declaration_family_label,
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
        }
    }
}
