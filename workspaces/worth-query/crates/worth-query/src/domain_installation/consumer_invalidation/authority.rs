use crate::runtime::{
    WorthQuerySharedExecutionOwnerIdentity, WorthQuerySharedProjectionLeaseIdentity,
};

use crate::domain_installation::WorthQueryBoundCapabilityGeneration;
use std::sync::Arc;
use worth_foundational::facade::admit_foundational_authority_identity;

use crate::identity_authority::{
    query_signal_invalidation_authority, QuerySignalInvalidationAuthorityIdentity,
    QuerySignalInvalidationIdentityKind,
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQuerySharedOwnerGeneration(u64);

impl WorthQuerySharedOwnerGeneration {
    pub(crate) const fn from_owner(owner: WorthQuerySharedExecutionOwnerIdentity) -> Self {
        Self(owner.generation())
    }

    pub const fn ordinal(self) -> u64 {
        self.0
    }
}

#[derive(Clone)]
pub struct WorthQueryConsumerInvalidationAuthority {
    runtime_authority: u64,
    installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    capability_identity: u64,
    capability_generation: WorthQueryBoundCapabilityGeneration,
    shared_owner_generation: WorthQuerySharedOwnerGeneration,
    owner: WorthQuerySharedExecutionOwnerIdentity,
    lease: WorthQuerySharedProjectionLeaseIdentity,
    source_identity: String,
    binding_identity: String,
    collection_delivery_contract_identity: Option<String>,
    _owner_identity:
        QuerySignalInvalidationAuthorityIdentity<Arc<str>, QuerySignalInvalidationIdentityKind>,
}

impl WorthQueryConsumerInvalidationAuthority {
    pub(crate) fn from_lease(
        readmission: &super::super::operation_execution::WorthQuerySharedProjectionLeaseReadmission<
            '_,
        >,
        installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    ) -> Self {
        let owner_identity = admit_foundational_authority_identity(
            Arc::<str>::from(crate::identity::hash_parts(&[
                "worth_query_consumer_invalidation_authority_v1".into(),
                format!("source:{}", readmission.source_identity),
                format!("binding:{}", readmission.binding_identity),
                format!(
                    "collection-delivery:{}",
                    readmission
                        .collection_delivery_contract_identity
                        .unwrap_or("not-declared")
                ),
                format!("owner-generation:{}", readmission.owner.generation()),
                format!("lease-generation:{}", readmission.lease.generation()),
            ])),
            query_signal_invalidation_authority(),
        );
        Self {
            runtime_authority: readmission.owner.runtime_authority(),
            installation_generation,
            capability_identity: readmission.capability_identity,
            capability_generation: readmission.capability_generation,
            shared_owner_generation: WorthQuerySharedOwnerGeneration::from_owner(readmission.owner),
            owner: readmission.owner,
            lease: readmission.lease,
            source_identity: readmission.source_identity.to_string(),
            binding_identity: readmission.binding_identity.to_string(),
            collection_delivery_contract_identity: readmission
                .collection_delivery_contract_identity
                .map(str::to_string),
            _owner_identity: owner_identity,
        }
    }

    pub const fn runtime_authority(&self) -> u64 {
        self.runtime_authority
    }

    pub const fn installation_generation(
        &self,
    ) -> super::super::WorthQueryDomainInstallationGeneration {
        self.installation_generation
    }

    pub const fn capability_generation(&self) -> WorthQueryBoundCapabilityGeneration {
        self.capability_generation
    }

    pub const fn capability_identity(&self) -> u64 {
        self.capability_identity
    }

    pub const fn shared_owner_generation(&self) -> WorthQuerySharedOwnerGeneration {
        self.shared_owner_generation
    }

    pub const fn owner_identity(&self) -> WorthQuerySharedExecutionOwnerIdentity {
        self.owner
    }

    pub const fn lease_identity(&self) -> WorthQuerySharedProjectionLeaseIdentity {
        self.lease
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub(crate) fn collection_delivery_contract_identity(&self) -> Option<&str> {
        self.collection_delivery_contract_identity.as_deref()
    }

    pub fn is_same_current_authority_as(&self, candidate: &Self) -> bool {
        self.runtime_authority == candidate.runtime_authority
            && self.installation_generation == candidate.installation_generation
            && self.capability_identity == candidate.capability_identity
            && self.capability_generation == candidate.capability_generation
            && self.shared_owner_generation.ordinal() == candidate.shared_owner_generation.ordinal()
            && self.owner == candidate.owner
            && self.lease == candidate.lease
            && self.source_identity == candidate.source_identity
            && self.binding_identity == candidate.binding_identity
            && self.collection_delivery_contract_identity
                == candidate.collection_delivery_contract_identity
    }

    pub(crate) fn readmits(
        &self,
        readmission: &super::super::operation_execution::WorthQuerySharedProjectionLeaseReadmission<
            '_,
        >,
        installation_generation: super::super::WorthQueryDomainInstallationGeneration,
    ) -> bool {
        self.runtime_authority == readmission.owner.runtime_authority()
            && self.installation_generation == installation_generation
            && self.capability_identity == readmission.capability_identity
            && self.capability_generation == readmission.capability_generation
            && self.shared_owner_generation.ordinal() == readmission.owner.generation()
            && self.owner == readmission.owner
            && self.lease == readmission.lease
            && self.source_identity == readmission.source_identity
            && self.binding_identity == readmission.binding_identity
            && self.collection_delivery_contract_identity.as_deref()
                == readmission.collection_delivery_contract_identity
    }
}

impl std::fmt::Debug for WorthQueryConsumerInvalidationAuthority {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryConsumerInvalidationAuthority")
            .field("posture", &"current")
            .finish_non_exhaustive()
    }
}
