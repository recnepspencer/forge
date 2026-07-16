use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference, RootPublicationValidationWitness,
};

use super::RecoveryPublicationDenial;
use crate::epoch::{ManifestEpoch, RootEpoch};
use crate::{CurrentPhysicalRoot, CurrentPhysicalRootBasis, PhysicalOrderingContract};

pub(super) fn successor_root(
    current: CurrentPhysicalRoot,
) -> Result<CurrentPhysicalRoot, RecoveryPublicationDenial> {
    let root = current
        .epoch()
        .get()
        .checked_add(1)
        .ok_or(RecoveryPublicationDenial::EpochExhausted)?;
    let manifest = current
        .manifest_epoch()
        .get()
        .checked_add(1)
        .ok_or(RecoveryPublicationDenial::EpochExhausted)?;
    CurrentPhysicalRoot::from_physical_isolation_entry(
        CurrentPhysicalRootBasis::new(
            RootEpoch::from_admitted_physical_basis(root),
            ManifestEpoch::from_admitted_physical_basis(manifest),
            current.store_authority_identity(),
        ),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .map_err(|_| RecoveryPublicationDenial::InvalidBinding)
}

pub(super) fn root_validation(
    root: u64,
    generation: u64,
) -> Result<RootPublicationValidationWitness, RecoveryPublicationDenial> {
    let root = PhysicalRootReference::from_raw(root)
        .map_err(|_| RecoveryPublicationDenial::InvalidBinding)?;
    let generation = PhysicalGeneration::from_raw(generation)
        .map_err(|_| RecoveryPublicationDenial::InvalidBinding)?;
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let cell = generations
        .root_publication_cell(root)
        .with_root_publication_generation(generation);
    references
        .validate_root_publication(references.admit_root_publication(cell), cell)
        .map_err(|_| RecoveryPublicationDenial::InvalidBinding)
}
