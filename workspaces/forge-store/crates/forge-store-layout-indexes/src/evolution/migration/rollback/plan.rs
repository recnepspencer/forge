use forge_store_authority::StoreCurrentAuthorityWitness;

use super::{
    LayoutBindingWitness, LayoutEvolutionDeclaration, LayoutEvolutionDenial, LayoutRebindRequired,
    LayoutVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRollbackRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    current_family: crate::AdmittedPhysicalArtifactFamily,
}

impl LayoutRollbackRequest {
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

    pub(super) fn resolve(self) -> RollbackResolution {
        let declared_family = self.declaration.family().declaration();
        let binding_family = self.binding.family().declaration();
        if declared_family != binding_family {
            return RollbackResolution::Denied(Box::new(LayoutEvolutionDenial::FamilyMismatch {
                declared: declared_family,
                binding: binding_family,
            }));
        }
        let current_family = self.current_family.lifecycle().declaration();
        if declared_family != current_family {
            return RollbackResolution::Denied(Box::new(LayoutEvolutionDenial::FamilyMismatch {
                declared: declared_family,
                binding: current_family,
            }));
        }
        if self.binding.observed_version() != self.declaration.rollback_source() {
            return RollbackResolution::Denied(Box::new(
                LayoutEvolutionDenial::UnsupportedRollbackTarget {
                    source: self.binding.observed_version(),
                    target: self.declaration.rollback_target(),
                },
            ));
        }
        RollbackResolution::Resolved(Box::new(ResolvedLayoutRollbackRequest {
            declaration: self.declaration,
            binding: self.binding,
            current_family: self.current_family,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResolvedLayoutRollbackRequest {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    current_family: crate::AdmittedPhysicalArtifactFamily,
}

impl ResolvedLayoutRollbackRequest {
    pub(super) fn lower(
        self,
        current_store_authority: &StoreCurrentAuthorityWitness,
    ) -> RollbackLowering {
        if self.current_family.authority_identity() != current_store_authority.authority_identity()
            || self.binding.bound_authority().identity() != current_store_authority.identity()
            || self.binding.security_identity() != self.current_family.security_identity()
        {
            return RollbackLowering::RebindRequired(Box::new(LayoutRebindRequired::new(
                self.declaration.family().declaration(),
                &self.binding,
                self.current_family,
                current_store_authority,
            )));
        }

        RollbackLowering::Lowered(Box::new(LoweredLayoutRollbackPlan {
            declaration: self.declaration,
            binding: self.binding,
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LoweredLayoutRollbackPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
}

impl LoweredLayoutRollbackPlan {
    pub(super) fn finish(self) -> Box<LayoutRollbackPlan> {
        let fingerprint = super::LayoutPlanFingerprint::for_declaration(
            self.binding.fingerprint(),
            self.binding.bound_authority().authority_identity(),
            self.declaration,
        );
        Box::new(LayoutRollbackPlan {
            declaration: self.declaration,
            binding: self.binding,
            fingerprint,
        })
    }
}

pub(super) enum RollbackResolution {
    Resolved(Box<ResolvedLayoutRollbackRequest>),
    Denied(Box<LayoutEvolutionDenial>),
}

pub(super) enum RollbackLowering {
    Lowered(Box<LoweredLayoutRollbackPlan>),
    RebindRequired(Box<LayoutRebindRequired>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutRollbackPlan {
    declaration: LayoutEvolutionDeclaration,
    binding: LayoutBindingWitness,
    fingerprint: super::LayoutPlanFingerprint,
}

impl LayoutRollbackPlan {
    pub const fn binding(&self) -> &LayoutBindingWitness {
        &self.binding
    }

    pub const fn rollback_target(&self) -> LayoutVersion {
        self.declaration.rollback_target()
    }

    pub const fn declaration(&self) -> LayoutEvolutionDeclaration {
        self.declaration
    }

    pub const fn authority(&self) -> &StoreCurrentAuthorityWitness {
        self.binding.bound_authority()
    }

    pub const fn fingerprint(&self) -> super::LayoutPlanFingerprint {
        self.fingerprint
    }
}
