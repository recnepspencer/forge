mod authority;
mod witness;

pub(super) use authority::{classify_quarantine_authority, LayoutQuarantineAuthorityClass};
pub use witness::LayoutQuarantineWitness;
