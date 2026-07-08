//! ```compile_fail
//! use forge_store_contracts::DurableArtifactFamilyId;
//! use forge_store_layout_indexes::layout_declarations;
//! use forge_store_security::{StoreKeyScope, StoreTenantScope};
//!
//! let declaration = layout_declarations()
//!     .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
//!     .unwrap();
//! let classification = layout_declarations().classify_family(declaration);
//! let role = layout_declarations().declare_authority_role(classification);
//! let accuracy = layout_declarations().declare_derived_accuracy_class(role);
//! let _ = layout_declarations().require_scope_partition(
//!     accuracy,
//!     StoreTenantScope::StoreInternal,
//!     StoreKeyScope::StoreManagedRoot,
//! );
//! ```
