use crate::application::{
    WorthQueryDeclarationAspectContract, WorthQueryDeclarationAspectCoverage,
    WorthQueryDeclarationAspectFit, WorthQueryDeclarationAuthorityAspectMismatch,
};
use crate::binding_pipeline::{
    WorthQueryBindingAspectConflict, WorthQueryBindingAspectFitReport,
    WorthQueryBindingAuthorityMismatch, WorthQueryBindingBasisMismatch,
    WorthQueryBindingExplicitNarrowingRequired, WorthQueryBindingLinkedArtifacts,
    WorthQueryBindingMissingRequiredAspect, WorthQueryBindingNarrowingDecision,
    WorthQueryBindingOutcome, WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript,
    WorthQueryBindingUnavailable, WorthQueryBindingWitnessCheck, WorthQueryBindingWrongHandle,
    WorthQueryBindingWrongWorld,
};
use crate::identity::hash_parts;
use crate::target_binding::WorthQueryBindingTarget;

pub(super) fn binding_outcome_from_fit<T>(
    fit: WorthQueryDeclarationAspectFit,
    partial_is_narrowing_required: bool,
) -> Option<WorthQueryBindingOutcome<T>> {
    match fit {
        WorthQueryDeclarationAspectFit::Exact
        | WorthQueryDeclarationAspectFit::CompatibleSuperset => None,
        WorthQueryDeclarationAspectFit::Partial if partial_is_narrowing_required => {
            Some(WorthQueryBindingOutcome::ExplicitNarrowingRequired(
                WorthQueryBindingExplicitNarrowingRequired::new(
                    "the retained artifact only partially covers the required semantic slice",
                ),
            ))
        }
        WorthQueryDeclarationAspectFit::Partial
        | WorthQueryDeclarationAspectFit::MissingRequired => {
            Some(WorthQueryBindingOutcome::MissingRequiredAspect(
                WorthQueryBindingMissingRequiredAspect::new(
                    "the retained artifact does not cover every required semantic aspect",
                ),
            ))
        }
        WorthQueryDeclarationAspectFit::Conflict => Some(WorthQueryBindingOutcome::AspectConflict(
            WorthQueryBindingAspectConflict::new(
                "the retained artifact conflicts with the required semantic aspect contract",
            ),
        )),
    }
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

pub(super) fn digest_for(
    kind: &str,
    family: &'static str,
    contract: &WorthQueryDeclarationAspectContract,
    outcome_key: &str,
    resolved_target: Option<&WorthQueryBindingTarget>,
    linked: &WorthQueryBindingLinkedArtifacts,
) -> String {
    hash_parts(&[
        "worth_query_binding_pipeline_v1".to_string(),
        kind.to_string(),
        family.to_string(),
        format!("contract:{contract:?}"),
        format!("outcome:{outcome_key}"),
        format!(
            "resolved:{}",
            resolved_target
                .map(|target| target.binding_digest().to_string())
                .unwrap_or_else(|| "none".to_string())
        ),
        format!("linked:{linked:?}"),
    ])
}

pub(super) fn alignment_failure<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: WorthQueryDeclarationAspectContract,
    resolved_target: WorthQueryBindingTarget,
    linked: WorthQueryBindingLinkedArtifacts,
    same_world: bool,
) -> WorthQueryBindingTranscript<T> {
    let (outcome, witness_check, outcome_key) = if same_world {
        (
            WorthQueryBindingOutcome::WrongHandle(WorthQueryBindingWrongHandle::new(
                "the retained artifact belongs to a different admitted handle",
            )),
            WorthQueryBindingWitnessCheck::failed(
                "handle_alignment",
                "handle identity digest did not match",
            ),
            "wrong_handle",
        )
    } else {
        (
            WorthQueryBindingOutcome::WrongWorld(WorthQueryBindingWrongWorld::new(
                "the retained artifact belongs to a different admitted world",
            )),
            WorthQueryBindingWitnessCheck::failed(
                "world_alignment",
                "operating context digest did not match",
            ),
            "wrong_world",
        )
    };
    WorthQueryBindingTranscript::new(
        WorthQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        Vec::new(),
        vec![witness_check],
        None,
        vec![WorthQueryBindingNarrowingDecision::new(
            "binding stopped at admitted alignment witness failure",
        )],
        Some(resolved_target.clone()),
        digest_for(
            request_kind,
            family,
            &contract,
            outcome_key,
            Some(&resolved_target),
            &linked,
        ),
        linked,
    )
}

