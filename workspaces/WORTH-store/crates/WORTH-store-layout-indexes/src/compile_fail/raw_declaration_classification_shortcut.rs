//! ```compile_fail
//! use worth_store_contracts::DurableArtifactFamilyId;
//! use worth_store_layout_indexes::layout_declarations;
//!
//! let declaration = layout_declarations()
//!     .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
//!     .unwrap();
//! let _ = declaration.authority();
//! ```
