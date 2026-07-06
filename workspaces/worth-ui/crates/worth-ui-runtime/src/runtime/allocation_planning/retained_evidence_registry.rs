use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::evidence::{
    with_retention_posture, UiAllocationPlanningInspectionReceipt, UiEvidenceRef,
    UiEvidenceSliceRef,
};
use worth_ui_inspection::UiEvidenceRetentionPosture;

#[derive(Debug, Default)]
pub(crate) struct WorthUiRetainedAllocationPlanningEvidenceRegistry {
    receipts_by_handle: RefCell<BTreeMap<u64, UiAllocationPlanningInspectionReceipt>>,
    handles_by_slice: RefCell<BTreeMap<UiEvidenceSliceRef, Box<[u64]>>>,
    discarded_handles: RefCell<BTreeSet<u64>>,
}

impl WorthUiRetainedAllocationPlanningEvidenceRegistry {
    pub(crate) fn register(&self, receipt: &UiAllocationPlanningInspectionReceipt) {
        let handle_digests = receipt
            .evidence_slice()
            .refs()
            .iter()
            .map(|reference| reference.handle().handle_digest())
            .collect::<Vec<_>>();
        if handle_digests.is_empty() {
            return;
        }

        {
            let mut discarded_handles = self.discarded_handles.borrow_mut();
            for handle_digest in handle_digests.iter().copied() {
                discarded_handles.remove(&handle_digest);
            }
        }

        let mut receipts_by_handle = self.receipts_by_handle.borrow_mut();
        for handle_digest in handle_digests.iter().copied() {
            receipts_by_handle.insert(handle_digest, receipt.clone());
        }
        self.handles_by_slice.borrow_mut().insert(
            receipt.evidence_slice().slice_ref(),
            handle_digests.into_boxed_slice(),
        );
    }

    pub(crate) fn retained_receipt(
        &self,
        handle_digest: u64,
    ) -> Option<UiAllocationPlanningInspectionReceipt> {
        self.receipts_by_handle
            .borrow()
            .get(&handle_digest)
            .cloned()
    }

    pub(crate) fn retained_receipts(&self) -> Box<[UiAllocationPlanningInspectionReceipt]> {
        self.receipts_by_handle
            .borrow()
            .values()
            .fold(BTreeMap::new(), |mut receipts_by_slice, receipt| {
                receipts_by_slice
                    .entry(receipt.evidence_slice().slice_ref())
                    .or_insert_with(|| receipt.clone());
                receipts_by_slice
            })
            .into_values()
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    pub(crate) fn current_generation_for(
        &self,
        handle_digest: u64,
    ) -> Option<worth_ui_inspection::UiEvidenceAuthorityGeneration> {
        self.receipts_by_handle
            .borrow()
            .get(&handle_digest)
            .map(|receipt| receipt.evidence_slice().authority_generation())
    }

    pub(crate) fn discarded_ref(&self, evidence_ref: UiEvidenceRef) -> Option<UiEvidenceRef> {
        self.discarded_handles
            .borrow()
            .contains(&evidence_ref.handle().handle_digest())
            .then(|| {
                with_retention_posture(
                    evidence_ref,
                    UiEvidenceRetentionPosture::DiscardedWithTombstone,
                )
            })
    }

    pub(crate) fn discard_slice(&self, slice_ref: UiEvidenceSliceRef) -> bool {
        let Some(handle_digests) = self.handles_by_slice.borrow_mut().remove(&slice_ref) else {
            return false;
        };

        let mut receipts_by_handle = self.receipts_by_handle.borrow_mut();
        let mut discarded_handles = self.discarded_handles.borrow_mut();
        for handle_digest in handle_digests.iter().copied() {
            receipts_by_handle.remove(&handle_digest);
            discarded_handles.insert(handle_digest);
        }
        true
    }
}
