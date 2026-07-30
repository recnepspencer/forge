use super::{
    projection::publish_projection, PlatformPulseNativeFrame, PlatformPulseProjectionRebindDenial,
    PlatformPulseTerminalError,
};

impl PlatformPulseNativeFrame {
    pub(super) fn publish_initial_projection(&mut self) {
        let observation = match self
            .query_lifecycle
            .as_mut()
            .expect("prepared Pulse retains its Query lifecycle")
            .issue_initial()
        {
            Ok(observation) => observation,
            Err(denial) => {
                self.fail(PlatformPulseTerminalError::QueryLifecycle(denial), Ok(()));
                return;
            }
        };
        self.publish_query_observation(observation, true);
    }

    pub(super) fn poll_query(&mut self) {
        while self.terminal_error.is_none() {
            let Some(event) = self
                .query_watch
                .as_ref()
                .and_then(crate::query_source::PlatformPulseExternalValueWatch::try_next)
            else {
                return;
            };
            match event {
                crate::query_source::PlatformPulseExternalValueEvent::Record(record) => {
                    let observation = match self
                        .query_lifecycle
                        .as_mut()
                        .expect("prepared Pulse retains its Query lifecycle")
                        .advance(record)
                    {
                        Ok(observation) => observation,
                        Err(denial) => {
                            self.fail(PlatformPulseTerminalError::QueryLifecycle(denial), Ok(()));
                            return;
                        }
                    };
                    self.publish_query_observation(observation, false);
                }
                crate::query_source::PlatformPulseExternalValueEvent::Failed(denial) => {
                    self.fail(PlatformPulseTerminalError::QueryWatch(denial), Ok(()));
                }
            }
        }
    }

    fn publish_query_observation(
        &mut self,
        observation: worth_ui::facade::query_binding::UiProjectionObservation,
        first: bool,
    ) {
        let evidence = match self.publisher.query_projection_issued(&observation) {
            Ok(evidence) => evidence,
            Err(error) => {
                self.fail(
                    PlatformPulseTerminalError::ObservationPublication,
                    Err(error),
                );
                return;
            }
        };
        let Some(mut shell) = self.shell.take() else {
            return;
        };
        self.tick = self.tick.saturating_add(1);
        let receipt = match publish_projection(&mut shell, observation, self.tick) {
            Ok(receipt) => receipt,
            Err(denial) => {
                self.shell = Some(shell);
                self.fail(PlatformPulseTerminalError::NativeProjection(denial), Ok(()));
                return;
            }
        };
        if first && !self.publish_query_first_frame(&receipt) {
            self.shell = Some(shell);
            return;
        }
        let Some(mounted) = receipt.mounted_publication() else {
            self.shell = Some(shell);
            self.fail(
                PlatformPulseTerminalError::NativeProjection(
                    PlatformPulseProjectionRebindDenial::Nonpublication(
                        "mounted publication absent from receipt".to_owned(),
                    ),
                ),
                Ok(()),
            );
            return;
        };
        if let Err(error) = self
            .publisher
            .query_projection_published(&evidence, mounted)
        {
            self.shell = Some(shell);
            self.fail(
                PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
            return;
        }
        if evidence.posture()
            == worth_ui_platform_pulse::observation_contract::
                PlatformPulseQueryProjectionPosture::Current
        {
            if let Err(denial) = self.visual_identity.refresh_after_content_rebind(
                &mut shell,
                self.tick,
                std::time::Instant::now(),
            ) {
                self.shell = Some(shell);
                self.fail_visual_identity(denial);
                return;
            }
        }
        if evidence.posture()
            == worth_ui_platform_pulse::observation_contract::
                PlatformPulseQueryProjectionPosture::Current
            && evidence.owner_order() == 2
        {
            if let Err(denial) = self
                .visual_identity
                .arm_after_first_frame(std::time::Instant::now())
            {
                self.shell = Some(shell);
                self.fail_visual_identity(denial);
                return;
            }
        }
        let fact = match receipt.release_scalar_projection_predecessor() {
            Ok(fact) => fact,
            Err(_) => {
                self.shell = Some(shell);
                self.fail(
                    PlatformPulseTerminalError::NativeProjection(
                        PlatformPulseProjectionRebindDenial::Nonpublication(
                            "scalar predecessor absent from receipt".to_owned(),
                        ),
                    ),
                    Ok(()),
                );
                return;
            }
        };
        let admission = self
            .query_lifecycle
            .as_mut()
            .expect("prepared Pulse retains its Query lifecycle")
            .admit_publication(fact);
        self.shell = Some(shell);
        if let Err(denial) = admission {
            self.fail(PlatformPulseTerminalError::QueryLifecycle(denial), Ok(()));
        }
    }

    fn publish_query_first_frame(
        &mut self,
        receipt: &worth_ui::facade::rebind::UiRebindReceipt,
    ) -> bool {
        let Some(source) = self.initial_source.take() else {
            self.fail(PlatformPulseTerminalError::UnexpectedInitialFrame, Ok(()));
            return false;
        };
        let Some(mounted) = receipt.mounted_publication() else {
            self.fail(PlatformPulseTerminalError::UnexpectedInitialFrame, Ok(()));
            return false;
        };
        if let Err(error) = self.publisher.first_frame(&source, mounted) {
            self.fail(
                PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
            return false;
        }
        true
    }
}
