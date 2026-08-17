//! Fixed certification progressions exposed through the public Worth UI facade.

pub use worth_ui_runtime::certification_support::UiGateDPinWorldEvidence;

/// Run the Gate-D native courtroom through Worth UI's application-facing
/// certification facade. The runtime and native host remain hidden behind the
/// same facade boundary used by the application composition root.
pub fn run_native_gate_d_pin_world() -> UiGateDPinWorldEvidence {
    worth_ui_runtime::certification_support::run_gate_d_pin_world()
}
