use worth_foundational::facade::FoundationalBoundaryEvidenceMaterializationProfile;

use crate::application::{
    WorthQueryAdmittedConfiguredDomainHandle, WorthQueryDeclarationFamilyMarker,
    WorthQueryDeclarationFoundationalEvidenceInput, WorthQueryDeclarationInput,
    WorthQueryDomainEntryMarker, WorthQueryDomainOperatingContext,
};
use crate::binding_pipeline::{
    WorthQueryBindingAspectFitReport, WorthQueryBindingLinkedArtifacts,
    WorthQueryBindingNarrowingDecision, WorthQueryBindingOutcome,
    WorthQueryBindingRequestDescriptor, WorthQueryBindingTranscript,
    WorthQueryResolveRouteFromTargetRequest, WorthQueryRouteResolverSubject,
};
use crate::target_binding::WorthQueryBindingTargetWitness;

use super::common::{alignment_failure, denied_on_fit, digest_for, fit_allowed};

pub(crate) fn bind_route_from_target_on_handle<
    D: WorthQueryDomainEntryMarker,
    C: WorthQueryDomainOperatingContext<D>,
    I: WorthQueryDeclarationInput<D>,
>(
    handle: &WorthQueryAdmittedConfiguredDomainHandle<D, C>,
    request: WorthQueryResolveRouteFromTargetRequest<D, I>,
) -> WorthQueryBindingTranscript<crate::application::WorthQueryDeclarationRoutePlanInput<D, I>> {
    let (source, contract, allow_superset, partial_narrowing, route_intent) = request.into_parts();
    let progressed = match source {
        WorthQueryRouteResolverSubject::Progression(progressed) => progressed,
    };
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
            "resolve_route_from_target",
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
            "resolve_route_from_target",
            contract,
            coverage,
            fit,
            partial_narrowing,
            binding_target,
            linked,
        );
    }
    let evidence = handle
        .describe_foundational_with_profile(
            WorthQueryDeclarationFoundationalEvidenceInput::admitted_progression(
                progressed.clone(),
            ),
            FoundationalBoundaryEvidenceMaterializationProfile::FullDescriptiveRichness,
        )
        .unwrap_or_else(|_| {
            panic!("same-handle admitted progression should always describe foundational evidence")
        });
    let input = match route_intent {
        Some(intent) => crate::application::WorthQueryDeclarationRoutePlanInput::with_intent(
            progressed, evidence, intent,
        ),
        None => {
            crate::application::WorthQueryDeclarationRoutePlanInput::admitted(progressed, evidence)
        }
    };
    WorthQueryBindingTranscript::new(
        WorthQueryBindingRequestDescriptor::new(
            I::Family::semantic_family_key(),
            "resolve_route_from_target",
            contract.clone(),
        ),
        WorthQueryBindingOutcome::Bound(input),
        Vec::new(),
        vec![crate::binding_pipeline::WorthQueryBindingWitnessCheck::passed("world_alignment")],
        Some(WorthQueryBindingAspectFitReport::new(
            fit,
            contract.clone(),
            coverage,
        )),
        vec![WorthQueryBindingNarrowingDecision::new(
            "binding resolved progression into explicit route-plan input",
        )],
        Some(binding_target.clone()),
        digest_for(
            "resolve_route_from_target",
            I::Family::semantic_family_key(),
            &contract,
            "bound",
            Some(&binding_target),
            &linked,
        ),
        linked,
    )
}
