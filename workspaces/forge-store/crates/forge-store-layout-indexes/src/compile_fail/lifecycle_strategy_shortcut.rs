//! ```compile_fail
//! use forge_store_contracts::DurableArtifactFamilyId;
//! use forge_store_layout_indexes::layout_declarations;
//!
//! let declaration = layout_declarations()
//!     .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
//!     .unwrap();
//! let _ = layout_declarations().require_production_authority(
//!     layout_declarations().classify_family(declaration),
//! );
//! let _ = layout_declarations().require_strategy_lifecycle(
//!     layout_declarations().classify_family(declaration),
//! );
//! ```
