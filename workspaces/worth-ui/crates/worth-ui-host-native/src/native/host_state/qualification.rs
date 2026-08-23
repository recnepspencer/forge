use std::collections::BTreeSet;

#[derive(Clone, Copy)]
pub(crate) struct UiNativePresentationExternalQualification {
    effects_indeterminate: bool,
    duplicate_completed: bool,
}

impl UiNativePresentationExternalQualification {
    pub(crate) const fn effects_indeterminate(self) -> bool {
        self.effects_indeterminate
    }

    pub(crate) const fn duplicate_completed(self) -> bool {
        self.duplicate_completed
    }
}

pub(crate) struct UiNativeQualificationState {
    deferred_presentations: [Option<u64>; 3],
    duplicate_completion_presentation: Option<u64>,
    duplicate_completion_observed: bool,
    effects_indeterminate_presentation: Option<u64>,
    presentation_submission_count: u64,
    effects_indeterminate_observed: bool,
    derived_state_loss: Option<crate::UiNativeDerivedStateLossClass>,
    derived_state_loss_after_completed_presentation: Option<u64>,
    completed_derived_state_loss_applied: bool,
    derived_state_loss_pending: Option<crate::UiNativeDerivedStateLossClass>,
    derived_state_reconstruction_pending: Option<crate::UiNativeDerivedStateLossClass>,
    derived_state_reconstruction_predecessors: BTreeSet<u64>,
    derived_state_reconstruction_successors: BTreeSet<u64>,
    derived_state_loss_count: u64,
    derived_state_reconstruction_count: u64,
    surface_basis_successor:
        Option<crate::qualification::UiNativeQualificationSurfaceBasisSuccessor>,
}

impl UiNativeQualificationState {
    pub(super) const fn ordinary() -> Self {
        Self {
            deferred_presentations: [None, None, None],
            duplicate_completion_presentation: None,
            duplicate_completion_observed: false,
            effects_indeterminate_presentation: None,
            presentation_submission_count: 0,
            effects_indeterminate_observed: false,
            derived_state_loss: None,
            derived_state_loss_after_completed_presentation: None,
            completed_derived_state_loss_applied: false,
            derived_state_loss_pending: None,
            derived_state_reconstruction_pending: None,
            derived_state_reconstruction_predecessors: BTreeSet::new(),
            derived_state_reconstruction_successors: BTreeSet::new(),
            derived_state_loss_count: 0,
            derived_state_reconstruction_count: 0,
            surface_basis_successor: None,
        }
    }

    pub(super) fn from_plan(plan: crate::UiNativeQualificationPlan) -> Self {
        Self {
            deferred_presentations: plan.deferred_presentations(),
            duplicate_completion_presentation: plan.duplicate_completion_presentation(),
            duplicate_completion_observed: false,
            effects_indeterminate_presentation: plan.effects_indeterminate_presentation(),
            presentation_submission_count: 0,
            effects_indeterminate_observed: false,
            derived_state_loss: plan.derived_state_loss(),
            derived_state_loss_after_completed_presentation: plan
                .completed_derived_state_loss_ordinal(),
            completed_derived_state_loss_applied: false,
            derived_state_loss_pending: None,
            derived_state_reconstruction_pending: None,
            derived_state_reconstruction_predecessors: BTreeSet::new(),
            derived_state_reconstruction_successors: BTreeSet::new(),
            derived_state_loss_count: 0,
            derived_state_reconstruction_count: 0,
            surface_basis_successor: plan.surface_basis_successor(),
        }
    }

    pub(crate) fn defer_next_presentation_initial_observation(&mut self) -> bool {
        self.presentation_submission_count = self.presentation_submission_count.saturating_add(1);
        self.deferred_presentations
            .contains(&Some(self.presentation_submission_count))
    }

    pub(crate) fn presentation_external_qualification(
        &self,
        identity: super::super::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) -> UiNativePresentationExternalQualification {
        UiNativePresentationExternalQualification {
            effects_indeterminate: !self.effects_indeterminate_observed
                && self.effects_indeterminate_presentation == Some(identity.sequence()),
            duplicate_completed: !self.duplicate_completion_observed
                && self.duplicate_completion_presentation == Some(identity.sequence()),
        }
    }

