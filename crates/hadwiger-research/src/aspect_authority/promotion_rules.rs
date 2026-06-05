use super::aspect_kinds::{require_non_empty, HadwigerAspectAuthorityError, HadwigerAspectKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerPromotionRuleDescriptor {
    target_aspect: HadwigerAspectKind,
    required_aspects: Vec<HadwigerAspectKind>,
    rule_name: String,
}

impl HadwigerPromotionRuleDescriptor {
    pub fn lower_bound_witness_rule(
        rule_name: impl Into<String>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        Ok(Self {
            target_aspect: HadwigerAspectKind::LowerBoundWitness,
            required_aspects: vec![
                HadwigerAspectKind::UnitDistanceEmbedding,
                HadwigerAspectKind::NotKColorable,
            ],
            rule_name: require_non_empty(rule_name, "promotion_rule_name")?,
        })
    }

    pub fn target_aspect(&self) -> HadwigerAspectKind {
        self.target_aspect
    }

    pub fn required_aspects(&self) -> &[HadwigerAspectKind] {
        &self.required_aspects
    }

    pub fn rule_name(&self) -> &str {
        &self.rule_name
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}
