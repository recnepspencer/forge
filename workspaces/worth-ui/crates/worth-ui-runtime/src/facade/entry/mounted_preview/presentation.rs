use worth_ui_host_contract::UiPresentationDeadline;

use super::{
    WorthUiMountedPreviewAdmissionRejection, WorthUiMountedPreviewCompletionRejection,
    WorthUiMountedPreviewDisposition, WorthUiMountedPreviewInFlight, WorthUiMountedPreviewOutcome,
    WorthUiMountedPreviewPorts, WorthUiPreparedMountedPreview, WorthUiResolvedMountedPreview,
};

impl<'session> WorthUiPreparedMountedPreview<'session> {
    pub fn frame(&self) -> &crate::mounting::UiPreparedMountedFrame {
        &self.frame
    }

    pub fn present(
        self,
        deadline: UiPresentationDeadline,
        now: u64,
    ) -> WorthUiMountedPreviewOutcome<'session> {
        let Self {
            frame,
            transition,
            planning_counters,
            ports,
        } = self;
        let capabilities = ports.host_session.capability_report().clone();
        let admission = match ports.presentation.admit_current(
            ports.identity,
            frame,
            &capabilities,
            deadline,
            now,
        ) {
            Ok(admission) => admission,
            Err(rejection) => {
                ports
                    .observations
                    .record_never_presented_frame(rejection.frame().canonical_core().frame());
                return WorthUiMountedPreviewOutcome::AdmissionDenied(
                    WorthUiMountedPreviewAdmissionRejection {
                        denial: rejection.denial(),
                        preview: WorthUiPreparedMountedPreview {
                            frame: rejection.into_frame(),
                            transition,
                            planning_counters,
                            ports,
                        },
                    },
                );
            }
        };
        let reservation = crate::mounting::UiMountedFramePublicationCandidate::reserve(
            &admission,
            ports.identity.view().current_frame(),
        );
        let attempt = admission.attempt();
        assert!(ports.reservations.insert(attempt, reservation).is_none());
        let before = transition.preview().capture_isolation_basis();
        let outcome = ports.presentation.present(
            admission.into_attempt(),
            ports.host_session.effect_port(),
            crate::mounting::UiMountedHostPresentationAuthority::new(
                ports.host_session.identity().as_u64(),
                ports.host_session.protocol(),
                &capabilities,
                ports.host_session.mounted_presentation_lease(),
            ),
            now,
        );
        finish_presentation(outcome, before, transition, planning_counters, ports)
    }

    pub fn supersede(self) -> WorthUiResolvedMountedPreview {
        resolve_transition(
            WorthUiMountedPreviewDisposition::Superseded,
            self.transition,
            self.planning_counters,
        )
    }
}

impl<'session> WorthUiMountedPreviewInFlight<'session> {
    pub fn handle(&self) -> &crate::mounting::UiMountedPresentationInFlight {
        &self.handle
    }

    pub fn complete(self, now: u64) -> WorthUiMountedPreviewOutcome<'session> {
        let Self {
            handle,
            before,
            transition,
            planning_counters,
            ports,
        } = self;
        match ports
            .presentation
            .complete(handle.clone(), ports.host_session.effect_port(), now)
        {
            Ok(outcome) => {
                finish_presentation(outcome, before, transition, planning_counters, ports)
            }
            Err(denial) => WorthUiMountedPreviewOutcome::CompletionDenied(
                WorthUiMountedPreviewCompletionRejection {
                    denial,
                    in_flight: WorthUiMountedPreviewInFlight {
                        handle,
                        before,
                        transition,
                        planning_counters,
                        ports,
                    },
                },
            ),
        }
    }
}

fn finish_presentation<'session>(
    outcome: crate::mounting::UiMountedPresentationOutcome,
    before: crate::runtime::UiAllocationTruthRevision,
    transition: crate::runtime::UiPendingMountedPreviewTransition<'session>,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
    ports: WorthUiMountedPreviewPorts<'session>,
) -> WorthUiMountedPreviewOutcome<'session> {
    match outcome {
        crate::mounting::UiMountedPresentationOutcome::Presented(presented) => {
            let attempt = presented.receipt().attempt();
            let reservation = ports
                .reservations
                .remove(&attempt)
                .expect("preview presentation owns one reservation");
            let receipt = reservation.commit_presented(presented, ports.identity);
            WorthUiMountedPreviewOutcome::Resolved(resolve_transition_from(
                WorthUiMountedPreviewDisposition::Published(receipt),
                transition,
                before,
                planning_counters,
            ))
        }
        crate::mounting::UiMountedPresentationOutcome::RejectedBeforeEffects(rejected) => {
            ports.reservations.remove(&rejected.attempt());
            ports
                .observations
                .record_rejected_frame(rejected.frame().canonical_core().frame());
            WorthUiMountedPreviewOutcome::Resolved(resolve_transition_from(
                WorthUiMountedPreviewDisposition::RejectedBeforeEffects(rejected),
                transition,
                before,
                planning_counters,
            ))
        }
        crate::mounting::UiMountedPresentationOutcome::PresentationIndeterminate(indeterminate) => {
            ports.reservations.remove(&indeterminate.report().attempt());
            ports.observations.record_indeterminate_frame(
                indeterminate.frame().canonical_core().frame(),
                indeterminate.report().affected_bindings(),
            );
            WorthUiMountedPreviewOutcome::Resolved(resolve_transition_from(
                WorthUiMountedPreviewDisposition::PresentationIndeterminate(indeterminate),
                transition,
                before,
                planning_counters,
            ))
        }
        crate::mounting::UiMountedPresentationOutcome::InFlight(handle) => {
            WorthUiMountedPreviewOutcome::InFlight(WorthUiMountedPreviewInFlight {
                handle,
                before,
                transition,
                planning_counters,
                ports,
            })
        }
    }
}

pub(super) fn resolve_transition(
    disposition: WorthUiMountedPreviewDisposition,
    transition: crate::runtime::UiPendingMountedPreviewTransition<'_>,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
) -> WorthUiResolvedMountedPreview {
    let before = transition.preview().capture_isolation_basis();
    resolve_transition_from(disposition, transition, before, planning_counters)
}

fn resolve_transition_from(
    disposition: WorthUiMountedPreviewDisposition,
    transition: crate::runtime::UiPendingMountedPreviewTransition<'_>,
    before: crate::runtime::UiAllocationTruthRevision,
    planning_counters: crate::runtime::UiFrameworkTransitionPlanningCounters,
) -> WorthUiResolvedMountedPreview {
    let resolved = transition.finish(before);
    WorthUiResolvedMountedPreview {
        disposition,
        isolation: resolved.isolation,
        follow_on: resolved.follow_on,
        planning_counters,
    }
}
