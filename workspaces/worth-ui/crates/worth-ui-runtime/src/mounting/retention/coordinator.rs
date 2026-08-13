use std::cell::RefCell;
use std::rc::Rc;

use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
};

use super::authority::{
    UiMountedFrameRetentionAuthority, UiMountedRetainedFrameLookup,
    UiMountedRetentionPinAdmissionDenial,
};
use super::successor_admission::admit_successor;
use super::{
    UiMountedFrameRetentionBudget, UiMountedFrameRetentionRejection,
    UiMountedObservationBasisLease, UiMountedObservationBasisRetentionDenial,
    UiMountedRetentionClass, UiPresentedFrameBasisDenial, UiPresentedFrameBasisRelation,
    UiPresentedHitTestBasis, UiRetentionPreparedMountedFrame,
};

mod inspection;
mod visual_lease;

pub(crate) struct UiMountedFrameRetentionCoordinator {
    authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
}

impl UiMountedFrameRetentionCoordinator {
    pub(crate) fn with_budget(budget: UiMountedFrameRetentionBudget) -> Self {
        Self {
            authority: Rc::new(RefCell::new(UiMountedFrameRetentionAuthority::new(budget))),
        }
    }

    pub(crate) fn prepare_publication(
        &mut self,
        admitted: super::super::UiAuthorityAdmittedMountedFrame,
    ) -> Result<UiRetentionPreparedMountedFrame, UiMountedFrameRetentionRejection> {
        self.prepare(admitted.into_frame(), false)
    }

    pub(crate) fn prepare_reconciliation(
        &mut self,
        admitted: super::super::UiAuthorityAdmittedMountedFrame,
    ) -> Result<UiRetentionPreparedMountedFrame, UiMountedFrameRetentionRejection> {
        self.prepare(admitted.into_frame(), true)
    }

    pub(crate) fn classify(
        &self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
        mounted_instance: Option<UiMountedInstanceIdentity>,
        node_receipt: Option<UiMountedNodeReceiptIdentity>,
    ) -> Result<UiPresentedFrameBasisRelation, UiPresentedFrameBasisDenial> {
        let authority = self.authority.borrow();
        let (evidence, relation) = match authority.frame(presentation.frame()) {
            UiMountedRetainedFrameLookup::Found {
                evidence, relation, ..
            } => (evidence, relation),
            UiMountedRetainedFrameLookup::Expired { .. } => {
                return Err(UiPresentedFrameBasisDenial::Expired)
            }
            UiMountedRetainedFrameLookup::Unknown { .. } => {
                return Err(UiPresentedFrameBasisDenial::Unknown)
            }
        };
        evidence.classify(presentation, mounted_instance, node_receipt)?;
        Ok(relation)
    }

    pub(crate) fn interaction_hit_test_basis(
        &self,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<UiPresentedHitTestBasis, UiPresentedFrameBasisDenial> {
        let authority = self.authority.borrow();
        let (evidence, relation) = match authority.frame(presentation.frame()) {
            UiMountedRetainedFrameLookup::Found {
                evidence, relation, ..
            } => (evidence, relation),
            UiMountedRetainedFrameLookup::Expired { .. } => {
                return Err(UiPresentedFrameBasisDenial::Expired)
            }
            UiMountedRetainedFrameLookup::Unknown { .. } => {
                return Err(UiPresentedFrameBasisDenial::Unknown)
            }
        };
        evidence.classify(presentation, None, None)?;
        let rows = evidence
            .visual_region_basis(presentation.binding())
            .hit_test();
        Ok(UiPresentedHitTestBasis::new(presentation, relation, rows))
    }

    pub(crate) fn current_projection_input(
        &self,
        slot: worth_ui_query_binding::UiProjectionInputSlot,
    ) -> Option<worth_ui_query_binding::UiProjectionInputFactReference> {
        let authority = self.authority.borrow();
        match authority.current_frame() {
            UiMountedRetainedFrameLookup::Found { evidence, .. } => {
                evidence.projection_input(slot).cloned()
            }
            UiMountedRetainedFrameLookup::Expired { .. }
            | UiMountedRetainedFrameLookup::Unknown { .. } => None,
        }
    }

    pub(crate) fn acquire_observation_basis(
        &self,
        frame: UiMountedFrameIdentity,
    ) -> Result<UiMountedObservationBasisLease, UiMountedObservationBasisRetentionDenial> {
        let structural_bytes = {
            let authority = self.authority.borrow();
            if authority.reservation_active {
                return Err(UiMountedObservationBasisRetentionDenial::FrameTransitionInFlight);
            }
            match authority.frame(frame) {
                UiMountedRetainedFrameLookup::Found { evidence, .. } => evidence.structural_bytes(),
                UiMountedRetainedFrameLookup::Expired { .. } => {
                    return Err(UiMountedObservationBasisRetentionDenial::ExpiredFrame)
                }
                UiMountedRetainedFrameLookup::Unknown { .. } => {
                    return Err(UiMountedObservationBasisRetentionDenial::UnknownFrame)
                }
            }
        };
        self.authority
            .borrow_mut()
            .reserve_pin(
                frame,
                UiMountedRetentionClass::ObservationBasis,
                structural_bytes,
            )
            .map_err(map_observation_pin_denial)?;
        Ok(UiMountedObservationBasisLease::from_reserved(
            &self.authority,
            frame,
            structural_bytes,
        ))
    }

    pub(crate) fn retention_snapshot(&self) -> super::UiMountedFrameRetentionSnapshot {
        self.authority.borrow().snapshot()
    }

    fn prepare(
        &mut self,
        frame: super::super::UiPreparedMountedFrame,
        reconciliation: bool,
    ) -> Result<UiRetentionPreparedMountedFrame, UiMountedFrameRetentionRejection> {
        let admission = {
            let mut authority = self.authority.borrow_mut();
            let admission = admit_successor(&authority, &frame, reconciliation);
            if let Ok(admission) = &admission {
                authority.reservation_active = true;
                authority.in_flight_structural_bytes = admission.structural_bytes();
            }
            admission
        };
        match admission {
            Ok(admission) => {
                let reservation = super::UiMountedRetentionReservation::new(
                    admission,
                    Rc::clone(&self.authority),
                );
                Ok(UiRetentionPreparedMountedFrame::new(frame, reservation))
            }
            Err(denial) => Err(UiMountedFrameRetentionRejection::new(frame, denial)),
        }
    }
}

impl Default for UiMountedFrameRetentionCoordinator {
    fn default() -> Self {
        Self::with_budget(Default::default())
    }
}

fn map_observation_pin_denial(
    denial: UiMountedRetentionPinAdmissionDenial,
) -> UiMountedObservationBasisRetentionDenial {
    match denial {
        UiMountedRetentionPinAdmissionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        } => UiMountedObservationBasisRetentionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        },
        UiMountedRetentionPinAdmissionDenial::AccountingOverflow => {
            UiMountedObservationBasisRetentionDenial::AccountingOverflow
        }
    }
}
