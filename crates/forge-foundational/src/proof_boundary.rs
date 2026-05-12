use forge_proof::{Artifact, PhaseMarker};

/// Phase marker proving the foundational crate boundary has been declared.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryDeclared;

impl PhaseMarker for FoundationalBoundaryDeclared {}

/// Payload carried by the Phase 1 boundary artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryDeclaration {
    crate_name: &'static str,
    standardizes: &'static str,
    does_not_standardize: &'static str,
}

impl FoundationalBoundaryDeclaration {
    pub const fn crate_name(&self) -> &'static str {
        self.crate_name
    }

    pub const fn standardizes(&self) -> &'static str {
        self.standardizes
    }

    pub const fn does_not_standardize(&self) -> &'static str {
        self.does_not_standardize
    }
}

pub type FoundationalBoundaryArtifact =
    Artifact<FoundationalBoundaryDeclared, FoundationalBoundaryDeclaration>;

pub fn declared_foundational_boundary() -> FoundationalBoundaryArtifact {
    Artifact::new(FoundationalBoundaryDeclaration {
        crate_name: "forge-foundational",
        standardizes: "shared boundary meaning",
        does_not_standardize: "hot-path runtime storage or proof progression law",
    })
}
