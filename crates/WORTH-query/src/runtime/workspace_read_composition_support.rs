use super::{
    WorthQueryReadCompositionPhaseGate, WorthQueryReadCompositionPhaseOneCloseout,
    WorthQueryReadCompositionSupportReport, WorthQueryWorkspace,
};

impl WorthQueryWorkspace {
    pub fn public_read_composition_support_report(&self) -> WorthQueryReadCompositionSupportReport {
        self.runtime.public_read_composition_support_report()
    }

    pub fn public_read_composition_phase_one_closeout(
        &self,
    ) -> WorthQueryReadCompositionPhaseOneCloseout {
        self.runtime.public_read_composition_phase_one_closeout()
    }

    pub fn public_read_composition_phase_gate(&self) -> WorthQueryReadCompositionPhaseGate {
        self.runtime.public_read_composition_phase_gate()
    }
}
