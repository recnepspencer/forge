use worth_ui_host_contract::{
    UiHostProtocolNegotiation, UiHostSessionReleaseOutcome, UiHostSessionReleaseReceipt,
    UiHostSurfacePresentationDenial, UiHostSurfacePresentationMode,
    UiHostSurfacePresentationOutcome, UiMountedFrameConsumptionView,
    UiMountedSurfacePresentationCompletion, WorthUiHostCapability, WorthUiHostCapabilityReport,
    WorthUiHostContract, WorthUiHostMechanicsAdapter,
};

impl WorthUiHostMechanicsAdapter for super::WorthUiHostEgui {
    fn mechanical_host_contract(&self) -> WorthUiHostContract {
        WorthUiHostContract::egui()
    }

    fn mechanical_capability_report(&self) -> WorthUiHostCapabilityReport {
        let mut capabilities = vec![
            WorthUiHostCapability::DpiObservation,
            WorthUiHostCapability::IdentityOverlay,
            WorthUiHostCapability::NativePaint,
            WorthUiHostCapability::ViewportObservation,
        ];
        capabilities.extend(self.input_translators.capabilities());
        WorthUiHostCapabilityReport::available(capabilities)
    }

    fn drain_mechanical_host_observations(
        &self,
        host_session_identity: u64,
    ) -> Result<
        worth_ui_host_contract::UiHostObservationDrain,
        worth_ui_host_contract::UiHostObservationDrainDenial,
    > {
        Ok(self.drain_native_observations(host_session_identity))
    }

    fn mechanical_visual_capture_capability(
        &self,
    ) -> worth_ui_host_contract::UiHostCaptureCapability {
        super::super::visual_snapshot::capture_capability()
    }

