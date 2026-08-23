use serde::{Deserialize, Serialize};

use crate::aspects::{AspectContract, AspectContractRevision, AspectIdentity, AspectKey};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableAspectContractBasis {
    key: AspectKey,
    identity: AspectIdentity,
    revision: AspectContractRevision,
}

impl PortableAspectContractBasis {
    pub fn new(key: AspectKey, identity: AspectIdentity, revision: AspectContractRevision) -> Self {
        Self {
            key,
            identity,
            revision,
        }
    }

    pub fn from_contract(contract: &AspectContract) -> Self {
        Self::new(
            contract.key().clone(),
            contract.identity(),
            contract.revision(),
        )
    }

    pub fn key(&self) -> &AspectKey {
        &self.key
    }

    pub fn identity(&self) -> AspectIdentity {
        self.identity
    }

    pub fn revision(&self) -> AspectContractRevision {
        self.revision
    }

    pub fn owned_allocation_capacity_bytes(&self) -> usize {
        self.key.owned_allocation_capacity_bytes()
    }
}

pub trait PortableAspectContractLookup {
    fn contract_for(&self, key: &AspectKey) -> Option<AspectContract>;

    fn exact_contract_for(
        &self,
        key: &AspectKey,
        identity: AspectIdentity,
        revision: AspectContractRevision,
    ) -> Option<AspectContract> {
        self.contract_for(key)
            .filter(|contract| contract.identity() == identity && contract.revision() == revision)
    }
}

impl<F> PortableAspectContractLookup for F
where
    F: Fn(&AspectKey) -> Option<AspectContract>,
{
    fn contract_for(&self, key: &AspectKey) -> Option<AspectContract> {
        self(key)
    }
}
