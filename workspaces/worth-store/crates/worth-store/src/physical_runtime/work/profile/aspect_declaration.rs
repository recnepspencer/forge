use worth_signal::facade::PartitionSubscription;
use worth_store_aspect_native::StoreAspectContractAdmission;

use super::PhysicalWorkSignalFamilySet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalSignalAspectRole {
    Dependency,
    Output,
    DependencyAndOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalSignalAspectDeclaration {
    contract: StoreAspectContractAdmission,
    role: PhysicalSignalAspectRole,
    families: PhysicalWorkSignalFamilySet,
    partition: Option<PartitionSubscription>,
}

impl PhysicalSignalAspectDeclaration {
    pub fn new(contract: StoreAspectContractAdmission, role: PhysicalSignalAspectRole) -> Self {
        Self {
            contract,
            role,
            families: PhysicalWorkSignalFamilySet::all(),
            partition: None,
        }
    }

    pub fn with_partition(mut self, partition: PartitionSubscription) -> Self {
        self.partition = Some(partition);
        self
    }

    pub const fn for_families(mut self, families: PhysicalWorkSignalFamilySet) -> Self {
        self.families = families;
        self
    }

    pub const fn contract(&self) -> &StoreAspectContractAdmission {
        &self.contract
    }

    pub const fn role(&self) -> PhysicalSignalAspectRole {
        self.role
    }

    pub const fn families(&self) -> PhysicalWorkSignalFamilySet {
        self.families
    }

    pub const fn partition(&self) -> Option<&PartitionSubscription> {
        self.partition.as_ref()
    }

    pub(super) fn from_contract(contract: StoreAspectContractAdmission) -> Self {
        let role = if contract.mutation_mask().is_some() {
            PhysicalSignalAspectRole::DependencyAndOutput
        } else {
            PhysicalSignalAspectRole::Dependency
        };
        Self::new(contract, role)
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        StoreAspectContractAdmission,
        PhysicalSignalAspectRole,
        PhysicalWorkSignalFamilySet,
        Option<PartitionSubscription>,
    ) {
        (self.contract, self.role, self.families, self.partition)
    }
}
