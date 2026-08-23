use worth_ui::facade::interaction::UiHostInteractionIngressOutcome;
use worth_ui::facade::observation_report::UiHostObservationBatch;
use worth_ui_host_contract::WorthUiHostMechanicsAdapter;

use super::InteractionWorld;

pub(in crate::intent) struct NativeRetainedIngress {
    adapter: worth_ui_host_egui::UiEguiRawInputIngressOutcome,
    batches: Box<[UiHostObservationBatch]>,
}

pub(in crate::intent) struct NativeInteractionIngress {
    adapter: worth_ui_host_egui::UiEguiRawInputIngressOutcome,
    runtime: Box<[UiHostInteractionIngressOutcome]>,
}

impl InteractionWorld {
    pub(in crate::intent) fn native_input(
        &mut self,
        events: Vec<egui::Event>,
    ) -> NativeInteractionIngress {
        let retained = self.retain_native_input(events);
        let adapter = retained.adapter();
        let runtime = retained
            .into_batches()
            .into_vec()
            .into_iter()
            .map(|batch| self.admit_native_batch(batch))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        NativeInteractionIngress { adapter, runtime }
    }

    pub(in crate::intent) fn retain_native_input(
        &self,
        events: Vec<egui::Event>,
    ) -> NativeRetainedIngress {
        let host = self.native_host();
        let adapter = host.observe_native_input(&egui::RawInput {
            events,
            ..Default::default()
        });
        let batches = host
            .drain_mechanical_host_observations(self.session.host_session_identity().as_u64())
            .expect("the native interaction drain is structurally bounded")
            .into_batches();
        NativeRetainedIngress { adapter, batches }
    }

    pub(in crate::intent) fn admit_native_batch(
        &mut self,
        batch: UiHostObservationBatch,
    ) -> UiHostInteractionIngressOutcome {
        self.session.admit_host_interaction_batch(batch)
    }

    pub(in crate::intent) fn native_host(&self) -> &worth_ui_host_egui::WorthUiHostEgui {
        self.native_host
            .as_ref()
            .expect("native host evidence requires the native world")
    }
}

impl NativeRetainedIngress {
    pub(in crate::intent) const fn adapter(
        &self,
    ) -> worth_ui_host_egui::UiEguiRawInputIngressOutcome {
        self.adapter
    }

    pub(in crate::intent) fn into_batches(self) -> Box<[UiHostObservationBatch]> {
        self.batches
    }
}

impl NativeInteractionIngress {
    pub(in crate::intent) const fn adapter(
        &self,
    ) -> worth_ui_host_egui::UiEguiRawInputIngressOutcome {
        self.adapter
    }

    pub(in crate::intent) fn into_runtime(self) -> Box<[UiHostInteractionIngressOutcome]> {
        self.runtime
    }
}
