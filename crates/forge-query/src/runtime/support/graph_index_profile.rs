use super::profile::ForgeQueryRuntimeSupportProfile;
use crate::runtime::{
    ForgeQueryAdmittedGraphReadRelationDirection, ForgeQueryGraphIndexInventory,
    ForgeQueryGraphIndexLifecycleClass, ForgeQueryGraphIndexLifecycleOwner,
    ForgeQueryGraphIndexPosture, ForgeQueryGraphIndexSupportRow, ForgeQueryGraphIndexSupportState,
    ForgeQueryGraphReadAccessComplexityContract, ForgeQueryGraphReadAccessInvalidationBasis,
    ForgeQueryGraphReadAccessRebuildBasis, ForgeQueryGraphReadAccessRequirementKind,
    ForgeQueryGraphReadLifecycleClass, ForgeQueryGraphReadOrderingPosture,
    ForgeQueryGraphReadPredicateFamily,
};

impl ForgeQueryRuntimeSupportProfile {
    pub fn graph_index_inventory(&self) -> ForgeQueryGraphIndexInventory {
        ForgeQueryGraphIndexInventory::from_rows(self.graph_index_support_rows.clone())
    }

    pub fn with_graph_index_support_omitted(
        mut self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    ) -> Self {
        self.graph_index_support_rows
            .retain(|row| row.requirement_kind() != &requirement_kind);
        self
    }

    pub fn with_graph_index_access_capability_registration_required(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        owning_milestone: impl Into<String>,
    ) -> Self {
        self.with_graph_index_support_posture(
            requirement_kind,
            ForgeQueryGraphIndexLifecycleOwner::DomainRegistration,
            ForgeQueryGraphIndexLifecycleClass::AccessCapabilityRegistrationRequired,
            ForgeQueryGraphIndexPosture::RequiresAccessCapabilityRegistration,
            ForgeQueryGraphIndexSupportState::Declared,
            Some(owning_milestone.into()),
        )
    }

    pub fn with_graph_index_temporarily_unavailable(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    ) -> Self {
        self.with_graph_index_support_posture(
            requirement_kind,
            ForgeQueryGraphIndexLifecycleOwner::LowerRuntime,
            ForgeQueryGraphIndexLifecycleClass::TemporarilyUnavailable,
            ForgeQueryGraphIndexPosture::TemporarilyUnavailable,
            ForgeQueryGraphIndexSupportState::TemporarilyUnavailable,
            None,
        )
    }

    pub fn with_graph_index_ephemeral_available(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
    ) -> Self {
        self.with_graph_index_support_posture(
            requirement_kind,
            ForgeQueryGraphIndexLifecycleOwner::QueryRuntime,
            ForgeQueryGraphIndexLifecycleClass::EphemeralRuntimeOwned,
            ForgeQueryGraphIndexPosture::EphemeralAvailable,
            ForgeQueryGraphIndexSupportState::Available,
            None,
        )
    }

    pub fn with_store_backed_graph_index_requirement(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        owning_milestone: impl Into<String>,
    ) -> Self {
        self.with_graph_index_support_posture(
            requirement_kind,
            ForgeQueryGraphIndexLifecycleOwner::StoreOwned,
            ForgeQueryGraphIndexLifecycleClass::StoreOwnedRequired,
            ForgeQueryGraphIndexPosture::RequiresStoreBackedPersistentIndex,
            ForgeQueryGraphIndexSupportState::StoreOwnedUnavailable,
            Some(owning_milestone.into()),
        )
    }

    pub fn with_graph_index_supported_relation_direction(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        direction: ForgeQueryAdmittedGraphReadRelationDirection,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_supported_relation_direction(direction)
        })
    }

    pub fn with_graph_index_supported_predicate_family(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        predicate_family: ForgeQueryGraphReadPredicateFamily,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_supported_predicate_family(predicate_family)
        })
    }

    pub fn with_graph_index_supported_ordering_posture(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        ordering_posture: ForgeQueryGraphReadOrderingPosture,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_supported_ordering_posture(ordering_posture)
        })
    }

    pub fn with_graph_index_supported_requirement_lifecycle(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        lifecycle: ForgeQueryGraphReadLifecycleClass,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_supported_requirement_lifecycle(lifecycle)
        })
    }

    pub fn with_graph_index_rebuild_basis(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_rebuild_basis(rebuild_basis)
        })
    }

    pub fn with_graph_index_invalidation_basis(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_invalidation_basis(invalidation_basis)
        })
    }

    pub fn with_graph_index_complexity_contract(
        self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        complexity_contract: ForgeQueryGraphReadAccessComplexityContract,
    ) -> Self {
        self.transform_graph_index_support_row(requirement_kind, |row| {
            row.with_complexity_contract(complexity_contract)
        })
    }

    fn with_graph_index_support_posture(
        mut self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        lifecycle_owner: ForgeQueryGraphIndexLifecycleOwner,
        lifecycle_class: ForgeQueryGraphIndexLifecycleClass,
        posture: ForgeQueryGraphIndexPosture,
        support_state: ForgeQueryGraphIndexSupportState,
        owning_milestone: Option<String>,
    ) -> Self {
        let row = ForgeQueryGraphIndexSupportRow::with_runtime_support_posture(
            requirement_kind,
            lifecycle_owner,
            lifecycle_class,
            posture,
            support_state,
            owning_milestone,
        );
        if let Some(index) = self
            .graph_index_support_rows
            .iter()
            .position(|candidate| candidate.requirement_kind() == row.requirement_kind())
        {
            self.graph_index_support_rows[index] = row;
        } else {
            self.graph_index_support_rows.push(row);
        }
        self
    }

    fn transform_graph_index_support_row(
        mut self,
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind,
        transform: impl FnOnce(ForgeQueryGraphIndexSupportRow) -> ForgeQueryGraphIndexSupportRow,
    ) -> Self {
        if let Some(index) = self
            .graph_index_support_rows
            .iter()
            .position(|candidate| candidate.requirement_kind() == &requirement_kind)
        {
            let row = self.graph_index_support_rows.remove(index);
            self.graph_index_support_rows.push(transform(row));
        }
        self
    }
}
