//! ```compile_fail
//! use forge_store_contracts::DurableArtifactFamilyId;
//! use forge_store_layout_indexes::layout_declarations;
//!
//! let declaration = layout_declarations()
//!     .declaration(DurableArtifactFamilyId::PhysicalPage)
//!     .unwrap();
//! let classification = layout_declarations().classify_family(declaration);
//! let role = layout_declarations().declare_authority_role(classification);
//! let accuracy = layout_declarations().declare_derived_accuracy_class(role);
//! let _ = layout_declarations().declare_comparator_law(accuracy);
//! ```
