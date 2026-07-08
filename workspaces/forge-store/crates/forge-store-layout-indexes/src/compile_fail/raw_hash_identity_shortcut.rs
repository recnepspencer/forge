//! ```compile_fail
//! use forge_store_layout_indexes::{key_domain_law, PhysicalKeyDomain};
//!
//! let _ = key_domain_law().require_exact_hash_identity_claim(PhysicalKeyDomain::ArtifactEnvelope);
//! ```
