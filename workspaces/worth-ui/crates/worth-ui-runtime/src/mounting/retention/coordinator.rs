use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedNodeReceiptIdentity,
    UiSurfaceBindingGeneration,
};

use super::{
    UiMountedFrameRetentionBudget, UiMountedFrameRetentionDenial, UiMountedFrameRetentionRejection,
    UiMountedRetentionClass, UiPresentedFrameBasisDenial, UiPresentedFrameBasisRelation,
    UiRetainedPresentedFrame,
};

#[derive(Clone, Default)]
pub(super) struct UiMountedRetainedFrameState {
    pub(super) current: Option<Rc<UiRetainedPresentedFrame>>,
    pub(super) predecessors: crate::runtime::persistent_index::UiPersistentOrdMap<
        UiMountedFrameIdentity,
        Rc<UiRetainedPresentedFrame>,
    >,
    pub(super) predecessor_order: VecDeque<UiMountedFrameIdentity>,
    pub(super) predecessor_structural_bytes: usize,
    pub(super) expired:
        crate::runtime::persistent_index::UiPersistentOrdSet<UiMountedFrameIdentity>,
    expiration_order: VecDeque<UiMountedFrameIdentity>,
}

pub(crate) struct UiMountedFrameRetentionCoordinator {
    pub(super) authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
}

pub(super) struct UiMountedFrameRetentionAuthority {
    pub(super) budget: UiMountedFrameRetentionBudget,
    pub(super) frames: UiMountedRetainedFrameState,
    pub(super) revision: u64,
    pub(super) reservation_active: bool,
}

pub(crate) struct UiRetentionPreparedMountedFrame {
    frame: super::super::UiPreparedMountedFrame,
    reservation: UiMountedRetentionReservation,
}

pub(crate) struct UiMountedRetentionReservation {
    successor: UiMountedRetainedFrameState,
    expected_revision: u64,
    successor_revision: u64,
    authority: Rc<RefCell<UiMountedFrameRetentionAuthority>>,
    release_on_drop: bool,
}

