use schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_equivalence_policy_identity;

use crate::derived_invalidation_compiled_product_admission::TopologyCompiledProductAdmittedInput;

use super::basis_identity::admit_topology_selected_basis_identities;
use super::catalog::TopologySelectedEquivalenceFamilyCatalog;
use super::error::{
    TopologySelectedEquivalenceFamilyError, TopologySelectedEquivalenceFamilyErrorKind,
};
use super::selected_family::SelectedTopologyEquivalenceFamily;

pub fn select_topology_equivalence_family(
    catalog: &TopologySelectedEquivalenceFamilyCatalog,
    admitted_input: &TopologyCompiledProductAdmittedInput,
) -> Result<SelectedTopologyEquivalenceFamily, TopologySelectedEquivalenceFamilyError> {
    let declaration = catalog
        .family_for_compiled_product(admitted_input.family_admitted_input().family_identity())
        .ok_or_else(|| {
            TopologySelectedEquivalenceFamilyError::new(
                TopologySelectedEquivalenceFamilyErrorKind::MissingDeclaredFamily,
                "topology admitted input did not map to a declared selected equivalence family",
            )
        })?
        .clone();
    let equivalence_policy_identity = admit_compiled_product_equivalence_policy_identity(
        declaration.equivalence_policy_name(),
        declaration
            .equivalence_dimensions()
            .iter()
            .map(|dimension| dimension.as_str()),
    )
    .map_err(|error| {
        TopologySelectedEquivalenceFamilyError::new(
            TopologySelectedEquivalenceFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("topology selected equivalence policy admission failed: {error:?}"),
        )
    })?;
    let (
        equivalence_basis_identity,
        compatibility_basis_identity,
        reuse_basis_identity,
        future_public_proof_seed_identity,
    ) = admit_topology_selected_basis_identities(
        &declaration,
        admitted_input,
        equivalence_policy_identity.identity_digest(),
    );
    Ok(SelectedTopologyEquivalenceFamily::new(
        declaration,
        equivalence_policy_identity,
        equivalence_basis_identity,
        compatibility_basis_identity,
        reuse_basis_identity,
        future_public_proof_seed_identity,
    ))
}
