use crate::application::{
    ForgeQueryDeclarationInput, ForgeQueryDeclarationReceipt, ForgeQueryDeclarationReceiptChecked,
    ForgeQueryDeclarationReceiptDeferred, ForgeQueryDeclarationReceiptDenied,
    ForgeQueryDeclarationReceiptFailed, ForgeQueryDomainEntryMarker,
};

pub enum ForgeQueryDeclarationEnvelopeInput<
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    IssuedReceipt(ForgeQueryDeclarationReceipt<D, I>),
    DeferredReceipt(ForgeQueryDeclarationReceiptDeferred<D, I>),
    DeniedReceipt(ForgeQueryDeclarationReceiptDenied<D, I>),
    FailedReceipt(ForgeQueryDeclarationReceiptFailed<D, I>),
    ReceiptChecked(ForgeQueryDeclarationReceiptChecked<D, I>),
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryDeclarationEnvelopeInput<D, I>
{
    pub fn issued(receipt: ForgeQueryDeclarationReceipt<D, I>) -> Self {
        Self::IssuedReceipt(receipt)
    }

    pub fn deferred(receipt: ForgeQueryDeclarationReceiptDeferred<D, I>) -> Self {
        Self::DeferredReceipt(receipt)
    }

    pub fn denied(receipt: ForgeQueryDeclarationReceiptDenied<D, I>) -> Self {
        Self::DeniedReceipt(receipt)
    }

    pub fn failed(receipt: ForgeQueryDeclarationReceiptFailed<D, I>) -> Self {
        Self::FailedReceipt(receipt)
    }

    pub fn receipt_checked(checked: ForgeQueryDeclarationReceiptChecked<D, I>) -> Self {
        Self::ReceiptChecked(checked)
    }
}
