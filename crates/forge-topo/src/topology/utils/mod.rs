//! Compatibility shim — re-exports from `change_detection` and `transactions` components.

pub use crate::change_detection::logic::diff_engine as diff;
pub use crate::transactions::logic::structural_signature as hashing;
