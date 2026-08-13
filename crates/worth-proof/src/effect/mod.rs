//! Evidence that something happened, and what may follow from it.
//!
//! The distinction milestone 9.16 Gate 8.7 turned on: a transition must
//! consume **proof that an action occurred**, not permission to attempt one.
//! `Admitted` says a recipe was allowed to run. [`Performed`] says it ran.
//! Q8.13 was exactly the gap between those two sentences, and the substrate
//! had no word for the second one.
//!
//! [`DerivedFrom`] and [`Inverts`] are the temporal counterpart to
//! `composition::{fork, join, family}`: those compose transitions
//! *structurally*, these say one transition legally follows another.
//!
//! **Scope.** Only the legality gate lives here. Chains, provenance, and
//! describing causality across a boundary are `worth-foundational`'s stated
//! territory — "evidence, provenance, lineage, support" — and a lineage chain
//! built here would be the wrong crate's vision.

mod causality;
mod performed;

pub use causality::{prove_derivation, prove_inversion, DerivedFrom, InverseOf, Inverts};
pub use performed::{ActionMarker, Performed};
