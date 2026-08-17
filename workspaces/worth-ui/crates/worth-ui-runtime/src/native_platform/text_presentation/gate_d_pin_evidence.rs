use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use worth_ui_host_contract::{
    UiHostSurfaceIdentity, UiHostSurfacePresentationMode, UiHostSurfaceRegistrationInput,
    UiHostSurfaceRegistrationOutcome, UiHostSurfaceRegistrationRequest,
    UiMountedFrameConsumptionInput, UiMountedFrameConsumptionView,
    UiMountedPresentationAttemptIdentity, UiMountedPresentationWorkView,
    UiMountedQualifiedTextResolver, UiMountedSurfaceBindingRequirement, UiPresentationDeadline,
    UiQualifiedTextLayoutIdentity, UiQualifiedTextLayoutView,
};
use worth_ui_host_native::{
    UiNativeEventLoopClient, UiNativeEventLoopClientCleanup, UiNativeEventLoopClientClose,
    UiNativeEventLoopDirective, UiNativeReadinessGrant, WorthUiPreparedNativeMechanics,
};

use crate::certification_support::{
    initial_presentation_mechanics_for_certification,
    semantic_text_projection_for_certification_with_text,
};
use crate::facade::prepared_application_authority::WorthUiHostSessionPlan;
use crate::facade::WorthUiHostSessionAuthority;
use crate::mounting::qualified_text_test_support::inert_qualified_layout;
use crate::native_platform::text_presentation::{
    prepare_mounted_semantic_text, UiMountedEventTimeDpiAuthority, UiNativeMountedTextCoordinator,
    UiNativeMountedTextOutcome, UiNativeMountedTextPending, UiNativeMountedTextReleaseOutcome,
    UiNativeTextPresentationPreparation,
};

const PRESSURE_TEXT: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!#$%&()*+,-./:;<=>?@[]^_{|}~";
const PRESSURE_SCALES: [u32; 5] = [20_000, 21_000, 22_000, 25_000, 23_000];

#[path = "gate_d_pin_evidence/receipt.rs"]
mod receipt;
pub use receipt::UiGateDPinWorldEvidence;
#[path = "gate_d_pin_evidence/lifecycle.rs"]
mod lifecycle;
#[path = "gate_d_pin_evidence/run.rs"]
mod run;
pub(crate) use run::run_gate_d_pin_world;

#[derive(Default)]
struct Evidence {
    first_committed: bool,
    second_committed: bool,
    first_release_was_local: bool,
    final_release_crossed_native: bool,
    cleanup_complete: bool,
    rasterized_glyphs: u32,
    pressure_transactions: u32,
    pressure_releases: u32,
    evictions: u32,
    expected_pins: Vec<worth_ui_host_contract::UiGlyphRasterPinRequest>,
    committed_pin_census: Vec<u32>,
}

struct LayoutResolver {
    layout: Arc<worth_ui_text::UiQualifiedTextLayout>,
}

impl UiMountedQualifiedTextResolver for LayoutResolver {
    fn resolve(
        &self,
        identity: UiQualifiedTextLayoutIdentity,
    ) -> Option<UiQualifiedTextLayoutView<'_>> {
        (identity == self.layout.identity()).then(|| self.layout.view())
    }
}

struct BindingWorld {
    requirement: UiMountedSurfaceBindingRequirement,
    registration: UiHostSurfaceRegistrationRequest,
    attempt: UiMountedPresentationAttemptIdentity,
    initial: worth_ui_host_contract::UiMountedPresentationInitial,
    pressure: bool,
}

struct PendingBinding {
    ordinal: u8,
    pending: UiNativeMountedTextPending,
}

struct GateDPinClient {
    host_session: WorthUiHostSessionAuthority,
    evidence: Rc<RefCell<Evidence>>,
    text: UiNativeMountedTextCoordinator,
    resolver: LayoutResolver,
    worlds: Vec<BindingWorld>,
    pending: Option<PendingBinding>,
    pressure_release_pending: bool,
    pressure_advance_pending: bool,
    next_pressure_scale: usize,
    next_world: usize,
    started: bool,
    finished: bool,
}

impl GateDPinClient {
    fn new(adapter: WorthUiPreparedNativeMechanics, evidence: Rc<RefCell<Evidence>>) -> Self {
        let plan = WorthUiHostSessionPlan::prepare(
            crate::native_platform::authorized_native_host::UiAuthorizedNativeHostAdapter::bind(
                adapter,
            ),
        );
        let host_session = WorthUiHostSessionAuthority::activate(&plan)
            .expect("the ordinary runtime host session must activate");
        Self {
            host_session,
            evidence,
            text: UiNativeMountedTextCoordinator::default(),
            resolver: LayoutResolver {
                layout: inert_qualified_layout(PRESSURE_TEXT),
            },
            worlds: Vec::new(),
            pending: None,
            pressure_release_pending: false,
            pressure_advance_pending: false,
            next_pressure_scale: 0,
            next_world: 0,
            started: false,
            finished: false,
        }
    }

