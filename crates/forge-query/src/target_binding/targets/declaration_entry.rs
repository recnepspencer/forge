use crate::application::{
    ForgeQueryAdmittedDeclarationProgression, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationInput,
    ForgeQueryDeclarationReceipt, ForgeQueryDeclarationRoutePlan, ForgeQueryDomainEntryMarker,
};

use crate::target_binding::{
    ForgeQueryBindingTarget, ForgeQueryBindingTargetKind, ForgeQueryBindingTargetSemantics,
    ForgeQueryBindingTargetWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryAdmittedDeclarationProgressionBindingTarget(ForgeQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationRoutePlanBindingTarget(ForgeQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationReceiptBindingTarget(ForgeQueryBindingTarget);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationEnvelopeBindingTarget(ForgeQueryBindingTarget);

#[derive(Clone)]
pub(crate) struct ForgeQueryAdmittedDeclarationProgressionBindingTargetSource<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    pub(crate) progressed: ForgeQueryAdmittedDeclarationProgression<D, I>,
    pub(crate) handle_identity_digest: String,
    pub(crate) operating_context_identity_digest: String,
    pub(crate) declaration_digest: String,
    pub(crate) progression_digest: String,
    pub(crate) declaration_family_label: &'static str,
    pub(crate) aspect_contract: ForgeQueryDeclarationAspectContract,
    pub(crate) reviewed_aspect_coverage: ForgeQueryDeclarationAspectCoverage,
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryAdmittedDeclarationProgressionBindingTargetSource<D, I>
{
    pub(crate) fn new(progressed: ForgeQueryAdmittedDeclarationProgression<D, I>) -> Self {
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

impl ForgeQueryAdmittedDeclarationProgressionBindingTarget {
    pub fn for_progressed<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        progressed: &ForgeQueryAdmittedDeclarationProgression<D, I>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::AdmittedDeclarationProgression,
            progressed.progression_digest().to_string(),
            ForgeQueryBindingTargetSemantics::for_progressed_declaration(progressed),
        ))
    }

    pub(crate) fn from_source<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        source: &ForgeQueryAdmittedDeclarationProgressionBindingTargetSource<D, I>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::AdmittedDeclarationProgression,
            source.progression_digest.clone(),
            ForgeQueryBindingTargetSemantics::for_admitted_declaration_progression(
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

impl ForgeQueryDeclarationRoutePlanBindingTarget {
    pub fn for_route_plan<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        route_plan: &ForgeQueryDeclarationRoutePlan<D, I>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::DeclarationRoutePlan,
            route_plan.route_plan_digest().to_string(),
            ForgeQueryBindingTargetSemantics::for_declaration_route_plan(route_plan),
        ))
    }
}

impl ForgeQueryDeclarationReceiptBindingTarget {
    pub fn for_receipt<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        receipt: &ForgeQueryDeclarationReceipt<D, I>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::DeclarationReceipt,
            format!("{:?}", receipt.receipt_digest()),
            ForgeQueryBindingTargetSemantics::for_declaration_receipt(receipt),
        ))
    }
}

impl ForgeQueryDeclarationEnvelopeBindingTarget {
    pub fn for_envelope<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>(
        envelope: &ForgeQueryDeclarationEnvelope<D, I>,
    ) -> Self {
        Self(ForgeQueryBindingTarget::new(
            ForgeQueryBindingTargetKind::DeclarationEnvelope,
            format!("{:?}", envelope.envelope_digest()),
            ForgeQueryBindingTargetSemantics::for_declaration_envelope(envelope),
        ))
    }
}

impl crate::target_binding::sealed::Sealed
    for ForgeQueryAdmittedDeclarationProgressionBindingTarget
{
}
impl crate::target_binding::sealed::Sealed for ForgeQueryDeclarationRoutePlanBindingTarget {}
impl crate::target_binding::sealed::Sealed for ForgeQueryDeclarationReceiptBindingTarget {}
impl crate::target_binding::sealed::Sealed for ForgeQueryDeclarationEnvelopeBindingTarget {}

impl ForgeQueryBindingTargetWitness for ForgeQueryAdmittedDeclarationProgressionBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryDeclarationRoutePlanBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryDeclarationReceiptBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}

impl ForgeQueryBindingTargetWitness for ForgeQueryDeclarationEnvelopeBindingTarget {
    fn erased_target(&self) -> &ForgeQueryBindingTarget {
        &self.0
    }

    fn into_erased_target(self) -> ForgeQueryBindingTarget {
        self.0
    }
}
