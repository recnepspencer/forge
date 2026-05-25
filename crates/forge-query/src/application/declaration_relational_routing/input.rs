use crate::application::{
    ForgeQueryDeclarationEnvelope, ForgeQueryDeclarationEnvelopeChecked,
    ForgeQueryDeclarationEnvelopeDeferred, ForgeQueryDeclarationEnvelopeDenied,
    ForgeQueryDeclarationEnvelopeFailed, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

pub enum ForgeQueryDeclarationRelationalRoutingInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    Enveloped(ForgeQueryDeclarationEnvelope<D, I>),
    Deferred(ForgeQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(ForgeQueryDeclarationEnvelopeDenied<D, I>),
    Failed(ForgeQueryDeclarationEnvelopeFailed<D, I>),
    EnvelopeChecked(ForgeQueryDeclarationEnvelopeChecked<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationRelationalRoutingInput<D, I>
{
    pub fn enveloped(envelope: ForgeQueryDeclarationEnvelope<D, I>) -> Self {
        Self::Enveloped(envelope)
    }

    pub fn deferred(envelope: ForgeQueryDeclarationEnvelopeDeferred<D, I>) -> Self {
        Self::Deferred(envelope)
    }

    pub fn denied(envelope: ForgeQueryDeclarationEnvelopeDenied<D, I>) -> Self {
        Self::Denied(envelope)
    }

    pub fn failed(envelope: ForgeQueryDeclarationEnvelopeFailed<D, I>) -> Self {
        Self::Failed(envelope)
    }

    pub fn envelope_checked(checked: ForgeQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::EnvelopeChecked(checked)
    }
}
