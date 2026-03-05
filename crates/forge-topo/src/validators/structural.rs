//! Structural topology validation (commit-time invariant checking).
//!
//! DOMAIN: Dispatches all invariant validators via `validator_for()`.
//! `ValidationLevel` maps to a `ValidatorCost` ceiling that filters
//! which validators run at commit time.

use crate::b_rep::TopologyArena;
use crate::validators::validate::ValidationLevel;
use crate::validators::invariant_id::{InvariantId, ValidatorCost, validator_for};
use forge_core::KernelError;

/// Validate structural topology of an arena with the specified strictness.
///
/// Called automatically by `MutableDraft::commit()`. Runs every registered
/// invariant validator whose cost is at or below the ceiling implied by
/// the `ValidationLevel`.
///
/// | Level          | Max Cost    | What runs                                    |
/// |:---------------|:------------|:---------------------------------------------|
/// | `None`         | —           | Nothing                                      |
/// | `Minimal`      | `Cheap`     | Pointer coherence, basic loop/radial checks  |
/// | `Intermediate` | `Medium`    | + ownership, membership, hierarchy           |
/// | `Full`         | `Expensive` | Everything incl. Euler, shell closure, disks |
pub fn validate_topology(arena: &TopologyArena, level: ValidationLevel) -> Result<(), KernelError> {
    let max_cost = match level {
        ValidationLevel::None => return Ok(()),
        ValidationLevel::Minimal => ValidatorCost::Cheap,
        ValidationLevel::Intermediate => ValidatorCost::Medium,
        ValidationLevel::Full => ValidatorCost::Expensive,
    };

    for &id in InvariantId::ALL {
        let entry = validator_for(id);
        if entry.cost <= max_cost {
            (entry.check)(arena)?;
        }
    }

    Ok(())
}

/// Re-export for external callers that need manifold checks directly.
pub use super::shell_closure::validate_manifold_edges;
