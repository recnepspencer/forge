use std::sync::Arc;

use super::super::WorthUiPresentationRequestBasis;

#[derive(Debug)]
pub(super) struct PresentationCorrespondenceAuthority {
    _sealed: (),
}

pub struct WorthUiPresentationCorrespondenceIssuer {
    authority: Arc<PresentationCorrespondenceAuthority>,
    next_nonce: u64,
}

pub struct WorthUiPresentationRuntimeCorrespondence {
    authority: Arc<PresentationCorrespondenceAuthority>,
    nonce: u64,
    basis: WorthUiPresentationRequestBasis,
}

pub struct WorthUiPresentationValidatedCompletion {
    authority: Arc<PresentationCorrespondenceAuthority>,
    receipt_authority: Arc<PresentationCorrespondenceAuthority>,
    pending_nonce: u64,
    payload_byte_len: u64,
}

pub struct WorthUiPresentationEffectsIndeterminateObservation {
    authority: Arc<PresentationCorrespondenceAuthority>,
    receipt_authority: Arc<PresentationCorrespondenceAuthority>,
    pending_nonce: u64,
    observed_payload_byte_len: u64,
}

pub struct WorthUiPresentationSupersededPhysicalObservation {
    authority: Arc<PresentationCorrespondenceAuthority>,
    receipt_authority: Arc<PresentationCorrespondenceAuthority>,
    pending_nonce: u64,
    observed_payload_byte_len: u64,
}

pub struct WorthUiPresentationCancellationEffectsObservation {
    indeterminate: WorthUiPresentationEffectsIndeterminateObservation,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiPresentationCorrespondenceIssuanceDenial {
    NonceExhausted,
}

pub(super) fn correspondence_authority_pair() -> (
    Arc<PresentationCorrespondenceAuthority>,
    WorthUiPresentationCorrespondenceIssuer,
) {
    let authority = Arc::new(PresentationCorrespondenceAuthority { _sealed: () });
    (
        Arc::clone(&authority),
        WorthUiPresentationCorrespondenceIssuer {
            authority,
            next_nonce: 0,
        },
    )
}

impl WorthUiPresentationCorrespondenceIssuer {
    pub fn issue(
        &mut self,
        basis: WorthUiPresentationRequestBasis,
    ) -> Result<
        WorthUiPresentationRuntimeCorrespondence,
        WorthUiPresentationCorrespondenceIssuanceDenial,
    > {
        self.next_nonce = self
            .next_nonce
            .checked_add(1)
            .ok_or(WorthUiPresentationCorrespondenceIssuanceDenial::NonceExhausted)?;
        Ok(WorthUiPresentationRuntimeCorrespondence {
            authority: Arc::clone(&self.authority),
            nonce: self.next_nonce,
            basis,
        })
    }

    pub fn certify_presented(
        &self,
        receipt: &super::WorthUiPresentationPendingReceipt,
        payload_byte_len: u64,
    ) -> WorthUiPresentationValidatedCompletion {
        WorthUiPresentationValidatedCompletion {
            authority: Arc::clone(&self.authority),
            receipt_authority: Arc::clone(&receipt.authority),
            pending_nonce: receipt.nonce,
            payload_byte_len,
        }
    }

    pub fn certify_effects_indeterminate(
        &self,
        receipt: &super::WorthUiPresentationPendingReceipt,
        observed_payload_byte_len: u64,
    ) -> WorthUiPresentationEffectsIndeterminateObservation {
        WorthUiPresentationEffectsIndeterminateObservation {
            authority: Arc::clone(&self.authority),
            receipt_authority: Arc::clone(&receipt.authority),
            pending_nonce: receipt.nonce,
            observed_payload_byte_len,
        }
    }

    pub fn certify_superseded_physical(
        &self,
        receipt: &super::WorthUiPresentationPendingReceipt,
        observed_payload_byte_len: u64,
    ) -> WorthUiPresentationSupersededPhysicalObservation {
        WorthUiPresentationSupersededPhysicalObservation {
            authority: Arc::clone(&self.authority),
            receipt_authority: Arc::clone(&receipt.authority),
            pending_nonce: receipt.nonce,
            observed_payload_byte_len,
        }
    }

    pub fn certify_cancellation_effects_may_have_begun(
        &self,
        receipt: &super::WorthUiPresentationPendingReceipt,
        observed_payload_byte_len: u64,
    ) -> WorthUiPresentationCancellationEffectsObservation {
        WorthUiPresentationCancellationEffectsObservation {
            indeterminate: self.certify_effects_indeterminate(receipt, observed_payload_byte_len),
        }
    }
}

impl WorthUiPresentationRuntimeCorrespondence {
    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<PresentationCorrespondenceAuthority>,
        u64,
        WorthUiPresentationRequestBasis,
    ) {
        (self.authority, self.nonce, self.basis)
    }
}

impl WorthUiPresentationValidatedCompletion {
    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<PresentationCorrespondenceAuthority>,
        Arc<PresentationCorrespondenceAuthority>,
        u64,
        u64,
    ) {
        (
            self.authority,
            self.receipt_authority,
            self.pending_nonce,
            self.payload_byte_len,
        )
    }
}

impl WorthUiPresentationEffectsIndeterminateObservation {
    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<PresentationCorrespondenceAuthority>,
        Arc<PresentationCorrespondenceAuthority>,
        u64,
        u64,
    ) {
        (
            self.authority,
            self.receipt_authority,
            self.pending_nonce,
            self.observed_payload_byte_len,
        )
    }
}

impl WorthUiPresentationSupersededPhysicalObservation {
    pub(super) fn into_parts(
        self,
    ) -> (
        Arc<PresentationCorrespondenceAuthority>,
        Arc<PresentationCorrespondenceAuthority>,
        u64,
        u64,
    ) {
        (
            self.authority,
            self.receipt_authority,
            self.pending_nonce,
            self.observed_payload_byte_len,
        )
    }
}

impl WorthUiPresentationCancellationEffectsObservation {
    pub(super) fn into_indeterminate(self) -> WorthUiPresentationEffectsIndeterminateObservation {
        self.indeterminate
    }
}

pub(super) fn is_correspondence_authority(
    expected: &Arc<PresentationCorrespondenceAuthority>,
    observed: &Arc<PresentationCorrespondenceAuthority>,
) -> bool {
    Arc::ptr_eq(expected, observed)
}
