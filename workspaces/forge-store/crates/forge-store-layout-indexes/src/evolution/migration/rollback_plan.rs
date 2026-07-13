use core::convert::Infallible;

use forge_proof::TransitionOutcome;
use forge_store_authority::StoreCurrentAuthorityWitness;

use super::{
    LayoutBindingWitness, LayoutEvolutionDeclaration, LayoutEvolutionDenial, LayoutRebindRequired,
    LayoutStaleBinding, LayoutVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRollbackRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LayoutRollbackRequest {
    pub fn new(declaration: LayoutEvolutionDeclaration, binding: LayoutBindingWitness) -> Self {
        Self {
            declaration,
            binding,
        }
    }

    pub(crate) fn try_resolve_ready(
        self,
    ) -> TransitionOutcome<ResolvedLayoutRollbackRequest, LayoutEvolutionDenial> {
        let declared_family = self.declaration.family().declaration();
        let binding_family = self.binding.family().declaration();
        if declared_family != binding_family {
            return TransitionOutcome::denied(LayoutEvolutionDenial::FamilyMismatch {
                declared: declared_family,
                binding: binding_family,
            });
        }
        if self.binding.observed_version() != self.declaration.rollback_source() {
            return TransitionOutcome::denied(LayoutEvolutionDenial::UnsupportedRollbackTarget {
                source: self.binding.observed_version(),
                target: self.declaration.rollback_target(),
            });
        }
        TransitionOutcome::success(ResolvedLayoutRollbackRequest {
            declaration: self.declaration,
            binding: self.binding,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedLayoutRollbackRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl ResolvedLayoutRollbackRequest {
    pub(crate) fn try_lower_ready(
        self,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> TransitionOutcome<
        LoweredLayoutRollbackPlan,
        LayoutEvolutionDenial,
        Infallible,
        Infallible,
        LayoutRebindRequired,
    > {
        if self.binding.bound_authority().identity() != current_store_authority.identity() {
            return TransitionOutcome::rebind_required(LayoutRebindRequired::new(
                self.declaration.family().declaration(),
                self.binding.bound_authority(),
            ));
        }

        TransitionOutcome::success(LoweredLayoutRollbackPlan {
            declaration: self.declaration,
            binding: self.binding,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoweredLayoutRollbackPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LoweredLayoutRollbackPlan {
    pub(crate) fn try_ready_now(
        self,
    ) -> TransitionOutcome<
        LayoutRollbackPlan,
        LayoutEvolutionDenial,
        Infallible,
        LayoutStaleBinding,
        Infallible,
    > {
        if self.binding.bound_version() != self.binding.observed_version() {
            return TransitionOutcome::stale(LayoutStaleBinding::new(
                self.declaration.family().declaration(),
                self.binding.bound_version(),
                self.binding.observed_version(),
            ));
        }

        TransitionOutcome::success(LayoutRollbackPlan {
            declaration: self.declaration,
            binding: self.binding,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRollbackPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LayoutRollbackPlan {
    pub const fn rollback_target(&self) -> LayoutVersion {
        self.declaration.rollback_target()
    }

    pub const fn declaration(&self) -> LayoutEvolutionDeclaration {
        self.declaration
    }

    pub const fn authority(&self) -> &StoreCurrentAuthorityWitness {
        self.binding.bound_authority()
    }
}
