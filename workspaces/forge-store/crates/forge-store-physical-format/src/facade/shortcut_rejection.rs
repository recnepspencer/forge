use crate::PhysicalShortcutBoundaryDenial;

use super::denials::{PlatformPhysicalFacadeDenial, PlatformPhysicalFacadeDenialKind};
use super::PlatformPhysicalFacade;

fn assemble_shortcut_denial(
    kind: PlatformPhysicalFacadeDenialKind,
    shortcut: PhysicalShortcutBoundaryDenial,
) -> PlatformPhysicalFacadeDenial {
    PlatformPhysicalFacadeDenial::new(kind).with_shortcut_denial(shortcut)
}

impl PlatformPhysicalFacade {
    pub fn reject_full_store_heap_materialization(
        &mut self,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        self.counters = self
            .counters
            .with_full_store_materialization_rejection();
        Err(assemble_shortcut_denial(
            PlatformPhysicalFacadeDenialKind::FullStoreMaterializationRejected,
            PhysicalShortcutBoundaryDenial::full_store_heap_materialization(),
        ))
    }

    pub fn reject_backend_residue_guess(&mut self) -> Result<(), PlatformPhysicalFacadeDenial> {
        self.counters = self.counters.with_backend_residue_guess_rejection();
        Err(assemble_shortcut_denial(
            PlatformPhysicalFacadeDenialKind::BackendResidueGuessRejected,
            PhysicalShortcutBoundaryDenial::backend_residue_guessing(),
        ))
    }

    pub fn reject_live_runtime_cache_shortcut(
        &mut self,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        Err(assemble_shortcut_denial(
            PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::live_runtime_cache(),
        ))
    }

    pub fn reject_backend_private_map_shortcut(
        &mut self,
    ) -> Result<(), PlatformPhysicalFacadeDenial> {
        Err(assemble_shortcut_denial(
            PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::backend_private_map(),
        ))
    }

    pub fn reject_raw_debug_dump_shortcut(&mut self) -> Result<(), PlatformPhysicalFacadeDenial> {
        Err(assemble_shortcut_denial(
            PlatformPhysicalFacadeDenialKind::ShortcutBoundaryRejected,
            PhysicalShortcutBoundaryDenial::raw_debug_dump(),
        ))
    }
}