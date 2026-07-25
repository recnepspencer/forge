use std::rc::Rc;

use super::authority::{UiMountedFrameRetentionAuthority, UiMountedRetainedFrameState};
use super::{UiMountedFrameRetentionDenial, UiMountedRetentionClass, UiRetainedPresentedFrame};

pub(super) struct UiMountedSuccessorRetentionAdmission {
    successor: UiMountedRetainedFrameState,
    expected_revision: u64,
    successor_revision: u64,
    structural_bytes: usize,
}

impl UiMountedSuccessorRetentionAdmission {
    pub(super) fn structural_bytes(&self) -> usize {
        self.structural_bytes
    }

    pub(super) fn into_parts(self) -> (UiMountedRetainedFrameState, u64, u64) {
        (
            self.successor,
            self.expected_revision,
            self.successor_revision,
        )
    }
}

pub(super) fn admit_successor(
    authority: &UiMountedFrameRetentionAuthority,
    frame: &super::super::UiPreparedMountedFrame,
    reconciliation: bool,
) -> Result<UiMountedSuccessorRetentionAdmission, UiMountedFrameRetentionDenial> {
    if authority.reservation_active {
        return Err(capacity_denial(
            UiMountedRetentionClass::InFlight,
            2,
            0,
            authority.budget.in_flight(),
        ));
    }
    let candidate = prepare_candidate(frame)?;
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
    let structural_bytes = candidate.structural_bytes();
    let successor_revision = authority.revision.checked_add(1).ok_or(
        UiMountedFrameRetentionDenial::AccountingOverflow {
            class: UiMountedRetentionClass::Current,
        },
    )?;
    let mut successor = authority.frames.clone();
    if reconciliation {
        successor.current = Some(candidate);
    } else {
        retain_current_as_predecessor(&mut successor, candidate)?;
        enforce_predecessor_budget(&mut successor, authority)?;
    }
    Ok(UiMountedSuccessorRetentionAdmission {
        successor,
        expected_revision: authority.revision,
        successor_revision,
        structural_bytes,
    })
}

fn prepare_candidate(
    frame: &super::super::UiPreparedMountedFrame,
) -> Result<Rc<UiRetainedPresentedFrame>, UiMountedFrameRetentionDenial> {
    UiRetainedPresentedFrame::prepare(
        frame.canonical_core().frame(),
        &frame
            .manifest()
            .surfaces()
            .iter()
            .map(|surface| surface.binding())
            .collect::<Vec<_>>(),
        frame.presented_receipt_basis().clone(),
        frame.cost_report(),
    )
    .map(Rc::new)
    .ok_or(UiMountedFrameRetentionDenial::AccountingOverflow {
        class: UiMountedRetentionClass::InFlight,
    })
}

fn retain_current_as_predecessor(
    successor: &mut UiMountedRetainedFrameState,
    candidate: Rc<UiRetainedPresentedFrame>,
) -> Result<(), UiMountedFrameRetentionDenial> {
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
    Ok(())
}

fn enforce_predecessor_budget(
    state: &mut UiMountedRetainedFrameState,
    authority: &UiMountedFrameRetentionAuthority,
) -> Result<(), UiMountedFrameRetentionDenial> {
    let class_budget = authority.budget.predecessor_inspection();
    while !class_budget.admits(state.predecessors.len(), state.predecessor_structural_bytes) {
        let Some(position) = state
            .predecessor_order
            .iter()
            .position(|frame| !authority.frame_is_pinned(*frame))
        else {
            return Err(capacity_denial(
                UiMountedRetentionClass::PredecessorInspection,
                state.predecessors.len(),
                state.predecessor_structural_bytes,
                class_budget,
            ));
        };
        expire_predecessor(state, position);
    }
    enforce_expired_identity_limit(state, authority.budget.expired_identity_limit());
    Ok(())
}

fn expire_predecessor(state: &mut UiMountedRetainedFrameState, position: usize) {
    let expired = state
        .predecessor_order
        .remove(position)
        .expect("selected predecessor position exists");
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

fn enforce_expired_identity_limit(state: &mut UiMountedRetainedFrameState, limit: usize) {
    while state.expiration_order.len() > limit {
        let forgotten = state
            .expiration_order
            .pop_front()
            .expect("an over-budget expiration queue is non-empty");
        state.expired.remove_with_work(&forgotten);
    }
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
