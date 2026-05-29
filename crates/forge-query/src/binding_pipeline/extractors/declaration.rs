use crate::application::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationFamilyMarker,
    ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker, ForgeQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    ForgeQueryBindingAspectFitReport, ForgeQueryBindingNarrowingDecision, ForgeQueryBindingOutcome,
    ForgeQueryBindingRequestDescriptor, ForgeQueryBindingTranscript, ForgeQueryBindingUnavailable,
    ForgeQueryDeclarationBindingRequest,
};

use super::common::{ambiguous, denied_on_fit, digest_for, fit_allowed, fit_rank};

pub(crate) fn bind_declaration_from_context_on_handle<
    D: ForgeQueryDomainEntryMarker,
    C: ForgeQueryDomainOperatingContext<D>,
    I: ForgeQueryDeclarationInput<D>,
>(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<D, C>,
    request: ForgeQueryDeclarationBindingRequest<I>,
) -> ForgeQueryBindingTranscript<crate::application::ForgeQueryCanonicalDeclarationArtifact<D, I>> {
    let (candidates, contract, allowed_sources, allow_superset, partial_narrowing) =
        request.into_parts();
    let mut selected = Vec::new();
    let mut records = Vec::new();
    let mut best_denied = None;
    for candidate in candidates {
        let (label, source_kind, specificity, input) = candidate.into_parts();
        if !allowed_sources.is_empty() && !allowed_sources.contains(&source_kind) {
            continue;
        }
        let declared = match handle.declare(input) {
            Ok(declared) => declared,
            Err(_) => continue,
        };
        let coverage = I::Family::aspect_coverage();
        let fit = coverage.fit_against(&contract);
        records.push(
            crate::binding_pipeline::ForgeQueryBindingCandidateRecord::new(
                label,
                source_kind,
                specificity,
                None,
            ),
        );
        if fit_allowed(fit, allow_superset) {
            selected.push((
                fit_rank(fit),
                usize::MAX - coverage.present().len(),
                specificity,
                fit,
                coverage,
                declared,
            ));
        } else if best_denied
            .as_ref()
            .map(
                |(best_fit, _): &(
                    crate::application::ForgeQueryDeclarationAspectFit,
                    crate::application::ForgeQueryDeclarationAspectCoverage,
                )| fit_rank(fit) > fit_rank(*best_fit),
            )
            .unwrap_or(true)
        {
            best_denied = Some((fit, coverage));
        }
    }
    if selected.is_empty() {
        if let Some((fit, coverage)) = best_denied {
            return denied_on_fit(
                I::Family::semantic_family_key(),
                "bind_declaration_from_context",
                contract,
                coverage,
                fit,
                partial_narrowing,
                records,
            );
        }
        return ForgeQueryBindingTranscript::new(
            ForgeQueryBindingRequestDescriptor::new(
                I::Family::semantic_family_key(),
                "bind_declaration_from_context",
                contract.clone(),
            ),
            ForgeQueryBindingOutcome::Unavailable(ForgeQueryBindingUnavailable::new(
                "no admissible declaration context candidates were available",
            )),
            records,
            Vec::new(),
            None,
            vec![ForgeQueryBindingNarrowingDecision::new(
                "binding stopped because no allowed context candidates could be declared",
            )],
            None,
            digest_for(
                I::Family::semantic_family_key(),
                "bind_declaration_from_context",
                &contract,
                "unavailable",
            ),
            crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
        );
    }
    selected.sort_by(|left, right| {
        right
            .0
            .cmp(&left.0)
            .then(right.1.cmp(&left.1))
            .then(right.2.cmp(&left.2))
    });
    if selected.len() > 1
        && selected[0].0 == selected[1].0
        && selected[0].1 == selected[1].1
        && selected[0].2 == selected[1].2
    {
        return ambiguous(
            I::Family::semantic_family_key(),
            "bind_declaration_from_context",
            contract,
            records,
        );
    }
    let (_, _, _, fit, coverage, declared) = selected.remove(0);
    ForgeQueryBindingTranscript::new(
        ForgeQueryBindingRequestDescriptor::new(
            I::Family::semantic_family_key(),
            "bind_declaration_from_context",
            contract.clone(),
        ),
        ForgeQueryBindingOutcome::Bound(declared),
        records,
        vec![crate::binding_pipeline::ForgeQueryBindingWitnessCheck::passed("family_admission")],
        Some(ForgeQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![ForgeQueryBindingNarrowingDecision::new(
            "binding selected the single best declaration candidate by aspect fit and specificity",
        )],
        None,
        digest_for(
            I::Family::semantic_family_key(),
            "bind_declaration_from_context",
            &contract,
            "bound",
        ),
        crate::binding_pipeline::ForgeQueryBindingLinkedArtifacts::new(),
    )
}
