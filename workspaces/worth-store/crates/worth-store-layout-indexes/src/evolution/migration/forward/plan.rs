use worth_store_authority::StoreCurrentAuthorityWitness;

use super::{
    LayoutBindingWitness, LayoutEvolutionDeclaration, LayoutEvolutionDenial,
    LayoutInterruptionState, LayoutPlanFingerprint, LayoutRebindRequired, LayoutVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMigrationRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    current_family: crate::AdmittedPhysicalArtifactFamily,
}

impl LayoutMigrationRequest {
    pub fn new(
        declaration: LayoutEvolutionDeclaration,
        binding: LayoutBindingWitness,
        current_family: crate::AdmittedPhysicalArtifactFamily,
    ) -> Self {
        Self {
            declaration,
            binding,
            current_family,
        }
    }

    pub(super) fn resolve(self) -> MigrationResolution {
        let declared_family = self.declaration.family().declaration();
        let binding_family = self.binding.family().declaration();
        if declared_family != binding_family {
            return MigrationResolution::Denied(Box::new(LayoutEvolutionDenial::FamilyMismatch {
                declared: declared_family,
                binding: binding_family,
            }));
        }
        let current_family = self.current_family.lifecycle().declaration();
        if declared_family != current_family {
            return MigrationResolution::Denied(Box::new(LayoutEvolutionDenial::FamilyMismatch {
                declared: declared_family,
                binding: current_family,
            }));
        }
        if !self
            .declaration
            .compatibility_window()
            .supports_read(self.binding.bound_version())
        {
            return MigrationResolution::Denied(Box::new(
                LayoutEvolutionDenial::IncompatibleSourceVersion {
                    source: self.binding.bound_version(),
                    minimum_readable: LayoutVersion::new(
                        self.declaration
                            .compatibility_window()
                            .artifact_window()
                            .minimum_readable(),
                        self.declaration.layout_version().semantic_version(),
                    ),
                    maximum_readable: LayoutVersion::new(
                        self.declaration
                            .compatibility_window()
                            .artifact_window()
                            .maximum_readable(),
                        self.declaration.layout_version().semantic_version(),
                    ),
                },
            ));
        }
        if self.binding.bound_version() != self.declaration.migration_source() {
            return MigrationResolution::Denied(Box::new(
                LayoutEvolutionDenial::UnsupportedMigrationTarget {
                    source: self.binding.bound_version(),
                    target: self.declaration.migration_target(),
                },
            ));
        }
        MigrationResolution::Resolved(Box::new(ResolvedLayoutMigrationRequest {
            declaration: self.declaration,
            binding: self.binding,
            current_family: self.current_family,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResolvedLayoutMigrationRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    current_family: crate::AdmittedPhysicalArtifactFamily,
}

impl ResolvedLayoutMigrationRequest {
    pub(super) fn lower(
        self,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> MigrationLowering {
        if self.current_family.authority_identity() != current_store_authority.authority_identity()
            || self.binding.bound_authority().identity() != current_store_authority.identity()
            || self.binding.security_identity() != self.current_family.security_identity()
        {
            return MigrationLowering::RebindRequired(Box::new(LayoutRebindRequired::new(
                self.declaration.family().declaration(),
                &self.binding,
                self.current_family,
                current_store_authority,
            )));
        }
        MigrationLowering::Lowered(Box::new(LoweredLayoutMigrationPlan {
            declaration: self.declaration,
            binding: self.binding,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LoweredLayoutMigrationPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LoweredLayoutMigrationPlan {
    pub(super) fn finish(self) -> Box<LayoutMigrationPlan> {
        let fingerprint = LayoutPlanFingerprint::for_declaration(
            self.binding.fingerprint(),
            self.binding.bound_authority().authority_identity(),
            self.declaration,
        );
        Box::new(LayoutMigrationPlan {
            declaration: self.declaration,
            binding: self.binding,
            fingerprint,
        })
    }
}

pub(super) enum MigrationResolution {
    Resolved(Box<ResolvedLayoutMigrationRequest>),
    Denied(Box<LayoutEvolutionDenial>),
}

pub(super) enum MigrationLowering {
    Lowered(Box<LoweredLayoutMigrationPlan>),
    RebindRequired(Box<LayoutRebindRequired>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMigrationPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    fingerprint: LayoutPlanFingerprint,
}

impl LayoutMigrationPlan {
    pub const fn declaration(&self) -> LayoutEvolutionDeclaration {
        self.declaration
    }

    pub const fn binding(&self) -> &LayoutBindingWitness {
        &self.binding
    }

    pub const fn fingerprint(&self) -> LayoutPlanFingerprint {
        self.fingerprint
    }

    pub const fn target_version(&self) -> LayoutVersion {
        self.declaration.migration_target()
    }

    pub fn interruption_state(&self) -> LayoutInterruptionState {
        LayoutInterruptionState::new(
            super::LayoutInterruptionFingerprint::plan(self.fingerprint),
            self.binding.clone(),
            super::LayoutInterruptionBoundary::SourceBound,
        )
    }

    pub fn resume_or_rollback(
        &self,
        interruption: LayoutInterruptionState,
    ) -> super::LayoutMigrationInterruptionOutcome {
        super::interruption::classify_migration_interruption(
            super::LayoutInterruptionFingerprint::plan(self.fingerprint),
            self.declaration,
            interruption,
        )
    }
}