    pub(crate) fn presentation_poll_override(
        &self,
        identity: super::super::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) -> Option<(
        super::super::physical_work_signal::UiNativePhysicalSignalStatus,
        Option<crate::UiNativeDerivedStateLossClass>,
    )> {
        (!self.effects_indeterminate_observed
            && self.effects_indeterminate_presentation == Some(identity.sequence()))
        .then_some((
            super::super::physical_work_signal::UiNativePhysicalSignalStatus::EffectsIndeterminate,
            self.derived_state_loss,
        ))
    }

    pub(crate) fn should_duplicate_completed_observation(
        &self,
        identity: super::super::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) -> bool {
        !self.duplicate_completion_observed
            && self.duplicate_completion_presentation == Some(identity.sequence())
    }

    pub(crate) fn commit_duplicate_completed_observation(
        &mut self,
        identity: super::super::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) {
        assert!(self.should_duplicate_completed_observation(identity));
        self.duplicate_completion_observed = true;
    }

    pub(crate) fn commit_presentation_poll_override(
        &mut self,
        identity: super::super::physical_work_signal::UiNativePhysicalPresentationIdentity,
    ) {
        assert!(
            self.presentation_poll_override(identity).is_some(),
            "only the selected owner observation may commit the qualification override"
        );
        self.effects_indeterminate_observed = true;
        self.derived_state_loss_pending = self.derived_state_loss;
    }

    pub(crate) fn take_derived_state_loss(
        &mut self,
    ) -> Option<crate::UiNativeDerivedStateLossClass> {
        self.derived_state_loss_pending.take()
    }

    pub(crate) fn completed_derived_state_loss(
        &self,
        completed_presentations: u64,
    ) -> Option<crate::UiNativeDerivedStateLossClass> {
        (!self.completed_derived_state_loss_applied
            && self.derived_state_loss_after_completed_presentation
                == Some(completed_presentations))
        .then_some(self.derived_state_loss)
        .flatten()
    }

    pub(crate) fn commit_completed_derived_state_loss(&mut self) {
        assert!(!self.completed_derived_state_loss_applied);
        self.completed_derived_state_loss_applied = true;
    }

    pub(crate) fn record_derived_state_loss(
        &mut self,
        class: crate::UiNativeDerivedStateLossClass,
        bindings: BTreeSet<u64>,
    ) {
        assert_eq!(self.derived_state_loss, Some(class));
        assert!(!bindings.is_empty());
        self.derived_state_reconstruction_pending = Some(class);
        self.derived_state_reconstruction_predecessors = bindings;
        self.derived_state_reconstruction_successors.clear();
        self.derived_state_loss_count = self.derived_state_loss_count.saturating_add(1);
    }

    pub(crate) const fn pending_reconstruction(
        &self,
    ) -> Option<crate::UiNativeDerivedStateLossClass> {
        self.derived_state_reconstruction_pending
    }

    pub(crate) fn record_derived_state_reconstruction(&mut self, binding: u64, restored: bool) {
        if !restored || !self.derived_state_reconstruction_successors.insert(binding) {
            return;
        }
        let _ = self.derived_state_reconstruction_predecessors.pop_first();
        if self.derived_state_reconstruction_predecessors.is_empty()
            && self.derived_state_reconstruction_pending.take().is_some()
        {
            self.derived_state_reconstruction_count =
                self.derived_state_reconstruction_count.saturating_add(1);
        }
    }

    pub(crate) const fn derived_state_reconstruction_observation(
        &self,
    ) -> Option<crate::UiNativeDerivedStateReconstructionObservation> {
        let Some(class) = self.derived_state_loss else {
            return None;
        };
        Some(
            crate::UiNativeDerivedStateReconstructionObservation::observed(
                class,
                self.derived_state_loss_count,
                self.derived_state_reconstruction_count,
            ),
        )
    }

    pub(crate) fn take_surface_basis_successor(
        &mut self,
        completed_presentations: u64,
    ) -> Option<crate::qualification::UiNativeQualificationSurfaceBasisSuccessor> {
        let successor = self.surface_basis_successor?;
        if completed_presentations < successor.after_completed_presentation() {
            return None;
        }
        self.surface_basis_successor.take()
    }
}