    fn prepare_worlds(&mut self) {
        for scale in [1_000, 1_000] {
            self.prepare_world(scale, false);
        }
        self.prepare_next_pressure_world();
    }

    fn prepare_next_pressure_world(&mut self) {
        let Some(scale) = PRESSURE_SCALES.get(self.next_pressure_scale).copied() else {
            return;
        };
        self.next_pressure_scale += 1;
        self.prepare_world(scale, true);
    }

    fn prepare_world(&mut self, device_scale_milli: u32, pressure: bool) {
        let capabilities = self.host_session.capability_report();
        let protocol = self.host_session.protocol();
        let host_session_identity = self.host_session.identity().as_u64();
        let projection = semantic_text_projection_for_certification_with_text(PRESSURE_TEXT);
        let host_surface = UiHostSurfaceIdentity::mint_unbound().unwrap();
        let baseline = UiMountedSurfaceBindingRequirement::new(
            projection.surface(),
            host_surface,
            projection.binding(),
            capabilities.observation_generation(),
            capabilities.profile_identity_digest(),
            UiHostSurfacePresentationMode::NativeDisplay,
        );
        let requirement = UiMountedSurfaceBindingRequirement::with_baseline_and_device_scale(
            projection.surface(),
            host_surface,
            projection.binding(),
            capabilities.observation_generation(),
            capabilities.profile_identity_digest(),
            UiHostSurfacePresentationMode::NativeDisplay,
            baseline.baseline(),
            device_scale_milli,
        );
        let registration =
            UiHostSurfaceRegistrationRequest::from_runtime(UiHostSurfaceRegistrationInput {
                host_session_identity,
                semantic_surface_identity: projection.surface(),
                host_surface_identity: host_surface,
                binding_generation: projection.binding(),
                protocol,
                capability_generation: capabilities.observation_generation(),
                capability_profile_digest: capabilities.profile_identity_digest(),
                presentation_mode: UiHostSurfacePresentationMode::NativeDisplay,
            });
        let host = self.host_session.effect_port();
        assert_eq!(
            host.adapter()
                .register_surface(host.authority(), registration),
            UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
        );
        self.worlds.push(BindingWorld {
            requirement,
            registration,
            attempt: UiMountedPresentationAttemptIdentity::mint_unbound().unwrap(),
            initial: initial_presentation_mechanics_for_certification(&projection, requirement),
            pressure,
        });
    }

    fn begin_next(&mut self) -> bool {
        let Some(world) = self.worlds.get(self.next_world) else {
            return self.release_and_close();
        };
        let Some(dpi) = UiMountedEventTimeDpiAuthority::from_requirement(world.requirement) else {
            panic!("the native requirement carries a nonzero DPI");
        };
        let Some(UiNativeTextPresentationPreparation::Prepared(prepared)) =
            prepare_mounted_semantic_text(
                UiMountedPresentationWorkView::Initial(&world.initial),
                dpi,
                |identity| {
                    (identity == self.resolver.layout.identity())
                        .then_some(self.resolver.layout.as_ref())
                },
            )
        else {
            panic!("the runtime must prepare exact mounted text demands");
        };
        let view =
            UiMountedFrameConsumptionView::from_inert_mechanics(UiMountedFrameConsumptionInput {
                authority: self
                    .host_session
                    .mounted_presentation_lease()
                    .mechanics_authority(),
                host_session_identity: self.host_session.identity().as_u64(),
                protocol: world.registration.protocol(),
                capability_generation: world.requirement.capability_generation(),
                capability_profile_digest: world.requirement.capability_profile_digest(),
                attempt: world.attempt,
                deadline: UiPresentationDeadline::at_tick(50),
                requirement: world.requirement,
                presentation_work: UiMountedPresentationWorkView::Initial(&world.initial),
                qualified_text: &self.resolver,
                text_raster_work: None,
            });
        let observation = self
            .text
            .begin(
                world.requirement.binding(),
                &prepared,
                |identity| {
                    (identity == self.resolver.layout.identity())
                        .then_some(self.resolver.layout.as_ref())
                },
                self.host_session.effect_port(),
                &view,
            )
            .expect("the ordinary runtime text coordinator must issue one transaction");
        let mut evidence = self.evidence.borrow_mut();
        if self.next_world == 0 || self.next_world == 2 {
            for pin in &observation.additions {
                if !evidence.expected_pins.contains(pin) {
                    evidence.expected_pins.push(*pin);
                }
            }
        }
        evidence.rasterized_glyphs = evidence
            .rasterized_glyphs
            .saturating_add(observation.outcome.raster_work().rasterized_glyphs());
        drop(evidence);
        self.accept_or_retain(observation.outcome)
    }

