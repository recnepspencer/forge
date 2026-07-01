use schema::facade::platform::authority::compiled_product_semantic_graph::admit_compiled_product_equivalence_policy_identity;

use crate::workload_platform::compiled_product_admission::SpatialCompiledProductAdmittedInput;

use super::basis_identity::admit_spatial_selected_basis_identities;
use super::catalog::SpatialSelectedEquivalenceFamilyCatalog;
use super::error::{
    SpatialSelectedEquivalenceFamilyError, SpatialSelectedEquivalenceFamilyErrorKind,
};
use super::selected_family::SelectedSpatialEquivalenceFamily;

pub fn select_spatial_equivalence_family(
    catalog: &SpatialSelectedEquivalenceFamilyCatalog,
    admitted_input: &SpatialCompiledProductAdmittedInput,
) -> Result<SelectedSpatialEquivalenceFamily, SpatialSelectedEquivalenceFamilyError> {
    let declaration = catalog
        .family_for_compiled_product(admitted_input.family_admitted_input().family_identity())
        .ok_or_else(|| {
            SpatialSelectedEquivalenceFamilyError::new(
                SpatialSelectedEquivalenceFamilyErrorKind::MissingDeclaredFamily,
                "spatial admitted input did not map to a declared selected equivalence family",
            )
        })?
        .clone();
    let equivalence_policy_identity = admit_compiled_product_equivalence_policy_identity(
        declaration.equivalence_policy_name(),
        declaration.equivalence_dimensions().iter().copied(),
    )
    .map_err(|error| {
        SpatialSelectedEquivalenceFamilyError::new(
            SpatialSelectedEquivalenceFamilyErrorKind::SchemaVocabularyAdmissionFailed,
            format!("spatial selected equivalence policy admission failed: {error:?}"),
        )
    })?;
    let (
        equivalence_basis_identity,
        compatibility_basis_identity,
        reuse_basis_identity,
        future_public_proof_seed_identity,
    ) = admit_spatial_selected_basis_identities(
        &declaration,
        admitted_input,
        equivalence_policy_identity.identity_digest(),
    );
    Ok(SelectedSpatialEquivalenceFamily::new(
        declaration,
        equivalence_policy_identity,
        equivalence_basis_identity,
        compatibility_basis_identity,
        reuse_basis_identity,
        future_public_proof_seed_identity,
    ))
}
