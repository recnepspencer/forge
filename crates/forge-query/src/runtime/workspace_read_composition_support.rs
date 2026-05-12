use super::{
    ForgeQueryReadCompositionPhaseGate, ForgeQueryReadCompositionPhaseOneCloseout,
    ForgeQueryReadCompositionSupportReport, ForgeQueryWorkspace,
};

impl ForgeQueryWorkspace {
    pub fn public_read_composition_support_report(&self) -> ForgeQueryReadCompositionSupportReport {
        self.runtime.public_read_composition_support_report()
    }

    pub fn public_read_composition_phase_one_closeout(
        &self,
    ) -> ForgeQueryReadCompositionPhaseOneCloseout {
        self.runtime.public_read_composition_phase_one_closeout()
    }

    pub fn public_read_composition_phase_gate(&self) -> ForgeQueryReadCompositionPhaseGate {
        self.runtime.public_read_composition_phase_gate()
    }
}
