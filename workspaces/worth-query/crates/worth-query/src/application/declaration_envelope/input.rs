use crate::application::{
    WorthQueryDeclarationInput, WorthQueryDeclarationReceipt, WorthQueryDeclarationReceiptChecked,
    WorthQueryDeclarationReceiptDeferred, WorthQueryDeclarationReceiptDenied,
    WorthQueryDeclarationReceiptFailed, WorthQueryDomainEntryMarker,
};

pub enum WorthQueryDeclarationEnvelopeInput<
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    IssuedReceipt(WorthQueryDeclarationReceipt<D, I>),
    DeferredReceipt(WorthQueryDeclarationReceiptDeferred<D, I>),
    DeniedReceipt(WorthQueryDeclarationReceiptDenied<D, I>),
    FailedReceipt(WorthQueryDeclarationReceiptFailed<D, I>),
    ReceiptChecked(WorthQueryDeclarationReceiptChecked<D, I>),
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryDeclarationEnvelopeInput<D, I>
{
    pub fn issued(receipt: WorthQueryDeclarationReceipt<D, I>) -> Self {
        Self::IssuedReceipt(receipt)
    }

    pub fn deferred(receipt: WorthQueryDeclarationReceiptDeferred<D, I>) -> Self {
        Self::DeferredReceipt(receipt)
    }

    pub fn denied(receipt: WorthQueryDeclarationReceiptDenied<D, I>) -> Self {
        Self::DeniedReceipt(receipt)
    }

    pub fn failed(receipt: WorthQueryDeclarationReceiptFailed<D, I>) -> Self {
        Self::FailedReceipt(receipt)
    }

    pub fn receipt_checked(checked: WorthQueryDeclarationReceiptChecked<D, I>) -> Self {
        Self::ReceiptChecked(checked)
    }
}
