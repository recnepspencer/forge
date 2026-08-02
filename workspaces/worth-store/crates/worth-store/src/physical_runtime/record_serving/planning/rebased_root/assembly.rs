use worth_store_physical_format::{
    BootstrapCatalog, CurrentRootCatalogEntry, CurrentRootCatalogGeneration,
    DurableFreeSpaceManifestHeader, RecordArtifactFile,
};

use super::{projection::ProjectedSuccessorRoot, RootRebaseContext};
use crate::physical_runtime::record_serving::publication::PublicationPlan;

pub(super) fn assemble_rebased_publication(
    mut publication: PublicationPlan,
    context: RootRebaseContext<'_>,
    generation: u64,
    projected: ProjectedSuccessorRoot,
) -> (PublicationPlan, DurableFreeSpaceManifestHeader) {
    projected.observe_discovery(&mut publication.observation);
    publication.generation = generation;
    publication.root = RecordArtifactFile::RootManifest { generation };
    publication.manifest = projected.root;
    publication.root_bytes = publication.manifest.encode(context.format.declaration());
    publication.catalog_bytes = BootstrapCatalog::new(
        context.media.store_identity(),
        context.format.declaration(),
        CurrentRootCatalogEntry::new(
            CurrentRootCatalogGeneration::new(generation).expect("successor is nonzero"),
        ),
    )
    .encode()
    .to_vec();
    publication.manifests = projected.manifests;
    (publication, projected.free_space)
}
