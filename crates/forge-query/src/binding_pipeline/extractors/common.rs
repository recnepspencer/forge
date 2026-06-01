use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectFit,
};
use crate::binding_pipeline::{
    ForgeQueryBindingAmbiguity, ForgeQueryBindingAspectConflict, ForgeQueryBindingAspectFitReport,
    ForgeQueryBindingCandidateRecord, ForgeQueryBindingExplicitNarrowingRequired,
    ForgeQueryBindingMissingRequiredAspect, ForgeQueryBindingNarrowingDecision,
    ForgeQueryBindingOutcome, ForgeQueryBindingRequestDescriptor, ForgeQueryBindingTranscript,
    ForgeQueryBindingUnavailable,
};
use crate::identity::hash_parts;

pub(super) fn digest_for(
    family: &'static str,
    request_kind: &'static str,
    contract: &ForgeQueryDeclarationAspectContract,
    outcome_key: &str,
) -> String {
    hash_parts(&[
        "forge_query_binding_pipeline_v1".to_string(),
        family.to_string(),
        request_kind.to_string(),
        format!("contract:{contract:?}"),
        format!("outcome:{outcome_key}"),
    ])
}

pub(super) fn fit_allowed(
    fit: ForgeQueryDeclarationAspectFit,
    allow_compatible_superset: bool,
) -> bool {
    match fit {
        ForgeQueryDeclarationAspectFit::Exact => true,
        ForgeQueryDeclarationAspectFit::CompatibleSuperset => allow_compatible_superset,
        ForgeQueryDeclarationAspectFit::Partial
        | ForgeQueryDeclarationAspectFit::MissingRequired
        | ForgeQueryDeclarationAspectFit::Conflict => false,
    }
}

pub(super) fn fit_rank(fit: ForgeQueryDeclarationAspectFit) -> u8 {
    match fit {
        ForgeQueryDeclarationAspectFit::Exact => 4,
        ForgeQueryDeclarationAspectFit::CompatibleSuperset => 3,
        ForgeQueryDeclarationAspectFit::Partial => 2,
        ForgeQueryDeclarationAspectFit::MissingRequired => 1,
        ForgeQueryDeclarationAspectFit::Conflict => 0,
    }
}

pub(super) fn denied_on_fit<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    fit: ForgeQueryDeclarationAspectFit,
    partial_narrowing: bool,
    candidates: Vec<ForgeQueryBindingCandidateRecord>,
) -> ForgeQueryBindingTranscript<T> {
    let outcome = match fit {
        ForgeQueryDeclarationAspectFit::Partial if partial_narrowing => {
            ForgeQueryBindingOutcome::ExplicitNarrowingRequired(
                ForgeQueryBindingExplicitNarrowingRequired::new(
                    "binding candidates only partially satisfy the requested semantic slice",
                ),
            )
        }
        ForgeQueryDeclarationAspectFit::Partial
        | ForgeQueryDeclarationAspectFit::MissingRequired => {
            ForgeQueryBindingOutcome::MissingRequiredAspect(
                ForgeQueryBindingMissingRequiredAspect::new(
                    "binding candidates do not cover every required semantic aspect",
                ),
            )
        }
        ForgeQueryDeclarationAspectFit::Conflict => {
            ForgeQueryBindingOutcome::AspectConflict(ForgeQueryBindingAspectConflict::new(
                "binding candidates conflict with the required semantic aspect contract",
            ))
        }
        ForgeQueryDeclarationAspectFit::Exact
        | ForgeQueryDeclarationAspectFit::CompatibleSuperset => {
            ForgeQueryBindingOutcome::Unavailable(ForgeQueryBindingUnavailable::new(
                "no admissible binding candidate remained after aspect filtering",
            ))
        }
    };
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        candidates,
        Vec::new(),
        Some(ForgeQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![ForgeQueryBindingNarrowingDecision::new(
            "binding denied after context candidate fit evaluation",
        )],
        None,
        digest_for(
            family,
            request_kind,
            &contract,
            match fit {
                ForgeQueryDeclarationAspectFit::Partial if partial_narrowing => {
                    "explicit_narrowing_required"
                }
                ForgeQueryDeclarationAspectFit::Partial
                | ForgeQueryDeclarationAspectFit::MissingRequired => "missing_required_aspect",
                ForgeQueryDeclarationAspectFit::Conflict => "aspect_conflict",
                ForgeQueryDeclarationAspectFit::Exact
                | ForgeQueryDeclarationAspectFit::CompatibleSuperset => "unavailable",
            },
        ),
        crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
    )
}

pub(super) fn ambiguous<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: ForgeQueryDeclarationAspectContract,
    candidates: Vec<ForgeQueryBindingCandidateRecord>,
) -> ForgeQueryBindingTranscript<T> {
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        ForgeQueryBindingOutcome::Ambiguous(ForgeQueryBindingAmbiguity::new(
            "multiple binding candidates remained tied after aspect-fit and specificity narrowing",
            candidates.len(),
        )),
        candidates,
        Vec::new(),
        None,
        vec![ForgeQueryBindingNarrowingDecision::new(
            "binding denied because no single exact winner remained",
        )],
        None,
        digest_for(family, request_kind, &contract, "ambiguous"),
        crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
    )
}
