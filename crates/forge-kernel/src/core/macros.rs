//! Kernel policy macros.
//!
//! DOMAIN: Macro helpers for tolerance checking and decision logging.
//! DEPENDENCIES: `forge_core::DecisionTier`

/// Macro for cleanly checking tolerance and logging decisions.
///
/// Keeps math code readable while ensuring every tolerance decision
/// is logged (Doctrine D2).
///
/// # Usage
/// ```ignore
/// if check_tolerance!(ctx, spatial_tolerance, distance, DecisionKind::NearBoundary { threshold: spatial_tolerance }) {
///     return Ok(TriSign::Zero);
/// }
/// ```
#[macro_export]
macro_rules! check_tolerance {
    ($ctx:expr, $threshold:expr, $value:expr, $location:expr, $kind:expr) => {{
        if $value < $threshold {
            $ctx.log_decision(
                $kind,
                forge_core::DecisionTier::NearBoundary,
                $location,
                $value,
                $threshold,
            );
            true
        } else {
            false
        }
    }};
}
