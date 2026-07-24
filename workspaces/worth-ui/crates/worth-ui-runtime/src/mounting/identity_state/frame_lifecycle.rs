use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiMountedProjectionAudience, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
};

use super::{UiMountedIdentityFrameCandidate, UiMountedIdentityState};
use crate::mounting::{
    UiMountedFrameIdentityView, UiMountedFramePublicationReceipt, UiMountedFrameReuseWitness,
    UiMountedGraphNodeHandle, UiMountedIdentityDenial, UiMountedIdentityView,
    UiMountedInstanceIdentityView, UiPreparedMountedFrame, UiPresentedFrameBasisDenial,
    UiPresentedFrameBasisRelation,
};

impl UiMountedIdentityState {
    pub(crate) fn advance_frame(
        &mut self,
    ) -> Result<UiMountedFrameIdentity, UiMountedIdentityDenial> {
        let candidate = self.prepare_frame_candidate()?;
        let frame = candidate.frame();
        self.publish_frame_candidate(candidate);
        Ok(frame)
    }

    pub(in crate::mounting) fn prepare_frame_candidate(
        &self,
    ) -> Result<UiMountedIdentityFrameCandidate, UiMountedIdentityDenial> {
        let frame = UiMountedFrameIdentity::mint_unbound()
            .map_err(|_| UiMountedIdentityDenial::IdentityExhausted)?;
        let mut receipts = std::collections::BTreeMap::new();
        for identity in &self.visible_order {
            receipts.insert(
                *identity,
                UiMountedFrameIdentityView::new(
                    frame,
                    *identity,
                    UiMountedNodeReceiptIdentity::mint_unbound()
                        .map_err(|_| UiMountedIdentityDenial::IdentityExhausted)?,
                ),
            );
        }
        Ok(UiMountedIdentityFrameCandidate { frame, receipts })
    }

    pub(in crate::mounting) fn publish_frame_candidate(
        &mut self,
        candidate: UiMountedIdentityFrameCandidate,
    ) {
        let UiMountedIdentityFrameCandidate { frame, receipts } = candidate;
        self.current_frame = Some(frame);
        self.current_receipts = receipts;
        self.current_projection = None;
        self.current_manifest = None;
        self.current_core = None;
        self.current_publication = None;
    }

    pub(crate) fn publish_presented_frame(
        &mut self,
        frame: UiPreparedMountedFrame,
        receipt: UiMountedFramePublicationReceipt,
        presented_basis: super::super::retention::UiPreparedPresentedFrameBasis,
    ) {
        let (candidate, manifest, core) = frame.into_publication_parts();
        self.current_manifest = Some(manifest);
        self.current_core = Some(core);
        let (projection, identity_candidate) = candidate.into_parts();
        let UiMountedIdentityFrameCandidate { frame, receipts } = identity_candidate;
        self.presented_frames.publish(presented_basis);
        self.current_frame = Some(frame);
        self.current_receipts = receipts;
        self.current_projection = Some(projection);
        self.current_publication = Some(receipt);
    }

    pub(crate) fn publish_reconciled_frame(
        &mut self,
        frame: UiPreparedMountedFrame,
        receipt: UiMountedFramePublicationReceipt,
        presented_basis: super::super::retention::UiPreparedPresentedFrameBasis,
    ) {
        debug_assert_eq!(self.current_frame, Some(frame.canonical_core().frame()));
        let (candidate, manifest, core) = frame.into_publication_parts();
        self.current_manifest = Some(manifest);
        self.current_core = Some(core);
        let (projection, identity_candidate) = candidate.into_parts();
        let UiMountedIdentityFrameCandidate { frame, receipts } = identity_candidate;
        self.presented_frames.reconcile_current(presented_basis);
        self.current_frame = Some(frame);
        self.current_receipts = receipts;
        self.current_projection = Some(projection);
        self.current_publication = Some(receipt);
    }

