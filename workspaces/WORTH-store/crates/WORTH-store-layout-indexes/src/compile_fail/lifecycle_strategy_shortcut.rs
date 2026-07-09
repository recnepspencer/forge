//! ```compile_fail
//! use worth_store_contracts::DurableArtifactFamilyId;
//! use worth_store_layout_indexes::layout_declarations;
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
