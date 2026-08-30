//! Stable semantic identities for Rust axes that enter portable Query meaning.

mod portable_type;
mod type_identity;
#[cfg(test)]
mod type_identity_tests;

pub use portable_type::WorthQueryPortableType;
pub use type_identity::WorthQueryPortableTypeIdentity;
