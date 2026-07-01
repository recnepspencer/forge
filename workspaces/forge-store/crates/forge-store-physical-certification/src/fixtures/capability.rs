use super::FixtureMutationBoundary;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureCapabilityDeclaration {
    mutation_boundary: FixtureMutationBoundary,
}

impl FixtureCapabilityDeclaration {
    pub const fn for_mutation_boundary(mutation_boundary: FixtureMutationBoundary) -> Self {
        Self { mutation_boundary }
    }

    pub const fn mutation_boundary(&self) -> FixtureMutationBoundary {
        self.mutation_boundary
    }
}
