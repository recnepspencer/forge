use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectFit, WorthQueryDeclarationAspectPublication,
};
use crate::intent_admission::{
    WorthQueryIntentAdmissionCoveredEntrypoint, WorthQueryIntentAdmissionFamily,
};
use crate::lower_runtime_routing::{
    WorthQueryLowerRuntimeCrossingClassification, WorthQueryLowerRuntimeRouteKind,
    WorthQueryLowerRuntimeSeamKey, WorthQueryLowerRuntimeSupportPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryBindingTargetSemantics {
    IntentDeclaration {
        name: String,
        strategy_name: String,
        strategy_version: String,
        input_contract: String,
        source_lane: crate::runtime::WorthQueryIntentSourceLane,
        target_lane: crate::runtime::WorthQueryAuthorityLane,
    },
    AdmittedIntentPlan {
        family: WorthQueryIntentAdmissionFamily,
        entrypoint: WorthQueryIntentAdmissionCoveredEntrypoint,
        request_digest: String,
        eligibility_digest: String,
        decision_digest: String,
    },
    LowerRuntimeBoundaryEnvelope {
        seam_key: WorthQueryLowerRuntimeSeamKey,
        capability_label: &'static str,
        crossing_classification: WorthQueryLowerRuntimeCrossingClassification,
        route_kind: WorthQueryLowerRuntimeRouteKind,
        support_posture: WorthQueryLowerRuntimeSupportPosture,
        envelope_digest: String,
    },
    AdmittedDeclarationProgression {
        handle_identity_digest: String,
        operating_context_identity_digest: String,
        declaration_digest: String,
        progression_digest: String,
        declaration_family_label: &'static str,
        aspect_contract: WorthQueryDeclarationAspectContract,
        reviewed_aspect_coverage: WorthQueryDeclarationAspectCoverage,
    },
    DeclarationRoutePlan {
        handle_identity_digest: String,
        declaration_digest: String,
        route_plan_digest: String,
        route_contract_family_label: &'static str,
        route_aspect_contract: WorthQueryDeclarationAspectContract,
        route_aspect_fit: WorthQueryDeclarationAspectFit,
        route_aspect_publication: WorthQueryDeclarationAspectPublication,
    },
    DeclarationReceipt {
        declaration_digest: String,
        route_plan_digest: Option<String>,
        receipt_digest: String,
        receipt_posture_class: &'static str,
        crossing_aspect_contract: WorthQueryDeclarationAspectContract,
        crossing_aspect_coverage: WorthQueryDeclarationAspectCoverage,
        crossing_aspect_publication: WorthQueryDeclarationAspectPublication,
    },
    DeclarationEnvelope {
        declaration_digest: String,
        route_plan_digest: Option<String>,
        receipt_digest: String,
        envelope_digest: String,
        published_aspect_contract: WorthQueryDeclarationAspectContract,
        published_aspect_publication: WorthQueryDeclarationAspectPublication,
    },
}

impl WorthQueryBindingTargetSemantics {
    pub fn intent_declaration(
        &self,
    ) -> Option<(
        &str,
        &str,
        &str,
        &str,
        crate::runtime::WorthQueryIntentSourceLane,
        crate::runtime::WorthQueryAuthorityLane,
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
        WorthQueryIntentAdmissionFamily,
        WorthQueryIntentAdmissionCoveredEntrypoint,
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
        WorthQueryLowerRuntimeSeamKey,
        &'static str,
        WorthQueryLowerRuntimeCrossingClassification,
        WorthQueryLowerRuntimeRouteKind,
        WorthQueryLowerRuntimeSupportPosture,
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
        &WorthQueryDeclarationAspectContract,
        &WorthQueryDeclarationAspectCoverage,
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
        &WorthQueryDeclarationAspectContract,
        WorthQueryDeclarationAspectFit,
        &WorthQueryDeclarationAspectPublication,
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
        &WorthQueryDeclarationAspectContract,
        &WorthQueryDeclarationAspectCoverage,
        &WorthQueryDeclarationAspectPublication,
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
        &WorthQueryDeclarationAspectContract,
        &WorthQueryDeclarationAspectPublication,
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

    pub(crate) fn binding_digest_material(&self) -> String {
        match self {
            Self::IntentDeclaration {
                name,
                strategy_name,
                strategy_version,
                input_contract,
                source_lane,
                target_lane,
            } => format!(
                "name:{name}|strategy_name:{strategy_name}|strategy_version:{strategy_version}|input_contract:{input_contract}|source_lane:{source_lane:?}|target_lane:{target_lane:?}"
            ),
            Self::AdmittedIntentPlan {
                family,
                entrypoint,
                request_digest,
                eligibility_digest,
                decision_digest,
            } => format!(
                "family:{family:?}|entrypoint:{entrypoint:?}|request_digest:{request_digest}|eligibility_digest:{eligibility_digest}|decision_digest:{decision_digest}"
            ),
            Self::LowerRuntimeBoundaryEnvelope {
                seam_key,
                capability_label,
                crossing_classification,
                route_kind,
                support_posture,
                envelope_digest,
            } => format!(
                "seam_key:{seam_key:?}|capability_label:{capability_label}|crossing_classification:{crossing_classification:?}|route_kind:{route_kind:?}|support_posture:{support_posture:?}|envelope_digest:{envelope_digest}"
            ),
            Self::AdmittedDeclarationProgression {
                handle_identity_digest,
                operating_context_identity_digest,
                declaration_digest,
                progression_digest,
                declaration_family_label,
                aspect_contract,
                reviewed_aspect_coverage,
            } => format!(
                "handle_identity_digest:{handle_identity_digest}|operating_context_identity_digest:{operating_context_identity_digest}|declaration_digest:{declaration_digest}|progression_digest:{progression_digest}|declaration_family_label:{declaration_family_label}|aspect_contract:{aspect_contract:?}|reviewed_aspect_coverage:{reviewed_aspect_coverage:?}"
            ),
            Self::DeclarationRoutePlan {
                handle_identity_digest,
                declaration_digest,
                route_plan_digest,
                route_contract_family_label,
                route_aspect_contract,
                route_aspect_fit,
                route_aspect_publication,
            } => format!(
                "handle_identity_digest:{handle_identity_digest}|declaration_digest:{declaration_digest}|route_plan_digest:{route_plan_digest}|route_contract_family_label:{route_contract_family_label}|route_aspect_contract:{route_aspect_contract:?}|route_aspect_fit:{route_aspect_fit:?}|route_aspect_publication:{route_aspect_publication:?}"
            ),
            Self::DeclarationReceipt {
                declaration_digest,
                route_plan_digest,
                receipt_digest,
                receipt_posture_class,
                crossing_aspect_contract,
                crossing_aspect_coverage,
                crossing_aspect_publication,
            } => format!(
                "declaration_digest:{declaration_digest}|route_plan_digest:{route_plan_digest:?}|receipt_digest:{receipt_digest}|receipt_posture_class:{receipt_posture_class}|crossing_aspect_contract:{crossing_aspect_contract:?}|crossing_aspect_coverage:{crossing_aspect_coverage:?}|crossing_aspect_publication:{crossing_aspect_publication:?}"
            ),
            Self::DeclarationEnvelope {
                declaration_digest,
                route_plan_digest,
                receipt_digest,
                envelope_digest,
                published_aspect_contract,
                published_aspect_publication,
            } => format!(
                "declaration_digest:{declaration_digest}|route_plan_digest:{route_plan_digest:?}|receipt_digest:{receipt_digest}|envelope_digest:{envelope_digest}|published_aspect_contract:{published_aspect_contract:?}|published_aspect_publication:{published_aspect_publication:?}"
            ),
        }
    }
}