pub(super) fn denied_on_fit<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    fit: WorthQueryDeclarationAspectFit,
    partial_narrowing: bool,
    resolved_target: WorthQueryBindingTarget,
    linked: WorthQueryBindingLinkedArtifacts,
) -> WorthQueryBindingTranscript<T> {
    let outcome = binding_outcome_from_fit(fit, partial_narrowing).unwrap_or_else(|| {
        WorthQueryBindingOutcome::Unavailable(WorthQueryBindingUnavailable::new(
            "the retained artifact did not satisfy the binding contract",
        ))
    });
    WorthQueryBindingTranscript::new(
        WorthQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        Vec::new(),
        vec![WorthQueryBindingWitnessCheck::passed("world_alignment")],
        Some(WorthQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![WorthQueryBindingNarrowingDecision::new(
            "binding denied after aspect-fit evaluation",
        )],
        Some(resolved_target.clone()),
        digest_for(
            request_kind,
            family,
            &contract,
            match fit {
                WorthQueryDeclarationAspectFit::Exact
                | WorthQueryDeclarationAspectFit::CompatibleSuperset => "unavailable",
                WorthQueryDeclarationAspectFit::Partial if partial_narrowing => {
                    "explicit_narrowing_required"
                }
                WorthQueryDeclarationAspectFit::Partial
                | WorthQueryDeclarationAspectFit::MissingRequired => "missing_required_aspect",
                WorthQueryDeclarationAspectFit::Conflict => "aspect_conflict",
            },
            Some(&resolved_target),
            &linked,
        ),
        linked,
    )
}

pub(super) fn denied_on_authority_mismatch<T>(
    family: &'static str,
    request_kind: &'static str,
    contract: WorthQueryDeclarationAspectContract,
    coverage: WorthQueryDeclarationAspectCoverage,
    fit: WorthQueryDeclarationAspectFit,
    mismatch: WorthQueryDeclarationAuthorityAspectMismatch,
    resolved_target: WorthQueryBindingTarget,
    linked: WorthQueryBindingLinkedArtifacts,
) -> WorthQueryBindingTranscript<T> {
    let (outcome, outcome_key) = match mismatch {
        WorthQueryDeclarationAuthorityAspectMismatch::MissingRequiredAspect => (
            WorthQueryBindingOutcome::MissingRequiredAspect(
                WorthQueryBindingMissingRequiredAspect::new(mismatch.reason()),
            ),
            "missing_required_aspect",
        ),
        WorthQueryDeclarationAuthorityAspectMismatch::AspectConflict => (
            WorthQueryBindingOutcome::AspectConflict(WorthQueryBindingAspectConflict::new(
                mismatch.reason(),
            )),
            "aspect_conflict",
        ),
        WorthQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap
        | WorthQueryDeclarationAuthorityAspectMismatch::AuthorityAspectAmbiguity => (
            WorthQueryBindingOutcome::AuthorityMismatch(WorthQueryBindingAuthorityMismatch::new(
                mismatch.reason(),
            )),
            "authority_mismatch",
        ),
        WorthQueryDeclarationAuthorityAspectMismatch::BasisAspectMismatch => (
            WorthQueryBindingOutcome::BasisMismatch(WorthQueryBindingBasisMismatch::new(
                mismatch.reason(),
            )),
            "basis_mismatch",
        ),
    };
    WorthQueryBindingTranscript::new(
        WorthQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        Vec::new(),
        vec![WorthQueryBindingWitnessCheck::failed(
            "authority_alignment",
            mismatch.reason(),
        )],
        Some(WorthQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![WorthQueryBindingNarrowingDecision::new(
            "binding denied after authority-scoped aspect evaluation",
        )],
        Some(resolved_target.clone()),
        digest_for(
            request_kind,
            family,
            &contract,
            outcome_key,
            Some(&resolved_target),
            &linked,
        ),
        linked,
    )
}
