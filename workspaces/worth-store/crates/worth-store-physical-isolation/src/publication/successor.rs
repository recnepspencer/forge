use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
    PhysicalRootReference,
};

use super::{PhysicalPublicationDenial, PublicationRootCandidate};
use crate::epoch::{ManifestEpoch, RootEpoch};
use crate::{CurrentPhysicalRoot, CurrentPhysicalRootBasis, PhysicalOrderingContract};

#[derive(Debug, Clone, Copy)]
pub struct PublicationRootSuccessorOwner;

impl PublicationRootSuccessorOwner {
    pub fn plan(
        current: PublicationRootCandidate,
        generation: PhysicalGeneration,
    ) -> Result<PublicationRootCandidate, PhysicalPublicationDenial> {
        let candidate = successor_root(current.root())?;
        let root = PhysicalRootReference::from_raw(candidate.scope())
            .map_err(|_| PhysicalPublicationDenial::PublicationEpochExhausted)?;
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
        let cell = generations
            .root_publication_cell(root)
            .with_root_publication_generation(generation);
        let validation = references
            .validate_root_publication(references.admit_root_publication(cell), cell)
            .map_err(|_| PhysicalPublicationDenial::RootPublicationValidationRootMismatch)?;
        PublicationRootCandidate::admit(candidate, validation)
    }
}

fn successor_root(
    current: CurrentPhysicalRoot,
) -> Result<CurrentPhysicalRoot, PhysicalPublicationDenial> {
    let root = current
        .epoch()
        .get()
        .checked_add(1)
        .ok_or(PhysicalPublicationDenial::PublicationEpochExhausted)?;
    let manifest = current
        .manifest_epoch()
        .get()
        .checked_add(1)
        .ok_or(PhysicalPublicationDenial::PublicationEpochExhausted)?;
    CurrentPhysicalRoot::from_physical_isolation_entry(
        CurrentPhysicalRootBasis::new(
            RootEpoch::from_admitted_physical_basis(root),
            ManifestEpoch::from_admitted_physical_basis(manifest),
            current.store_authority_identity(),
        ),
        PhysicalOrderingContract::root_swap_acquire_release(),
    )
    .map_err(PhysicalPublicationDenial::WeakOrdering)
}

#[cfg(test)]
mod tests {
    use worth_store_physical_format::{
        PhysicalGeneration, PhysicalGenerationAuthority, PhysicalReferenceAuthority,
        PhysicalRootReference, PhysicalStoreIdentity,
    };

    use super::{successor_root, PublicationRootSuccessorOwner};
    use crate::{
        CurrentPhysicalRoot, CurrentPhysicalRootBasis, ManifestEpoch, PhysicalOrderingContract,
        PhysicalPublicationDenial, PublicationRootCandidate, RootEpoch,
    };

    #[test]
    fn successor_advances_both_epochs_and_binds_the_requested_generation() {
        let current = root(7, 11);
        let candidate =
            PublicationRootCandidate::admit(current, validation(current.scope(), 1)).unwrap();
        let requested_generation = generation(43);

        let successor = PublicationRootSuccessorOwner::plan(candidate, requested_generation)
            .expect("ordinary publication successor must plan");

        assert_eq!(successor.root().epoch().get(), 8);
        assert_eq!(successor.root().manifest_epoch().get(), 12);
        assert_eq!(
            successor.root().store_authority_identity(),
            current.store_authority_identity()
        );
        assert_eq!(
            successor
                .validation()
                .reference()
                .root_reference()
                .unwrap()
                .get(),
            successor.root().scope()
        );
        assert_eq!(
            successor.validation().owner().generation(),
            requested_generation
        );
    }

    #[test]
    fn successor_rejects_exhausted_root_or_manifest_epoch() {
        assert_eq!(
            successor_root(root(u64::MAX, 11)).unwrap_err(),
            PhysicalPublicationDenial::PublicationEpochExhausted
        );
        assert_eq!(
            successor_root(root(7, u64::MAX)).unwrap_err(),
            PhysicalPublicationDenial::PublicationEpochExhausted
        );
    }

    fn root(root_epoch: u64, manifest_epoch: u64) -> CurrentPhysicalRoot {
        CurrentPhysicalRoot::from_physical_isolation_entry(
            CurrentPhysicalRootBasis::new(
                RootEpoch::from_admitted_physical_basis(root_epoch),
                ManifestEpoch::from_admitted_physical_basis(manifest_epoch),
                PhysicalStoreIdentity::physical_format_default().authority_identity(),
            ),
            PhysicalOrderingContract::root_swap_acquire_release(),
        )
        .unwrap()
    }

    fn validation(
        root: u64,
        generation: u64,
    ) -> worth_store_physical_format::RootPublicationValidationWitness {
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let references = PhysicalReferenceAuthority::for_canonical_physical_format();
        let cell = generations
            .root_publication_cell(PhysicalRootReference::from_raw(root).unwrap())
            .with_root_publication_generation(self::generation(generation));
        references
            .validate_root_publication(references.admit_root_publication(cell), cell)
            .unwrap()
    }

    fn generation(value: u64) -> PhysicalGeneration {
        PhysicalGeneration::from_raw(value).unwrap()
    }
}
