use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    aspect_coverage_from_publication, checked_route_plan_from_progressed_with_profile,
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationInput, WorthQueryDeclarationReceiptInput, WorthQueryDomainEntryMarker,
    WorthQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    WorthQueryBindingAspectFitReport, WorthQueryBindingLinkedArtifacts,
    WorthQueryBindingNarrowingDecision, WorthQueryBindingOutcome,
    WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript,
    WorthQueryReceiptResolverSubject, WorthQueryResolveReceiptFromTargetRequest,
};
use crate::target_binding::WorthQueryBindingTargetWitness;

use super::common::{alignment_failure, denied_on_fit, digest_for, fit_allowed};

pub(crate) fn bind_receipt_from_target_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    request: WorthQueryResolveReceiptFromTargetRequest<D, I>,
) -> WorthQueryBindingTranscript<WorthQueryDeclarationReceiptInput<D, I>> {
    let (source, contract, allow_superset, partial_narrowing, route_intent) = request.into_parts();
    match source {
        WorthQueryReceiptResolverSubject::Progression(progressed) => {
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
                    "resolve_receipt_from_target",
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
                    "resolve_receipt_from_target",
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
            WorthQueryBindingTranscript::new(
                WorthQueryBindingRequestDescriptor::new(
                    I::Family::semantic_family_key(),
                    "resolve_receipt_from_target",
                    contract.clone(),
                ),
                WorthQueryBindingOutcome::Bound(WorthQueryDeclarationReceiptInput::route_checked(
                    route_checked,
                )),
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
                    "binding resolved progression into explicit receipt input",
                )],
                Some(binding_target.clone()),
                digest_for(
                    "resolve_receipt_from_target",
                    I::Family::semantic_family_key(),
                    &contract,
                    "bound",
                    Some(&binding_target),
                    &linked,
                ),
                linked,
            )
        }
        WorthQueryReceiptResolverSubject::RoutePlan(route_plan) => {
            let binding_target = route_plan.binding_target().into_erased_target();
            let coverage = aspect_coverage_from_publication(route_plan.aspect_publication());
            let fit = coverage.fit_against(&contract);
            let linked = WorthQueryBindingLinkedArtifacts::new()
                .with_declaration_digest(route_plan.declaration_digest().to_string())
                .with_progression_digest(route_plan.progression_digest().to_string())
                .with_route_plan_digest(route_plan.route_plan_digest().to_string());
            let same_world = route_plan.operating_context_identity_digest()
                == handle.operating_context_identity_digest();
            let same_handle =
                route_plan.handle_identity_digest() == handle.handle_identity_digest();
            if !same_world || !same_handle {
                return alignment_failure(
                    I::Family::semantic_family_key(),
                    "resolve_receipt_from_target",
                    contract,
                    binding_target,
                    linked,
                    same_world,
                );
            }
            if !fit_allowed(fit, allow_superset) {
                return denied_on_fit(
                    I::Family::semantic_family_key(),
                    "resolve_receipt_from_target",
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
                    "resolve_receipt_from_target",
                    contract.clone(),
                ),
                WorthQueryBindingOutcome::Bound(WorthQueryDeclarationReceiptInput::planned(
                    route_plan,
                )),
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
                    "binding resolved route plan into explicit receipt input",
                )],
                Some(binding_target.clone()),
                digest_for(
                    "resolve_receipt_from_target",
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
