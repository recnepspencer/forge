use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::query_native_planar_topology_contract::domain::{
    PlanarTopologyContractCompletenessDeclarationFamily,
    PlanarTopologyContractCompletenessQueryDomain,
};
use crate::planar_contracts::topology_contract_completeness::{
    planar_topology_contract_authority_entries, PlanarTopologyContractCompletenessBasis,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarTopologyContractCompletenessCase {
    basis: PlanarTopologyContractCompletenessBasis,
}

impl PlanarTopologyContractCompletenessCase {
    pub fn from_topology_query_receipt(basis: PlanarTopologyContractCompletenessBasis) -> Self {
        Self { basis }
    }

    pub fn basis(&self) -> &PlanarTopologyContractCompletenessBasis {
        &self.basis
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarTopologyContractCompletenessEntry {
    case: PlanarTopologyContractCompletenessCase,
}

impl PlanarTopologyContractCompletenessEntry {
    pub fn case(&self) -> &PlanarTopologyContractCompletenessCase {
        &self.case
    }
}

impl ForgeQueryDeclarationInput<PlanarTopologyContractCompletenessQueryDomain>
    for PlanarTopologyContractCompletenessEntry
{
    type Family = PlanarTopologyContractCompletenessDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        planar_topology_contract_authority_entries(self.case.basis())
            .into_iter()
            .map(|entry| ForgeQueryDeclarationCanonicalEntry::text(entry.locus(), entry.value()))
            .collect()
    }
}

pub fn planar_topology_contract_completeness_entry(
    case: PlanarTopologyContractCompletenessCase,
) -> PlanarTopologyContractCompletenessEntry {
    PlanarTopologyContractCompletenessEntry { case }
}
