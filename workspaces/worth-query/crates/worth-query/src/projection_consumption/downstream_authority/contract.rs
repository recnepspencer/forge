use std::collections::BTreeSet;

use crate::projection_consumption::{
    DeclaredNativeFactContract, NativeFactDeclarationConflict, ProjectMaterializedFacts,
    ProjectionFactFieldPath, ProjectionFactRequest,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectionAuthorityRequirement {
    SettledConsumption,
    SourceAuthority,
    BasisGeneration,
    TargetIdentity,
}

impl ProjectionAuthorityRequirement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SettledConsumption => "settled_consumption",
            Self::SourceAuthority => "source_authority",
            Self::BasisGeneration => "basis_generation",
            Self::TargetIdentity => "target_identity",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectionAuthorityContract {
    requirements: BTreeSet<ProjectionAuthorityRequirement>,
    requested_facts: ProjectMaterializedFacts,
}

impl ProjectionAuthorityContract {
    pub fn declare() -> Self {
        Self::default()
    }

    pub fn require_settled_consumption(mut self) -> Self {
        self.requirements
            .insert(ProjectionAuthorityRequirement::SettledConsumption);
        self
    }

    pub fn require_source_authority(mut self) -> Self {
        self.requirements
            .insert(ProjectionAuthorityRequirement::SourceAuthority);
        self
    }

    pub fn require_basis_generation(mut self) -> Self {
        self.requirements
            .insert(ProjectionAuthorityRequirement::BasisGeneration);
        self
    }

    pub fn require_target_identity(mut self) -> Self {
        self.requirements
            .insert(ProjectionAuthorityRequirement::TargetIdentity);
        self.requested_facts = self.requested_facts.target_identity();
        self
    }

    pub fn require_entity_identities(mut self) -> Self {
        self.requested_facts = self.requested_facts.entity_identities();
        self
    }

    pub fn require_view_local_identities(mut self) -> Self {
        self.requested_facts = self.requested_facts.view_local_identities();
        self
    }

    pub fn require_source_references(mut self) -> Self {
        self.requested_facts = self.requested_facts.source_references();
        self
    }

    pub fn require_effect_continuity_facts(mut self) -> Self {
        self.requested_facts = self.requested_facts.effect_continuity_facts();
        self
    }

    pub fn require_memberships(mut self) -> Self {
        self.requested_facts = self.requested_facts.memberships();
        self
    }

    pub fn require_relation_endpoints(mut self) -> Self {
        self.requested_facts = self.requested_facts.relation_endpoints();
        self
    }

    pub fn require_display_field(mut self, field: ProjectionFactFieldPath) -> Self {
        self.requested_facts = self.requested_facts.display_field_path(field);
        self
    }

    pub fn require_derived_field(mut self, field: ProjectionFactFieldPath) -> Self {
        self.requested_facts = self.requested_facts.derived_field_path(field);
        self
    }

    pub(crate) fn require_display_native(
        mut self,
        contract: DeclaredNativeFactContract,
    ) -> Result<Self, NativeFactDeclarationConflict> {
        self.requested_facts = self.requested_facts.display_native(contract)?;
        Ok(self)
    }

    pub(crate) fn require_derived_native(
        mut self,
        contract: DeclaredNativeFactContract,
    ) -> Result<Self, NativeFactDeclarationConflict> {
        self.requested_facts = self.requested_facts.derived_native(contract)?;
        Ok(self)
    }

    pub fn requirements(&self) -> impl Iterator<Item = ProjectionAuthorityRequirement> + '_ {
        self.requirements.iter().copied()
    }

    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    pub fn requested_facts(&self) -> impl Iterator<Item = &ProjectionFactRequest> {
        self.requested_facts.requested()
    }

    pub fn requested_fact_count(&self) -> usize {
        self.requested_facts.requested_count()
    }

    pub(super) fn fact_request(&self) -> ProjectMaterializedFacts {
        self.requested_facts.clone()
    }

    pub(super) fn from_consumed_request(requested_facts: ProjectMaterializedFacts) -> Self {
        Self {
            requirements: BTreeSet::from([
                ProjectionAuthorityRequirement::SettledConsumption,
                ProjectionAuthorityRequirement::SourceAuthority,
            ]),
            requested_facts,
        }
    }

    pub(in crate::projection_consumption) fn certification(
        requested_facts: ProjectMaterializedFacts,
        requirements: impl IntoIterator<Item = ProjectionAuthorityRequirement>,
    ) -> Self {
        Self {
            requirements: requirements.into_iter().collect(),
            requested_facts,
        }
    }
}
