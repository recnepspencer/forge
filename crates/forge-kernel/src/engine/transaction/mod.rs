//! Transactional state lifecycle for kernel operations.
//!
//! DOMAIN: Manages the full create → mutate → commit/rollback lifecycle
//! for topology + geometry state. "transaction" because that's what it is:
//! `KernelState` is the resting state, `KernelDraft` is the open transaction,
//! and `BRepWorkspace` wraps draft + config for operation use.
//!
//! ## Structure
//!
//! ```text
//! transaction/
//! ├── data/
//! │   ├── state.rs     ← KernelState (resting-state bundle)
//! │   └── summary.rs   ← FinalizationSummary, TopologyHashBoundary, etc.
//! ├── logic/
//! │   ├── draft.rs      ← KernelDraft (transactional mutation handle)
//! │   ├── workspace.rs  ← BRepWorkspace (lifecycle wrapper)
//! │   └── finalizer.rs  ← OperationFinalizer (drain + commit)
//! └── facade.rs
//! ```

mod data;
mod logic;

pub mod facade;