    fn perform_visual_capture(
        &self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureObservationOutcome {
        super::super::visual_snapshot::capture(
            &self.context,
            &self.registrations,
            &self.visual_captures,
            request,
        )
    }

    fn perform_visual_capture_cancellation(
        &self,
        request: worth_ui_host_contract::UiHostVisualCaptureRequest,
    ) -> worth_ui_host_contract::UiHostCaptureCancellationOutcome {
        super::super::visual_snapshot::cancel(&self.visual_captures, request)
    }

    fn mechanical_measurement_environment_report(
        &self,
    ) -> worth_ui_host_contract::UiHostMeasurementEnvironmentReport {
        let viewport = self.context.input(|input| {
            let size = input.viewport_rect().size();
            (size.x.to_bits(), size.y.to_bits())
        });
        let dpi = self.context.pixels_per_point().to_bits();
        let mut environment = self.measurement_environment.lock().unwrap();
        if environment.viewport != Some(viewport) {
            environment.viewport = Some(viewport);
            environment.viewport_generation = next_generation(environment.viewport_generation);
        }
        if environment.dpi != Some(dpi) {
            environment.dpi = Some(dpi);
            environment.dpi_generation = next_generation(environment.dpi_generation);
        }
        worth_ui_host_contract::UiHostMeasurementEnvironmentReport::new(
            Some(environment.viewport_generation),
            Some(environment.dpi_generation),
            None,
            None,
        )
    }

    fn perform_surface_registration(
        &self,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceRegistrationOutcome {
        let current_protocol = match self.mechanical_protocol_contract().negotiate() {
            UiHostProtocolNegotiation::Compatible(agreement) => agreement,
            UiHostProtocolNegotiation::Incompatible(_) => {
                return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                    worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
                );
            }
        };
        let capabilities = self.mechanical_capability_report();
        if request.protocol() != current_protocol
            || request.capability_profile_digest() != capabilities.profile_identity_digest()
        {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        let mut registrations = self.registrations.lock().unwrap();
        if registrations
            .get(&request.binding_generation())
            .is_some_and(|registered| *registered != request)
        {
            return worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        registrations.insert(request.binding_generation(), request);
        worth_ui_host_contract::UiHostSurfaceRegistrationOutcome::RegisteredKnownEmpty
    }

    fn perform_surface_deregistration(
        &self,
        request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
    ) -> worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome {
        let mut registrations = self.registrations.lock().unwrap();
        if registrations.remove(&request.binding_generation()) != Some(request) {
            return worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::RejectedBeforeEffects(
                worth_ui_host_contract::UiHostSurfaceRegistrationDenial::ForeignRegistration,
            );
        }
        self.retained_presentations
            .lock()
            .unwrap()
            .remove(&request.binding_generation());
        super::super::input_observation::remove_binding(
            &self.input_observation,
            request.binding_generation(),
        );
        super::super::visual_snapshot::remove_binding(
            &self.visual_captures,
            request.binding_generation(),
        );
        worth_ui_host_contract::UiHostSurfaceDeregistrationOutcome::Deregistered(
            worth_ui_host_contract::UiHostSurfaceDeregistrationReceipt::from_runtime(
                request.host_session_identity(),
                request.host_surface_identity(),
            ),
        )
    }

    fn perform_mounted_surface_presentation(
        &self,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> UiHostSurfacePresentationOutcome {
        if let Some(denial) = self.validate_mounted_view(view) {
            return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial);
        }
        let binding = view.requirement().binding();
        let mut retained = self.retained_presentations.lock().unwrap();
        let candidate =
            match super::super::mounted_presentation::UiEguiPresentationCandidate::prepare(
                &self.context,
                view,
                retained.get(&binding),
            ) {
                Ok(candidate) => candidate,
                Err(denial) => {
                    return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial);
                }
            };
        let cost = match candidate.cost(view) {
            Ok(cost) => cost,
            Err(denial) => {
                return UiHostSurfacePresentationOutcome::RejectedBeforeEffects(denial);
            }
        };
        let effects = candidate.completed_effects();
        let realized_regions = candidate.realized_regions();
        let epoch = worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(
            view.attempt().diagnostic_value(),
        );
        candidate.paint(&self.context);
        retained.insert(binding, candidate.presentation);
        drop(retained);
        super::super::visual_snapshot::record_presentation(
            &self.visual_captures,
            view,
            epoch,
            realized_regions,
        );
        super::super::input_observation::record_completed_presentation(
            &self.input_observation,
            view,
            epoch,
        );
        UiHostSurfacePresentationOutcome::Presented(UiMountedSurfacePresentationCompletion::new(
            UiHostSurfacePresentationMode::NativeDisplay,
            epoch,
            effects,
            cost,
        ))
    }

    fn release_mechanical_host_session(
        &self,
        host_session_identity: u64,
    ) -> UiHostSessionReleaseOutcome {
        let mut registrations = self.registrations.lock().unwrap();
        let released_bindings = registrations
            .iter()
            .filter_map(|(binding, request)| {
                (request.host_session_identity() == host_session_identity).then_some(*binding)
            })
            .collect::<Vec<_>>();
        let before = registrations.len();
        registrations.retain(|_, request| request.host_session_identity() != host_session_identity);
        let mut retained = self.retained_presentations.lock().unwrap();
        for binding in released_bindings {
            retained.remove(&binding);
            super::super::visual_snapshot::remove_binding(&self.visual_captures, binding);
        }
        super::super::input_observation::release_session(
            &self.input_observation,
            host_session_identity,
        );
        self.observation_retention
            .release_session(host_session_identity);
        UiHostSessionReleaseOutcome::Released(UiHostSessionReleaseReceipt::released(
            host_session_identity,
            before - registrations.len(),
        ))
    }
}

fn next_generation(current: u64) -> u64 {
    current
        .checked_add(1)
        .expect("egui measurement environment generation exhausted")
}

impl super::WorthUiHostEgui {
    fn validate_mounted_view(
        &self,
        view: &UiMountedFrameConsumptionView<'_>,
    ) -> Option<UiHostSurfacePresentationDenial> {
        let requirement = view.requirement();
        if requirement.presentation_mode() != UiHostSurfacePresentationMode::NativeDisplay {
            return Some(
                UiHostSurfacePresentationDenial::UnsupportedPresentationMode(
                    requirement.presentation_mode(),
                ),
            );
        }
        let live_protocol = match self.mechanical_protocol_contract().negotiate() {
            UiHostProtocolNegotiation::Compatible(agreement) => agreement,
            UiHostProtocolNegotiation::Incompatible(denial) => {
                return Some(UiHostSurfacePresentationDenial::Protocol(denial));
            }
        };
        if view.protocol() != live_protocol {
            return Some(UiHostSurfacePresentationDenial::ProtocolChanged);
        }
        let capabilities = self.mechanical_capability_report();
        if view.capability_profile_digest() != capabilities.profile_identity_digest() {
            return Some(UiHostSurfacePresentationDenial::CapabilityProfileChanged);
        }
        let registered = self
            .registrations
            .lock()
            .unwrap()
            .get(&requirement.binding())
            .copied();
        if !registration_matches(registered, view) {
            return Some(UiHostSurfacePresentationDenial::SurfaceBindingChanged);
        }
        None
    }
}

fn registration_matches(
    registered: Option<worth_ui_host_contract::UiHostSurfaceRegistrationRequest>,
    view: &UiMountedFrameConsumptionView<'_>,
) -> bool {
    let Some(registered) = registered else {
        return false;
    };
    let requirement = view.requirement();
    registered.host_session_identity() == view.host_session_identity()
        && registered.semantic_surface_identity() == requirement.semantic_surface()
        && registered.host_surface_identity() == requirement.host_surface()
        && registered.binding_generation() == requirement.binding()
        && registered.protocol() == view.protocol()
        && registered.capability_generation() == view.capability_generation()
        && registered.capability_profile_digest() == view.capability_profile_digest()
        && registered.presentation_mode() == requirement.presentation_mode()
        && view.surface() == requirement.semantic_surface()
        && view.binding() == requirement.binding()
}
