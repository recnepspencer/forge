//! ```compile_fail
//! use forge_store_contracts::DurableArtifactFamilyId;
//! use forge_store_layout_indexes::layout_declarations;
//!
//! let declaration = layout_declarations()
//!     .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
//!     .unwrap();
//! let _ = declaration.authority();
//! ```
