use super::tags::{
    WorthQueryDeclarationSupportsBatchGrouping, WorthQueryDeclarationSupportsBridgeContinuation,
    WorthQueryDeclarationSupportsNeighborhoodGrouping,
    WorthQueryDeclarationSupportsRelationalTruth, WorthQueryDeclarationSupportsSignalCompatibility,
};
use crate::application::{
    WorthQueryCanonicalDeclarationArtifact, WorthQueryDeclarationInput, WorthQueryDomainEntryMarker,
};

pub struct WorthQueryRelationalTruthDeclaration<
    'a,
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    artifact: &'a WorthQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct WorthQueryBridgeContinuationDeclaration<
    'a,
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    artifact: &'a WorthQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct WorthQuerySignalCompatibleDeclaration<
    'a,
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    artifact: &'a WorthQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct WorthQueryNeighborhoodCapableDeclaration<
    'a,
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    artifact: &'a WorthQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct WorthQueryBatchCapableDeclaration<
    'a,
    D: WorthQueryDomainEntryMarker,
    I: WorthQueryDeclarationInput<D>,
> {
    artifact: &'a WorthQueryCanonicalDeclarationArtifact<D, I>,
}

macro_rules! witness_impl {
    ($name:ident) => {
        impl<'a, D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>> $name<'a, D, I> {
            pub(crate) fn new(artifact: &'a WorthQueryCanonicalDeclarationArtifact<D, I>) -> Self {
                Self { artifact }
            }

            pub fn artifact(&self) -> &'a WorthQueryCanonicalDeclarationArtifact<D, I> {
                self.artifact
            }
        }
    };
}

witness_impl!(WorthQueryRelationalTruthDeclaration);
witness_impl!(WorthQueryBridgeContinuationDeclaration);
witness_impl!(WorthQuerySignalCompatibleDeclaration);
witness_impl!(WorthQueryNeighborhoodCapableDeclaration);
witness_impl!(WorthQueryBatchCapableDeclaration);

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: WorthQueryDeclarationSupportsRelationalTruth<D>,
{
    pub fn relational_truth(&self) -> WorthQueryRelationalTruthDeclaration<'_, D, I> {
        WorthQueryRelationalTruthDeclaration::new(self)
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: WorthQueryDeclarationSupportsBridgeContinuation<D>,
{
    pub fn bridge_continuation(&self) -> WorthQueryBridgeContinuationDeclaration<'_, D, I> {
        WorthQueryBridgeContinuationDeclaration::new(self)
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: WorthQueryDeclarationSupportsSignalCompatibility<D>,
{
    pub fn signal_compatible(&self) -> WorthQuerySignalCompatibleDeclaration<'_, D, I> {
        WorthQuerySignalCompatibleDeclaration::new(self)
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: WorthQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    pub fn neighborhood_capable(&self) -> WorthQueryNeighborhoodCapableDeclaration<'_, D, I> {
        WorthQueryNeighborhoodCapableDeclaration::new(self)
    }
}

impl<D: WorthQueryDomainEntryMarker, I: WorthQueryDeclarationInput<D>>
    WorthQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: WorthQueryDeclarationSupportsBatchGrouping<D>,
{
    pub fn batch_capable(&self) -> WorthQueryBatchCapableDeclaration<'_, D, I> {
        WorthQueryBatchCapableDeclaration::new(self)
    }
}
