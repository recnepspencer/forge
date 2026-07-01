use super::FixtureCapabilityDeclaration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FixtureMutationBoundary {
    PageImage,
    FrameBody,
    WalFrame,
    Manifest,
    Index,
    Chunk,
    AuditRecord,
    KeyEnvelope,
    TenantMetadata,
    RepairArtifact,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureMutationBoundarySet {
    boundaries: Vec<FixtureMutationBoundary>,
}

impl FixtureMutationBoundarySet {
    pub fn from_boundaries(
        boundaries: impl IntoIterator<Item = FixtureMutationBoundary>,
    ) -> Option<Self> {
        let mut boundaries: Vec<_> = boundaries.into_iter().collect();
        boundaries.sort_unstable();
        boundaries.dedup();
        if boundaries.is_empty() {
            None
        } else {
            Some(Self { boundaries })
        }
    }

    pub fn from_capabilities(declarations: &[FixtureCapabilityDeclaration]) -> Option<Self> {
        Self::from_boundaries(
            declarations
                .iter()
                .map(FixtureCapabilityDeclaration::mutation_boundary),
        )
    }

    pub fn contains(&self, boundary: FixtureMutationBoundary) -> bool {
        self.boundaries.binary_search(&boundary).is_ok()
    }

    pub fn iter(&self) -> impl Iterator<Item = FixtureMutationBoundary> + '_ {
        self.boundaries.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.boundaries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.boundaries.is_empty()
    }
}
