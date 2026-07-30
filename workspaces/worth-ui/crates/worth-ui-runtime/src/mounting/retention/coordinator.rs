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
    UiMountedDiagnosticInspectionBasis, UiMountedDiagnosticInspectionDenial,
    UiMountedDiagnosticRetentionLease, UiMountedFrameInspectionBasis,
    UiMountedFrameInspectionDenial, UiMountedFrameInspectionSelection,
    UiMountedFrameInspectionTarget, UiMountedFrameRetentionBudget,
    UiMountedFrameRetentionRejection, UiMountedObservationBasisLease,
    UiMountedObservationBasisRetentionDenial, UiMountedRetentionClass, UiMountedRetentionLease,
    UiPresentedFrameBasisDenial, UiPresentedFrameBasisRelation, UiRetainedMountedDiagnostics,
    UiRetentionPreparedMountedFrame,
};

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

    pub(crate) fn inspect(
        &self,
        selection: UiMountedFrameInspectionSelection,
    ) -> Result<UiMountedFrameInspectionBasis, UiMountedFrameInspectionDenial> {
        let selected = self.select_inspection_basis(selection)?;
        self.reserve_inspection_lease(selected)
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

    fn select_inspection_basis(
        &self,
        selection: UiMountedFrameInspectionSelection,
    ) -> Result<UiSelectedMountedFrameInspection, UiMountedFrameInspectionDenial> {
        let authority = self.authority.borrow();
        if authority.reservation_active {
            return Err(UiMountedFrameInspectionDenial::FrameTransitionInFlight);
        }
        let lookup = match selection.target {
            UiMountedFrameInspectionTarget::Current => authority.current_frame(),
            UiMountedFrameInspectionTarget::Frame(frame) => authority.frame(frame),
        };
        let (evidence, relation, frame_index_probes) = inspection_lookup(selection.target, lookup)?;
        let (selected_node_receipt, instance_index_probes) = match selection.instance {
            Some(instance) => {
                let (receipt, probes) = evidence.receipt_for_with_probes(instance);
                let receipt =
                    receipt.ok_or(UiMountedFrameInspectionDenial::InstanceNotPresented {
                        frame_index_probes,
                        instance_index_probes: probes,
                    })?;
                (Some(receipt), probes)
            }
            None => (None, 0),
        };
        Ok(UiSelectedMountedFrameInspection {
            frame: evidence.frame(),
            relation,
            presentation: evidence
                .presentation_receipt()
                .expect("published retained frames carry completed presentation truth")
                .clone(),
            presented_binding_count: evidence.presented_binding_count(),
            mounted_instance_count: evidence.mounted_instance_count(),
            selected_node_receipt,
            mount_cost: evidence.mount_cost(),
            retained_structural_bytes: evidence.structural_bytes(),
            frame_index_probes,
            instance_index_probes,
            diagnostics_requested: selection.diagnostics,
            diagnostics: authority.diagnostics(evidence.frame()),
        })
    }

    fn reserve_inspection_lease(
        &self,
        selected: UiSelectedMountedFrameInspection,
    ) -> Result<UiMountedFrameInspectionBasis, UiMountedFrameInspectionDenial> {
        self.authority
            .borrow_mut()
            .reserve_pin(
                selected.frame,
                UiMountedRetentionClass::PredecessorInspection,
                selected.retained_structural_bytes,
            )
            .map_err(map_inspection_pin_denial)?;
        let diagnostics = self.reserve_diagnostic_lease(&selected);
        Ok(UiMountedFrameInspectionBasis {
            frame: selected.frame,
            relation: selected.relation,
            presentation: selected.presentation,
            presented_binding_count: selected.presented_binding_count,
            mounted_instance_count: selected.mounted_instance_count,
            selected_node_receipt: selected.selected_node_receipt,
            mount_cost: selected.mount_cost,
            retained_structural_bytes: selected.retained_structural_bytes,
            frame_index_probes: selected.frame_index_probes,
            instance_index_probes: selected.instance_index_probes,
            diagnostics,
            lease: UiMountedRetentionLease::from_reserved(
                &self.authority,
                selected.frame,
                selected.retained_structural_bytes,
            ),
        })
    }

    fn reserve_diagnostic_lease(
        &self,
        selected: &UiSelectedMountedFrameInspection,
    ) -> UiMountedDiagnosticInspectionBasis {
        if !selected.diagnostics_requested {
            return UiMountedDiagnosticInspectionBasis::NotRequested;
        }
        let Some(evidence) = selected.diagnostics.clone() else {
            return UiMountedDiagnosticInspectionBasis::Omitted(
                UiMountedDiagnosticInspectionDenial::NotRetained,
            );
        };
        let structural_bytes = evidence.structural_bytes();
        match self.authority.borrow_mut().reserve_pin(
            selected.frame,
            UiMountedRetentionClass::Diagnostic,
            structural_bytes,
        ) {
            Ok(()) => UiMountedDiagnosticInspectionBasis::Available {
                evidence,
                lease: UiMountedDiagnosticRetentionLease::from_reserved(
                    &self.authority,
                    selected.frame,
                    structural_bytes,
                ),
            },
            Err(denial) => {
                UiMountedDiagnosticInspectionBasis::Omitted(map_diagnostic_pin_denial(denial))
            }
        }
    }
}

