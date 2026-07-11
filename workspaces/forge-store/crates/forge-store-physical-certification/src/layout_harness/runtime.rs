use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;
use forge_store_physical_format::{
    PlatformPhysicalRuntimeOperation, PlatformPhysicalRuntimeReceipt,
    PlatformPhysicalRuntimeStrategy,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutRuntimeFamily {
    CorePhysical,
    Recovery,
    Blob,
    MaintenanceIo,
    SecurityCustodyExport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayoutRuntimeObligation {
    Success,
    UnsupportedShapeDenial,
    StaleRebind,
    CorruptDerived,
    CorruptAuthority,
    Rebuild,
    MigrationRollback,
    HiddenScanDenial,
    Readmission,
    CostEnvelope,
}

impl LayoutRuntimeObligation {
    pub const fn all() -> [Self; 10] {
        [
            Self::Success,
            Self::UnsupportedShapeDenial,
            Self::StaleRebind,
            Self::CorruptDerived,
            Self::CorruptAuthority,
            Self::Rebuild,
            Self::MigrationRollback,
            Self::HiddenScanDenial,
            Self::Readmission,
            Self::CostEnvelope,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRuntimeRecordDenial {
    DuplicateOwnerOperation,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LayoutRuntimeEvidence {
    PlatformPhysical(PlatformPhysicalRuntimeReceipt),
}

impl LayoutRuntimeEvidence {
    pub const fn family(&self) -> LayoutRuntimeFamily {
        match self {
            Self::PlatformPhysical(_) => LayoutRuntimeFamily::CorePhysical,
        }
    }

    pub const fn strategy(&self) -> S8LayoutStrategyFamily {
        match self {
            Self::PlatformPhysical(receipt) => match receipt.strategy() {
                PlatformPhysicalRuntimeStrategy::BaselineBTreeRange => {
                    S8LayoutStrategyFamily::BaselineBTreeRange
                }
            },
        }
    }

    pub const fn operation(&self) -> PlatformPhysicalRuntimeOperation {
        match self {
            Self::PlatformPhysical(receipt) => receipt.operation(),
        }
    }

    pub const fn satisfies(&self, obligation: LayoutRuntimeObligation) -> bool {
        matches!(
            (self.operation(), obligation),
            (
                PlatformPhysicalRuntimeOperation::AppendPhysicalRecord,
                LayoutRuntimeObligation::Success
            ) | (
                PlatformPhysicalRuntimeOperation::DenyHiddenBroadScan,
                LayoutRuntimeObligation::HiddenScanDenial
            )
        )
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LayoutRuntimeCoverageMatrix {
    rows: Vec<LayoutRuntimeEvidence>,
}

impl LayoutRuntimeCoverageMatrix {
    pub fn record(
        &mut self,
        execution: LayoutRuntimeEvidence,
    ) -> Result<(), LayoutRuntimeRecordDenial> {
        if self.rows.iter().any(|row| {
            row.family() == execution.family() && row.operation() == execution.operation()
        }) {
            return Err(LayoutRuntimeRecordDenial::DuplicateOwnerOperation);
        }
        self.rows.push(execution);
        Ok(())
    }

    pub fn rows(&self) -> &[LayoutRuntimeEvidence] {
        &self.rows
    }

    pub fn is_executed(
        &self,
        family: LayoutRuntimeFamily,
        obligation: LayoutRuntimeObligation,
    ) -> bool {
        self.rows
            .iter()
            .any(|row| row.family() == family && row.satisfies(obligation))
    }

    pub fn is_strategy_executed(
        &self,
        strategy: S8LayoutStrategyFamily,
        obligation: LayoutRuntimeObligation,
    ) -> bool {
        self.rows
            .iter()
            .any(|row| row.strategy() == strategy && row.satisfies(obligation))
    }
}
