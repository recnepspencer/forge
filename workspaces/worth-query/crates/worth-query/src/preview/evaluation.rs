#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadOnlyPreviewEvaluation(());

impl ReadOnlyPreviewEvaluation {
    fn new() -> Self {
        Self(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionEligiblePreviewEvaluation(());

impl PromotionEligiblePreviewEvaluation {
    fn new() -> Self {
        Self(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreviewEvaluationClass {
    ReadOnly(ReadOnlyPreviewEvaluation),
    PromotionEligible(PromotionEligiblePreviewEvaluation),
}

impl PreviewEvaluationClass {
    pub fn read_only() -> Self {
        Self::ReadOnly(ReadOnlyPreviewEvaluation::new())
    }

    pub fn promotion_eligible() -> Self {
        Self::PromotionEligible(PromotionEligiblePreviewEvaluation::new())
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadOnly(_) => "read_only",
            Self::PromotionEligible(_) => "promotion_eligible",
        }
    }
}
