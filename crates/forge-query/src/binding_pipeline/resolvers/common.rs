use crate::application::{
    ForgeQueryDeclarationAspectContract, ForgeQueryDeclarationAspectCoverage,
    ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationAuthorityAspectMismatch,
};
use crate::binding_pipeline::{
    ForgeQueryBindingAspectConflict, ForgeQueryBindingAspectFitReport,
    ForgeQueryBindingAuthorityMismatch, ForgeQueryBindingBasisMismatch,
    ForgeQueryBindingExplicitNarrowingRequired, ForgeQueryBindingLinkedArtifacts,
    ForgeQueryBindingMissingRequiredAspect, ForgeQueryBindingNarrowingDecision,
    ForgeQueryBindingOutcome, ForgeQueryBindingRequestDescriptor, ForgeQueryBindingTranscript,
    ForgeQueryBindingUnavailable, ForgeQueryBindingWitnessCheck, ForgeQueryBindingWrongHandle,
    ForgeQueryBindingWrongWorld,
};
use crate::identity::hash_parts;
use crate::target_binding::ForgeQueryBindingTarget;

pub(super) fn binding_outcome_from_fit<T>(
    fit: ForgeQueryDeclarationAspectFit,
    partial_is_narrowing_required: bool,
) -> Option<ForgeQueryBindingOutcome<T>> {
    match fit {
        ForgeQueryDeclarationAspectFit::Exact
        | ForgeQueryDeclarationAspectFit::CompatibleSuperset => None,
        ForgeQueryDeclarationAspectFit::Partial if partial_is_narrowing_required => {
            Some(ForgeQueryBindingOutcome::ExplicitNarrowingRequired(
                ForgeQueryBindingExplicitNarrowingRequired::new(
                    "the retained artifact only partially covers the required semantic slice",
                ),
            ))
        }
        ForgeQueryDeclarationAspectFit::Partial
        | ForgeQueryDeclarationAspectFit::MissingRequired => {
            Some(ForgeQueryBindingOutcome::MissingRequiredAspect(
                ForgeQueryBindingMissingRequiredAspect::new(
                    "the retained artifact does not cover every required semantic aspect",
                ),
            ))
        }
        ForgeQueryDeclarationAspectFit::Conflict => Some(ForgeQueryBindingOutcome::AspectConflict(
            ForgeQueryBindingAspectConflict::new(
                "the retained artifact conflicts with the required semantic aspect contract",
            ),
        )),
    }
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

pub(super) fn digest_for(
    kind: &str,
    family: &'static str,
    contract: &ForgeQueryDeclarationAspectContract,
    outcome_key: &str,
    resolved_target: Option<&ForgeQueryBindingTarget>,
    linked: &ForgeQueryBindingLinkedArtifacts,
) -> String {
    hash_parts(&[
        "forge_query_binding_pipeline_v1".to_string(),
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
    contract: ForgeQueryDeclarationAspectContract,
    resolved_target: ForgeQueryBindingTarget,
    linked: ForgeQueryBindingLinkedArtifacts,
    same_world: bool,
) -> ForgeQueryBindingTranscript<T> {
    let (outcome, witness_check, outcome_key) = if same_world {
        (
            ForgeQueryBindingOutcome::WrongHandle(ForgeQueryBindingWrongHandle::new(
                "the retained artifact belongs to a different admitted handle",
            )),
            ForgeQueryBindingWitnessCheck::failed(
                "handle_alignment",
                "handle identity digest did not match",
            ),
            "wrong_handle",
        )
    } else {
        (
            ForgeQueryBindingOutcome::WrongWorld(ForgeQueryBindingWrongWorld::new(
                "the retained artifact belongs to a different admitted world",
            )),
            ForgeQueryBindingWitnessCheck::failed(
                "world_alignment",
                "operating context digest did not match",
            ),
            "wrong_world",
        )
    };
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        Vec::new(),
        vec![witness_check],
        None,
        vec![ForgeQueryBindingNarrowingDecision::new(
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
    contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    fit: ForgeQueryDeclarationAspectFit,
    partial_narrowing: bool,
    resolved_target: ForgeQueryBindingTarget,
    linked: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryBindingTranscript<T> {
    let outcome = binding_outcome_from_fit(fit, partial_narrowing).unwrap_or_else(|| {
        ForgeQueryBindingOutcome::Unavailable(ForgeQueryBindingUnavailable::new(
            "the retained artifact did not satisfy the binding contract",
        ))
    });
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        Vec::new(),
        vec![ForgeQueryBindingWitnessCheck::passed("world_alignment")],
        Some(ForgeQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![ForgeQueryBindingNarrowingDecision::new(
            "binding denied after aspect-fit evaluation",
        )],
        Some(resolved_target.clone()),
        digest_for(
            request_kind,
            family,
            &contract,
            match fit {
                ForgeQueryDeclarationAspectFit::Exact
                | ForgeQueryDeclarationAspectFit::CompatibleSuperset => "unavailable",
                ForgeQueryDeclarationAspectFit::Partial if partial_narrowing => {
                    "explicit_narrowing_required"
                }
                ForgeQueryDeclarationAspectFit::Partial
                | ForgeQueryDeclarationAspectFit::MissingRequired => "missing_required_aspect",
                ForgeQueryDeclarationAspectFit::Conflict => "aspect_conflict",
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
    contract: ForgeQueryDeclarationAspectContract,
    coverage: ForgeQueryDeclarationAspectCoverage,
    fit: ForgeQueryDeclarationAspectFit,
    mismatch: ForgeQueryDeclarationAuthorityAspectMismatch,
    resolved_target: ForgeQueryBindingTarget,
    linked: ForgeQueryBindingLinkedArtifacts,
) -> ForgeQueryBindingTranscript<T> {
    let (outcome, outcome_key) = match mismatch {
        ForgeQueryDeclarationAuthorityAspectMismatch::MissingRequiredAspect => (
            ForgeQueryBindingOutcome::MissingRequiredAspect(
                ForgeQueryBindingMissingRequiredAspect::new(mismatch.reason()),
            ),
            "missing_required_aspect",
        ),
        ForgeQueryDeclarationAuthorityAspectMismatch::AspectConflict => (
            ForgeQueryBindingOutcome::AspectConflict(ForgeQueryBindingAspectConflict::new(
                mismatch.reason(),
            )),
            "aspect_conflict",
        ),
        ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectGap
        | ForgeQueryDeclarationAuthorityAspectMismatch::AuthorityAspectAmbiguity => (
            ForgeQueryBindingOutcome::AuthorityMismatch(ForgeQueryBindingAuthorityMismatch::new(
                mismatch.reason(),
            )),
            "authority_mismatch",
        ),
        ForgeQueryDeclarationAuthorityAspectMismatch::BasisAspectMismatch => (
            ForgeQueryBindingOutcome::BasisMismatch(ForgeQueryBindingBasisMismatch::new(
                mismatch.reason(),
            )),
            "basis_mismatch",
        ),
    };
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(family, request_kind, contract.clone()),
        outcome,
        Vec::new(),
        vec![ForgeQueryBindingWitnessCheck::failed(
            "authority_alignment",
            mismatch.reason(),
        )],
        Some(ForgeQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![ForgeQueryBindingNarrowingDecision::new(
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
