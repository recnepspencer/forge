use super::{
    binding::LayoutBindingFingerprint, LayoutEvolutionDeclaration, LayoutInterruptionPolicy,
    LayoutVersion,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutPlanFingerprint {
    binding: LayoutBindingFingerprint,
    family_id: worth_store_contracts::DurableArtifactFamilyId,
    store_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
    source: LayoutVersion,
    target: LayoutVersion,
    rollback_source: LayoutVersion,
    rollback_target: LayoutVersion,
    interruption_policy: LayoutInterruptionPolicy,
}

impl LayoutPlanFingerprint {
    pub(super) const fn for_declaration(
        binding: LayoutBindingFingerprint,
        store_authority: worth_store_authority::StoreCurrentAuthorityIdentity,
        declaration: LayoutEvolutionDeclaration,
    ) -> Self {
        Self {
            binding,
            family_id: declaration.family().family_id(),
            store_authority,
            source: declaration.migration_source(),
            target: declaration.migration_target(),
            rollback_source: declaration.rollback_source(),
            rollback_target: declaration.rollback_target(),
            interruption_policy: declaration.interruption_policy(),
        }
    }
}
