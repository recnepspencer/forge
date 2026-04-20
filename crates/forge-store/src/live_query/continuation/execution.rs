#[path = "execution/core.rs"]
mod core;
#[path = "execution/selection.rs"]
mod selection;
#[path = "execution/strategies.rs"]
mod strategies;

pub(crate) use core::{
    execute_cursor_continuation, verify_cursor_continuation_budget, ContinuationExecutionEffect,
};
