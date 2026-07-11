use super::{
    bypass::LegacyAccessPathBypass, disposition::LegacySurfaceDisposition,
    surface_row::LegacySurfaceInventoryRow,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacyAccessPathBypassInventory {
    rows: &'static [LegacySurfaceInventoryRow],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LegacySurfaceDispositionOutcome {
    disposition: LegacySurfaceDisposition,
}

impl LegacySurfaceDispositionOutcome {
    pub const fn disposition(self) -> LegacySurfaceDisposition {
        self.disposition
    }

    pub const fn production_transition(
        self,
    ) -> crate::production_transition::S8LayoutProductionTransition {
        Self::classified_transition()
    }

    const fn classified_transition() -> crate::production_transition::S8LayoutProductionTransition {
        crate::production_transition::owner_transition(
            crate::production_transition::S8LayoutStateMachine::LegacyDisposition,
            crate::production_transition::S8LayoutProductionOperation::ClassifyLegacyDisposition,
            "Classified",
            crate::production_transition::S8LayoutMachineState::Unclassified,
            crate::production_transition::S8LayoutMachineTransition::Classify,
            crate::production_transition::S8LayoutMachineState::Admitted,
        )
    }

    pub(crate) fn owner_transition_contract(
    ) -> crate::production_transition::S8OwnerTransitionContract {
        static FACTS: [crate::production_transition::S8LayoutProductionTransition; 1] =
            [LegacySurfaceDispositionOutcome::classified_transition()];
        crate::production_transition::S8OwnerTransitionContract::from_owner_outcomes(
            crate::production_transition::S8LayoutStateMachine::LegacyDisposition,
            crate::production_transition::S8LayoutProductionOperation::ClassifyLegacyDisposition,
            &FACTS,
        )
    }
}

impl PartialEq<LegacySurfaceDisposition> for LegacySurfaceDispositionOutcome {
    fn eq(&self, other: &LegacySurfaceDisposition) -> bool {
        self.disposition == *other
    }
}

impl LegacyAccessPathBypassInventory {
    pub const fn new(rows: &'static [LegacySurfaceInventoryRow]) -> Self {
        Self { rows }
    }

    pub const fn rows(self) -> &'static [LegacySurfaceInventoryRow] {
        self.rows
    }

    pub fn disposition_for(self, surface: &str) -> LegacySurfaceDispositionOutcome {
        let disposition = self
            .rows
            .iter()
            .find(|row| row.surface() == surface)
            .unwrap_or_else(|| panic!("missing legacy surface disposition for {surface}"))
            .disposition();
        LegacySurfaceDispositionOutcome { disposition }
    }

    pub fn bypass_for(self, surface: &str) -> LegacyAccessPathBypass {
        self.rows
            .iter()
            .find(|row| row.surface() == surface)
            .unwrap_or_else(|| panic!("missing legacy surface bypass posture for {surface}"))
            .bypass()
    }
}

pub(crate) use super::rows::legacy_surface_rows;
