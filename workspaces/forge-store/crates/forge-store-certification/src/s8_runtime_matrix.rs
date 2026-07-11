use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;
use forge_store_physical_certification::layout_harness::runtime::{
    S8RuntimeCase, S8RuntimeCoverageMatrix, S8RuntimeFamily,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeMatrixDenial {
    MissingExecutedStrategyCase {
        strategy: S8LayoutStrategyFamily,
        equivalence_class: S8RuntimeStrategyEquivalenceClass,
        case: S8RuntimeCase,
    },
    MissingExecutedFamilyCase {
        family: S8RuntimeFamily,
        case: S8RuntimeCase,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8RuntimeStrategyEquivalenceClass {
    CorePhysicalStructure,
    RecoveryReplayStructure,
    BlobLayoutStructure,
    MaintenanceIoStructure,
    SecurityCustodyExportStructure,
}

impl S8RuntimeStrategyEquivalenceClass {
    pub const fn for_strategy(strategy: S8LayoutStrategyFamily) -> Self {
        match strategy {
            S8LayoutStrategyFamily::HeapFile
            | S8LayoutStrategyFamily::PageTable
            | S8LayoutStrategyFamily::BaselineBTreeRange
            | S8LayoutStrategyFamily::BaselineLsmWriteOptimized
            | S8LayoutStrategyFamily::SparseIndex
            | S8LayoutStrategyFamily::BitmapAllocationMap
            | S8LayoutStrategyFamily::RangeMap => Self::CorePhysicalStructure,
            S8LayoutStrategyFamily::AppendLog => Self::RecoveryReplayStructure,
            S8LayoutStrategyFamily::ChunkTree
            | S8LayoutStrategyFamily::HashEqualityIndex
            | S8LayoutStrategyFamily::QuarantineMap => Self::BlobLayoutStructure,
            S8LayoutStrategyFamily::StreamingCursorIndex | S8LayoutStrategyFamily::ExactScan => {
                Self::MaintenanceIoStructure
            }
            S8LayoutStrategyFamily::ManifestTable => Self::SecurityCustodyExportStructure,
        }
    }

    pub const fn covering_family(self) -> S8RuntimeFamily {
        match self {
            Self::CorePhysicalStructure => S8RuntimeFamily::CorePhysical,
            Self::RecoveryReplayStructure => S8RuntimeFamily::Recovery,
            Self::BlobLayoutStructure => S8RuntimeFamily::Blob,
            Self::MaintenanceIoStructure => S8RuntimeFamily::MaintenanceIo,
            Self::SecurityCustodyExportStructure => S8RuntimeFamily::SecurityCustodyExport,
        }
    }
}

/// Courtroom-only completeness gate for Phase 33 runtime evidence.
///
/// A declared or unsupported lower capability never satisfies this gate.
pub fn require_complete_s8_runtime_matrix(
    matrix: &S8RuntimeCoverageMatrix,
) -> Result<(), S8RuntimeMatrixDenial> {
    for strategy in required_strategies() {
        for case in required_s8_runtime_cases() {
            let equivalence_class = S8RuntimeStrategyEquivalenceClass::for_strategy(strategy);
            if !matrix.is_executed(equivalence_class.covering_family(), case) {
                return Err(S8RuntimeMatrixDenial::MissingExecutedStrategyCase {
                    strategy,
                    equivalence_class,
                    case,
                });
            }
        }
    }
    for family in required_s8_runtime_families() {
        for case in required_s8_runtime_cases() {
            if !matrix.is_executed(family, case) {
                return Err(S8RuntimeMatrixDenial::MissingExecutedFamilyCase { family, case });
            }
        }
    }
    Ok(())
}

pub const fn required_s8_runtime_families() -> [S8RuntimeFamily; 5] {
    [
        S8RuntimeFamily::CorePhysical,
        S8RuntimeFamily::Recovery,
        S8RuntimeFamily::Blob,
        S8RuntimeFamily::MaintenanceIo,
        S8RuntimeFamily::SecurityCustodyExport,
    ]
}

const fn required_strategies() -> [S8LayoutStrategyFamily; 14] {
    [
        S8LayoutStrategyFamily::AppendLog,
        S8LayoutStrategyFamily::HeapFile,
        S8LayoutStrategyFamily::PageTable,
        S8LayoutStrategyFamily::BaselineBTreeRange,
        S8LayoutStrategyFamily::BaselineLsmWriteOptimized,
        S8LayoutStrategyFamily::SparseIndex,
        S8LayoutStrategyFamily::ChunkTree,
        S8LayoutStrategyFamily::ManifestTable,
        S8LayoutStrategyFamily::BitmapAllocationMap,
        S8LayoutStrategyFamily::HashEqualityIndex,
        S8LayoutStrategyFamily::RangeMap,
        S8LayoutStrategyFamily::QuarantineMap,
        S8LayoutStrategyFamily::StreamingCursorIndex,
        S8LayoutStrategyFamily::ExactScan,
    ]
}

/// Courtroom requirements are intentionally explicit: no lifecycle case may be
/// silently treated as an equivalence class by a matrix caller.
pub const fn required_s8_runtime_cases() -> [S8RuntimeCase; 10] {
    S8RuntimeCase::all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_store_physical_certification::layout_harness::runtime::S8RuntimeCoverageMatrix;
    use forge_store_physical_certification::layout_harness::runtime_execution::execute_phase33_runtime_matrix;

    #[test]
    fn metadata_without_an_execution_witness_cannot_satisfy_runtime_completeness() {
        let matrix = S8RuntimeCoverageMatrix::default();
        assert_eq!(
            require_complete_s8_runtime_matrix(&matrix),
            Err(S8RuntimeMatrixDenial::MissingExecutedStrategyCase {
                strategy: S8LayoutStrategyFamily::AppendLog,
                equivalence_class: S8RuntimeStrategyEquivalenceClass::RecoveryReplayStructure,
                case: S8RuntimeCase::Success,
            })
        );
    }

    #[test]
    fn runtime_gate_preserves_each_distinct_lifecycle_obligation() {
        let cases = required_s8_runtime_cases();
        assert_eq!(cases.len(), 10);
        assert!(cases.contains(&S8RuntimeCase::UnsupportedShapeDenial));
        assert!(cases.contains(&S8RuntimeCase::StaleRebind));
        assert!(cases.contains(&S8RuntimeCase::CorruptDerived));
        assert!(cases.contains(&S8RuntimeCase::CorruptAuthority));
        assert!(cases.contains(&S8RuntimeCase::MigrationRollback));
        assert!(cases.contains(&S8RuntimeCase::HiddenScanDenial));
    }

    #[test]
    fn strategy_equivalence_classes_are_explicit_courtroom_law() {
        assert_eq!(
            S8RuntimeStrategyEquivalenceClass::for_strategy(S8LayoutStrategyFamily::ChunkTree)
                .covering_family(),
            S8RuntimeFamily::Blob
        );
        assert_eq!(
            S8RuntimeStrategyEquivalenceClass::for_strategy(S8LayoutStrategyFamily::ExactScan)
                .covering_family(),
            S8RuntimeFamily::MaintenanceIo
        );
    }

    #[test]
    fn owner_executed_phase33_matrix_satisfies_closeout_gate() {
        let matrix = execute_phase33_runtime_matrix().unwrap();
        require_complete_s8_runtime_matrix(&matrix).unwrap();
    }
}
