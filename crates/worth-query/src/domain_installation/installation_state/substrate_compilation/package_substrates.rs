use worth_relational::facade::runtime::CustomInvariantRegistration;

use crate::application::WorthQueryDomainEntryMarker;
use crate::runtime::{
    WorthQueryGraphObligationRegistration, WorthQueryGraphReadOperationRegistration,
};

use super::super::{
    WorthQueryAdmittedDomainPackage, WorthQueryDomainInstallationDenial,
    WorthQueryPendingPackageCandidate,
};
use super::invariant::compile_invariant_definition;

pub(in super::super) struct WorthQueryLoweredPackageSubstrates {
    pub(in super::super) graph_read_operations: Vec<WorthQueryGraphReadOperationRegistration>,
    pub(in super::super) graph_obligations: Vec<WorthQueryGraphObligationRegistration>,
}

pub(in super::super) fn lower_package_substrates<D: WorthQueryDomainEntryMarker>(
    package: &WorthQueryAdmittedDomainPackage<D>,
    candidate: &WorthQueryPendingPackageCandidate,
) -> WorthQueryLoweredPackageSubstrates {
    let graph_read_operations = package
        .graph_read_operations
        .iter()
        .map(|definition| {
            definition.lower_with_owner(
                &candidate.domain_owner,
                candidate.substrate_provenance.clone(),
            )
        })
        .collect();
    let graph_obligations = package
        .graph_obligations
        .iter()
        .map(|definition| {
            definition.lower_with_owner(
                &candidate.domain_owner,
                candidate.substrate_provenance.clone(),
            )
        })
        .collect();
    WorthQueryLoweredPackageSubstrates {
        graph_read_operations,
        graph_obligations,
    }
}

pub(in super::super) fn compile_package_invariants<D: WorthQueryDomainEntryMarker>(
    package: &WorthQueryAdmittedDomainPackage<D>,
    candidate: &WorthQueryPendingPackageCandidate,
) -> Result<Vec<CustomInvariantRegistration>, WorthQueryDomainInstallationDenial> {
    package
        .invariant_definitions
        .iter()
        .map(|definition| compile_invariant_definition(&candidate.substrate_provenance, definition))
        .collect()
}
