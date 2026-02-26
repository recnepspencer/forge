//! Feature-sliced domain modules.
//!
//! DOMAIN: High-level modeling features (Extrude, Boolean, etc.)
//! INVARIANTS: Features must be independent and use Euler operators for mutation.
//! DEPENDENCIES: forge-topo (Euler operators, State), forge-geom (Evaluation)
//!
//! Each feature lives in its own subdirectory following the Bento Box pattern:
//! ```text
//! features/
//! ├── mod.rs          ← this file (Table of Contents)
//! └── extrude/        ← example feature
//!     ├── mod.rs      ← exports only
//!     ├── schema.rs   ← data definitions
//!     ├── eval.rs     ← business logic
//!     ├── topo.rs     ← Euler operator implementations
//!     └── tests.rs    ← unit tests
//! ```
//!
//! See `CONVENTIONS.md` and `docs/FILE_NAMING.md` for naming rules.

pub mod intent;
pub mod traits;

// Features will be added here as milestones are implemented:
pub mod tree;
pub mod wrappers;
pub mod pipeline;
// pub mod extrude;   // Phase 2.3
// pub mod boolean;   // Phase 2
// pub mod fillet;    // Phase 4
// pub mod chamfer;   // Phase 4
