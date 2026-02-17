//! FPU initialization for environmental isolation (Doctrine D8).

/// Resets FPU control words to standard IEEE 754 defaults:
/// - Disables DAZ/FTZ (denormals are valid geometry)
/// - Sets rounding mode to Nearest
///
/// Currently a no-op on aarch64 where denormals are handled correctly
/// by default. Will implement FPU control word reset for x86-64 targets
/// in Milestone 0.2.
#[cfg(feature = "strict_env")]
pub fn init_fpu() {
}
