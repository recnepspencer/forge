//! Apply Euler operators transactionally via an existing MutableDraft.
//!
//! DOMAIN: Thin orchestration that runs a caller-provided closure of Euler op
//! calls against an already-open `MutableDraft`. The caller (forge-kernel,
//! wrapped in a `KernelDraft`) owns the draft lifecycle; this step is
//! responsible only for executing the operation sequence cleanly.
//!
//! Geometry mutations happen via separate `GeometryPatch` — not here.
//! This step is purely structural (D6 atomic topology mutation).
//!
//! DEPENDENCIES: forge-topo (MutableDraft), forge-core (KernelError)

use forge_core::KernelError;
use forge_topo::state::MutableDraft;

/// Execute a sequence of Euler operators on an open `MutableDraft`.
///
/// The closure receives `&mut MutableDraft` and must return `Ok(R)` on
/// success. On failure the draft is abandoned (caller must handle rollback
/// at the `KernelDraft` level). The draft is NOT committed here — the step
/// is composable; the caller commits when all steps succeed.
///
/// # Pattern
/// ```ignore
/// let (result, audit) = PipelineBuilder::start(ctx, initial_state)
///     .then(&ApplyEulerOps, |state, ctx| {
///         apply_euler_ops(ctx.draft_mut(), |draft| {
///             let vid = draft.insert_vertex(...)?;
///             // ...
///             Ok(vid)
///         })
///     })?
///     .finish();
/// ```
pub fn apply_euler_ops<R, F>(draft: &mut MutableDraft, ops: F) -> Result<R, KernelError>
where
    F: FnOnce(&mut MutableDraft) -> Result<R, KernelError>,
{
    ops(draft)
}
