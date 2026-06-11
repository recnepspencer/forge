use super::{reject_duplicate_identity, require_non_empty};
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotifSeedDeclaration {
    motif_id: String,
    source_family: Option<String>,
    novelty_signature: Option<String>,
}

impl MotifSeedDeclaration {
    pub fn new(motif_id: impl Into<String>) -> Self {
        Self::try_new(motif_id).expect("motif_id must be non-empty")
    }

    pub fn try_new(
        motif_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            motif_id: require_non_empty(motif_id, "motif_id")?,
            source_family: None,
            novelty_signature: None,
        })
    }

    pub fn with_source_family(self, source_family: impl Into<String>) -> Self {
        self.try_with_source_family(source_family)
            .expect("source_family must be non-empty")
    }

    pub fn try_with_source_family(
        mut self,
        source_family: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.source_family = Some(require_non_empty(source_family, "source_family")?);
        Ok(self)
    }

    pub fn with_novelty_signature(self, novelty_signature: impl Into<String>) -> Self {
        self.try_with_novelty_signature(novelty_signature)
            .expect("novelty_signature must be non-empty")
    }

    pub fn try_with_novelty_signature(
        mut self,
        novelty_signature: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.novelty_signature = Some(require_non_empty(novelty_signature, "novelty_signature")?);
        Ok(self)
    }

    pub(crate) fn motif_id(&self) -> &str {
        &self.motif_id
    }

    pub(crate) fn source_family(&self) -> Option<&str> {
        self.source_family.as_deref()
    }

    pub(crate) fn novelty_signature(&self) -> Option<&str> {
        self.novelty_signature.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalForcingStudyDeclaration {
    study_id: String,
    motif_ref: String,
    terminals: Vec<String>,
    relation_goal: Option<String>,
}

impl TerminalForcingStudyDeclaration {
    pub fn new(study_id: impl Into<String>, motif_ref: impl Into<String>) -> Self {
        Self::try_new(study_id, motif_ref).expect("study_id and motif_ref must be non-empty")
    }

    pub fn try_new(
        study_id: impl Into<String>,
        motif_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            study_id: require_non_empty(study_id, "study_id")?,
            motif_ref: require_non_empty(motif_ref, "motif_ref")?,
            terminals: Vec::new(),
            relation_goal: None,
        })
    }

    pub fn with_terminal(
        self,
        terminal: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        let mut next = self;
        let terminal = require_non_empty(terminal, "terminal")?;
        reject_duplicate_identity(&next.terminals, &terminal, "terminal")?;
        next.terminals.push(terminal);
        next.terminals.sort();
        Ok(next)
    }

    pub fn with_relation_goal(self, relation_goal: impl Into<String>) -> Self {
        self.try_with_relation_goal(relation_goal)
            .expect("relation_goal must be non-empty")
    }

    pub fn try_with_relation_goal(
        mut self,
        relation_goal: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.relation_goal = Some(require_non_empty(relation_goal, "relation_goal")?);
        Ok(self)
    }

    pub(crate) fn study_id(&self) -> &str {
        &self.study_id
    }

    pub(crate) fn motif_ref(&self) -> &str {
        &self.motif_ref
    }

    pub(crate) fn terminals(&self) -> &[String] {
        &self.terminals
    }

    pub(crate) fn relation_goal(&self) -> Option<&str> {
        self.relation_goal.as_deref()
    }
}
