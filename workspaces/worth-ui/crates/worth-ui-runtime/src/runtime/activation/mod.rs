//! Activation lane — staging, frame gate, atomic swap.

#[path = "../activation_staging/mod.rs"]
pub mod activation_staging;
#[path = "../atomic_plan_swap/mod.rs"]
pub mod atomic_plan_swap;
#[path = "../frame_activation_gate/mod.rs"]
pub mod frame_activation_gate;

mod gate;
mod swap;
mod transitions;

pub use transitions::WorthUiActivationLaneInput;
