use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    checked_route_plan_from_progressed_with_profile, worth_query_checked_declaration_receipt,
    WorthQueryDeclarationFamilyMarker, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceiptInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext, WorthQueryInstalledDomainDeclarationContext,
};
use crate::binding_pipeline::{
    WorthQueryBindingAspectFitReport, WorthQueryBindingLinkedArtifacts,
    WorthQueryBindingNarrowingDecision, WorthQueryBindingOutcome,
    WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript,
    WorthQueryEnvelopeResolverSubject, WorthQueryResolveEnvelopeFromTargetRequest,
};
use crate::target_binding::WorthQueryBindingTargetWitness;

use super::common::{alignment_failure, denied_on_fit, digest_for, fit_allowed};

pub(crate) fn bind_envelope_from_target_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryInstalledDomainDeclarationContext<D, C>,
    request: WorthQueryResolveEnvelopeFromTargetRequest<D, I>,
) -> WorthQueryBindingTranscript<crate::application::WorthQueryDeclarationEnvelopeInput<D, I>> {
    let (source, contract, allow_superset, partial_narrowing, route_intent) = request.into_parts();
    match source {
        WorthQueryEnvelopeResolverSubject::Progression(progressed) => {
            let binding_target = progressed.binding_target().into_erased_target();
            let linked = WorthQueryBindingLinkedArtifacts::new()
                .with_declaration_digest(format!(
                    "{:?}",
                    progressed.canonical_declaration().declaration_digest()
                ))
                .with_progression_digest(progressed.progression_digest().to_string());
            let same_world = progressed.operating_context_identity_digest()
                == handle.operating_context_identity_digest();
            let same_handle = progressed.canonical_declaration().handle_identity_digest()
                == handle.handle_identity_digest();
            if !same_world || !same_handle {
                return alignment_failure(
                    I::Family::semantic_family_key(),
                    "resolve_envelope_from_target",
                    contract,
                    binding_target,
                    linked,
                    same_world,
                );
            }
            let coverage = progressed.reviewed_aspect_coverage().clone();
            let fit = coverage.fit_against(&contract);
            if !fit_allowed(fit, allow_superset) {
                return denied_on_fit(
                    I::Family::semantic_family_key(),
                    "resolve_envelope_from_target",
                    contract,
                    coverage,
                    fit,
                    partial_narrowing,
                    binding_target,
                    linked,
                );
            }
            let route_checked = checked_route_plan_from_progressed_with_profile(
                handle,
                progressed,
                route_intent,
                FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
            );
            let receipt_checked = worth_query_checked_declaration_receipt(
                WorthQueryDeclarationReceiptInput::route_checked(route_checked),
            );
            WorthQueryBindingTranscript::new(
                WorthQueryBindingRequestDescriptor::new(
                    I::Family::semantic_family_key(),
                    "resolve_envelope_from_target",
                    contract.clone(),
                ),
                WorthQueryBindingOutcome::Bound(
                    crate::application::WorthQueryDeclarationEnvelopeInput::receipt_checked(
                        receipt_checked,
                    ),
                ),
                Vec::new(),
                vec![
                    crate::binding_pipeline::WorthQueryBindingWitnessCheck::passed(
                        "world_alignment",
                    ),
                ],
                Some(WorthQueryBindingAspectFitReport::new(
                    fit,
                    contract.clone(),
                    coverage,
                )),
                vec![WorthQueryBindingNarrowingDecision::new(
                    "binding resolved progression into explicit envelope input",
                )],
                Some(binding_target.clone()),
                digest_for(
                    "resolve_envelope_from_target",
                    I::Family::semantic_family_key(),
                    &contract,
                    "bound",
                    Some(&binding_target),
                    &linked,
                ),
                linked,
            )
        }
        WorthQueryEnvelopeResolverSubject::Receipt(receipt) => {
            let binding_target = receipt.binding_target().into_erased_target();
            let coverage = receipt.aspect_coverage().clone();
            let fit = coverage.fit_against(&contract);
            let mut linked = WorthQueryBindingLinkedArtifacts::new()
                .with_declaration_digest(receipt.declaration_digest().to_string())
                .with_receipt_digest(format!("{:?}", receipt.receipt_digest()));
            if let Some(progression_digest) = receipt.progression_digest() {
                linked = linked.with_progression_digest(progression_digest.to_string());
            }
            if let Some(route_plan_digest) = receipt.route_plan_digest() {
                linked = linked.with_route_plan_digest(route_plan_digest.to_string());
            }
            let same_world = receipt.operating_context_identity_digest()
                == handle.operating_context_identity_digest();
            let same_handle = receipt.handle_identity_digest() == handle.handle_identity_digest();
            if !same_world || !same_handle {
                return alignment_failure(
                    I::Family::semantic_family_key(),
                    "resolve_envelope_from_target",
                    contract,
                    binding_target,
                    linked,
                    same_world,
                );
            }
            if !fit_allowed(fit, allow_superset) {
                return denied_on_fit(
                    I::Family::semantic_family_key(),
                    "resolve_envelope_from_target",
                    contract,
                    coverage,
                    fit,
                    partial_narrowing,
                    binding_target,
                    linked,
                );
            }
            WorthQueryBindingTranscript::new(
                WorthQueryBindingRequestDescriptor::new(
                    I::Family::semantic_family_key(),
                    "resolve_envelope_from_target",
                    contract.clone(),
                ),
                WorthQueryBindingOutcome::Bound(
                    crate::application::WorthQueryDeclarationEnvelopeInput::issued(receipt),
                ),
                Vec::new(),
                vec![
                    crate::binding_pipeline::WorthQueryBindingWitnessCheck::passed(
                        "world_alignment",
                    ),
                ],
                Some(WorthQueryBindingAspectFitReport::new(
                    fit,
                    contract.clone(),
                    coverage,
                )),
                vec![WorthQueryBindingNarrowingDecision::new(
                    "binding resolved receipt into explicit envelope input",
                )],
                Some(binding_target.clone()),
                digest_for(
                    "resolve_envelope_from_target",
                    I::Family::semantic_family_key(),
                    &contract,
                    "bound",
                    Some(&binding_target),
                    &linked,
                ),
                linked,
            )
        }
    }
}
