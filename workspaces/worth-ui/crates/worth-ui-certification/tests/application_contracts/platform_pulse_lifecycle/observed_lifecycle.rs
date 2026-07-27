use worth_ui::facade::app::{
    UiMountedFramePublicationReceipt, WorthUiNativeApplicationShutdownReceipt,
};
use worth_ui::facade::source::WorthUiFilesystemWatcherShutdownReceipt;
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseReplacementDenialFamily,
};

use super::real_watcher_world::{
    InitialPulsePublication, PreservedPulseReplacement, PublishedPulseReplacement,
};

pub(super) struct ObservedPulseLifecycle {
    stream: PlatformPulseLifecycleObservationStream,
    next_sequence: u64,
}

impl ObservedPulseLifecycle {
    pub(super) fn start() -> Self {
        let (stream, started) = PlatformPulseLifecycleObservationStream::start();
        assert_eq!(started.sequence().value(), 1);
        assert!(matches!(
            started.outcome(),
            PlatformPulseLifecycleObservation::ProcessStarted(_)
        ));
        Self {
            stream,
            next_sequence: 2,
        }
    }

    pub(super) fn first_publication(&mut self, publication: &InitialPulsePublication) {
        let observed = self
            .stream
            .project_first_frame(&publication.source, &publication.mounted)
            .expect("real initial publication projects");
        self.assert_next_sequence(&observed);
        assert!(matches!(
            observed.outcome(),
            PlatformPulseLifecycleObservation::FirstFramePublished(_)
        ));
    }

    pub(super) fn replacement(&mut self, publication: &PublishedPulseReplacement) {
        let observed = self
            .stream
            .project_replacement(
                &publication.source,
                &publication.application,
                &publication.mounted,
            )
            .expect("real cutover and mounted receipts project");
        self.assert_next_sequence(&observed);
        assert!(matches!(
            observed.outcome(),
            PlatformPulseLifecycleObservation::ReplacementPublished(_)
        ));
    }

    pub(super) fn reject_stale_replacement(&mut self, publication: &PublishedPulseReplacement) {
        assert_eq!(
            self.stream.project_replacement(
                &publication.source,
                &publication.application,
                &publication.mounted,
            ),
            Err(PlatformPulseLifecycleObservationProjectionDenial::PriorGenerationMismatch)
        );
    }

    pub(super) fn preservation(&mut self, preservation: &PreservedPulseReplacement) {
        let observed = self
            .stream
            .project_preserved_predecessor(&preservation.source, &preservation.denial)
            .expect("real malformed-source denial projects predecessor preservation");
        self.assert_next_sequence(&observed);
        let PlatformPulseLifecycleObservation::ReplacementDeniedPreserving(observed) =
            observed.outcome()
        else {
            panic!("malformed source should project predecessor preservation");
        };
        assert_eq!(
            observed.denial_family(),
            PlatformPulseReplacementDenialFamily::DslCompilation
        );
        assert_eq!(
            observed.active_generation().semantic_package_fingerprint(),
            preservation
                .generation
                .semantic_package_identity()
                .narrowing_fingerprint()
        );
    }

    pub(super) fn reject_mismatched_mounted_receipt(
        &mut self,
        replacement: &PublishedPulseReplacement,
        wrong_mounted: &UiMountedFramePublicationReceipt,
    ) {
        assert_eq!(
            self.stream.project_replacement(
                &replacement.source,
                &replacement.application,
                wrong_mounted,
            ),
            Err(PlatformPulseLifecycleObservationProjectionDenial::ActiveGenerationMismatch)
        );
    }

    pub(super) fn shutdown(
        &mut self,
        watcher: &WorthUiFilesystemWatcherShutdownReceipt,
        application: WorthUiNativeApplicationShutdownReceipt,
    ) {
        let observed = self
            .stream
            .project_shutdown(watcher, application)
            .expect("real watcher and native-shell shutdown receipts project");
        self.assert_next_sequence(&observed);
        assert!(matches!(
            observed.outcome(),
            PlatformPulseLifecycleObservation::ShutdownCompleted(_)
        ));
        assert_eq!(
            self.stream.project_source_worker_panic(),
            Err(PlatformPulseLifecycleObservationProjectionDenial::StreamTerminated)
        );
    }

    fn assert_next_sequence(&mut self, observed: &PlatformPulseLifecycleObservationEnvelope) {
        assert_eq!(observed.sequence().value(), self.next_sequence);
        self.next_sequence += 1;
    }
}
