use super::{
    rebind::{normalize_rebind, PlatformPulseRebindAction},
    PlatformPulseNativeFrame, PlatformPulseTerminalError,
};
use crate::source_watch::{PlatformPulseSourceEvent, PlatformPulseSourceWatch};

impl PlatformPulseNativeFrame {
    pub(super) fn poll_source(&mut self) {
        while self.terminal_error.is_none() {
            let Some(event) = self
                .source_watch
                .as_ref()
                .and_then(PlatformPulseSourceWatch::try_next)
            else {
                return;
            };
            match event {
                PlatformPulseSourceEvent::Settled(snapshot) => self.replace_from(snapshot),
                PlatformPulseSourceEvent::Failed(denial) => {
                    let observation = self.publisher.filesystem_watcher_failure(&denial);
                    self.fail(
                        PlatformPulseTerminalError::SourceWatcher(denial),
                        observation,
                    );
                }
            }
        }
    }

    fn replace_from(
        &mut self,
        snapshot: Box<worth_ui::facade::source::WorthUiSettledSourceSnapshot>,
    ) {
        let Some(mut shell) = self.shell.take() else {
            return;
        };
        let source = snapshot.source_revision().clone();
        self.presentation_tick = self.presentation_tick.saturating_add(1);
        let deadline = self.presentation_tick.saturating_add(1);
        let action = normalize_rebind(&mut shell, *snapshot, deadline, self.presentation_tick);
        self.shell = Some(shell);
        match action {
            PlatformPulseRebindAction::SourceDenied(denial) => {
                let failure = denial
                    .source_failure()
                    .expect("source-denied action retains exact source failure");
                let observation = self.publisher.preserved_predecessor(&source, failure);
                if let Err(error) = observation {
                    self.fail(
                        PlatformPulseTerminalError::ObservationPublication,
                        Err(error),
                    );
                } else {
                    eprintln!(
                        "WORTH UI platform pulse kept its predecessor after source denial: {failure:?}"
                    );
                }
            }
            PlatformPulseRebindAction::Published(receipt) => {
                let publication = self.publisher.replacement(&source, &receipt);
                if let Err(error) = publication {
                    self.fail(
                        PlatformPulseTerminalError::ObservationPublication,
                        Err(error),
                    );
                    return;
                }
                let shell = self
                    .shell
                    .as_mut()
                    .expect("normalized rebind restores the native shell");
                if let Err(denial) = self.visual_identity.compare_after_rebind(
                    shell,
                    receipt,
                    self.presentation_tick,
                    std::time::Instant::now(),
                ) {
                    self.fail_visual_identity(denial);
                }
            }
            PlatformPulseRebindAction::ObservedNoChange => {}
            PlatformPulseRebindAction::NonterminalDisposed => {
                self.fail(
                    PlatformPulseTerminalError::ObservationPublication,
                    self.publisher.native_rebind_outcome_failure(),
                );
            }
            PlatformPulseRebindAction::Denied(denial) => {
                let observation = self.publisher.native_rebind_failure(&denial);
                self.fail(
                    PlatformPulseTerminalError::NativeRebind(denial),
                    observation,
                );
            }
        }
    }
}
