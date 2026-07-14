use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectFit,
};
use crate::binding_pipeline::{
    WorthQueryBindingAmbiguity, WorthQueryBindingAspectConflict, WorthQueryBindingAspectFitReport,
    WorthQueryBindingCandidateRecord, WorthQueryBindingExplicitNarrowingRequired,
    WorthQueryBindingMissingRequiredAspect, WorthQueryBindingNarrowingDecision,
    WorthQueryBindingOutcome, WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript,
    WorthQueryBindingUnavailable,
};
use crate::identity::hash_parts;

pub(super) fn digest_for(
    family: &'static str,
    request_kind: &'static str,
    contract: &WorthQueryDeclarationAspectContract,
    outcome_key: &str,
) -> String {
    hash_parts(&[
        "worth_query_binding_pipeline_v1".to_string(),
        family.to_string(),
        request_kind.to_string(),
        format!("contract:{contract:?}"),
        format!("outcome:{outcome_key}"),
    ])
}

pub(super) fn fit_allowed(
    fit: WorthQueryDeclarationAspectFit,
    allow_compatible_superset: bool,
) -> bool {
    match fit {
        WorthQueryDeclarationAspectFit::Exact => true,
        WorthQueryDeclarationAspectFit::CompatibleSuperset => allow_compatible_superset,
        WorthQueryDeclarationAspectFit::Partial
        | WorthQueryDeclarationAspectFit::MissingRequired
        | WorthQueryDeclarationAspectFit::Conflict => false,
    }
}

pub(super) fn fit_rank(fit: WorthQueryDeclarationAspectFit) -> u8 {
    match fit {
        WorthQueryDeclarationAspectFit::Exact => 4,
        WorthQueryDeclarationAspectFit::CompatibleSuperset => 3,
        WorthQueryDeclarationAspectFit::Partial => 2,
        WorthQueryDeclarationAspectFit::MissingRequired => 1,
        WorthQueryDeclarationAspectFit::Conflict => 0,
    }
}

pub(super) fn denied_on_fit<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    fit: WorthQueryDeclarationAspectFit,
    partial_narrowing: bool,
    candidates: Vec<WorthQueryBindingCandidateRecord>,
) -> WorthQueryBindingTranscript<T> {
    let outcome = match fit {
        WorthQueryDeclarationAspectFit::Partial if partial_narrowing => {
            WorthQueryBindingOutcome::ExplicitNarrowingRequired(
                WorthQueryBindingExplicitNarrowingRequired::new(
                    "binding candidates only partially satisfy the requested semantic slice",
                ),
            )
        }
        WorthQueryDeclarationAspectFit::Partial
        | WorthQueryDeclarationAspectFit::MissingRequired => {
            WorthQueryBindingOutcome::MissingRequiredAspect(
                WorthQueryBindingMissingRequiredAspect::new(
                    "binding candidates do not cover every required semantic aspect",
                ),
            )
        }
        WorthQueryDeclarationAspectFit::Conflict => {
            WorthQueryBindingOutcome::AspectConflict(WorthQueryBindingAspectConflict::new(
                "binding candidates conflict with the required semantic aspect contract",
            ))
        }
        WorthQueryDeclarationAspectFit::Exact
        | WorthQueryDeclarationAspectFit::CompatibleSuperset => {
            WorthQueryBindingOutcome::Unavailable(WorthQueryBindingUnavailable::new(
                "no admissible binding candidate remained after aspect filtering",
            ))
        }
    };
    WorthQueryBindingTranscript::new(
        WorthQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        candidates,
        Vec::new(),
        Some(WorthQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![WorthQueryBindingNarrowingDecision::new(
            "binding denied after context candidate fit evaluation",
        )],
        None,
        digest_for(
            family,
            request_kind,
            &contract,
            match fit {
                WorthQueryDeclarationAspectFit::Partial if partial_narrowing => {
                    "explicit_narrowing_required"
                }
                WorthQueryDeclarationAspectFit::Partial
                | WorthQueryDeclarationAspectFit::MissingRequired => "missing_required_aspect",
                WorthQueryDeclarationAspectFit::Conflict => "aspect_conflict",
                WorthQueryDeclarationAspectFit::Exact
                | WorthQueryDeclarationAspectFit::CompatibleSuperset => "unavailable",
            },
        ),
        crate::binding_pipeline::WorthQueryBindingLinkedArtifacts::new(),
    )
}

pub(super) fn ambiguous<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: WorthQueryDeclarationAspectContract,
    candidates: Vec<WorthQueryBindingCandidateRecord>,
) -> WorthQueryBindingTranscript<T> {
    WorthQueryBindingTranscript::new(
        WorthQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        WorthQueryBindingOutcome::Ambiguous(WorthQueryBindingAmbiguity::new(
            "multiple binding candidates remained tied after aspect-fit and specificity narrowing",
            candidates.len(),
        )),
        candidates,
        Vec::new(),
        None,
        vec![WorthQueryBindingNarrowingDecision::new(
            "binding denied because no single exact winner remained",
        )],
        None,
        digest_for(family, request_kind, &contract, "ambiguous"),
        crate::binding_pipeline::WorthQueryBindingLinkedArtifacts::new(),
    )
}
