use super::{ForgeQueryGraphReadAccessCostEstimate, ForgeQueryGraphReadAccessCostEstimateDigest};
use crate::identity::hash_parts;

pub(crate) const DEFAULT_INLINE_EPHEMERAL_INDEX_BYTES: usize = 4096;
pub(crate) const DEFAULT_INLINE_EPHEMERAL_RESULT_BYTES: usize = 2048;
pub(crate) const DEFAULT_INLINE_EPHEMERAL_INTERMEDIATE_SET_SIZE: usize = 16;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBudgetDigest(String);

impl ForgeQueryGraphReadBudgetDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBudget {
    digest: ForgeQueryGraphReadBudgetDigest,
    max_inline_index_bytes: usize,
    max_inline_result_bytes: usize,
    max_inline_intermediate_set_size: usize,
}

impl ForgeQueryGraphReadBudget {
    pub fn inline_ephemeral_default() -> Self {
        Self::new(
            DEFAULT_INLINE_EPHEMERAL_INDEX_BYTES,
            DEFAULT_INLINE_EPHEMERAL_RESULT_BYTES,
            DEFAULT_INLINE_EPHEMERAL_INTERMEDIATE_SET_SIZE,
        )
    }

    pub fn digest(&self) -> &ForgeQueryGraphReadBudgetDigest {
        &self.digest
    }

    pub fn max_inline_index_bytes(&self) -> usize {
        self.max_inline_index_bytes
    }

    pub fn max_inline_result_bytes(&self) -> usize {
        self.max_inline_result_bytes
    }

    pub fn max_inline_intermediate_set_size(&self) -> usize {
        self.max_inline_intermediate_set_size
    }

    pub fn check_supported_cost(
        &self,
        estimate: &ForgeQueryGraphReadAccessCostEstimate,
    ) -> ForgeQueryGraphReadBudgetCheck {
        let class = if estimate.supported().index_bytes() > self.max_inline_index_bytes
            || estimate.supported().result_bytes() > self.max_inline_result_bytes
            || estimate.intrinsic().intermediate_set_size() > self.max_inline_intermediate_set_size
        {
            ForgeQueryGraphReadBudgetClass::exceeds_inline_ephemeral_budget()
        } else {
            ForgeQueryGraphReadBudgetClass::inline_ephemeral_candidate()
        };
        ForgeQueryGraphReadBudgetCheck::new(self, estimate.digest(), class)
    }

    fn new(
        max_inline_index_bytes: usize,
        max_inline_result_bytes: usize,
        max_inline_intermediate_set_size: usize,
    ) -> Self {
        let parts = vec![
            format!("max_index:{max_inline_index_bytes}"),
            format!("max_result:{max_inline_result_bytes}"),
            format!("max_intermediate:{max_inline_intermediate_set_size}"),
        ];
        Self {
            digest: ForgeQueryGraphReadBudgetDigest::from_parts(&parts),
            max_inline_index_bytes,
            max_inline_result_bytes,
            max_inline_intermediate_set_size,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryGraphReadBudgetClassKind {
    InlineEphemeralCandidate,
    ExceedsInlineEphemeralBudget,
}

impl ForgeQueryGraphReadBudgetClassKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineEphemeralCandidate => "inline_ephemeral_candidate",
            Self::ExceedsInlineEphemeralBudget => "exceeds_inline_ephemeral_budget",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBudgetClass {
    kind: ForgeQueryGraphReadBudgetClassKind,
}

impl ForgeQueryGraphReadBudgetClass {
    pub fn kind(&self) -> &ForgeQueryGraphReadBudgetClassKind {
        &self.kind
    }

    pub fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }

    fn inline_ephemeral_candidate() -> Self {
        Self {
            kind: ForgeQueryGraphReadBudgetClassKind::InlineEphemeralCandidate,
        }
    }

    fn exceeds_inline_ephemeral_budget() -> Self {
        Self {
            kind: ForgeQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphReadBudgetCheck {
    budget_digest: String,
    cost_estimate_digest: String,
    class: ForgeQueryGraphReadBudgetClass,
}

impl ForgeQueryGraphReadBudgetCheck {
    pub fn budget_digest(&self) -> &str {
        &self.budget_digest
    }

    pub fn cost_estimate_digest(&self) -> &str {
        &self.cost_estimate_digest
    }

    pub fn class(&self) -> &ForgeQueryGraphReadBudgetClass {
        &self.class
    }

    fn new(
        budget: &ForgeQueryGraphReadBudget,
        estimate_digest: &ForgeQueryGraphReadAccessCostEstimateDigest,
        class: ForgeQueryGraphReadBudgetClass,
    ) -> Self {
        Self {
            budget_digest: budget.digest().as_str().to_string(),
            cost_estimate_digest: estimate_digest.as_str().to_string(),
            class,
        }
    }
}
