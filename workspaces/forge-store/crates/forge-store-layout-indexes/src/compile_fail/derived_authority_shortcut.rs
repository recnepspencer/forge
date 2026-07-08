//! ```compile_fail
//! use forge_store_contracts::ArtifactFamilyAuthorityClass;
//! use forge_store_layout_indexes::layout_declarations;
//!
//! let _ = layout_declarations()
//!     .require_production_authority(ArtifactFamilyAuthorityClass::Authoritative);
//! ```
