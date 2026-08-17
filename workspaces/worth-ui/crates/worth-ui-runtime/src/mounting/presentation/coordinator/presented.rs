use super::{UiMountedPresentationCoordinator, UiMountedPresentationSettlement};
use crate::mounting::presentation::outcome::{
    UiMountedPresentationOutcome, UiMountedPresentationReceipt, UiMountedPresentationWitness,
    UiMountedPresentedFrame, UiMountedSurfacePresentationReceipt,
};
use crate::mounting::presentation::terminal::UiIndeterminatePresentationEvidence;

impl UiMountedPresentationCoordinator {
    pub(super) fn finish_presented(
        &mut self,
        mut settlement: UiMountedPresentationSettlement<'_>,
    ) -> UiMountedPresentationOutcome {
        self.active.borrow_mut().remove(&settlement.attempt);
        let attempt = settlement.attempt;
        let frame_identity = settlement.frame.canonical_core().frame();
        let frame_cost = settlement.frame.cost_report();
        let mut completed = std::mem::take(&mut settlement.completed);
        completed.sort_by_key(UiMountedSurfacePresentationReceipt::binding);
        let cost = match UiMountedPresentationReceipt::compose_cost(frame_cost, &completed) {
            Ok(cost) => cost,
            Err(_) => {
                let affected = settlement
                    .frame
                    .surfaces()
                    .iter()
                    .map(|surface| surface.requirement().binding())
                    .collect();
                return self.indeterminate(
                    settlement.frame,
                    settlement.retention,
                    attempt,
                    UiIndeterminatePresentationEvidence::new(affected, completed),
                );
            }
        };
        let receipt = UiMountedPresentationReceipt::new(attempt, frame_identity, cost, completed);
        self.presentation_states = std::mem::take(&mut settlement.candidates);
        UiMountedPresentationOutcome::Presented(UiMountedPresentedFrame::new(
            settlement.frame,
            settlement.retention,
            receipt,
            UiMountedPresentationWitness::new(attempt),
        ))
    }
}
