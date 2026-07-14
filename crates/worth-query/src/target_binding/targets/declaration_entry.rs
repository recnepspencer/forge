use crate::application::{
    WorthQueryAdmittedDeclarationProgression, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationEnvelope, WorthQueryDeclarationInput,
    WorthQueryDeclarationReceipt, WorthQueryDeclarationRoutePlan, WorthQueryDomainEntryMarker,
};

use crate::target_binding::{
    WorthQueryBindingTarget, WorthQueryBindingTargetKind, WorthQueryBindingTargetSemantics,
    WorthQueryBindingTargetWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryAdmittedDeclarationProgressionBindingTarget(WorthQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationRoutePlanBindingTarget(WorthQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationReceiptBindingTarget(WorthQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDeclarationEnvelopeBindingTarget(WorthQueryBindingTarget);

#[derive(Clone)]
pub(crate) struct WorthQueryAdmittedDeclarationProgressionBindingTargetSource<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    pub(crate) progressed: WorthQueryAdmittedDeclarationProgression<D, I>,
    pub(crate) handle_identity_digest: String,
    pub(crate) operating_context_identity_digest: String,
    pub(crate) declaration_digest: String,
    pub(crate) progression_digest: String,
    pub(crate) declaration_family_label: &'static str,
    pub(crate) aspect_contract: WorthQueryDeclarationAspectContract,
    pub(crate) reviewed_aspect_coverage: WorthQueryDeclarationAspectCoverage,
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryAdmittedDeclarationProgressionBindingTargetSource<D, I>
{
    pub(crate) fn new(progressed: WorthQueryAdmittedDeclarationProgression<D, I>) -> Self {
        let handle_identity_digest = progressed
            .canonical_declaration()
            .handle_identity_digest()
            .to_string();
        let operating_context_identity_digest =
            progressed.operating_context_identity_digest().to_string();
        let declaration_digest = format!(
            "{:?}",
            progressed.canonical_declaration().declaration_digest()
        );
        let progression_digest = progressed.progression_digest().to_string();
        let declaration_family_label = progressed.declaration_family_key();
        let aspect_contract = progressed.aspect_contract().clone();
        let reviewed_aspect_coverage = progressed.reviewed_aspect_coverage().clone();
        Self {
            progressed,
            handle_identity_digest,
            operating_context_identity_digest,
            declaration_digest,
            progression_digest,
            declaration_family_label,
            aspect_contract,
            reviewed_aspect_coverage,
        }
    }
}

impl WorthQueryAdmittedDeclarationProgressionBindingTarget {
    pub fn for_progressed<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
        progressed: &WorthQueryAdmittedDeclarationProgression<D, I>,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::AdmittedDeclarationProgression,
            progressed.progression_digest().to_string(),
            WorthQueryBindingTargetSemantics::for_progressed_declaration(progressed),
        ))
    }

    pub(crate) fn from_source<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
        source: &WorthQueryAdmittedDeclarationProgressionBindingTargetSource<D, I>,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::AdmittedDeclarationProgression,
            source.progression_digest.clone(),
            WorthQueryBindingTargetSemantics::for_admitted_declaration_progression(
                source.handle_identity_digest.clone(),
                source.operating_context_identity_digest.clone(),
                source.declaration_digest.clone(),
                source.progression_digest.clone(),
                source.declaration_family_label,
                source.aspect_contract.clone(),
                source.reviewed_aspect_coverage.clone(),
            ),
        ))
    }
}

impl WorthQueryDeclarationRoutePlanBindingTarget {
    pub fn for_route_plan<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
        route_plan: &WorthQueryDeclarationRoutePlan<D, I>,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::DeclarationRoutePlan,
            route_plan.route_plan_digest().to_string(),
            WorthQueryBindingTargetSemantics::for_declaration_route_plan(route_plan),
        ))
    }
}

impl WorthQueryDeclarationReceiptBindingTarget {
    pub fn for_receipt<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
        receipt: &WorthQueryDeclarationReceipt<D, I>,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::DeclarationReceipt,
            format!("{:?}", receipt.receipt_digest()),
            WorthQueryBindingTargetSemantics::for_declaration_receipt(receipt),
        ))
    }
}

impl WorthQueryDeclarationEnvelopeBindingTarget {
    pub fn for_envelope<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>(
        envelope: &WorthQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self(WorthQueryBindingTarget::new(
            WorthQueryBindingTargetKind::DeclarationEnvelope,
            format!("{:?}", envelope.envelope_digest()),
            WorthQueryBindingTargetSemantics::for_declaration_envelope(envelope),
        ))
    }
}

impl crate::target_binding::sealed::Sealed
    for WorthQueryAdmittedDeclarationProgressionBindingTarget
{
}
impl crate::target_binding::sealed::Sealed for WorthQueryDeclarationRoutePlanBindingTarget {}
impl crate::target_binding::sealed::Sealed for WorthQueryDeclarationReceiptBindingTarget {}
impl crate::target_binding::sealed::Sealed for WorthQueryDeclarationEnvelopeBindingTarget {}

impl WorthQueryBindingTargetWitness for WorthQueryAdmittedDeclarationProgressionBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryDeclarationRoutePlanBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryDeclarationReceiptBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}

impl WorthQueryBindingTargetWitness for WorthQueryDeclarationEnvelopeBindingTarget {
    fn erased_target(&self) -> &WorthQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> WorthQueryBindingTarget {
        self.0
    }
}