impl Default for UiMountedFrameRetentionCoordinator {
    fn default() -> Self {
        Self::with_budget(Default::default())
    }
}

struct UiSelectedMountedFrameInspection {
    frame: UiMountedFrameIdentity,
    relation: UiPresentedFrameBasisRelation,
    presentation: super::super::UiMountedPresentationReceipt,
    presented_binding_count: usize,
    mounted_instance_count: usize,
    selected_node_receipt: Option<UiMountedNodeReceiptIdentity>,
    mount_cost: super::super::UiMountCostReport,
    retained_structural_bytes: usize,
    frame_index_probes: usize,
    instance_index_probes: usize,
    diagnostics_requested: bool,
    diagnostics: Option<Rc<UiRetainedMountedDiagnostics>>,
}

fn map_diagnostic_pin_denial(
    denial: UiMountedRetentionPinAdmissionDenial,
) -> UiMountedDiagnosticInspectionDenial {
    match denial {
        UiMountedRetentionPinAdmissionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        } => UiMountedDiagnosticInspectionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        },
        UiMountedRetentionPinAdmissionDenial::AccountingOverflow => {
            UiMountedDiagnosticInspectionDenial::AccountingOverflow
        }
    }
}

fn inspection_lookup<'a>(
    target: UiMountedFrameInspectionTarget,
    lookup: UiMountedRetainedFrameLookup<'a>,
) -> Result<
    (
        &'a super::UiRetainedPresentedFrame,
        UiPresentedFrameBasisRelation,
        usize,
    ),
    UiMountedFrameInspectionDenial,
> {
    match lookup {
        UiMountedRetainedFrameLookup::Found {
            evidence,
            relation,
            frame_index_probes,
        } => Ok((evidence, relation, frame_index_probes)),
        UiMountedRetainedFrameLookup::Expired { frame_index_probes } => {
            Err(UiMountedFrameInspectionDenial::ExpiredFrame { frame_index_probes })
        }
        UiMountedRetainedFrameLookup::Unknown { frame_index_probes } => match target {
            UiMountedFrameInspectionTarget::Current => {
                Err(UiMountedFrameInspectionDenial::NoCurrentFrame)
            }
            UiMountedFrameInspectionTarget::Frame(_) => {
                Err(UiMountedFrameInspectionDenial::UnknownFrame { frame_index_probes })
            }
        },
    }
}

fn map_inspection_pin_denial(
    denial: UiMountedRetentionPinAdmissionDenial,
) -> UiMountedFrameInspectionDenial {
    match denial {
        UiMountedRetentionPinAdmissionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        } => UiMountedFrameInspectionDenial::CapacityExceeded {
            required_leases,
            required_structural_bytes,
            budget,
        },
        UiMountedRetentionPinAdmissionDenial::AccountingOverflow => {
            UiMountedFrameInspectionDenial::AccountingOverflow
        }
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