    pub(crate) fn prepare_current_reconciliation_frame(
        &self,
        replacements: &[super::super::UiMountedSurfaceReconciliationBinding],
    ) -> Result<UiPreparedMountedFrame, UiMountedIdentityDenial> {
        if replacements.is_empty() {
            return Err(UiMountedIdentityDenial::ReconciliationBasisMismatch);
        }
        let replacement_views = replacements
            .iter()
            .map(|replacement| {
                self.bindings
                    .values()
                    .find(|record| record.view.binding_generation() == replacement.replacement())
                    .map(|record| (replacement.affected(), record.view))
                    .ok_or(UiMountedIdentityDenial::UnknownSurfaceBinding)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let projection = self
            .current_projection
            .as_ref()
            .ok_or(UiMountedIdentityDenial::NoPublishedMountedFrame)?
            .rebound(&replacement_views)
            .map_err(|_| UiMountedIdentityDenial::ReconciliationBasisMismatch)?;
        let current_core = self
            .current_core
            .ok_or(UiMountedIdentityDenial::NoPublishedMountedFrame)?;
        let current_manifest = self
            .current_manifest
            .as_ref()
            .ok_or(UiMountedIdentityDenial::NoPublishedMountedFrame)?;
        let requirements = current_manifest
            .surfaces()
            .iter()
            .map(|requirement| {
                replacement_views
                    .iter()
                    .find(|(affected, _)| requirement.binding() == *affected)
                    .map(|(_, replacement)| super::super::binding_requirement(*replacement))
                    .unwrap_or(*requirement)
            })
            .collect();
        let manifest = worth_ui_host_contract::UiMountedFrameManifest::new(
            requirements,
            current_manifest.lane_contributions().to_vec(),
        );
        let candidate = super::super::UiProjectedMountedFrameCandidate {
            frame: projection,
            identity_candidate: UiMountedIdentityFrameCandidate {
                frame: current_core.frame(),
                receipts: self.current_receipts.clone(),
            },
        };
        UiPreparedMountedFrame::admit(
            candidate,
            self.current_publication
                .as_ref()
                .ok_or(UiMountedIdentityDenial::NoPublishedMountedFrame)?
                .generation()
                .clone(),
            manifest,
            current_core.graph_world(),
            current_core.allocation_truth_revision(),
        )
        .map_err(|_| UiMountedIdentityDenial::ReconciliationBasisMismatch)
    }

    pub(crate) fn publication_receipt(&self) -> Option<&UiMountedFramePublicationReceipt> {
        self.current_publication.as_ref()
    }

    pub(crate) fn current_requires_binding(&self, binding: UiSurfaceBindingGeneration) -> bool {
        self.current_manifest.as_ref().is_some_and(|manifest| {
            manifest
                .surfaces()
                .iter()
                .any(|requirement| requirement.binding() == binding)
        })
    }

    pub(crate) fn reuse_witness(&self) -> Option<UiMountedFrameReuseWitness> {
        let receipt = self.current_publication.as_ref()?;
        let mut current_bindings = self
            .bindings
            .values()
            .map(|record| record.view.binding_generation())
            .collect::<Vec<_>>();
        current_bindings.sort();
        if current_bindings != receipt.bindings() {
            return None;
        }
        Some(UiMountedFrameReuseWitness::new(
            receipt.frame(),
            receipt.bindings().to_vec().into_boxed_slice(),
        ))
    }

    pub(crate) fn reuse_receipt(
        &self,
        witness: &UiMountedFrameReuseWitness,
    ) -> Option<UiMountedFramePublicationReceipt> {
        let receipt = self.current_publication.as_ref()?;
        (receipt.frame() == witness.frame() && receipt.bindings() == witness.bindings())
            .then(|| receipt.clone())
    }

    pub(in crate::mounting) fn projection_identity_view(
        &self,
        candidate: &UiMountedIdentityFrameCandidate,
    ) -> UiMountedIdentityView {
        let mounted_instances = self
            .visible_order
            .iter()
            .filter_map(|identity| {
                self.instances.get(identity).map(|record| {
                    UiMountedInstanceIdentityView::new(*identity, record.basis.clone())
                })
            })
            .collect();
        let surface_bindings = self.bindings.values().map(|record| record.view).collect();
        let frame_receipts = candidate.receipts.values().copied().collect();
        UiMountedIdentityView::new(
            mounted_instances,
            surface_bindings,
            Some(candidate.frame),
            frame_receipts,
        )
    }

    pub(in crate::mounting) fn audience_for(
        &self,
        surface: UiSemanticSurfaceIdentity,
    ) -> Option<UiMountedProjectionAudience> {
        self.semantic_surfaces.get(&surface).copied()
    }

    pub(crate) fn instances_for(
        &self,
        handle: UiMountedGraphNodeHandle,
    ) -> Result<Box<[UiMountedInstanceIdentity]>, UiMountedIdentityDenial> {
        self.require_handle(handle)?;
        Ok(self
            .by_graph
            .get(&handle.graph_node_identity())
            .into_iter()
            .flat_map(|instances| instances.iter().copied())
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub(crate) fn validate_binding(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.bindings
            .values()
            .any(|record| record.view.binding_generation() == binding)
            .then_some(())
            .ok_or(UiMountedIdentityDenial::UnknownSurfaceBinding)
    }

    pub(crate) fn validate_current_frame(
        &self,
        frame: UiMountedFrameIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        (self.current_frame == Some(frame))
            .then_some(())
            .ok_or(UiMountedIdentityDenial::FrameNotCurrent)
    }

    pub(crate) fn validate_current_receipt(
        &self,
        instance: UiMountedInstanceIdentity,
        receipt: UiMountedNodeReceiptIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        let current = self
            .current_receipts
            .get(&instance)
            .ok_or(UiMountedIdentityDenial::NodeReceiptNotCurrent)?;
        (current.node_receipt_identity() == receipt)
            .then_some(())
            .ok_or(UiMountedIdentityDenial::NodeReceiptNotCurrent)
    }

    pub(crate) fn classify_presented_frame_basis(
        &self,
        frame: UiMountedFrameIdentity,
        binding: UiSurfaceBindingGeneration,
        mounted_instance: Option<UiMountedInstanceIdentity>,
        node_receipt: Option<UiMountedNodeReceiptIdentity>,
    ) -> Result<UiPresentedFrameBasisRelation, UiPresentedFrameBasisDenial> {
        self.presented_frames
            .classify(frame, binding, mounted_instance, node_receipt)
    }

    pub(crate) fn view(&self) -> UiMountedIdentityView {
        let mounted_instances = self
            .visible_order
            .iter()
            .filter_map(|identity| {
                self.instances.get(identity).map(|record| {
                    UiMountedInstanceIdentityView::new(*identity, record.basis.clone())
                })
            })
            .collect();
        let surface_bindings = self.bindings.values().map(|record| record.view).collect();
        let frame_receipts = self.current_receipts.values().copied().collect();
        UiMountedIdentityView::new(
            mounted_instances,
            surface_bindings,
            self.current_frame,
            frame_receipts,
        )
    }
}
