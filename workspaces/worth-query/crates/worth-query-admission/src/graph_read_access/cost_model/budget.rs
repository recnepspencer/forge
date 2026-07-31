use super::{WorthQueryGraphReadAccessCostEstimate, WorthQueryGraphReadAccessCostEstimateDigest};
use crate::admission_digest::hash_parts;

pub(crate) const DEFAULT_INLINE_EPHEMERAL_INDEX_BYTES: usize = 5120;
pub(crate) const DEFAULT_INLINE_EPHEMERAL_RESULT_BYTES: usize = 2048;
pub(crate) const DEFAULT_INLINE_EPHEMERAL_INTERMEDIATE_SET_SIZE: usize = 16;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBudgetDigest(String);

impl WorthQueryGraphReadBudgetDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBudget {
    digest: WorthQueryGraphReadBudgetDigest,
    max_inline_index_bytes: usize,
    max_inline_result_bytes: usize,
    max_inline_intermediate_set_size: usize,
}

impl WorthQueryGraphReadBudget {
    pub fn inline_ephemeral_default() -> Self {
        Self::new(
            DEFAULT_INLINE_EPHEMERAL_INDEX_BYTES,
            DEFAULT_INLINE_EPHEMERAL_RESULT_BYTES,
            DEFAULT_INLINE_EPHEMERAL_INTERMEDIATE_SET_SIZE,
        )
    }

    pub fn bounded(
        max_inline_index_bytes: usize,
        max_inline_result_bytes: usize,
        max_inline_intermediate_set_size: usize,
    ) -> Self {
        Self::new(
            max_inline_index_bytes,
            max_inline_result_bytes,
            max_inline_intermediate_set_size,
        )
    }

    pub fn digest(&self) -> &WorthQueryGraphReadBudgetDigest {
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
        estimate: &WorthQueryGraphReadAccessCostEstimate,
    ) -> WorthQueryGraphReadBudgetCheck {
        let class = if estimate.supported().index_bytes() > self.max_inline_index_bytes
            || estimate.supported().result_bytes() > self.max_inline_result_bytes
            || estimate.intrinsic().intermediate_set_size() > self.max_inline_intermediate_set_size
        {
            WorthQueryGraphReadBudgetClass::exceeds_inline_ephemeral_budget()
        } else {
            WorthQueryGraphReadBudgetClass::inline_ephemeral_candidate()
        };
        WorthQueryGraphReadBudgetCheck::new(self, estimate.digest(), class)
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
            digest: WorthQueryGraphReadBudgetDigest::from_parts(&parts),
            max_inline_index_bytes,
            max_inline_result_bytes,
            max_inline_intermediate_set_size,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadBudgetClassKind {
    InlineEphemeralCandidate,
    ExceedsInlineEphemeralBudget,
}

impl WorthQueryGraphReadBudgetClassKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InlineEphemeralCandidate => "inline_ephemeral_candidate",
            Self::ExceedsInlineEphemeralBudget => "exceeds_inline_ephemeral_budget",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBudgetClass {
    kind: WorthQueryGraphReadBudgetClassKind,
}

impl WorthQueryGraphReadBudgetClass {
    pub fn kind(&self) -> &WorthQueryGraphReadBudgetClassKind {
        &self.kind
    }

    pub fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }

    fn inline_ephemeral_candidate() -> Self {
        Self {
            kind: WorthQueryGraphReadBudgetClassKind::InlineEphemeralCandidate,
        }
    }

    fn exceeds_inline_ephemeral_budget() -> Self {
        Self {
            kind: WorthQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBudgetCheck {
    budget_digest: String,
    cost_estimate_digest: String,
    max_inline_index_bytes: usize,
    max_inline_result_bytes: usize,
    max_inline_intermediate_set_size: usize,
    class: WorthQueryGraphReadBudgetClass,
    inline_ephemeral_allowance: WorthQueryGraphReadInlineEphemeralAllowance,
}

impl WorthQueryGraphReadBudgetCheck {
    pub fn budget_digest(&self) -> &str {
        &self.budget_digest
    }

    pub fn cost_estimate_digest(&self) -> &str {
        &self.cost_estimate_digest
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

    pub fn class(&self) -> &WorthQueryGraphReadBudgetClass {
        &self.class
    }

    pub fn inline_ephemeral_allowance(&self) -> &WorthQueryGraphReadInlineEphemeralAllowance {
        &self.inline_ephemeral_allowance
    }

    fn new(
        budget: &WorthQueryGraphReadBudget,
        estimate_digest: &WorthQueryGraphReadAccessCostEstimateDigest,
        class: WorthQueryGraphReadBudgetClass,
    ) -> Self {
        let inline_ephemeral_allowance =
            WorthQueryGraphReadInlineEphemeralAllowance::from_class(&class);
        Self {
            budget_digest: budget.digest().as_str().to_string(),
            cost_estimate_digest: estimate_digest.as_str().to_string(),
            max_inline_index_bytes: budget.max_inline_index_bytes(),
            max_inline_result_bytes: budget.max_inline_result_bytes(),
            max_inline_intermediate_set_size: budget.max_inline_intermediate_set_size(),
            class,
            inline_ephemeral_allowance,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryGraphReadInlineEphemeralAllowanceKind {
    Allowed,
    Rejected,
}

impl WorthQueryGraphReadInlineEphemeralAllowanceKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Rejected => "rejected",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadInlineEphemeralAllowance {
    kind: WorthQueryGraphReadInlineEphemeralAllowanceKind,
}

impl WorthQueryGraphReadInlineEphemeralAllowance {
    pub fn kind(&self) -> &WorthQueryGraphReadInlineEphemeralAllowanceKind {
        &self.kind
    }

    pub fn as_str(&self) -> &'static str {
        self.kind.as_str()
    }

    fn from_class(class: &WorthQueryGraphReadBudgetClass) -> Self {
        let kind = match class.kind() {
            WorthQueryGraphReadBudgetClassKind::InlineEphemeralCandidate => {
                WorthQueryGraphReadInlineEphemeralAllowanceKind::Allowed
            }
            WorthQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget => {
                WorthQueryGraphReadInlineEphemeralAllowanceKind::Rejected
            }
        };
        Self { kind }
    }
}
