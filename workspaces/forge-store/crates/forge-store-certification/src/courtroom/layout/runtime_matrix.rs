use forge_store_layout_indexes::layout_strategy_admission::S8LayoutStrategyFamily;
use forge_store_physical_certification::layout_harness::runtime::{
    LayoutRuntimeCoverageMatrix, LayoutRuntimeFamily, LayoutRuntimeObligation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRuntimeCompletenessDenial {
    MissingExecutedStrategyCase {
        strategy: S8LayoutStrategyFamily,
        equivalence_class: LayoutRuntimeStrategyEquivalenceClass,
        case: LayoutRuntimeObligation,
    },
    MissingExecutedFamilyCase {
        family: LayoutRuntimeFamily,
        case: LayoutRuntimeObligation,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutRuntimeStrategyEquivalenceClass {
    CorePhysicalStructure,
    RecoveryReplayStructure,
    BlobLayoutStructure,
    MaintenanceIoStructure,
    SecurityCustodyExportStructure,
}

impl LayoutRuntimeStrategyEquivalenceClass {
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

    pub const fn covering_family(self) -> LayoutRuntimeFamily {
        match self {
            Self::CorePhysicalStructure => LayoutRuntimeFamily::CorePhysical,
            Self::RecoveryReplayStructure => LayoutRuntimeFamily::Recovery,
            Self::BlobLayoutStructure => LayoutRuntimeFamily::Blob,
            Self::MaintenanceIoStructure => LayoutRuntimeFamily::MaintenanceIo,
            Self::SecurityCustodyExportStructure => LayoutRuntimeFamily::SecurityCustodyExport,
        }
    }
}

/// Courtroom-only completeness gate for layout runtime evidence.
///
/// A declared or unsupported lower capability never satisfies this gate.
pub fn require_complete_layout_runtime_matrix(
    matrix: &LayoutRuntimeCoverageMatrix,
) -> Result<(), LayoutRuntimeCompletenessDenial> {
    for strategy in required_strategies() {
        for case in required_layout_runtime_obligations() {
            let equivalence_class = LayoutRuntimeStrategyEquivalenceClass::for_strategy(strategy);
            if !matrix.is_executed(equivalence_class.covering_family(), case) {
                return Err(
                    LayoutRuntimeCompletenessDenial::MissingExecutedStrategyCase {
                        strategy,
                        equivalence_class,
                        case,
                    },
                );
            }
        }
    }
    for family in required_layout_runtime_families() {
        for case in required_layout_runtime_obligations() {
            if !matrix.is_executed(family, case) {
                return Err(LayoutRuntimeCompletenessDenial::MissingExecutedFamilyCase {
                    family,
                    case,
                });
            }
        }
    }
    Ok(())
}

pub const fn required_layout_runtime_families() -> [LayoutRuntimeFamily; 5] {
    [
        LayoutRuntimeFamily::CorePhysical,
        LayoutRuntimeFamily::Recovery,
        LayoutRuntimeFamily::Blob,
        LayoutRuntimeFamily::MaintenanceIo,
        LayoutRuntimeFamily::SecurityCustodyExport,
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
pub const fn required_layout_runtime_obligations() -> [LayoutRuntimeObligation; 10] {
    LayoutRuntimeObligation::all()
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_store_physical_certification::layout_harness::runtime::LayoutRuntimeCoverageMatrix;
    use forge_store_physical_certification::layout_harness::runtime_execution::execute_layout_runtime_observations;

    #[test]
    fn metadata_without_an_execution_witness_cannot_satisfy_runtime_completeness() {
        let matrix = LayoutRuntimeCoverageMatrix::default();
        assert_eq!(
            require_complete_layout_runtime_matrix(&matrix),
            Err(
                LayoutRuntimeCompletenessDenial::MissingExecutedStrategyCase {
                    strategy: S8LayoutStrategyFamily::AppendLog,
                    equivalence_class:
                        LayoutRuntimeStrategyEquivalenceClass::RecoveryReplayStructure,
                    case: LayoutRuntimeObligation::Success,
                }
            )
        );
    }

    #[test]
    fn runtime_gate_preserves_each_distinct_lifecycle_obligation() {
        let cases = required_layout_runtime_obligations();
        assert_eq!(cases.len(), 10);
        assert!(cases.contains(&LayoutRuntimeObligation::UnsupportedShapeDenial));
        assert!(cases.contains(&LayoutRuntimeObligation::StaleRebind));
        assert!(cases.contains(&LayoutRuntimeObligation::CorruptDerived));
        assert!(cases.contains(&LayoutRuntimeObligation::CorruptAuthority));
        assert!(cases.contains(&LayoutRuntimeObligation::MigrationRollback));
        assert!(cases.contains(&LayoutRuntimeObligation::HiddenScanDenial));
    }

    #[test]
    fn strategy_equivalence_classes_are_explicit_courtroom_law() {
        assert_eq!(
            LayoutRuntimeStrategyEquivalenceClass::for_strategy(S8LayoutStrategyFamily::ChunkTree)
                .covering_family(),
            LayoutRuntimeFamily::Blob
        );
        assert_eq!(
            LayoutRuntimeStrategyEquivalenceClass::for_strategy(S8LayoutStrategyFamily::ExactScan)
                .covering_family(),
            LayoutRuntimeFamily::MaintenanceIo
        );
    }

    #[test]
    fn observed_matrix_reports_unimplemented_runtime_obligations() {
        let matrix = execute_layout_runtime_observations().unwrap();
        assert_eq!(
            require_complete_layout_runtime_matrix(&matrix),
            Err(
                LayoutRuntimeCompletenessDenial::MissingExecutedStrategyCase {
                    strategy: S8LayoutStrategyFamily::AppendLog,
                    equivalence_class:
                        LayoutRuntimeStrategyEquivalenceClass::RecoveryReplayStructure,
                    case: LayoutRuntimeObligation::Success,
                }
            )
        );
    }
}
