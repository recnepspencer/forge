use crate::PhysicalShortcutBoundaryDenial;

use super::denials::{PhysicalStoreRuntimeDenial, PhysicalStoreRuntimeDenialKind};
use super::PhysicalStoreRuntime;

fn assemble_shortcut_denial(
    kind: PhysicalStoreRuntimeDenialKind,
    shortcut: PhysicalShortcutBoundaryDenial,
) -> PhysicalStoreRuntimeDenial {
    PhysicalStoreRuntimeDenial::new(kind).with_shortcut_denial(shortcut)
}

impl PhysicalStoreRuntime {
    pub fn reject_full_store_heap_materialization(
        &mut self,
    ) -> Result<(), PhysicalStoreRuntimeDenial> {
        self.counters = self.counters.with_full_store_materialization_rejection();
        Err(assemble_shortcut_denial(
            PhysicalStoreRuntimeDenialKind::FullStoreMaterializationRejected,
            PhysicalShortcutBoundaryDenial::full_store_heap_materialization(),
        ))
    }

    pub fn reject_backend_residue_guess(&mut self) -> Result<(), PhysicalStoreRuntimeDenial> {
        self.counters = self.counters.with_backend_residue_guess_rejection();
        Err(assemble_shortcut_denial(
            PhysicalStoreRuntimeDenialKind::BackendResidueGuessRejected,
            PhysicalShortcutBoundaryDenial::backend_residue_guessing(),
        ))
    }

    pub fn reject_live_runtime_cache_shortcut(&mut self) -> Result<(), PhysicalStoreRuntimeDenial> {
        Err(assemble_shortcut_denial(
            PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::live_runtime_cache(),
        ))
    }

    pub fn reject_backend_private_map_shortcut(
        &mut self,
    ) -> Result<(), PhysicalStoreRuntimeDenial> {
        Err(assemble_shortcut_denial(
            PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::backend_private_map(),
        ))
    }

    pub fn reject_raw_debug_dump_shortcut(&mut self) -> Result<(), PhysicalStoreRuntimeDenial> {
        Err(assemble_shortcut_denial(
            PhysicalStoreRuntimeDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::raw_debug_dump(),
        ))
    }
}
