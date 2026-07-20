use crate::PhysicalShortcutBoundaryDenial;

use super::denials::{InMemoryPhysicalFormatModelDenial, InMemoryPhysicalFormatModelDenialKind};
use super::InMemoryPhysicalFormatModel;

fn assemble_shortcut_denial(
    kind: InMemoryPhysicalFormatModelDenialKind,
    shortcut: PhysicalShortcutBoundaryDenial,
) -> InMemoryPhysicalFormatModelDenial {
    InMemoryPhysicalFormatModelDenial::new(kind).with_shortcut_denial(shortcut)
}

impl InMemoryPhysicalFormatModel {
    pub fn reject_full_store_heap_materialization(
        &mut self,
    ) -> Result<(), InMemoryPhysicalFormatModelDenial> {
        self.counters = self.counters.with_full_store_materialization_rejection();
        Err(assemble_shortcut_denial(
            InMemoryPhysicalFormatModelDenialKind::FullStoreMaterializationRejected,
            PhysicalShortcutBoundaryDenial::full_store_heap_materialization(),
        ))
    }

    pub fn reject_backend_residue_guess(
        &mut self,
    ) -> Result<(), InMemoryPhysicalFormatModelDenial> {
        self.counters = self.counters.with_backend_residue_guess_rejection();
        Err(assemble_shortcut_denial(
            InMemoryPhysicalFormatModelDenialKind::BackendResidueGuessRejected,
            PhysicalShortcutBoundaryDenial::backend_residue_guessing(),
        ))
    }

    pub fn reject_live_runtime_cache_shortcut(
        &mut self,
    ) -> Result<(), InMemoryPhysicalFormatModelDenial> {
        Err(assemble_shortcut_denial(
            InMemoryPhysicalFormatModelDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::live_runtime_cache(),
        ))
    }

    pub fn reject_backend_private_map_shortcut(
        &mut self,
    ) -> Result<(), InMemoryPhysicalFormatModelDenial> {
        Err(assemble_shortcut_denial(
            InMemoryPhysicalFormatModelDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::backend_private_map(),
        ))
    }

    pub fn reject_raw_debug_dump_shortcut(
        &mut self,
    ) -> Result<(), InMemoryPhysicalFormatModelDenial> {
        Err(assemble_shortcut_denial(
            InMemoryPhysicalFormatModelDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::raw_debug_dump(),
        ))
    }
}
