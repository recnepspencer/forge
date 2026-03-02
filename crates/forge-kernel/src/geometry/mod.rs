//! Unified geometry domain — handle → geometry bindings.
//!
//! DOMAIN: Single source of truth for all geometric properties attached
//! to topology entities. Uses the generic `PropertyLayer<K, V>` pattern
//! (from forge-core) to eliminate per-property CRUD boilerplate.
//!
//! ## Structure
//!
//! ```text
//! geometry/
//! ├── contracts/
//! │   └── view.rs        ← GeometryView trait (unified read interface)
//! ├── data/
//! │   ├── layer.rs       ← Re-export of PropertyLayer from forge-core
//! │   ├── position.rs    ← ExactPosition
//! │   ├── store.rs       ← GeometryStore (pure data)
//! │   └── draft.rs       ← GeometryDraft (pure data + commit/rollback)
//! ├── logic/
//! │   ├── coalescence.rs ← snap_or_coalesce_vertex
//! │   ├── eval.rs        ← build_position_lookup
//! │   ├── split.rs       ← propagate_curve_on_split
//! │   ├── transforms.rs  ← transform_geometry, inverse_transform_geometry
//! │   ├── validation.rs  ← validate_bindings, validate_completeness
//! │   ├── tolerance.rs   ← compute_model_scale, GeometryToleranceProvider
//! │   └── source_adapter.rs ← GeometrySourceAdapter
//! └── facade.rs
//! ```

mod contracts;
mod data;
mod logic;

pub mod facade;
