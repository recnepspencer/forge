use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionDomainInvariantSummary,
    ForgeQueryGraphCompositionProgram, ForgeQueryWriteCommand,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionInvariantPackViolation {
    invariant_family: String,
    message: String,
    violation_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphCompositionInvariantPackViolation {
    pub fn new(invariant_family: impl Into<String>, message: impl Into<String>) -> Self {
        let invariant_family = invariant_family.into();
        let message = message.into();
        let violation_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphCompositionInvariantViolation,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("invariant_family"),
            invariant_family.as_str(),
        )
        .seal();
        Self {
            invariant_family,
            message,
            violation_digest,
        }
    }

    pub fn invariant_family(&self) -> &str {
        &self.invariant_family
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn violation_digest(&self) -> &str {
        self.violation_digest.as_str()
    }

    pub fn violation_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.violation_digest
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ForgeQueryGraphCompositionInvariantPackContext<'a> {
    commands: &'a [ForgeQueryWriteCommand],
    breadth: &'a ForgeQueryGraphCompositionBreadth,
    program: &'a ForgeQueryGraphCompositionProgram,
}

impl<'a> ForgeQueryGraphCompositionInvariantPackContext<'a> {
    pub(crate) fn new(
        commands: &'a [ForgeQueryWriteCommand],
        breadth: &'a ForgeQueryGraphCompositionBreadth,
        program: &'a ForgeQueryGraphCompositionProgram,
    ) -> Self {
        Self {
            commands,
            breadth,
            program,
        }
    }

    pub fn commands(&self) -> &'a [ForgeQueryWriteCommand] {
        self.commands
    }

    pub fn graph_composition_breadth(&self) -> &'a ForgeQueryGraphCompositionBreadth {
        self.breadth
    }

    pub fn graph_composition_program(&self) -> &'a ForgeQueryGraphCompositionProgram {
        self.program
    }

    pub fn graph_composition_domain_invariant_summary(
        &self,
    ) -> ForgeQueryGraphCompositionDomainInvariantSummary {
        ForgeQueryGraphCompositionDomainInvariantSummary::derive(
            self.program,
            self.breadth,
            self.commands,
        )
    }
}
