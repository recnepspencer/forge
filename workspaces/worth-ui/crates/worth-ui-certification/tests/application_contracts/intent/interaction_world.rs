use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui::facade::interaction::UiHostInteractionIngressOutcome;
use worth_ui::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationMountedBasis, UiHostObservationPayload, UiHostObservationPresentationBasis,
    UiHostObservationReport, UiHostObservationSequence, UiHostObservationSequenceRange,
    UiHostObservationTimeBasis, UiHostPointerButton, UiHostPointerButtonTransition,
    UiHostPointerCaptureEpoch, UiHostPointerIdentity, UiHostProtocolContract,
    UiHostProtocolNegotiation, UiHostSurfacePosition, UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT,
};
use worth_ui_host_contract::WorthUiHostMechanicsAdapter;
use worth_ui_runtime::facade::mounted::{
    UiMountedFrameOutcome, UiMountedHitTestMechanic, UiPresentationDeadline,
    UiSurfaceBindingGeneration,
};
use worth_ui_test_support::WorthUiMountedPublicationCertificationExt;

use super::super::filesystem_mounted_world::{
    establish_allocation, launch_clipped_world, launch_native_world, launch_world, prepare_frame,
    HitOrderProfile,
};

pub(super) struct InteractionWorld {
    pub(super) session: WorthUiActiveApplicationSession,
    pub(super) binding: UiSurfaceBindingGeneration,
    pub(super) presentation: UiHostObservationPresentationBasis,
    pub(super) hit_rows: Box<[UiMountedHitTestMechanic]>,
    next_sequence: u64,
    native_host: Option<worth_ui_host_egui::WorthUiHostEgui>,
}

pub(super) struct NativeInteractionIngress {
    adapter: worth_ui_host_egui::UiEguiRawInputIngressOutcome,
    runtime: Box<[UiHostInteractionIngressOutcome]>,
}

impl InteractionWorld {
    pub(super) fn canonical() -> Self {
        Self::launch(launch_world(HitOrderProfile::Canonical), None)
    }

    pub(super) fn clipped() -> Self {
        Self::launch(launch_clipped_world(), None)
    }

    pub(super) fn native() -> Self {
        let context = egui::Context::default();
        let _ = context.run_ui(egui::RawInput::default(), |_| {});
        let host = worth_ui_host_egui::WorthUiHostEgui::new(context);
        Self::launch(launch_native_world(host.clone()), Some(host))
    }

    pub(super) fn from_session(session: WorthUiActiveApplicationSession) -> Self {
        Self::launch(session, None)
    }

    fn launch(
        mut session: WorthUiActiveApplicationSession,
        native_host: Option<worth_ui_host_egui::WorthUiHostEgui>,
    ) -> Self {
        establish_allocation(&mut session, 3);
        let (presentation, binding, hit_rows) = publish(&mut session);
        Self {
            session,
            binding,
            presentation,
            hit_rows,
            next_sequence: 1,
            native_host,
        }
    }

    pub(super) fn publish_successor(&mut self) {
        let (presentation, binding, hit_rows) = publish(&mut self.session);
        assert_eq!(binding, self.binding);
        self.presentation = presentation;
        self.hit_rows = hit_rows;
    }

    pub(super) fn button(
        &mut self,
        pointer: u64,
        capture_epoch: u64,
        transition: UiHostPointerButtonTransition,
        point: [i64; 2],
    ) -> UiHostInteractionIngressOutcome {
        self.admit(
            self.presentation,
            UiHostObservationLoss::Complete,
            vec![UiHostObservationPayload::PointerButton {
                pointer: UiHostPointerIdentity::new(pointer),
                capture_epoch: UiHostPointerCaptureEpoch::new(capture_epoch),
                button: UiHostPointerButton::Primary,
                transition,
                position: position(point),
            }],
        )
    }

