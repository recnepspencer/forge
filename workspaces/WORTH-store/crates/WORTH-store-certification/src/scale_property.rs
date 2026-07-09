use crate::PhysicalHostileScaleFixtureReport;
use worth_store_physical_format::PhysicalOperationKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalScalePropertyEvidence {
    CounterStableAcrossUnrelatedGrowth {
        fixture: PhysicalHostileScaleFixtureReport,
    },
    FragmentedFreeSpaceBoundedOrDeferred {
        fixture: PhysicalHostileScaleFixtureReport,
    },
}

impl PhysicalScalePropertyEvidence {
    pub const fn operation(&self) -> PhysicalOperationKind {
        self.fixture().operation()
    }

    pub const fn fixture(&self) -> &PhysicalHostileScaleFixtureReport {
        match self {
            Self::CounterStableAcrossUnrelatedGrowth { fixture }
            | Self::FragmentedFreeSpaceBoundedOrDeferred { fixture } => fixture,
        }
    }

    pub fn is_satisfied(&self) -> bool {
        match self {
            Self::CounterStableAcrossUnrelatedGrowth { fixture } => {
                fixture.baseline_counters() == fixture.grown_counters()
                    && fixture.proves_unrelated_growth()
            }
            Self::FragmentedFreeSpaceBoundedOrDeferred { fixture } => fixture
                .free_space_report()
                .is_some_and(|report| report.is_admitted() || report.pressure().exceeds_policy()),
        }
    }
}
