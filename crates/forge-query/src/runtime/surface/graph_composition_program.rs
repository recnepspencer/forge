use crate::identity::hash_parts;

use super::ForgeQueryGraphCompositionBreadth;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphCompositionProgramStepKind {
    SymbolicEntityDeclaration,
    SymbolicEntityFollowupMutation,
    RelationDeclaration,
    SymbolicRelationDeclaration,
    SymbolicRelationFollowupMutation,
    SymbolicRelationRetirement,
    ExistingTargetFollowupMutation,
    ExistingTargetRetarget,
    ExistingTargetSupersession,
    ExistingTargetRetirement,
    ExistingTargetVerifiedFollowupMutation,
    ExistingTargetVerifiedRetarget,
    ExistingTargetVerifiedSupersession,
    ExistingTargetVerifiedRetirement,
}

impl ForgeQueryGraphCompositionProgramStepKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SymbolicEntityDeclaration => "symbolic-entity-declaration",
            Self::SymbolicEntityFollowupMutation => "symbolic-entity-followup-mutation",
            Self::RelationDeclaration => "relation-declaration",
            Self::SymbolicRelationDeclaration => "symbolic-relation-declaration",
            Self::SymbolicRelationFollowupMutation => "symbolic-relation-followup-mutation",
            Self::SymbolicRelationRetirement => "symbolic-relation-retirement",
            Self::ExistingTargetFollowupMutation => "existing-target-followup-mutation",
            Self::ExistingTargetRetarget => "existing-target-retarget",
            Self::ExistingTargetSupersession => "existing-target-supersession",
            Self::ExistingTargetRetirement => "existing-target-retirement",
            Self::ExistingTargetVerifiedFollowupMutation => {
                "existing-target-verified-followup-mutation"
            }
            Self::ExistingTargetVerifiedRetarget => "existing-target-verified-retarget",
            Self::ExistingTargetVerifiedSupersession => "existing-target-verified-supersession",
            Self::ExistingTargetVerifiedRetirement => "existing-target-verified-retirement",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionProgramStep {
    component_index: usize,
    kind: ForgeQueryGraphCompositionProgramStepKind,
    declared_collection: String,
    declared_symbol: Option<String>,
}

impl ForgeQueryGraphCompositionProgramStep {
    pub(crate) fn new(
        component_index: usize,
        kind: ForgeQueryGraphCompositionProgramStepKind,
        declared_collection: impl Into<String>,
        declared_symbol: Option<String>,
    ) -> Self {
        Self {
            component_index,
            kind,
            declared_collection: declared_collection.into(),
            declared_symbol,
        }
    }

    pub fn component_index(&self) -> usize {
        self.component_index
    }

    pub fn kind(&self) -> ForgeQueryGraphCompositionProgramStepKind {
        self.kind
    }

    pub fn declared_collection(&self) -> &str {
        &self.declared_collection
    }

    pub fn declared_symbol(&self) -> Option<&str> {
        self.declared_symbol.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionProgram {
    steps: Vec<ForgeQueryGraphCompositionProgramStep>,
    program_digest: String,
}

impl ForgeQueryGraphCompositionProgram {
    pub(crate) fn new(
        steps: Vec<ForgeQueryGraphCompositionProgramStep>,
        breadth: &ForgeQueryGraphCompositionBreadth,
    ) -> Self {
        let program_digest = hash_parts(
            &std::iter::once("forge_query_graph_composition_program_v1".to_string())
                .chain(std::iter::once(format!(
                    "breadth:{}",
                    breadth.breadth_digest()
                )))
                .chain(steps.iter().map(|step| {
                    format!(
                        "{}:{}:{}:{}",
                        step.component_index(),
                        step.kind().as_str(),
                        step.declared_collection(),
                        step.declared_symbol().unwrap_or("none")
                    )
                }))
                .collect::<Vec<_>>(),
        );
        Self {
            steps,
            program_digest,
        }
    }

    pub(crate) fn empty() -> Self {
        Self::new(Vec::new(), &ForgeQueryGraphCompositionBreadth::empty())
    }

    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }

    pub fn component_count(&self) -> usize {
        self.steps.len()
    }

    pub fn steps(&self) -> &[ForgeQueryGraphCompositionProgramStep] {
        &self.steps
    }

    pub fn program_digest(&self) -> &str {
        &self.program_digest
    }
}
