use super::{reject_duplicate_identity, require_non_empty};
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeriodicQuotientCellDeclaration {
    cell_id: String,
    lattice_basis_ref: Option<String>,
    boundary_ownership_ref: Option<String>,
}

impl PeriodicQuotientCellDeclaration {
    pub fn new(cell_id: impl Into<String>) -> Self {
        Self::try_new(cell_id).expect("cell_id must be non-empty")
    }

    pub fn try_new(
        cell_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            cell_id: require_non_empty(cell_id, "cell_id")?,
            lattice_basis_ref: None,
            boundary_ownership_ref: None,
        })
    }

    pub fn with_lattice_basis_ref(self, lattice_basis_ref: impl Into<String>) -> Self {
        self.try_with_lattice_basis_ref(lattice_basis_ref)
            .expect("lattice_basis_ref must be non-empty")
    }

    pub fn try_with_lattice_basis_ref(
        mut self,
        lattice_basis_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.lattice_basis_ref = Some(require_non_empty(lattice_basis_ref, "lattice_basis_ref")?);
        Ok(self)
    }

    pub fn with_boundary_ownership_ref(self, boundary_ownership_ref: impl Into<String>) -> Self {
        self.try_with_boundary_ownership_ref(boundary_ownership_ref)
            .expect("boundary_ownership_ref must be non-empty")
    }

    pub fn try_with_boundary_ownership_ref(
        mut self,
        boundary_ownership_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.boundary_ownership_ref = Some(require_non_empty(
            boundary_ownership_ref,
            "boundary_ownership_ref",
        )?);
        Ok(self)
    }

    pub(crate) fn cell_id(&self) -> &str {
        &self.cell_id
    }

    pub(crate) fn lattice_basis_ref(&self) -> Option<&str> {
        self.lattice_basis_ref.as_deref()
    }

    pub(crate) fn boundary_ownership_ref(&self) -> Option<&str> {
        self.boundary_ownership_ref.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedPatternClosureDeclaration {
    closure_id: String,
    pattern_ref: String,
    generators: Vec<String>,
}

impl GeneratedPatternClosureDeclaration {
    pub fn new(closure_id: impl Into<String>, pattern_ref: impl Into<String>) -> Self {
        Self::try_new(closure_id, pattern_ref)
            .expect("closure_id and pattern_ref must be non-empty")
    }

    pub fn try_new(
        closure_id: impl Into<String>,
        pattern_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            closure_id: require_non_empty(closure_id, "closure_id")?,
            pattern_ref: require_non_empty(pattern_ref, "pattern_ref")?,
            generators: Vec::new(),
        })
    }

    pub fn with_generator(
        self,
        generator: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        let mut next = self;
        let generator = require_non_empty(generator, "generator")?;
        reject_duplicate_identity(&next.generators, &generator, "generator")?;
        next.generators.push(generator);
        next.generators.sort();
        Ok(next)
    }

    pub(crate) fn closure_id(&self) -> &str {
        &self.closure_id
    }

    pub(crate) fn pattern_ref(&self) -> &str {
        &self.pattern_ref
    }

    pub(crate) fn generators(&self) -> &[String] {
        &self.generators
    }
}
