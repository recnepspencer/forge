use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphCompositionBreadth, WorthQueryGraphCompositionDomainInvariantSummary,
    WorthQueryGraphCompositionProgram, WorthQueryWriteCommand,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphCompositionInvariantPackViolation {
    invariant_family: String,
    message: String,
    violation_digest: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphCompositionInvariantPackViolation {
    pub fn new(invariant_family: impl Into<String>, message: impl Into<String>) -> Self {
        let invariant_family = invariant_family.into();
        let message = message.into();
        let violation_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphCompositionInvariantViolation,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("invariant_family"),
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

    pub fn violation_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.violation_digest
    }
}

#[derive(Clone, Copy, Debug)]
pub struct WorthQueryGraphCompositionInvariantPackContext<'a> {
    commands: &'a [WorthQueryWriteCommand],
    breadth: &'a WorthQueryGraphCompositionBreadth,
    program: &'a WorthQueryGraphCompositionProgram,
}

impl<'a> WorthQueryGraphCompositionInvariantPackContext<'a> {
    pub(crate) fn new(
        commands: &'a [WorthQueryWriteCommand],
        breadth: &'a WorthQueryGraphCompositionBreadth,
        program: &'a WorthQueryGraphCompositionProgram,
    ) -> Self {
        Self {
            commands,
            breadth,
            program,
        }
    }

    pub fn commands(&self) -> &'a [WorthQueryWriteCommand] {
        self.commands
    }

    pub fn graph_composition_breadth(&self) -> &'a WorthQueryGraphCompositionBreadth {
        self.breadth
    }

    pub fn graph_composition_program(&self) -> &'a WorthQueryGraphCompositionProgram {
        self.program
    }

    pub fn graph_composition_domain_invariant_summary(
        &self,
    ) -> WorthQueryGraphCompositionDomainInvariantSummary {
        WorthQueryGraphCompositionDomainInvariantSummary::derive(
            self.program,
            self.breadth,
            self.commands,
        )
    }
}