    pub(super) fn motion(
        &mut self,
        pointer: u64,
        capture_epoch: u64,
        point: [i64; 2],
    ) -> UiHostInteractionIngressOutcome {
        self.admit(
            self.presentation,
            UiHostObservationLoss::Complete,
            vec![UiHostObservationPayload::PointerMotion {
                pointer: UiHostPointerIdentity::new(pointer),
                capture_epoch: UiHostPointerCaptureEpoch::new(capture_epoch),
                pressed_buttons:
                    worth_ui::facade::observation_report::UiHostPressedPointerButtons::from_buttons(
                        [UiHostPointerButton::Primary],
                    ),
                position: position(point),
            }],
        )
    }

    pub(super) fn focus_loss(&mut self) -> UiHostInteractionIngressOutcome {
        self.admit(
            self.presentation,
            UiHostObservationLoss::Complete,
            vec![UiHostObservationPayload::Focus { focused: false }],
        )
    }

    pub(super) fn native_input(&mut self, events: Vec<egui::Event>) -> NativeInteractionIngress {
        let host = self
            .native_host
            .as_ref()
            .expect("native input requires the production egui host world");
        let adapter = host.observe_native_input(&egui::RawInput {
            events,
            ..Default::default()
        });
        let runtime = host
            .drain_mechanical_host_observations(self.session.host_session_identity().as_u64())
            .expect("the native interaction drain is structurally bounded")
            .into_batches()
            .into_vec()
            .into_iter()
            .map(|batch| self.session.admit_host_interaction_batch(batch))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        NativeInteractionIngress { adapter, runtime }
    }

    pub(super) fn native_host(&self) -> &worth_ui_host_egui::WorthUiHostEgui {
        self.native_host
            .as_ref()
            .expect("native host evidence requires the native world")
    }

    pub(super) fn payload_at(
        &mut self,
        sequence: u64,
        tick: u64,
        payload: UiHostObservationPayload,
    ) -> UiHostInteractionIngressOutcome {
        let sequence = UiHostObservationSequence::new(sequence);
        let report = UiHostObservationReport::new(
            sequence,
            UiHostObservationTimeBasis::HostMonotonicTick(tick),
            payload,
        );
        self.admit_range(
            self.presentation,
            (sequence, sequence),
            UiHostObservationLoss::Complete,
            vec![report],
        )
    }

    pub(super) fn pointer_button_overflow(&mut self) -> UiHostInteractionIngressOutcome {
        let sequence = UiHostObservationSequence::new(self.next_sequence);
        self.next_sequence += 1;
        let loss = UiHostObservationLoss::Overflow {
            family: worth_ui::facade::observation_report::UiHostObservationFamily::PointerButton,
            affected: UiHostObservationSequenceRange::new(sequence, sequence),
        };
        self.admit_range(self.presentation, (sequence, sequence), loss, Vec::new())
    }

    pub(super) fn button_at_presentation(
        &mut self,
        presentation: UiHostObservationPresentationBasis,
        pointer: u64,
        transition: UiHostPointerButtonTransition,
        point: [i64; 2],
    ) -> UiHostInteractionIngressOutcome {
        self.admit(
            presentation,
            UiHostObservationLoss::Complete,
            vec![UiHostObservationPayload::PointerButton {
                pointer: UiHostPointerIdentity::new(pointer),
                capture_epoch: UiHostPointerCaptureEpoch::new(1),
                button: UiHostPointerButton::Primary,
                transition,
                position: position(point),
            }],
        )
    }

