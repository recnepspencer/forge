use core::convert::Infallible;

use forge_proof::TransitionOutcome;
use forge_store_authority::StoreCurrentAuthorityWitness;

use super::{
    LayoutBindingWitness, LayoutEvolutionDeclaration, LayoutEvolutionDenial,
    LayoutInterruptedMigrationDisposition, LayoutInterruptionPolicy, LayoutInterruptionState,
    LayoutRollbackRequest, LayoutVersion, S8LayoutRebindRequired, S8LayoutStaleBinding,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LayoutPlanFingerprint {
    family_id: forge_store_contracts::DurableArtifactFamilyId,
    source: LayoutVersion,
    target: LayoutVersion,
    rollback_source: LayoutVersion,
    rollback_target: LayoutVersion,
    interruption_policy: LayoutInterruptionPolicy,
}

impl LayoutPlanFingerprint {
    pub(crate) const fn new(
        family_id: forge_store_contracts::DurableArtifactFamilyId,
        source: LayoutVersion,
        target: LayoutVersion,
        rollback_source: LayoutVersion,
        rollback_target: LayoutVersion,
        interruption_policy: LayoutInterruptionPolicy,
    ) -> Self {
        Self {
            family_id,
            source,
            target,
            rollback_source,
            rollback_target,
            interruption_policy,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMigrationRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LayoutMigrationRequest {
    pub fn new(declaration: LayoutEvolutionDeclaration, binding: LayoutBindingWitness) -> Self {
        Self {
            declaration,
            binding,
        }
    }

    pub fn try_resolve_ready(
        self,
    ) -> TransitionOutcome<ResolvedLayoutMigrationRequest, LayoutEvolutionDenial> {
        let declared_family = self.declaration.family().declaration();
        let binding_family = self.binding.family().declaration();
        if declared_family != binding_family {
            return TransitionOutcome::denied(LayoutEvolutionDenial::FamilyMismatch {
                declared: declared_family,
                binding: binding_family,
            });
        }
        if !self
            .declaration
            .compatibility_window()
            .supports_read(self.binding.bound_version())
        {
            return TransitionOutcome::denied(LayoutEvolutionDenial::IncompatibleSourceVersion {
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
            });
        }
        if self.binding.bound_version() != self.declaration.migration_source() {
            return TransitionOutcome::denied(LayoutEvolutionDenial::UnsupportedMigrationTarget {
                source: self.binding.bound_version(),
                target: self.declaration.migration_target(),
            });
        }
        TransitionOutcome::success(ResolvedLayoutMigrationRequest {
            declaration: self.declaration,
            binding: self.binding,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLayoutMigrationRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl ResolvedLayoutMigrationRequest {
    pub fn try_lower_ready(
        self,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> TransitionOutcome<
        LoweredLayoutMigrationPlan,
        LayoutEvolutionDenial,
        Infallible,
        Infallible,
        S8LayoutRebindRequired,
    > {
        if self.binding.bound_authority().identity() != current_store_authority.identity() {
            return TransitionOutcome::rebind_required(S8LayoutRebindRequired::new(
                self.declaration.family().declaration(),
                self.binding.bound_authority(),
            ));
        }
        TransitionOutcome::success(LoweredLayoutMigrationPlan {
            declaration: self.declaration,
            binding: self.binding,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredLayoutMigrationPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LoweredLayoutMigrationPlan {
    pub fn try_ready_now(
        self,
    ) -> TransitionOutcome<
        LayoutMigrationPlan,
        LayoutEvolutionDenial,
        Infallible,
        S8LayoutStaleBinding,
        S8LayoutRebindRequired,
    > {
        if self.binding.bound_version() != self.binding.observed_version() {
            return TransitionOutcome::stale(S8LayoutStaleBinding::new(
                self.declaration.family().declaration(),
                self.binding.bound_version(),
                self.binding.observed_version(),
            ));
        }

        TransitionOutcome::success(LayoutMigrationPlan {
            declaration: self.declaration,
            binding: self.binding,
            fingerprint: LayoutPlanFingerprint::new(
                self.declaration.family().family_id(),
                self.declaration.migration_source(),
                self.declaration.migration_target(),
                self.declaration.rollback_source(),
                self.declaration.rollback_target(),
                self.declaration.interruption_policy(),
            ),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutMigrationPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    fingerprint: LayoutPlanFingerprint,
}

pub type LayoutMigrationOutcome = TransitionOutcome<
    LayoutMigrationPlan,
    LayoutEvolutionDenial,
    Infallible,
    S8LayoutStaleBinding,
    S8LayoutRebindRequired,
>;

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

    pub const fn interruption_state(&self) -> LayoutInterruptionState {
        LayoutInterruptionState::new(self.fingerprint)
    }

    pub fn resume_or_rollback(
        &self,
        interruption: LayoutInterruptionState,
    ) -> TransitionOutcome<LayoutInterruptedMigrationDisposition, LayoutEvolutionDenial> {
        if interruption.fingerprint() != self.fingerprint {
            return TransitionOutcome::denied(
                LayoutEvolutionDenial::InterruptStateDoesNotMatchPlan {
                    expected: self.fingerprint,
                    actual: interruption.fingerprint(),
                },
            );
        }

        match self.declaration.interruption_policy() {
            LayoutInterruptionPolicy::ResumeDeclaredMigration => TransitionOutcome::success(
                LayoutInterruptedMigrationDisposition::Resume(interruption),
            ),
            LayoutInterruptionPolicy::RollbackDeclaredMigration => {
                TransitionOutcome::success(LayoutInterruptedMigrationDisposition::Rollback(
                    LayoutRollbackRequest::new(self.declaration, self.binding.clone()),
                ))
            }
        }
    }
}
