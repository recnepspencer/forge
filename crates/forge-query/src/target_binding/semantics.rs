use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationAspectFit,
    ForgeQueryDeclarationAspectPublication, ForgeQueryDeclarationEnvelope,
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
        aspect_contract: ForgeQueryDeclarationAspectContract,
        reviewed_aspect_coverage: ForgeQueryDeclarationAspectCoverage,
    },
    DeclarationRoutePlan {
        handle_identity_digest: String,
        declaration_digest: String,
        route_plan_digest: String,
        route_contract_family_label: &'static str,
        route_aspect_contract: ForgeQueryDeclarationAspectContract,
        route_aspect_fit: ForgeQueryDeclarationAspectFit,
        route_aspect_publication: ForgeQueryDeclarationAspectPublication,
    },
    DeclarationReceipt {
        declaration_digest: String,
        route_plan_digest: Option<String>,
        receipt_digest: String,
        receipt_posture_class: &'static str,
        crossing_aspect_contract: ForgeQueryDeclarationAspectContract,
        crossing_aspect_coverage: ForgeQueryDeclarationAspectCoverage,
        crossing_aspect_publication: ForgeQueryDeclarationAspectPublication,
    },
    DeclarationEnvelope {
        declaration_digest: String,
        route_plan_digest: Option<String>,
        receipt_digest: String,
        envelope_digest: String,
        published_aspect_contract: ForgeQueryDeclarationAspectContract,
        published_aspect_publication: ForgeQueryDeclarationAspectPublication,
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
    ) -> Option<(
        &str,
        &str,
        &str,
        &str,
        &'static str,
        &ForgeQueryDeclarationAspectContract,
        &ForgeQueryDeclarationAspectCoverage,
    )> {
        match self {
            Self::AdmittedDeclarationProgression {
                handle_identity_digest,
                operating_context_identity_digest,
                declaration_digest,
                progression_digest,
                declaration_family_label,
                aspect_contract,
                reviewed_aspect_coverage,
            } => Some((
                handle_identity_digest.as_str(),
                operating_context_identity_digest.as_str(),
                declaration_digest.as_str(),
                progression_digest.as_str(),
                *declaration_family_label,
                aspect_contract,
                reviewed_aspect_coverage,
            )),
            _ => None,
        }
    }

    pub fn declaration_route_plan(
        &self,
    ) -> Option<(
        &str,
        &str,
        &str,
        &'static str,
        &ForgeQueryDeclarationAspectContract,
        ForgeQueryDeclarationAspectFit,
        &ForgeQueryDeclarationAspectPublication,
    )> {
        match self {
            Self::DeclarationRoutePlan {
                handle_identity_digest,
                declaration_digest,
                route_plan_digest,
                route_contract_family_label,
                route_aspect_contract,
                route_aspect_fit,
                route_aspect_publication,
            } => Some((
                handle_identity_digest.as_str(),
                declaration_digest.as_str(),
                route_plan_digest.as_str(),
                *route_contract_family_label,
                route_aspect_contract,
                *route_aspect_fit,
                route_aspect_publication,
            )),
            _ => None,
        }
    }

    pub fn declaration_receipt(
        &self,
    ) -> Option<(
        &str,
        Option<&str>,
        &str,
        &'static str,
        &ForgeQueryDeclarationAspectContract,
        &ForgeQueryDeclarationAspectCoverage,
        &ForgeQueryDeclarationAspectPublication,
    )> {
        match self {
            Self::DeclarationReceipt {
                declaration_digest,
                route_plan_digest,
                receipt_digest,
                receipt_posture_class,
                crossing_aspect_contract,
                crossing_aspect_coverage,
                crossing_aspect_publication,
            } => Some((
                declaration_digest.as_str(),
                route_plan_digest.as_deref(),
                receipt_digest.as_str(),
                *receipt_posture_class,
                crossing_aspect_contract,
                crossing_aspect_coverage,
                crossing_aspect_publication,
            )),
            _ => None,
        }
    }

    pub fn declaration_envelope(
        &self,
    ) -> Option<(
        &str,
        Option<&str>,
        &str,
        &str,
        &ForgeQueryDeclarationAspectContract,
        &ForgeQueryDeclarationAspectPublication,
    )> {
        match self {
            Self::DeclarationEnvelope {
                declaration_digest,
                route_plan_digest,
                receipt_digest,
                envelope_digest,
                published_aspect_contract,
                published_aspect_publication,
            } => Some((
                declaration_digest.as_str(),
                route_plan_digest.as_deref(),
                receipt_digest.as_str(),
                envelope_digest.as_str(),
                published_aspect_contract,
                published_aspect_publication,
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

    pub(crate) fn binding_digest_material(&self) -> String {
        match self {
            Self::AdmittedDeclarationProgression {
                aspect_contract,
                reviewed_aspect_coverage,
                ..
            } => format!(
                "aspect_contract:{aspect_contract:?}|reviewed_aspect_coverage:{reviewed_aspect_coverage:?}"
            ),
            Self::DeclarationRoutePlan {
                route_aspect_contract,
                route_aspect_fit,
                route_aspect_publication,
                ..
            } => format!(
                "route_aspect_contract:{route_aspect_contract:?}|route_aspect_fit:{route_aspect_fit:?}|route_aspect_publication:{route_aspect_publication:?}"
            ),
            Self::DeclarationReceipt {
                crossing_aspect_contract,
                crossing_aspect_coverage,
                crossing_aspect_publication,
                ..
            } => format!(
                "crossing_aspect_contract:{crossing_aspect_contract:?}|crossing_aspect_coverage:{crossing_aspect_coverage:?}|crossing_aspect_publication:{crossing_aspect_publication:?}"
            ),
            Self::DeclarationEnvelope {
                published_aspect_contract,
                published_aspect_publication,
                ..
            } => format!(
                "published_aspect_contract:{published_aspect_contract:?}|published_aspect_publication:{published_aspect_publication:?}"
            ),
            _ => String::new(),
        }
    }
}
