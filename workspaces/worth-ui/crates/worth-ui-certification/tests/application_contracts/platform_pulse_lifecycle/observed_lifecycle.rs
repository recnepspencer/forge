use worth_ui::facade::app::WorthUiNativeApplicationShutdownReceipt;
use worth_ui::facade::source::WorthUiFilesystemWatcherShutdownReceipt;
use worth_ui_platform_pulse::observation_contract::{
    PlatformPulseLifecycleObservation, PlatformPulseLifecycleObservationEnvelope,
    PlatformPulseLifecycleObservationProjectionDenial, PlatformPulseLifecycleObservationStream,
    PlatformPulseLiveQueryResidue, PlatformPulseQueryProjectionResidue,
    PlatformPulseQueryShutdownEvidence, PlatformPulseQueryWatcherShutdownEvidence,
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
            .project_replacement(&publication.source, &publication.receipt)
            .expect("real cutover and mounted receipts project");
        self.assert_next_sequence(&observed);
        assert!(matches!(
            observed.outcome(),
            PlatformPulseLifecycleObservation::RebindPublished(_)
        ));
    }

    pub(super) fn reject_stale_replacement(&mut self, publication: &PublishedPulseReplacement) {
        assert_eq!(
            self.stream
                .project_replacement(&publication.source, &publication.receipt),
            Err(PlatformPulseLifecycleObservationProjectionDenial::PriorGenerationMismatch)
        );
    }

    pub(super) fn preservation(&mut self, preservation: &PreservedPulseReplacement) {
        let observed = self
            .stream
            .project_preserved_predecessor(
                &preservation.source,
                preservation
                    .denial
                    .source_failure()
                    .expect("preservation retains exact source denial"),
            )
            .expect("real malformed-source denial projects predecessor preservation");
        self.assert_next_sequence(&observed);
        let PlatformPulseLifecycleObservation::RebindDeniedPreserving(observed) =
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

    pub(super) fn require_unified_mounted_receipt(&self, replacement: &PublishedPulseReplacement) {
        let mounted = replacement
            .receipt
            .mounted_publication()
            .expect("changed rebind has mounted publication");
        assert_eq!(
            mounted.generation(),
            replacement.receipt.active_generation(),
            "the opaque rebind receipt binds its mounted publication to its active generation"
        );
    }

    pub(super) fn shutdown(
        &mut self,
        watcher: &WorthUiFilesystemWatcherShutdownReceipt,
        application: WorthUiNativeApplicationShutdownReceipt,
    ) {
        let observed = self
            .stream
            .project_shutdown(watcher, query_lifecycle_not_started(), application)
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

fn query_lifecycle_not_started() -> PlatformPulseQueryShutdownEvidence {
    PlatformPulseQueryShutdownEvidence::new(
        PlatformPulseQueryWatcherShutdownEvidence::new(false, 0),
        false,
        PlatformPulseLiveQueryResidue::new(0, 0, 0, 0),
        PlatformPulseQueryProjectionResidue::new(0, 0),
    )
}
