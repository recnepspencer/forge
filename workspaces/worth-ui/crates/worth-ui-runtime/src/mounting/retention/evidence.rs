use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiSurfaceBindingGeneration,
};

#[derive(Clone)]
pub(crate) struct UiRetainedPresentedFrame {
    frame: UiMountedFrameIdentity,
    bindings: Box<[UiSurfaceBindingGeneration]>,
    receipts: super::super::UiMountedNodeReceiptBasis,
    structural_bytes: usize,
    mount_cost: super::super::UiMountCostReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentedFrameBasisRelation {
    Current,
    Retained,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiPresentedFrameBasisDenial {
    Expired,
    Unknown,
    BindingNotPresented,
    InstanceNotPresented,
    NodeReceiptMismatch,
}

impl UiRetainedPresentedFrame {
    pub(crate) fn prepare(
        frame: UiMountedFrameIdentity,
        bindings: &[UiSurfaceBindingGeneration],
        receipts: super::super::UiMountedNodeReceiptBasis,
        mount_cost: super::super::UiMountCostReport,
    ) -> Option<Self> {
        let mut bindings = bindings.to_vec();
        bindings.sort();
        bindings.dedup();
        let binding_bytes = bindings
            .len()
            .checked_mul(std::mem::size_of::<UiSurfaceBindingGeneration>())?;
        let structural_bytes = std::mem::size_of::<Self>()
            .checked_add(binding_bytes)?
            .checked_add(receipts.retained_structural_bytes()?)?;
        Some(Self {
            frame,
            bindings: bindings.into_boxed_slice(),
            receipts,
            structural_bytes,
            mount_cost,
        })
    }

    pub(crate) fn frame(&self) -> UiMountedFrameIdentity {
        self.frame
    }

    pub(crate) fn structural_bytes(&self) -> usize {
        self.structural_bytes
    }

    pub(crate) fn presented_binding_count(&self) -> usize {
        self.bindings.len()
    }

    pub(crate) fn mounted_instance_count(&self) -> usize {
        self.receipts.len()
    }

    pub(crate) fn mount_cost(&self) -> super::super::UiMountCostReport {
        self.mount_cost
    }

    pub(crate) fn set_mount_cost(&mut self, mount_cost: super::super::UiMountCostReport) {
        self.mount_cost = mount_cost;
    }

    pub(crate) fn receipt_for_with_probes(
        &self,
        mounted_instance: UiMountedInstanceIdentity,
    ) -> (Option<UiMountedNodeReceiptIdentity>, usize) {
        self.receipts.receipt_for_with_probes(mounted_instance)
    }

    pub(crate) fn classify(
        &self,
        binding: UiSurfaceBindingGeneration,
        mounted_instance: Option<UiMountedInstanceIdentity>,
        node_receipt: Option<UiMountedNodeReceiptIdentity>,
    ) -> Result<(), UiPresentedFrameBasisDenial> {
        if self.bindings.binary_search(&binding).is_err() {
            return Err(UiPresentedFrameBasisDenial::BindingNotPresented);
        }
        match (mounted_instance, node_receipt) {
            (None, None) => Ok(()),
            (Some(instance), Some(receipt)) => {
                let expected = self
                    .receipts
                    .receipt_for(instance)
                    .ok_or(UiPresentedFrameBasisDenial::InstanceNotPresented)?;
                (expected == receipt)
                    .then_some(())
                    .ok_or(UiPresentedFrameBasisDenial::NodeReceiptMismatch)
            }
            _ => Err(UiPresentedFrameBasisDenial::InstanceNotPresented),
        }
    }
}
