use super::{
    presentation_deadline, stop_admitted, UiIntentConsequenceAdmitted, UiIntentConsequenceInFlight,
    UiIntentConsequencePublicationCompletion, UiIntentConsequencePublicationOutcome,
};
use crate::facade::entry::WorthUiActiveApplicationSession;

pub(in crate::facade::entry) struct DetachedUiIntentConsequenceInFlight {
    session_identity: crate::facade::WorthUiActiveApplicationSessionIdentity,
    plan: crate::runtime::rebind::UiRebindPlan,
    reservation: crate::runtime::rebind::UiRebindReservation,
    transfer:
        crate::facade::entry::intent_consequence_rebind::WorthUiIntentConsequenceRebindTransfer,
    query: Option<worth_ui_query_binding::WorthUiAdmittedCollectionChangePublication>,
    mounted: crate::mounting::UiMountedPresentationInFlight,
}

impl UiIntentConsequencePublicationCompletion<'_> {
    pub(in crate::facade::entry) fn detach_for_native(
        mut self,
    ) -> DetachedUiIntentConsequenceInFlight {
        let state = self.take_state();
        let UiIntentConsequenceInFlight { admitted, mounted } = *state;
        let UiIntentConsequenceAdmitted {
            session,
            plan,
            reservation,
            transfer,
            query,
        } = admitted;
        DetachedUiIntentConsequenceInFlight {
            session_identity: session.session_identity(),
            plan,
            reservation,
            transfer,
            query,
            mounted,
        }
    }
}

impl DetachedUiIntentConsequenceInFlight {
    pub(in crate::facade::entry) const fn session_identity(
        &self,
    ) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.session_identity
    }

    pub(in crate::facade::entry) fn matches_native_progress(
        &self,
        progress: &crate::native_platform::UiNativeApplicationPhysicalProgress,
    ) -> bool {
        let class = match progress.class() {
            worth_ui_host_native::UiNativePhysicalProgressClass::Presentation => {
                worth_ui_host_contract::UiHostPresentationProgressClass::PhysicalSurface
            }
            worth_ui_host_native::UiNativePhysicalProgressClass::TextAtlas => {
                worth_ui_host_contract::UiHostPresentationProgressClass::TextAtlas
            }
            worth_ui_host_native::UiNativePhysicalProgressClass::PresentationRecovery => {
                return false;
            }
        };
        self.mounted.awaits_progress_class(class)
            && progress.presentation().map_or(true, |presentation| {
                presentation.attempt() == self.mounted.attempt()
                    && self
                        .mounted
                        .pending_bindings()
                        .any(|binding| binding == presentation.binding())
            })
    }

    pub(in crate::facade::entry) fn complete<'session>(
        self,
        session: &'session mut WorthUiActiveApplicationSession,
        now_tick: u64,
    ) -> UiIntentConsequencePublicationOutcome<'session> {
        let (admitted, mounted) = self.into_admitted(session);
        let outcome = admitted
            .session
            .complete_mounted_presentation(mounted, now_tick);
        finish_progressed(admitted, outcome, now_tick)
    }

    pub(in crate::facade::entry) fn cancel<'session>(
        self,
        session: &'session mut WorthUiActiveApplicationSession,
    ) -> UiIntentConsequencePublicationOutcome<'session> {
        let (admitted, mounted) = self.into_admitted(session);
        let outcome = admitted.session.cancel_mounted_presentation(mounted);
        super::finish_completion(admitted, outcome)
    }

    fn into_admitted<'session>(
        self,
        session: &'session mut WorthUiActiveApplicationSession,
    ) -> (
        UiIntentConsequenceAdmitted<'session>,
        crate::mounting::UiMountedPresentationInFlight,
    ) {
        let Self {
            session_identity: _,
            plan,
            reservation,
            transfer,
            query,
            mounted,
        } = self;
        (
            UiIntentConsequenceAdmitted {
                session,
                plan,
                reservation,
                transfer,
                query,
            },
            mounted,
        )
    }
}

fn finish_progressed<'session>(
    mut admitted: UiIntentConsequenceAdmitted<'session>,
    outcome: crate::mounting::UiMountedFrameOutcome,
    now_tick: u64,
) -> UiIntentConsequencePublicationOutcome<'session> {
    let crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected) = outcome else {
        return super::finish_completion(admitted, outcome);
    };
    if rejected.rejections().is_empty()
        || !rejected.rejections().iter().all(|rejection| {
            rejection.denial()
                == worth_ui_host_contract::UiHostSurfacePresentationDenial::
                    TextAtlasPresentationDeferred
        })
    {
        return super::finish_completion(
            admitted,
            crate::mounting::UiMountedFrameOutcome::RejectedBeforeEffects(rejected),
        );
    }
    admitted
        .reservation
        .return_to_pending()
        .expect("progressed text deferral returns consequence reservation to pending");
    if let Err(denial) = admitted.reservation.begin_effecting() {
        return stop_admitted(
            admitted,
            crate::runtime::intent_execution::UiIntentConsequenceStopReason::RebindAdmission(
                denial,
            ),
        );
    }
    let deadline = presentation_deadline(&admitted.plan);
    let outcome = admitted.session.present_prepared_mounted_frame_internal(
        rejected.into_frame(),
        deadline,
        now_tick,
    );
    super::finish_completion(admitted, outcome)
}
