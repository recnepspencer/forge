use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use forge_relational::facade::identity::KindId;

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
    relation_kind_id: Option<KindId>,
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
            relation_kind_id: None,
            declared_symbol,
        }
    }

    pub(crate) fn with_relation_kind_id(mut self, relation_kind_id: KindId) -> Self {
        self.relation_kind_id = Some(relation_kind_id);
        self
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

    pub fn relation_kind_id(&self) -> Option<KindId> {
        self.relation_kind_id
    }

    pub fn declared_symbol(&self) -> Option<&str> {
        self.declared_symbol.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphCompositionProgram {
    steps: Vec<ForgeQueryGraphCompositionProgramStep>,
    program_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphCompositionProgram {
    pub(crate) fn new(
        steps: Vec<ForgeQueryGraphCompositionProgramStep>,
        breadth: &ForgeQueryGraphCompositionBreadth,
    ) -> Self {
        let step_digests = steps
            .iter()
            .map(|step| {
                forge_query_evidence_identity(
                    ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-program-step",
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("component"),
                    step.component_index(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), step.kind().as_str())
                .field_value(
                    ForgeQueryEvidenceTag::new("declared_collection"),
                    step.declared_collection(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("relation_kind_id"),
                    step.relation_kind_id()
                        .map(|kind_id| kind_id.0.to_string())
                        .as_deref(),
                )
                .optional_value(
                    ForgeQueryEvidenceTag::new("declared_symbol"),
                    step.declared_symbol(),
                )
                .seal()
            })
            .collect::<Vec<_>>();
        let program_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "graph-composition-program",
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("breadth"),
                    breadth.breadth_evidence_digest(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("step"),
                    step_digests.iter(),
                )
                .seal();
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
        self.program_digest.as_str()
    }

    pub fn program_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.program_digest
    }
}
