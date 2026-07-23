#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryArtifactEvidenceContract {
    basis_family: String,
    provenance_family: String,
    dependency_family: String,
    invalidation_family: String,
    equivalence_family: String,
}

impl WorthQueryArtifactEvidenceContract {
    pub fn new(
        basis_family: impl Into<String>,
        provenance_family: impl Into<String>,
        dependency_family: impl Into<String>,
        invalidation_family: impl Into<String>,
        equivalence_family: impl Into<String>,
    ) -> Self {
        Self {
            basis_family: basis_family.into(),
            provenance_family: provenance_family.into(),
            dependency_family: dependency_family.into(),
            invalidation_family: invalidation_family.into(),
            equivalence_family: equivalence_family.into(),
        }
    }

    pub fn basis_family(&self) -> &str {
        &self.basis_family
    }

    pub fn provenance_family(&self) -> &str {
        &self.provenance_family
    }

    pub fn dependency_family(&self) -> &str {
        &self.dependency_family
    }

    pub fn invalidation_family(&self) -> &str {
        &self.invalidation_family
    }

    pub fn equivalence_family(&self) -> &str {
        &self.equivalence_family
    }

    pub(crate) fn fields_are_portable(&self) -> bool {
        [
            &self.basis_family,
            &self.provenance_family,
            &self.dependency_family,
            &self.invalidation_family,
            &self.equivalence_family,
        ]
        .into_iter()
        .all(|field| {
            !field.trim().is_empty()
                && field.trim() == field
                && !field.chars().any(char::is_whitespace)
        })
    }
}
