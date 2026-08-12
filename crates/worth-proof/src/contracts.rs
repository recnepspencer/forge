//! The up-front contract vocabulary.
//!
//! [`crate::prelude`] is for *progression* — recipes, stages, transitions,
//! outcomes. This module is for the question that comes before any of it:
//! **what makes this operation legal, and what could a caller still supply
//! that the runtime ought to derive?**
//!
//! It exists because a team that knew about this crate built a parallel one.
//! Someone designing an authority model reached for the recipe prelude, did
//! not find authority words, and hand-rolled fifteen types. The words were
//! either missing or filed under progression.
//!
//! # Choosing
//!
//! | Reach for | When you need to say | Carries |
//! |---|---|---|
//! | [`Proof`] | a structural fact holds — sorted, unique, disjoint | nothing; type-level |
//! | [`AuthorityWitness`] | the caller is in the authorized lane | nothing; type-level |
//! | [`AssumptionBasis`] | *what* was assumed, as data | a value |
//! | [`Branded`] | this value belongs to **this instance**, not a peer | nothing; type-level |
//! | [`Binding`] | the facts a capability was issued against | values, one per axis |
//! | [`LinearResource`] | this exists once and ends once | its id |
//! | [`TerminalReceipt`] | it ended, and here is which way | id and terminal |
//! | [`Performed`] | the action **ran** — not that it was allowed to | its outcome |
//! | [`Inverts`] | this action completed as the inverse of its predecessor | nothing; type-level |
//! | [`EvaluatedFreshness`] | how stale a basis is, decided by a source | the typed basis |
//!
//! # Failures these prevent, in the words of the audit that found them
//!
//! - **A witness cannot distinguish two instances of the same runtime.** It
//!   proves the lane, not which lane-holder. Two runtimes in one process
//!   accept each other's witnesses, and the type system is content. Reach for
//!   [`Branded`] within a scope, or an owner-held identity value compared at
//!   run time — never a witness.
//! - **`Admitted` is not `Performed`.** A recipe admitted for external effect
//!   may still never have escaped. A retry that consumes admission re-sends;
//!   a retry that consumes [`Performed`] cannot be reached until the effect
//!   occurred.
//! - **Eleven correct comparisons say nothing about the twelfth axis.**
//!   [`crate::binding_axes!`] makes the missing axis a compile error, because
//!   the audit question was never "is this comparison right."
//! - **A caller who supplies the clock reading chooses their own freshness.**
//!   [`evaluate_freshness`] samples the source itself; there is no parameter
//!   for a caller to fill in.
//! - **Causality is not current authority.** [`Inverts`] proves the historical
//!   relationship needed by a redo lane; the owner must still re-admit the
//!   redo against current policy, identity, and world truth.
//!
//! The unifying defect in all four: **the caller supplies what the runtime
//! must derive or prove.** When adding a type here, ask what a caller could
//! hold that would let them bypass it.

pub use crate::assumption::{
    evaluate_freshness, take_sample, AssumptionBasis, AuthorityRevalidationRequired,
    CurrentValidity, EvaluatedFreshness, FreshnessEvaluation, FreshnessPolicy, FreshnessSample,
    FreshnessScopedBasis, FreshnessSource, FreshnessVerdict, RebindRequired, StaleReadable,
};
pub use crate::binding::{Binding, BindingAxes};
pub use crate::brand::{with_brand, Brand, Branded};
pub use crate::effect::{
    prove_derivation, prove_inversion, ActionMarker, DerivedFrom, InverseOf, Inverts, Performed,
};
pub use crate::linear::{LinearResource, TerminalReceipt, TerminalState};
pub use crate::proof::{
    AuthorityMarker, AuthorityProves, AuthorityWitness, CapabilityMarker, CapabilityWitness, Proof,
    ProofMarker,
};