    pub(super) fn button_with_mounted_basis(
        &mut self,
        mounted: UiHostObservationMountedBasis,
        pointer: u64,
        transition: UiHostPointerButtonTransition,
        point: [i64; 2],
    ) -> UiHostInteractionIngressOutcome {
        let sequence = UiHostObservationSequence::new(self.next_sequence);
        self.next_sequence += 1;
        let report = UiHostObservationReport::new(
            sequence,
            UiHostObservationTimeBasis::HostMonotonicTick(sequence.value()),
            UiHostObservationPayload::PointerButton {
                pointer: UiHostPointerIdentity::new(pointer),
                capture_epoch: UiHostPointerCaptureEpoch::new(1),
                button: UiHostPointerButton::Primary,
                transition,
                position: position(point),
            },
        )
        .with_mounted_basis(mounted);
        self.admit_range(
            self.presentation,
            (sequence, sequence),
            UiHostObservationLoss::Complete,
            vec![report],
        )
    }

    fn admit(
        &mut self,
        presentation: UiHostObservationPresentationBasis,
        loss: UiHostObservationLoss,
        payloads: Vec<UiHostObservationPayload>,
    ) -> UiHostInteractionIngressOutcome {
        let first = UiHostObservationSequence::new(self.next_sequence);
        let reports = payloads
            .into_iter()
            .map(|payload| {
                let sequence = UiHostObservationSequence::new(self.next_sequence);
                self.next_sequence += 1;
                UiHostObservationReport::new(
                    sequence,
                    UiHostObservationTimeBasis::HostMonotonicTick(sequence.value()),
                    payload,
                )
            })
            .collect::<Vec<_>>();
        let last = reports
            .last()
            .map_or(first, UiHostObservationReport::sequence);
        self.admit_range(presentation, (first, last), loss, reports)
    }

    fn admit_range(
        &mut self,
        presentation: UiHostObservationPresentationBasis,
        sequences: (UiHostObservationSequence, UiHostObservationSequence),
        loss: UiHostObservationLoss,
        reports: Vec<UiHostObservationReport>,
    ) -> UiHostInteractionIngressOutcome {
        let batch = UiHostObservationBatch::new(UiHostObservationBatchInput {
            protocol: protocol(),
            host_session: self.session.host_session_identity().as_u64(),
            presentation,
            sequences: UiHostObservationSequenceRange::new(sequences.0, sequences.1),
            loss,
            reports,
        })
        .expect("gesture world emits a structurally valid raw batch");
        self.session.admit_host_interaction_batch(batch)
    }
}

impl NativeInteractionIngress {
    pub(super) const fn adapter(&self) -> worth_ui_host_egui::UiEguiRawInputIngressOutcome {
        self.adapter
    }

    pub(super) fn into_runtime(self) -> Box<[UiHostInteractionIngressOutcome]> {
        self.runtime
    }
}

fn publish(
    session: &mut WorthUiActiveApplicationSession,
) -> (
    UiHostObservationPresentationBasis,
    UiSurfaceBindingGeneration,
    Box<[UiMountedHitTestMechanic]>,
) {
    let prepared = prepare_frame(session).expect("gesture world completes mounted projection");
    let hit_rows = prepared.surfaces()[0]
        .projection()
        .hit_tests()
        .rows()
        .to_vec()
        .into_boxed_slice();
    let publication = match session.present_prepared_mounted_frame(
        prepared,
        UiPresentationDeadline::at_tick(1_000),
        0,
    ) {
        UiMountedFrameOutcome::Published(publication) => publication,
        _ => panic!("gesture world must publish"),
    };
    let binding = publication.bindings()[0];
    let presentation = UiHostObservationPresentationBasis::new(
        publication.frame(),
        binding,
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(
            publication.attempt().diagnostic_value(),
        ),
    );
    (presentation, binding, hit_rows)
}

fn position(point: [i64; 2]) -> UiHostSurfacePosition {
    UiHostSurfacePosition::viewport_logical(
        point[0] * UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT,
        point[1] * UI_HOST_SURFACE_POSITION_SUBPIXELS_PER_UNIT,
    )
}

fn protocol() -> worth_ui::facade::observation_report::UiHostProtocolAgreement {
    match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => {
            panic!("current host observation protocol must negotiate")
        }
    }
}