    fn accept_or_retain(&mut self, outcome: UiNativeMountedTextOutcome) -> bool {
        match outcome {
            UiNativeMountedTextOutcome::Committed { receipt, .. } => {
                self.pressure_advance_pending = false;
                self.finish_committed(receipt)
            }
            UiNativeMountedTextOutcome::Pending(pending) => {
                self.pressure_advance_pending = false;
                self.pending = Some(PendingBinding {
                    ordinal: self.next_world as u8,
                    pending,
                });
                false
            }
            UiNativeMountedTextOutcome::RejectedBeforeEffects { denial, .. } => {
                if denial
                    == worth_ui_host_contract::UiGlyphRasterTransactionDenial::ReservationConflict
                {
                    return false;
                }
                panic!(
                    "the qualified native atlas transaction was denied before effects: {denial:?}"
                )
            }
            UiNativeMountedTextOutcome::RejectedAfterRasterization { denial, .. } => {
                panic!("the qualified native atlas transaction was denied after rasterization: {denial:?}")
            }
            UiNativeMountedTextOutcome::EffectsIndeterminate { .. } => {
                panic!("the qualified native atlas transaction became indeterminate")
            }
        }
    }

    fn complete_pending(&mut self) -> bool {
        let pending = self.pending.take().expect("pending work remains retained");
        let outcome = self
            .text
            .complete(self.host_session.effect_port(), pending.pending);
        match outcome {
            UiNativeMountedTextOutcome::Pending(token) => {
                self.pending = Some(PendingBinding {
                    pending: token,
                    ..pending
                });
                false
            }
            UiNativeMountedTextOutcome::Committed { receipt, .. } => {
                assert_eq!(pending.ordinal as usize, self.next_world);
                self.finish_committed(receipt)
            }
            _ => panic!("the physical Signal wake must settle the native upload"),
        }
    }
}

impl UiNativeEventLoopClient for GateDPinClient {
    fn native_surface_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        self.prepare_worlds();
        Ok(UiNativeEventLoopDirective::Continue)
    }

    fn redraw_ready(
        &mut self,
        _grant: UiNativeReadinessGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        if !self.started {
            self.started = true;
            let close = self.begin_next()
                || (self.pending.is_none() && self.progress_settled_transactions());
            return Ok(if close {
                UiNativeEventLoopDirective::Close
            } else {
                UiNativeEventLoopDirective::Continue
            });
        }
        Ok(if self.finished {
            UiNativeEventLoopDirective::Close
        } else {
            UiNativeEventLoopDirective::Continue
        })
    }

    fn physical_work_progressed(
        &mut self,
        _grant: worth_ui_host_native::UiNativePhysicalProgressGrant,
    ) -> Result<UiNativeEventLoopDirective, ()> {
        Ok(self.progress_text_transactions())
    }

    fn presentation_attribution(
        &self,
    ) -> Option<worth_ui_host_native::UiNativeClientPresentationAttribution> {
        None
    }

    fn close(self) -> UiNativeEventLoopClientClose {
        if self.finished {
            self.evidence.borrow_mut().cleanup_complete = true;
            UiNativeEventLoopClientClose::Complete
        } else {
            UiNativeEventLoopClientClose::Incomplete(Box::new(self))
        }
    }
}

impl GateDPinClient {
    fn progress_text_transactions(&mut self) -> UiNativeEventLoopDirective {
        for _ in 0..8 {
            if self.pending.is_none() || self.complete_pending() {
                break;
            }
        }
        let _ = self.progress_settled_transactions();
        if self.finished {
            UiNativeEventLoopDirective::Close
        } else {
            UiNativeEventLoopDirective::Continue
        }
    }

    fn progress_settled_transactions(&mut self) -> bool {
        if self.pressure_release_pending {
            let _ = self.release_pressure_world();
        }
        if self.pressure_advance_pending {
            let _ = self.begin_next();
        }
        self.finished
    }
}

impl UiNativeEventLoopClientCleanup for GateDPinClient {
    fn retry(mut self: Box<Self>) -> UiNativeEventLoopClientClose {
        if self.pending.is_some() {
            let _ = self.complete_pending();
        }
        UiNativeEventLoopClient::close(*self)
    }
}
