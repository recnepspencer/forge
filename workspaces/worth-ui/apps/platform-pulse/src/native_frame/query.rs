use worth_ui::facade::app::{UiMountedFramePublicationReceipt, WorthUiNativeApplicationShell};
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseQueryProjectionEvidence, PlatformPulseQueryProjectionPosture,
};

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
        if !self.publish_query_projection_evidence(&evidence, mounted) {
            self.shell = Some(shell);
            return;
        }
        if let Err(denial) = self.refresh_query_visual_identity(&mut shell, &evidence) {
            self.shell = Some(shell);
            self.fail_visual_identity(denial);
            return;
        }
        self.admit_query_publication(receipt);
        self.shell = Some(shell);
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
        if let Err(error) = self.publish_first_frame(&source, mounted) {
            self.fail(
                PlatformPulseTerminalError::ObservationPublication,
                Err(error),
            );
            return false;
        }
        true
    }

    fn publish_query_projection_evidence(
        &mut self,
        evidence: &PlatformPulseQueryProjectionEvidence,
        mounted: &UiMountedFramePublicationReceipt,
    ) -> bool {
        match self.publisher.query_projection_published(evidence, mounted) {
            Ok(()) => true,
            Err(error) => {
                self.fail(
                    PlatformPulseTerminalError::ObservationPublication,
                    Err(error),
                );
                false
            }
        }
    }

    fn refresh_query_visual_identity(
        &mut self,
        shell: &mut WorthUiNativeApplicationShell,
        evidence: &PlatformPulseQueryProjectionEvidence,
    ) -> Result<(), crate::visual_identity_execution::PlatformPulseVisualExecutionDenial> {
        if evidence.posture() != PlatformPulseQueryProjectionPosture::Current {
            return Ok(());
        }
        self.visual_identity.refresh_after_content_rebind(
            shell,
            self.tick,
            std::time::Instant::now(),
        )?;
        if evidence.owner_order() == 2 {
            self.visual_identity
                .arm_after_first_frame(std::time::Instant::now())?;
        }
        Ok(())
    }

    fn admit_query_publication(&mut self, receipt: worth_ui::facade::rebind::UiRebindReceipt) {
        let observation = match receipt.release_scalar_projection_observation() {
            Ok(observation) => observation,
            Err(_) => {
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
            .admit_publication(observation);
        if let Err(denial) = admission {
            self.fail(PlatformPulseTerminalError::QueryLifecycle(denial), Ok(()));
        }
    }
}