impl UiMountedFrameRetentionCoordinator {
    pub(crate) fn with_budget(budget: UiMountedFrameRetentionBudget) -> Self {
        Self {
            authority: Rc::new(RefCell::new(UiMountedFrameRetentionAuthority {
                budget,
                frames: Default::default(),
                revision: 0,
                reservation_active: false,
            })),
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
        frame: UiMountedFrameIdentity,
        binding: UiSurfaceBindingGeneration,
        mounted_instance: Option<UiMountedInstanceIdentity>,
        node_receipt: Option<UiMountedNodeReceiptIdentity>,
    ) -> Result<UiPresentedFrameBasisRelation, UiPresentedFrameBasisDenial> {
        let authority = self.authority.borrow();
        let (basis, relation) = if authority
            .frames
            .current
            .as_ref()
            .is_some_and(|basis| basis.frame() == frame)
        {
            (
                authority
                    .frames
                    .current
                    .as_deref()
                    .expect("current frame matched"),
                UiPresentedFrameBasisRelation::Current,
            )
        } else if let Some(basis) = authority.frames.predecessors.get(&frame) {
            (basis.as_ref(), UiPresentedFrameBasisRelation::Retained)
        } else if authority.frames.expired.contains_with_probes(&frame).0 {
            return Err(UiPresentedFrameBasisDenial::Expired);
        } else {
            return Err(UiPresentedFrameBasisDenial::Unknown);
        };
        basis.classify(binding, mounted_instance, node_receipt)?;
        Ok(relation)
    }

    fn prepare(
        &mut self,
        frame: super::super::UiPreparedMountedFrame,
        reconciliation: bool,
    ) -> Result<UiRetentionPreparedMountedFrame, UiMountedFrameRetentionRejection> {
        let prepared = {
            let mut authority = self.authority.borrow_mut();
            let prepared = prepare_successor(&authority, &frame, reconciliation);
            if prepared.is_ok() {
                authority.reservation_active = true;
            }
            prepared
        };
        match prepared {
            Ok((successor, expected_revision, successor_revision)) => {
                Ok(UiRetentionPreparedMountedFrame {
                    frame,
                    reservation: UiMountedRetentionReservation {
                        successor,
                        expected_revision,
                        successor_revision,
                        authority: Rc::clone(&self.authority),
                        release_on_drop: true,
                    },
                })
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

impl UiRetentionPreparedMountedFrame {
    pub(crate) fn frame(&self) -> &super::super::UiPreparedMountedFrame {
        &self.frame
    }

    pub(crate) fn into_parts(
        self,
    ) -> (
        super::super::UiPreparedMountedFrame,
        UiMountedRetentionReservation,
    ) {
        (self.frame, self.reservation)
    }
}

impl UiMountedRetentionReservation {
    pub(crate) fn commit(mut self) {
        let mut authority = self.authority.borrow_mut();
        debug_assert_eq!(
            authority.revision, self.expected_revision,
            "retention authority cannot change while its presentation is in flight"
        );
        authority.frames = std::mem::take(&mut self.successor);
        authority.revision = self.successor_revision;
        authority.reservation_active = false;
        self.release_on_drop = false;
    }
}

impl Drop for UiMountedRetentionReservation {
    fn drop(&mut self) {
        if self.release_on_drop {
            let mut authority = self.authority.borrow_mut();
            authority.reservation_active = false;
        }
    }
}

fn prepare_successor(
    authority: &UiMountedFrameRetentionAuthority,
    frame: &super::super::UiPreparedMountedFrame,
    reconciliation: bool,
) -> Result<(UiMountedRetainedFrameState, u64, u64), UiMountedFrameRetentionDenial> {
    if authority.reservation_active {
        return Err(capacity_denial(
            UiMountedRetentionClass::InFlight,
            2,
            0,
            authority.budget.in_flight(),
        ));
    }
    let candidate = Rc::new(
        UiRetainedPresentedFrame::prepare(
            frame.canonical_core().frame(),
            &frame
                .manifest()
                .surfaces()
                .iter()
                .map(|surface| surface.binding())
                .collect::<Vec<_>>(),
            frame.presented_receipt_basis().clone(),
        )
        .ok_or(UiMountedFrameRetentionDenial::AccountingOverflow {
            class: UiMountedRetentionClass::InFlight,
        })?,
    );
    require_capacity(
        UiMountedRetentionClass::Current,
        1,
        candidate.structural_bytes(),
        authority.budget.current(),
    )?;
    require_capacity(
        UiMountedRetentionClass::InFlight,
        1,
        candidate.structural_bytes(),
        authority.budget.in_flight(),
    )?;
    let successor_revision = authority.revision.checked_add(1).ok_or(
        UiMountedFrameRetentionDenial::AccountingOverflow {
            class: UiMountedRetentionClass::Current,
        },
    )?;
    let mut successor = authority.frames.clone();
    if reconciliation {
        successor.current = Some(candidate);
    } else {
        if let Some(predecessor) = successor.current.replace(candidate) {
            successor.predecessor_structural_bytes = successor
                .predecessor_structural_bytes
                .checked_add(predecessor.structural_bytes())
                .ok_or(UiMountedFrameRetentionDenial::AccountingOverflow {
                    class: UiMountedRetentionClass::PredecessorInspection,
                })?;
            let frame = predecessor.frame();
            successor.predecessors.insert(frame, predecessor);
            successor.predecessor_order.push_back(frame);
        }
        enforce_predecessor_budget(&mut successor, authority.budget)?;
    }
    Ok((successor, authority.revision, successor_revision))
}

fn enforce_predecessor_budget(
    state: &mut UiMountedRetainedFrameState,
    budget: UiMountedFrameRetentionBudget,
) -> Result<(), UiMountedFrameRetentionDenial> {
    let class_budget = budget.predecessor_inspection();
    loop {
        if class_budget.admits(state.predecessors.len(), state.predecessor_structural_bytes) {
            break;
        }
        let expired = state
            .predecessor_order
            .pop_front()
            .expect("an over-budget predecessor queue is non-empty");
        let removed = state
            .predecessors
            .get(&expired)
            .expect("predecessor order references indexed evidence")
            .structural_bytes();
        state.predecessors.remove(&expired);
        state.predecessor_structural_bytes = state
            .predecessor_structural_bytes
            .checked_sub(removed)
            .expect("retained predecessor bytes include removed evidence");
        if state.expired.insert(expired) {
            state.expiration_order.push_back(expired);
        }
    }
    while state.expiration_order.len() > budget.expired_identity_limit() {
        let forgotten = state
            .expiration_order
            .pop_front()
            .expect("an over-budget expiration queue is non-empty");
        state.expired.remove_with_work(&forgotten);
    }
    Ok(())
}

fn require_capacity(
    class: UiMountedRetentionClass,
    required_frames: usize,
    required_structural_bytes: usize,
    budget: super::UiMountedRetentionClassBudget,
) -> Result<(), UiMountedFrameRetentionDenial> {
    budget
        .admits(required_frames, required_structural_bytes)
        .then_some(())
        .ok_or_else(|| capacity_denial(class, required_frames, required_structural_bytes, budget))
}

fn capacity_denial(
    class: UiMountedRetentionClass,
    required_frames: usize,
    required_structural_bytes: usize,
    budget: super::UiMountedRetentionClassBudget,
) -> UiMountedFrameRetentionDenial {
    UiMountedFrameRetentionDenial::CapacityExceeded {
        class,
        required_frames,
        required_structural_bytes,
        budget,
    }
}
