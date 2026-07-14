use crate::application::{
    WorthQueryDeclarationEnvelope, WorthQueryDeclarationEnvelopeChecked,
    WorthQueryDeclarationEnvelopeDeferred, WorthQueryDeclarationEnvelopeDenied,
    WorthQueryDeclarationEnvelopeFailed, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

pub enum WorthQueryDeclarationRelationalRoutingInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    Enveloped(WorthQueryDeclarationEnvelope<D, I>),
    Deferred(WorthQueryDeclarationEnvelopeDeferred<D, I>),
    Denied(WorthQueryDeclarationEnvelopeDenied<D, I>),
    Failed(WorthQueryDeclarationEnvelopeFailed<D, I>),
    EnvelopeChecked(WorthQueryDeclarationEnvelopeChecked<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationRelationalRoutingInput<D, I>
{
    pub fn enveloped(envelope: WorthQueryDeclarationEnvelope<D, I>) -> Self {
        Self::Enveloped(envelope)
    }

    pub fn deferred(envelope: WorthQueryDeclarationEnvelopeDeferred<D, I>) -> Self {
        Self::Deferred(envelope)
    }

    pub fn denied(envelope: WorthQueryDeclarationEnvelopeDenied<D, I>) -> Self {
        Self::Denied(envelope)
    }

    pub fn failed(envelope: WorthQueryDeclarationEnvelopeFailed<D, I>) -> Self {
        Self::Failed(envelope)
    }

    pub fn envelope_checked(checked: WorthQueryDeclarationEnvelopeChecked<D, I>) -> Self {
        Self::EnvelopeChecked(checked)
    }
}
