use crate::candidate_screening::{
    ColorPermutation, MonodromyColorHolonomyCertificate, ScreeningSolverTranscript,
};

use super::replay_errors::GeneratedPatternReplayShapeError;
use super::replay_errors::{require_replay_non_empty, GeneratedPatternReplayError};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ColorPermutationRule {
    rule_id: String,
    from_color: String,
    to_color: String,
}

impl ColorPermutationRule {
    pub fn new(
        rule_id: impl Into<String>,
        from_color: impl Into<String>,
        to_color: impl Into<String>,
    ) -> Result<Self, GeneratedPatternReplayShapeError> {
        Ok(Self {
            rule_id: require_replay_non_empty(rule_id, "color_permutation_rule_id")?,
            from_color: require_replay_non_empty(from_color, "from_color")?,
            to_color: require_replay_non_empty(to_color, "to_color")?,
        })
    }

    pub fn stable_token(&self) -> String {
        format!("{}:{}:{}", self.rule_id, self.from_color, self.to_color)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorHolonomyLoopCertificate {
    certificate_id: String,
    tracked_tile_id: String,
    tracked_color: String,
    permutation_rules: Vec<ColorPermutationRule>,
}

impl ColorHolonomyLoopCertificate {
    pub fn builder(
        certificate_id: impl Into<String>,
        tracked_tile_id: impl Into<String>,
        tracked_color: impl Into<String>,
    ) -> ColorHolonomyLoopCertificateBuilder {
        ColorHolonomyLoopCertificateBuilder {
            certificate_id: certificate_id.into(),
            tracked_tile_id: tracked_tile_id.into(),
            tracked_color: tracked_color.into(),
            permutation_rules: Vec::new(),
        }
    }

    pub fn tracked_tile_id(&self) -> &str {
        &self.tracked_tile_id
    }

    pub fn tracked_color(&self) -> &str {
        &self.tracked_color
    }

    pub(crate) fn to_screening_certificate(
        &self,
    ) -> Result<MonodromyColorHolonomyCertificate, GeneratedPatternReplayError> {
        let mapping = self
            .permutation_rules
            .iter()
            .map(|rule| (rule.from_color.clone(), rule.to_color.clone()))
            .collect::<Vec<_>>();
        Ok(MonodromyColorHolonomyCertificate::new(
            self.certificate_id.clone(),
            self.tracked_color.clone(),
            vec![ColorPermutation::new(mapping)?],
            ScreeningSolverTranscript::new(
                "generated_pattern_replay",
                "phase4",
                self.stable_token(),
                "bounded_certificate",
            )?,
        )?)
    }

    pub fn stable_token(&self) -> String {
        let rules = self
            .permutation_rules
            .iter()
            .map(ColorPermutationRule::stable_token)
            .collect::<Vec<_>>()
            .join("|");
        format!(
            "{}:{}:{}:{}",
            self.certificate_id, self.tracked_tile_id, self.tracked_color, rules
        )
    }
}

#[derive(Clone, Debug)]
pub struct ColorHolonomyLoopCertificateBuilder {
    certificate_id: String,
    tracked_tile_id: String,
    tracked_color: String,
    permutation_rules: Vec<ColorPermutationRule>,
}

impl ColorHolonomyLoopCertificateBuilder {
    pub fn with_color_permutation<I, L, R>(
        mut self,
        rule_id: impl Into<String>,
        mapping: I,
    ) -> Result<Self, GeneratedPatternReplayShapeError>
    where
        I: IntoIterator<Item = (L, R)>,
        L: Into<String>,
        R: Into<String>,
    {
        let rule_id = require_replay_non_empty(rule_id, "color_permutation_rule_id")?;
        for (left, right) in mapping {
            self.permutation_rules.push(ColorPermutationRule::new(
                rule_id.clone(),
                left.into(),
                right.into(),
            )?);
        }
        self.permutation_rules.sort();
        Ok(self)
    }

    pub fn finish(self) -> Result<ColorHolonomyLoopCertificate, GeneratedPatternReplayShapeError> {
        if self.permutation_rules.is_empty() {
            return Err(GeneratedPatternReplayShapeError::EmptyField {
                field: "color_permutation_rules",
            });
        }
        Ok(ColorHolonomyLoopCertificate {
            certificate_id: require_replay_non_empty(
                self.certificate_id,
                "color_holonomy_certificate_id",
            )?,
            tracked_tile_id: require_replay_non_empty(self.tracked_tile_id, "tracked_tile_id")?,
            tracked_color: require_replay_non_empty(self.tracked_color, "tracked_color")?,
            permutation_rules: self.permutation_rules,
        })
    }
}
