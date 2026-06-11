use super::replay_errors::{require_replay_non_empty, GeneratedPatternReplayShapeError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct PeriodicTranslationRule {
    rule_id: String,
    source_tile_id: String,
    target_tile_id: String,
    lattice_vector_id: String,
    color_preserved: bool,
}

impl PeriodicTranslationRule {
    pub fn new(
        rule_id: impl Into<String>,
        source_tile_id: impl Into<String>,
        target_tile_id: impl Into<String>,
    ) -> PeriodicTranslationRuleBuilder {
        PeriodicTranslationRuleBuilder {
            rule_id: rule_id.into(),
            source_tile_id: source_tile_id.into(),
            target_tile_id: target_tile_id.into(),
            lattice_vector_id: None,
            color_preserved: false,
        }
    }

    pub fn rule_id(&self) -> &str {
        &self.rule_id
    }

    pub fn source_tile_id(&self) -> &str {
        &self.source_tile_id
    }

    pub fn target_tile_id(&self) -> &str {
        &self.target_tile_id
    }

    pub fn lattice_vector_id(&self) -> &str {
        &self.lattice_vector_id
    }

    pub fn color_preserved(&self) -> bool {
        self.color_preserved
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.rule_id,
            self.source_tile_id,
            self.target_tile_id,
            self.lattice_vector_id,
            self.color_preserved
        )
    }
}

#[derive(Clone, Debug)]
pub struct PeriodicTranslationRuleBuilder {
    rule_id: String,
    source_tile_id: String,
    target_tile_id: String,
    lattice_vector_id: Option<String>,
    color_preserved: bool,
}

impl PeriodicTranslationRuleBuilder {
    pub fn with_translation(
        mut self,
        lattice_vector_id: impl Into<String>,
    ) -> Result<Self, GeneratedPatternReplayShapeError> {
        self.lattice_vector_id = Some(require_replay_non_empty(
            lattice_vector_id,
            "translation_lattice_vector_id",
        )?);
        Ok(self)
    }

    pub fn with_color_preserved(
        mut self,
    ) -> Result<PeriodicTranslationRule, GeneratedPatternReplayShapeError> {
        self.color_preserved = true;
        self.finish()
    }

    pub fn finish(self) -> Result<PeriodicTranslationRule, GeneratedPatternReplayShapeError> {
        Ok(PeriodicTranslationRule {
            rule_id: require_replay_non_empty(self.rule_id, "translation_rule_id")?,
            source_tile_id: require_replay_non_empty(self.source_tile_id, "source_tile_id")?,
            target_tile_id: require_replay_non_empty(self.target_tile_id, "target_tile_id")?,
            lattice_vector_id: self.lattice_vector_id.ok_or(
                GeneratedPatternReplayShapeError::EmptyField {
                    field: "translation_lattice_vector_id",
                },
            )?,
            color_preserved: self.color_preserved,
        })
    }
}
