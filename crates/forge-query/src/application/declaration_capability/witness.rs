use super::tags::{
    ForgeQueryDeclarationSupportsBatchGrouping, ForgeQueryDeclarationSupportsBridgeContinuation,
    ForgeQueryDeclarationSupportsNeighborhoodGrouping,
    ForgeQueryDeclarationSupportsRelationalTruth, ForgeQueryDeclarationSupportsSignalCompatibility,
};
use crate::application::{
    ForgeQueryCanonicalDeclarationArtifact, ForgeQueryDeclarationInput, ForgeQueryDomainEntryMarker,
};

pub struct ForgeQueryRelationalTruthDeclaration<
    'a,
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    artifact: &'a ForgeQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct ForgeQueryBridgeContinuationDeclaration<
    'a,
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    artifact: &'a ForgeQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct ForgeQuerySignalCompatibleDeclaration<
    'a,
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    artifact: &'a ForgeQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct ForgeQueryNeighborhoodCapableDeclaration<
    'a,
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    artifact: &'a ForgeQueryCanonicalDeclarationArtifact<D, I>,
}

pub struct ForgeQueryBatchCapableDeclaration<
    'a,
    D: ForgeQueryDomainEntryMarker,
    I: ForgeQueryDeclarationInput<D>,
> {
    artifact: &'a ForgeQueryCanonicalDeclarationArtifact<D, I>,
}

macro_rules! witness_impl {
    ($name:ident) => {
        impl<'a, D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>> $name<'a, D, I> {
            pub(crate) fn new(artifact: &'a ForgeQueryCanonicalDeclarationArtifact<D, I>) -> Self {
                Self { artifact }
            }

            pub fn artifact(&self) -> &'a ForgeQueryCanonicalDeclarationArtifact<D, I> {
                self.artifact
            }
        }
    };
}

witness_impl!(ForgeQueryRelationalTruthDeclaration);
witness_impl!(ForgeQueryBridgeContinuationDeclaration);
witness_impl!(ForgeQuerySignalCompatibleDeclaration);
witness_impl!(ForgeQueryNeighborhoodCapableDeclaration);
witness_impl!(ForgeQueryBatchCapableDeclaration);

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: ForgeQueryDeclarationSupportsRelationalTruth<D>,
{
    pub fn relational_truth(&self) -> ForgeQueryRelationalTruthDeclaration<'_, D, I> {
        ForgeQueryRelationalTruthDeclaration::new(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: ForgeQueryDeclarationSupportsBridgeContinuation<D>,
{
    pub fn bridge_continuation(&self) -> ForgeQueryBridgeContinuationDeclaration<'_, D, I> {
        ForgeQueryBridgeContinuationDeclaration::new(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: ForgeQueryDeclarationSupportsSignalCompatibility<D>,
{
    pub fn signal_compatible(&self) -> ForgeQuerySignalCompatibleDeclaration<'_, D, I> {
        ForgeQuerySignalCompatibleDeclaration::new(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: ForgeQueryDeclarationSupportsNeighborhoodGrouping<D>,
{
    pub fn neighborhood_capable(&self) -> ForgeQueryNeighborhoodCapableDeclaration<'_, D, I> {
        ForgeQueryNeighborhoodCapableDeclaration::new(self)
    }
}

impl<D: ForgeQueryDomainEntryMarker, I: ForgeQueryDeclarationInput<D>>
    ForgeQueryCanonicalDeclarationArtifact<D, I>
where
    I::Family: ForgeQueryDeclarationSupportsBatchGrouping<D>,
{
    pub fn batch_capable(&self) -> ForgeQueryBatchCapableDeclaration<'_, D, I> {
        ForgeQueryBatchCapableDeclaration::new(self)
    }
}
