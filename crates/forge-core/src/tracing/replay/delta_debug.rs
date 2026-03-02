//! Delta-debug binary search for minimal failure-inducing step.
//!
//! DOMAIN: Causal replay infrastructure (P3.2). Given a chain of N operations
//! where one step introduces a failure, binary-searches for the exact step.
//!
//! DEPENDENCIES: `forge_core::KernelError`

use crate::errors::KernelError;

/// Result of a delta-debug binary search.
#[derive(Debug, Clone)]
pub struct DeltaDebugResult {
    /// The minimal step index that first produces the failure.
    failing_step: usize,
    /// Total number of probe calls made during the search.
    probes_used: usize,
    /// Total steps in the original chain.
    total_steps: usize,
}

impl DeltaDebugResult {
    /// The minimal step that produces the failure.
    pub fn get_failing_step(&self) -> usize {
        self.failing_step
    }

    /// How many probe calls were needed.
    pub fn get_probes_used(&self) -> usize {
        self.probes_used
    }

    /// Total steps in the chain.
    pub fn get_total_steps(&self) -> usize {
        self.total_steps
    }
}

impl std::fmt::Display for DeltaDebugResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "DeltaDebug: failure at step {}/{} (found in {} probes)",
            self.failing_step, self.total_steps, self.probes_used
        )
    }
}

/// Binary search over an operation chain to find the minimal step that
/// introduces a failure.
///
/// The probe function `is_failing(step)` must return `Ok(true)` if the
/// prefix `[0..=step]` exhibits the failure, and `Ok(false)` otherwise.
///
/// Preconditions (validated):
/// - `step_count >= 2` (need at least a passing and failing step)
/// - `is_failing(step_count - 1)` must return `true` (the full chain fails)
///
/// Returns the index of the first step where the failure appears.
pub fn delta_debug<F>(step_count: usize, mut is_failing: F) -> Result<DeltaDebugResult, KernelError>
where
    F: FnMut(usize) -> Result<bool, KernelError>,
{
    if step_count < 2 {
        return Err(KernelError::InternalError {
            message: format!("delta_debug requires at least 2 steps, got {}", step_count),
            context: None,
        });
    }

    let mut probes = 0;

    probes += 1;
    let last_fails = is_failing(step_count - 1)?;
    if !last_fails {
        return Err(KernelError::InternalError {
            message: "delta_debug: the full chain does not fail — nothing to bisect".into(),
            context: None,
        });
    }

    let mut lo: usize = 0;
    let mut hi: usize = step_count - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        probes += 1;
        let mid_fails = is_failing(mid)?;

        if mid_fails {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }

    Ok(DeltaDebugResult {
        failing_step: lo,
        probes_used: probes,
        total_steps: step_count,
    })
}
