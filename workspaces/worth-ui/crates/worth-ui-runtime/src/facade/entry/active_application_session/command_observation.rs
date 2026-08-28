use super::super::WorthUiActiveApplicationSession;

impl WorthUiActiveApplicationSession {
    pub(in crate::facade::entry) fn observe_command_report(
        &mut self,
        report: &worth_ui_host_contract::UiHostObservationReport,
        presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Option<crate::runtime::UiCommandRoutingOutcome> {
        if !self.command_routing.is_installed() {
            return None;
        }
        self.observe_ime_composition(report.payload());
        let (stroke, repeat) = crate::runtime::command_routing::keyboard_stroke(report.payload())?;
        let surface = self
            .mounted
            .current_surface_for_binding(presentation.binding())?;
        let semantic_focus = self
            .focus
            .as_ref()
            .and_then(|owner| owner.inspect().current());
        let focused_target = semantic_focus.map(|current| current.mounted_target());
        let presented_focus = focused_target.and_then(|target| {
            crate::runtime::interaction::targeting::resolve_presented_focus_target(
                &self.mounted,
                presentation,
                target,
            )
            .ok()
            .flatten()
        });
        let text_entry_active =
            presented_focus.is_some_and(|target| self.mounted.input_text_profile(target).is_some());
        let context = self
            .current_command_routing_context(surface)
            .with_host_observation(presentation, report.sequence(), report.time_basis())
            .with_text_input(self.ime_composing, text_entry_active);
        let generation = self.active_generation_identity();
        Some(
            self.command_routing
                .as_mut()
                .expect("Command installation was checked above")
                .route_input_stroke(stroke, repeat, context, &generation),
        )
    }

    fn observe_ime_composition(
        &mut self,
        payload: &worth_ui_host_contract::UiHostObservationPayload,
    ) {
        let worth_ui_host_contract::UiHostObservationPayload::ImeComposition { phase, .. } =
            payload
        else {
            return;
        };
        self.ime_composing = matches!(
            phase,
            worth_ui_host_contract::UiHostImeCompositionPhase::Preedit(_)
        );
        if self.ime_composing {
            if let Some(command_routing) = self.command_routing.as_mut() {
                command_routing.cancel_prefix();
            }
        }
    }
}
