#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum S8RuntimeFamily {
    CorePhysical,
    Recovery,
    Blob,
    MaintenanceIo,
    SecurityCustodyExport,
}

pub use forge_store_contracts::{S8RuntimeCase, S8RuntimeOutcome, S8RuntimeOwnerFact};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeSimulationDenial {
    DuplicateFamilyCase,
    OutcomeDoesNotSatisfyCase,
    UnsupportedExecutedScenario,
    FamilyDoesNotMatchExecutedStrategy,
    CaseDoesNotMatchExecutedScenario,
}

/// An exhaustive union of sealed, lower-owner execution receipts.  This is
/// the matrix row: certification may inspect it but cannot construct a second
/// adopted execution witness from it.
#[derive(Debug, PartialEq, Eq)]
pub enum S8RuntimeEvidence {
    PlatformPhysical(PlatformPhysicalRuntimeReceipt),
    Recovery(forge_store_recovery_physics::S8RecoveryRuntimeReceipt),
    Blob(forge_store_blob_chunks::S8BlobRuntimeReceipt),
    MaintenanceIo(forge_store_io_scheduler::S8MaintenanceIoRuntimeReceipt),
    SecurityCustodyExport(forge_store_operations::S8SecurityCustodyExportRuntimeReceipt),
}

impl S8RuntimeEvidence {
    pub fn family(&self) -> S8RuntimeFamily {
        match self {
            Self::PlatformPhysical(_) => S8RuntimeFamily::CorePhysical,
            Self::Recovery(_) => S8RuntimeFamily::Recovery,
            Self::Blob(_) => S8RuntimeFamily::Blob,
            Self::MaintenanceIo(_) => S8RuntimeFamily::MaintenanceIo,
            Self::SecurityCustodyExport(_) => S8RuntimeFamily::SecurityCustodyExport,
        }
    }

    pub fn case(&self) -> S8RuntimeCase {
        self.fact().case()
    }

    pub fn fact(&self) -> S8RuntimeOwnerFact {
        match self {
            Self::PlatformPhysical(receipt) => receipt.fact(),
            Self::Recovery(receipt) => receipt.fact(),
            Self::Blob(receipt) => receipt.fact(),
            Self::MaintenanceIo(receipt) => receipt.fact(),
            Self::SecurityCustodyExport(receipt) => receipt.fact(),
        }
    }

    pub fn strategy(&self) -> S8LayoutStrategyFamily {
        match self {
            Self::PlatformPhysical(receipt) => match receipt.strategy() {
                PlatformPhysicalRuntimeStrategy::BaselineBTreeRange => {
                    S8LayoutStrategyFamily::BaselineBTreeRange
                }
            },
            Self::Recovery(receipt) => match receipt.strategy() {
                forge_store_recovery_physics::S8RecoveryRuntimeStrategy::AppendLog => {
                    S8LayoutStrategyFamily::AppendLog
                }
            },
            Self::Blob(receipt) => receipt.strategy(),
            Self::MaintenanceIo(receipt) => receipt.strategy(),
            Self::SecurityCustodyExport(receipt) => receipt.strategy(),
        }
    }

    pub fn outcome(&self) -> S8RuntimeOutcome {
        self.fact().outcome()
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct S8RuntimeCoverageMatrix {
    /// The matrix retains the full sealed witness. It must never reduce a
    /// family execution to a label-only coverage claim.
    rows: Vec<S8RuntimeEvidence>,
}

impl S8RuntimeCoverageMatrix {
    pub fn record(
        &mut self,
        execution: S8RuntimeEvidence,
    ) -> Result<(), S8RuntimeSimulationDenial> {
        if !execution.fact().is_coherent() {
            return Err(S8RuntimeSimulationDenial::OutcomeDoesNotSatisfyCase);
        }
        let strategy = execution.strategy();
        let case = execution.case();
        // A strategy is shared grammar, not runtime authority.  Two owners
        // may execute the same admitted strategy and lifecycle independently;
        // collapsing them would make the five-family completion gate
        // unachievable and would replace owner evidence with a representative
        // row.  Only a second receipt for this exact owner execution is a
        // duplicate.
        if self.rows.iter().any(|row| {
            row.family() == execution.family() && row.strategy() == strategy && row.case() == case
        }) {
            return Err(S8RuntimeSimulationDenial::DuplicateFamilyCase);
        }
        self.rows.push(execution);
        Ok(())
    }

    /// Evidence-bearing rows, not a copied coverage projection.
    pub fn rows(&self) -> &[S8RuntimeEvidence] {
        &self.rows
    }

    pub fn is_executed(&self, family: S8RuntimeFamily, case: S8RuntimeCase) -> bool {
        self.rows
            .iter()
            .any(|row| row.family() == family && row.case() == case)
    }

    pub fn is_strategy_executed(
        &self,
        strategy: S8LayoutStrategyFamily,
        case: S8RuntimeCase,
    ) -> bool {
        self.rows
            .iter()
            .any(|row| row.strategy() == strategy && row.case() == case)
    }
}

use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;
use forge_store_physical_format::{
    PlatformPhysicalRuntimeReceipt, PlatformPhysicalRuntimeStrategy,
};
