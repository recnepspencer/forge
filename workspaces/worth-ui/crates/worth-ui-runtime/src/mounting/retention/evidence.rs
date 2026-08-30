use worth_ui_host_contract::{
    UiHostPresentationEpoch, UiMountedFrameIdentity, UiMountedInstanceIdentity,
    UiMountedNodeReceiptIdentity, UiSurfaceBindingGeneration,
};

#[derive(Clone)]
pub(crate) struct UiRetainedPresentedFrame {
    frame: UiMountedFrameIdentity,
    bindings: Box<[UiSurfaceBindingGeneration]>,
    presentation_epochs: Box<[(UiSurfaceBindingGeneration, UiHostPresentationEpoch)]>,
    presentation: Option<super::super::UiMountedPresentationReceipt>,
    receipts: super::super::UiMountedNodeReceiptBasis,
    visual_regions: super::super::UiMountedVisualRegionBasis,
    identity_trace_basis: super::super::UiMountedIdentityTraceBasis,
    structural_bytes: usize,
    mount_cost: super::super::UiMountCostReport,
}

pub(super) struct UiRetainedPresentedFrameInput {
    pub(super) frame: UiMountedFrameIdentity,
    pub(super) bindings: Box<[UiSurfaceBindingGeneration]>,
    pub(super) receipts: super::super::UiMountedNodeReceiptBasis,
    pub(super) mount_cost: super::super::UiMountCostReport,
    pub(super) visual_regions: super::super::UiMountedVisualRegionBasis,
    pub(super) identity_trace_basis: super::super::UiMountedIdentityTraceBasis,
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
    PresentationEpochMismatch,
    PresentationTruthUnavailable,
    InstanceNotPresented,
    NodeReceiptMismatch,
}

impl UiRetainedPresentedFrame {
    pub(super) fn prepare(input: UiRetainedPresentedFrameInput) -> Option<Self> {
        let mut bindings = input.bindings.into_vec();
        bindings.sort();
        bindings.dedup();
        let binding_bytes = bindings
            .len()
            .checked_mul(std::mem::size_of::<UiSurfaceBindingGeneration>())?;
        let presentation_surface_bytes = bindings.len().checked_mul(std::mem::size_of::<
            super::super::UiMountedSurfacePresentationReceipt,
        >())?;
        let presentation_epoch_bytes = bindings.len().checked_mul(std::mem::size_of::<(
            UiSurfaceBindingGeneration,
            UiHostPresentationEpoch,
        )>())?;
        let visual_region_structural_bytes = input.visual_regions.retained_structural_bytes()?;
        let identity_trace_structural_bytes =
            input.identity_trace_basis.retained_structural_bytes()?;
        let structural_bytes = std::mem::size_of::<Self>()
            .checked_add(binding_bytes)?
            .checked_add(std::mem::size_of::<
                super::super::UiMountedPresentationReceipt,
            >())?
            .checked_add(presentation_surface_bytes)?
            .checked_add(presentation_epoch_bytes)?
            .checked_add(input.receipts.retained_structural_bytes()?)?
            .checked_add(visual_region_structural_bytes)?
            .checked_add(identity_trace_structural_bytes)?;
        Some(Self {
            frame: input.frame,
            bindings: bindings.into_boxed_slice(),
            presentation_epochs: Box::default(),
            presentation: None,
            receipts: input.receipts,
            visual_regions: input.visual_regions,
            identity_trace_basis: input.identity_trace_basis,
            structural_bytes,
            mount_cost: input.mount_cost,
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

    pub(crate) fn visual_region_basis(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> super::super::UiMountedVisualRegionBasis {
        self.visual_regions
            .for_binding(binding, self.receipts.clone())
    }

    pub(crate) fn identity_trace_basis(&self) -> super::super::UiMountedIdentityTraceBasis {
        self.identity_trace_basis.clone()
    }

    pub(crate) fn projection_input(
        &self,
        slot: worth_ui_query_binding::UiProjectionInputSlot,
    ) -> Option<&worth_ui_query_binding::UiProjectionInputFactReference> {
        self.identity_trace_basis.projection_input(slot)
    }

    pub(crate) fn set_mount_cost(&mut self, mount_cost: super::super::UiMountCostReport) {
        self.mount_cost = mount_cost;
    }

    pub(crate) fn set_presentation_receipt(
        &mut self,
        presentation: super::super::UiMountedPresentationReceipt,
    ) {
        debug_assert_eq!(presentation.frame(), self.frame);
        let mut presentation_epochs = presentation
            .surfaces()
            .iter()
            .map(|surface| (surface.binding(), surface.epoch()))
            .collect::<Vec<_>>();
        presentation_epochs.sort_by_key(|(binding, _)| *binding);
        self.presentation_epochs = presentation_epochs.into_boxed_slice();
        self.presentation = Some(presentation);
    }

    pub(crate) fn update_presentation_epoch(
        &mut self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<(), UiPresentedFrameBasisDenial> {
        if presentation.frame() != self.frame {
            return Err(UiPresentedFrameBasisDenial::Unknown);
        }
        let index = self
            .presentation_epochs
            .binary_search_by_key(&presentation.binding(), |(binding, _)| *binding)
            .map_err(|_| UiPresentedFrameBasisDenial::BindingNotPresented)?;
        self.presentation_epochs[index].1 = presentation.epoch();
        Ok(())
    }

    pub(crate) fn presentation_receipt(
        &self,
    ) -> Option<&super::super::UiMountedPresentationReceipt> {
        self.presentation.as_ref()
    }

    pub(crate) fn receipt_for_with_probes(
        &self,
        mounted_instance: UiMountedInstanceIdentity,
    ) -> (Option<UiMountedNodeReceiptIdentity>, usize) {
        self.receipts.receipt_for_with_probes(mounted_instance)
    }

    pub(crate) fn classify(
        &self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        mounted_instance: Option<UiMountedInstanceIdentity>,
        node_receipt: Option<UiMountedNodeReceiptIdentity>,
    ) -> Result<(), UiPresentedFrameBasisDenial> {
        let binding = presentation.binding();
        if self.bindings.binary_search(&binding).is_err() {
            return Err(UiPresentedFrameBasisDenial::BindingNotPresented);
        }
        let retained_epoch = self
            .presentation_epochs
            .binary_search_by_key(&binding, |(candidate, _)| *candidate)
            .ok()
            .map(|index| self.presentation_epochs[index].1)
            .ok_or(UiPresentedFrameBasisDenial::BindingNotPresented)?;
        if retained_epoch != presentation.epoch() {
            return Err(UiPresentedFrameBasisDenial::PresentationEpochMismatch);
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
