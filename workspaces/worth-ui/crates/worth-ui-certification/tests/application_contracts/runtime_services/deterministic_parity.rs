use crate::intent::{
    run_native_runtime_service_scenario,
    runtime_services_kit::{run_headless_runtime_service_scenario, RuntimeServiceSemanticOutcome},
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum SemanticEvent {
    PortalPublished,
    FocusPlaced,
    HotRebind,
    DismissalAccepted,
    DuplicateDismissal,
    PhysicalEffectIndeterminate,
    ReconciledFromHostTruth,
    ProposalSettled,
    FocusRestored,
    ShutdownBegun,
    PortalOwnerRetired,
    FocusOwnerRetired,
    HostSessionReleased,
}

const HEADLESS_SCHEDULE: &[SemanticEvent] = &[
    SemanticEvent::PortalPublished,
    SemanticEvent::FocusPlaced,
    SemanticEvent::HotRebind,
    SemanticEvent::DismissalAccepted,
    SemanticEvent::ProposalSettled,
    SemanticEvent::FocusRestored,
    SemanticEvent::DuplicateDismissal,
    SemanticEvent::ShutdownBegun,
    SemanticEvent::PortalOwnerRetired,
    SemanticEvent::FocusOwnerRetired,
    SemanticEvent::HostSessionReleased,
];

const NATIVE_SCHEDULE: &[SemanticEvent] = &[
    SemanticEvent::PortalPublished,
    SemanticEvent::FocusPlaced,
    SemanticEvent::DismissalAccepted,
    SemanticEvent::DuplicateDismissal,
    SemanticEvent::PhysicalEffectIndeterminate,
    SemanticEvent::ReconciledFromHostTruth,
    SemanticEvent::ProposalSettled,
    SemanticEvent::FocusRestored,
    SemanticEvent::ShutdownBegun,
    SemanticEvent::PortalOwnerRetired,
    SemanticEvent::FocusOwnerRetired,
    SemanticEvent::HostSessionReleased,
];

struct IndependentSemanticModel {
    portal_was_visible: bool,
    focus_was_placed: bool,
    dismissal_closed_only_top: bool,
    focus_restored_to_previous: bool,
    duplicate_was_idempotent: bool,
    physical_effect_indeterminate: bool,
    live_proposals: usize,
    live_service_owners: usize,
    host_session_live: bool,
    shutdown_observed: bool,
}

impl IndependentSemanticModel {
    fn new() -> Self {
        Self {
            portal_was_visible: false,
            focus_was_placed: false,
            dismissal_closed_only_top: false,
            focus_restored_to_previous: false,
            duplicate_was_idempotent: false,
            physical_effect_indeterminate: false,
            live_proposals: 0,
            live_service_owners: 0,
            host_session_live: true,
            shutdown_observed: false,
        }
    }

    fn apply(&mut self, event: SemanticEvent) {
        match event {
            SemanticEvent::PortalPublished => {
                self.portal_was_visible = true;
                self.live_service_owners += 1;
            }
            SemanticEvent::FocusPlaced => {
                assert!(self.portal_was_visible);
                self.focus_was_placed = true;
                self.live_service_owners += 1;
            }
            SemanticEvent::HotRebind => {
                assert!(self.portal_was_visible && self.focus_was_placed)
            }
            SemanticEvent::DismissalAccepted => {
                assert!(self.portal_was_visible);
                self.dismissal_closed_only_top = true;
                self.live_proposals += 1;
            }
            SemanticEvent::DuplicateDismissal => {
                assert!(self.dismissal_closed_only_top);
                self.duplicate_was_idempotent = true;
            }
            SemanticEvent::PhysicalEffectIndeterminate => {
                assert!(self.dismissal_closed_only_top);
                self.physical_effect_indeterminate = true;
            }
            SemanticEvent::ReconciledFromHostTruth => {
                assert!(self.physical_effect_indeterminate);
                self.physical_effect_indeterminate = false;
            }
            SemanticEvent::ProposalSettled => {
                self.live_proposals = self
                    .live_proposals
                    .checked_sub(1)
                    .expect("proposal settlement requires one live proposal");
            }
            SemanticEvent::FocusRestored => {
                assert!(self.dismissal_closed_only_top);
                self.focus_restored_to_previous = true;
            }
            SemanticEvent::ShutdownBegun => {
                assert!(!self.physical_effect_indeterminate);
                assert_eq!(self.live_proposals, 0);
                self.shutdown_observed = true;
            }
            SemanticEvent::PortalOwnerRetired | SemanticEvent::FocusOwnerRetired => {
                assert!(self.shutdown_observed);
                self.live_service_owners = self
                    .live_service_owners
                    .checked_sub(1)
                    .expect("shutdown cannot retire an absent service owner");
            }
            SemanticEvent::HostSessionReleased => {
                assert!(self.shutdown_observed && self.host_session_live);
                self.host_session_live = false;
            }
        }
    }

    fn terminal(self) -> RuntimeServiceSemanticOutcome {
        assert!(self.shutdown_observed);
        RuntimeServiceSemanticOutcome {
            portal_was_visible: self.portal_was_visible,
            focus_was_placed: self.focus_was_placed,
            dismissal_closed_only_top: self.dismissal_closed_only_top,
            focus_restored_to_previous: self.focus_restored_to_previous,
            duplicate_was_idempotent: self.duplicate_was_idempotent,
            proposals_are_zero: self.live_proposals == 0,
            terminal_resources_are_zero: self.live_service_owners == 0 && !self.host_session_live,
        }
    }
}

fn independent_semantic_oracle(schedule: &[SemanticEvent]) -> RuntimeServiceSemanticOutcome {
    let mut model = IndependentSemanticModel::new();
    for event in schedule.iter().copied() {
        model.apply(event);
    }
    model.terminal()
}

#[test]
fn equivalent_external_schedules_converge_across_headless_and_native_paths() {
    let expected_headless = independent_semantic_oracle(HEADLESS_SCHEDULE);
    let expected_native = independent_semantic_oracle(NATIVE_SCHEDULE);
    assert_eq!(expected_headless, expected_native);

    let headless = run_headless_runtime_service_scenario();
    let headless_repeat = run_headless_runtime_service_scenario();
    let native = run_native_runtime_service_scenario();
    let native_repeat = run_native_runtime_service_scenario();

    assert_eq!(
        headless, headless_repeat,
        "headless evidence must be deterministic"
    );
    assert_eq!(
        native, native_repeat,
        "native evidence must be deterministic"
    );
    assert_eq!(headless.semantic, expected_headless);
    assert_eq!(native.semantic, expected_native);
    assert_eq!(headless.semantic, native.semantic);
    assert!(headless.hot_rebind_preserved_portal);
    assert!(headless.focus_retargeted_to_successor);
    assert!(headless.inspection_was_bounded);
    assert!(native.indeterminate_effect_retained);
    assert!(native.reconciled_from_exact_host_truth);
    assert!(native.predecessor_was_reconstructed);
}
