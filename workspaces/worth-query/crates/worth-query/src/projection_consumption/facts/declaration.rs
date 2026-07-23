use std::collections::{BTreeMap, BTreeSet};

use super::{ProjectionFactFieldPath, ProjectionFactRequest};
use crate::projection_consumption::DeclaredNativeFactContract;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProjectMaterializedFacts {
    requested: BTreeSet<ProjectionFactRequest>,
    native_contracts: BTreeMap<ProjectionFactRequest, DeclaredNativeFactContract>,
}

impl ProjectMaterializedFacts {
    pub fn declare() -> Self {
        Self::default()
    }

    pub fn entity_identities(mut self) -> Self {
        self.requested.insert(ProjectionFactRequest::EntityIdentity);
        self
    }

    pub fn view_local_identities(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::ViewLocalIdentity);
        self
    }

    pub fn target_identity(mut self) -> Self {
        self.requested.insert(ProjectionFactRequest::TargetIdentity);
        self
    }

    pub fn source_references(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::SourceReference);
        self
    }

    pub fn effect_continuity_facts(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::EffectContinuity);
        self
    }

    pub fn memberships(mut self) -> Self {
        self.requested.insert(ProjectionFactRequest::Membership);
        self
    }

    pub fn relation_endpoints(mut self) -> Self {
        self.requested
            .insert(ProjectionFactRequest::RelationEndpoint);
        self
    }

    pub fn display_field_path(mut self, field: ProjectionFactFieldPath) -> Self {
        self.requested
            .insert(ProjectionFactRequest::DisplayField(field));
        self
    }

    pub fn derived_field_path(mut self, field: ProjectionFactFieldPath) -> Self {
        self.requested
            .insert(ProjectionFactRequest::DerivedField(field));
        self
    }

    pub fn requested(&self) -> impl Iterator<Item = &ProjectionFactRequest> {
        self.requested.iter()
    }

    pub fn requested_count(&self) -> usize {
        self.requested.len()
    }

    pub(crate) fn display_native(
        mut self,
        contract: DeclaredNativeFactContract,
    ) -> Result<Self, NativeFactDeclarationConflict> {
        self.insert_native(
            ProjectionFactRequest::DisplayField(contract.field_path().clone()),
            contract,
        )?;
        Ok(self)
    }

    pub(crate) fn derived_native(
        mut self,
        contract: DeclaredNativeFactContract,
    ) -> Result<Self, NativeFactDeclarationConflict> {
        self.insert_native(
            ProjectionFactRequest::DerivedField(contract.field_path().clone()),
            contract,
        )?;
        Ok(self)
    }

    pub(crate) fn native_contract_for(
        &self,
        request: &ProjectionFactRequest,
    ) -> Option<&DeclaredNativeFactContract> {
        self.native_contracts.get(request)
    }

    fn insert_native(
        &mut self,
        request: ProjectionFactRequest,
        contract: DeclaredNativeFactContract,
    ) -> Result<(), NativeFactDeclarationConflict> {
        if let Some(existing) = self.native_contracts.get(&request) {
            if existing != &contract {
                return Err(NativeFactDeclarationConflict);
            }
            return Ok(());
        }
        self.requested.insert(request.clone());
        self.native_contracts.insert(request, contract);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativeFactDeclarationConflict;
