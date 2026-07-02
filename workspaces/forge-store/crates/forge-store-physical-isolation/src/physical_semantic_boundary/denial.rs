use forge_proof::TransitionOutcome;

use super::{SemanticVisibilityReference, SemanticVisibilityReferenceKind};

pub type PhysicalSemanticBoundaryOutcome = TransitionOutcome<(), PhysicalSemanticBoundaryDenial>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PhysicalSemanticBoundaryDenial {
    SemanticVisibilityCannotMintPhysicalStability(SemanticVisibilityCannotMintPhysicalStability),
    DiagnosticCorrelationCannotMintPhysicalStability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticVisibilityCannotMintPhysicalStability {
    kind: SemanticVisibilityReferenceKind,
    semantic_id: String,
}

pub fn deny_semantic_visibility_as_physical_stability(
    semantic: &SemanticVisibilityReference,
) -> PhysicalSemanticBoundaryOutcome {
    TransitionOutcome::denied(
        PhysicalSemanticBoundaryDenial::SemanticVisibilityCannotMintPhysicalStability(
            SemanticVisibilityCannotMintPhysicalStability::from_reference(semantic),
        ),
    )
}

impl SemanticVisibilityCannotMintPhysicalStability {
    fn from_reference(semantic: &SemanticVisibilityReference) -> Self {
        Self {
            kind: semantic.kind(),
            semantic_id: semantic.semantic_id().to_string(),
        }
    }

    pub const fn kind(&self) -> SemanticVisibilityReferenceKind {
        self.kind
    }

    pub fn semantic_id(&self) -> &str {
        &self.semantic_id
    }
}
