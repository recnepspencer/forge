use std::collections::BTreeMap;

use worth_foundational::facade::{AspectContract, AspectKey};

use super::WorthQueryAspectContractRegistrationDenial;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryNativeAspectContractRegistry {
    contracts: BTreeMap<AspectKey, AspectContract>,
}

impl WorthQueryNativeAspectContractRegistry {
    pub(crate) fn from_contracts(
        contracts: impl IntoIterator<Item = AspectContract>,
    ) -> Result<Self, WorthQueryAspectContractRegistrationDenial> {
        let mut indexed = BTreeMap::new();
        for contract in contracts {
            match indexed.get(contract.key()) {
                Some(existing) if existing == &contract => continue,
                Some(_) => {
                    return Err(
                        WorthQueryAspectContractRegistrationDenial::conflicting_contract(
                            contract.key().clone(),
                        ),
                    );
                }
                None => {
                    indexed.insert(contract.key().clone(), contract);
                }
            }
        }
        Ok(Self { contracts: indexed })
    }

    pub(crate) fn contract(&self, key: &AspectKey) -> Option<&AspectContract> {
        self.contracts.get(key)
    }

    pub(crate) fn install(
        &mut self,
        contract: AspectContract,
    ) -> Result<(), WorthQueryAspectContractRegistrationDenial> {
        match self.contracts.get(contract.key()) {
            Some(existing) if existing == &contract => Ok(()),
            Some(_) => Err(
                WorthQueryAspectContractRegistrationDenial::conflicting_contract(
                    contract.key().clone(),
                ),
            ),
            None => {
                self.contracts.insert(contract.key().clone(), contract);
                Ok(())
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.contracts.len()
    }
}

impl worth_foundational::facade::PortableAspectContractLookup
    for WorthQueryNativeAspectContractRegistry
{
    fn contract_for(&self, key: &AspectKey) -> Option<AspectContract> {
        self.contract(key).cloned()
    }
}
