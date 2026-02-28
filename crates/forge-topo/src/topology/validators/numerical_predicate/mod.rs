//! Numerical and predicate pipeline validators.
//!
//! DOMAIN: Float vs interval vs exact predicate divergence
//! classification, interval bound soundness, fallback escalation
//! policy, condition number triggers, and bit-budget accounting.
//!
//! VALIDATORS (from validators.md §11):
//! - ValidatePredicateDivergenceClassification
//! - ValidateIntervalBoundsSoundness
//! - ValidateFallbackEscalationPolicy
//! - ValidateConditionNumberTriggers
//! - ValidateBitBudgetAccounting
//!
//! DEPENDENCIES: `forge-math` (predicates, exact arithmetic)
