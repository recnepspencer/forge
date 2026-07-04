use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};

use crate::evidence::{with_retention_posture, UiEvidenceRef, UiEvidenceSliceRef};
use crate::facade::UiInspectionReceipt;
use crate::obligations::selection::UiSelectedObligationSet;
use worth_ui_inspection::UiEvidenceRetentionPosture;

#[derive(Default)]
pub(crate) struct WorthUiRetainedObligationRegistry {
    selections_by_handle: RefCell<BTreeMap<u64, UiSelectedObligationSet>>,
    handles_by_slice: RefCell<BTreeMap<UiEvidenceSliceRef, Box<[u64]>>>,
    discarded_handles: RefCell<BTreeSet<u64>>,
}

impl WorthUiRetainedObligationRegistry {
    pub(crate) fn register(&self, selected: &UiSelectedObligationSet, receipt: &UiInspectionReceipt) {
        let Some(slice) = receipt.evidence_slice() else {
            return;
        };

        let handle_digests = slice
            .refs()
            .iter()
            .map(|reference| reference.handle().handle_digest())
            .collect::<Vec<_>>();
        if handle_digests.is_empty() {
            return;
        }

        let selected = selected.clone();
        {
            let mut selections_by_handle = self.selections_by_handle.borrow_mut();
            for handle_digest in handle_digests.iter().copied() {
                selections_by_handle.insert(handle_digest, selected.clone());
            }
        }

        if let Some(slice_ref) = receipt.evidence_slice_ref() {
            self.handles_by_slice
                .borrow_mut()
                .insert(slice_ref, handle_digests.into_boxed_slice());
        }
    }

    pub(crate) fn retained_selection(&self, handle_digest: u64) -> Option<UiSelectedObligationSet> {
        self.selections_by_handle.borrow().get(&handle_digest).cloned()
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

        let mut selections_by_handle = self.selections_by_handle.borrow_mut();
        let mut discarded_handles = self.discarded_handles.borrow_mut();
        for handle_digest in handle_digests.iter().copied() {
            selections_by_handle.remove(&handle_digest);
            discarded_handles.insert(handle_digest);
        }
        true
    }
}
