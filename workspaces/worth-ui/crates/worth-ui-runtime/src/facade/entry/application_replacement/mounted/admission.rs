use super::{
    WorthUiActiveApplicationSession, WorthUiMountedApplicationReplacementOutcome,
    WorthUiMountedReplacementAdmissionDenial, WorthUiMountedReplacementPublicationState,
    WorthUiMountedReplacementRetentionDenial, WorthUiPreparedApplicationActivation,
    WorthUiPreparedMountedApplicationReplacement,
};

pub(super) struct WorthUiMountedReplacementAdmissionInput<'session> {
    pub(super) session: &'session mut WorthUiActiveApplicationSession,
    pub(super) application: Box<WorthUiPreparedApplicationActivation>,
    pub(super) mounted_successor: crate::mounting::UiMountedIdentityState,
    pub(super) frame: crate::mounting::UiPreparedMountedFrame,
}

pub(super) struct WorthUiAdmittedMountedReplacement<'session> {
    state: WorthUiMountedReplacementPublicationState<'session>,
    admission: crate::mounting::UiMountedPresentationAdmission,
    capability_report: worth_ui_host_contract::WorthUiHostCapabilityReport,
}

pub(super) fn prepare_replacement_presentation(
    input: WorthUiMountedReplacementAdmissionInput<'_>,
    deadline: worth_ui_host_contract::UiPresentationDeadline,
    now: u64,
) -> Result<WorthUiAdmittedMountedReplacement<'_>, WorthUiMountedApplicationReplacementOutcome<'_>>
{
    let WorthUiMountedReplacementAdmissionInput {
        session,
        application,
        mounted_successor,
        frame,
    } = input;
    let capability_report = session.host_session.capability_report().clone();
    let admitted = match mounted_successor.admit_prepared_frame_authority(frame) {
        Ok(admitted) => admitted,
        Err(rejection) => {
            let denial = rejection.denial();
            session
                .host_observations
                .record_never_presented_frame(rejection.frame().canonical_core().frame());
            return Err(
                WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(
                    WorthUiMountedReplacementAdmissionDenial {
                        denial,
                        replacement: Box::new(WorthUiPreparedMountedApplicationReplacement {
                            session,
                            application,
                            mounted_successor,
                            frame: rejection.into_frame(),
                        }),
                    },
                ),
            );
        }
    };
    let retained = match session.mounted_retention.prepare_publication(admitted) {
        Ok(retained) => retained,
        Err(rejection) => {
            session
                .host_observations
                .record_never_presented_frame(rejection.frame().canonical_core().frame());
            return Err(
                WorthUiMountedApplicationReplacementOutcome::RetentionDenied(
                    WorthUiMountedReplacementRetentionDenial {
                        denial: rejection.denial(),
                        replacement: Box::new(WorthUiPreparedMountedApplicationReplacement {
                            session,
                            application,
                            mounted_successor,
                            frame: rejection.into_frame(),
                        }),
                    },
                ),
            );
        }
    };
    let admission = match session.mounted_presentation.admit_current(
        retained,
        &capability_report,
        deadline,
        now,
    ) {
        Ok(admission) => admission,
        Err(rejection) => {
            let denial = rejection.denial();
            session
                .host_observations
                .record_never_presented_frame(rejection.frame().canonical_core().frame());
            return Err(
                WorthUiMountedApplicationReplacementOutcome::AdmissionDenied(
                    WorthUiMountedReplacementAdmissionDenial {
                        denial,
                        replacement: Box::new(WorthUiPreparedMountedApplicationReplacement {
                            session,
                            application,
                            mounted_successor,
                            frame: rejection.into_frame(),
                        }),
                    },
                ),
            );
        }
    };
    let publication = crate::mounting::UiMountedFramePublicationCandidate::reserve(
        &admission,
        session.mounted_identity.view().current_frame(),
    );
    Ok(WorthUiAdmittedMountedReplacement {
        state: WorthUiMountedReplacementPublicationState {
            session,
            application,
            mounted_successor,
            publication,
        },
        admission,
        capability_report,
    })
}

impl<'session> WorthUiAdmittedMountedReplacement<'session> {
    pub(super) fn into_parts(
        self,
    ) -> (
        WorthUiMountedReplacementPublicationState<'session>,
        crate::mounting::UiMountedPresentationAdmission,
        worth_ui_host_contract::WorthUiHostCapabilityReport,
    ) {
        (self.state, self.admission, self.capability_report)
    }
}
