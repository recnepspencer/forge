use worth_ui_host_contract::UiMountedFrameIdentity;

use super::super::authority::{UiMountedRetainedFrameLookup, UiMountedRetentionPinAdmissionDenial};
#[cfg(any(test, feature = "certification-support"))]
use super::super::lease::UiMountedVisualOverlayClass;
#[cfg(any(test, feature = "certification-support"))]
use super::super::UiMountedVisualOverlayLease;
use super::super::{
    UiMountedRetentionClass, UiMountedVisualCaptureBasis, UiMountedVisualLease,
    UiMountedVisualLeaseClass, UiMountedVisualRetentionDenial, UiMountedVisualSnapshotClass,
};
use super::UiMountedFrameRetentionCoordinator;

impl UiMountedFrameRetentionCoordinator {
    pub(crate) fn visual_snapshot_relation(
        &self,
        frame: UiMountedFrameIdentity,
    ) -> Option<worth_ui_inspection::UiVisualSnapshotRelation> {
        match self.authority.borrow().frame(frame) {
            UiMountedRetainedFrameLookup::Found { relation, .. } => Some(match relation {
                super::super::UiPresentedFrameBasisRelation::Current => {
                    worth_ui_inspection::UiVisualSnapshotRelation::Current
                }
                super::super::UiPresentedFrameBasisRelation::Retained => {
                    worth_ui_inspection::UiVisualSnapshotRelation::RetainedPredecessor
                }
            }),
            UiMountedRetainedFrameLookup::Expired { .. }
            | UiMountedRetainedFrameLookup::Unknown { .. } => None,
        }
    }

    pub(crate) fn acquire_visual_snapshot(
        &self,
        frame: UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<UiMountedVisualCaptureBasis, UiMountedVisualRetentionDenial> {
        let (lease, visual_regions, identity_trace) =
            self.acquire_visual_lease::<UiMountedVisualSnapshotClass>(frame, binding)?;
        Ok(UiMountedVisualCaptureBasis::new(
            lease,
            visual_regions,
            identity_trace,
        ))
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn acquire_visual_overlay_for_certification(
        &self,
        frame: UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<UiMountedVisualOverlayLease, UiMountedVisualRetentionDenial> {
        self.acquire_visual_lease::<UiMountedVisualOverlayClass>(frame, binding)
            .map(|(lease, _, _)| lease)
    }

    fn acquire_visual_lease<Class: UiMountedVisualLeaseClass>(
        &self,
        frame: UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<
        (
            UiMountedVisualLease<Class>,
            super::super::super::UiMountedVisualRegionBasis,
            super::super::super::UiMountedIdentityTraceBasis,
        ),
        UiMountedVisualRetentionDenial,
    > {
        let (structural_bytes, visual_regions, identity_trace) =
            self.visual_lease_basis(frame, binding)?;
        self.authority
            .borrow_mut()
            .reserve_pin(frame, Class::CLASS, structural_bytes)
            .map_err(|denial| map_visual_pin_denial(denial, Class::CLASS))?;
        Ok((
            UiMountedVisualLease::<Class>::from_reserved(&self.authority, frame, structural_bytes),
            visual_regions,
            identity_trace,
        ))
    }

    fn visual_lease_basis(
        &self,
        frame: UiMountedFrameIdentity,
        binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    ) -> Result<
        (
            usize,
            super::super::super::UiMountedVisualRegionBasis,
            super::super::super::UiMountedIdentityTraceBasis,
        ),
        UiMountedVisualRetentionDenial,
    > {
        match self.authority.borrow().frame(frame) {
            UiMountedRetainedFrameLookup::Found { evidence, .. } => Ok((
                evidence.structural_bytes(),
                evidence.visual_region_basis(binding),
                evidence.identity_trace_basis(),
            )),
            UiMountedRetainedFrameLookup::Expired { .. } => {
                Err(UiMountedVisualRetentionDenial::ExpiredFrame)
            }
            UiMountedRetainedFrameLookup::Unknown { .. } => {
                Err(UiMountedVisualRetentionDenial::UnknownFrame)
            }
        }
    }
}

fn map_visual_pin_denial(
    denial: UiMountedRetentionPinAdmissionDenial,
    class: UiMountedRetentionClass,
) -> UiMountedVisualRetentionDenial {
    match denial {
        UiMountedRetentionPinAdmissionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        } => UiMountedVisualRetentionDenial::CapacityExceeded {
            class,
            required_leases,
            required_structural_bytes,
            budget,
        },
        UiMountedRetentionPinAdmissionDenial::AccountingOverflow => {
            UiMountedVisualRetentionDenial::AccountingOverflow { class }
        }
    }
}
