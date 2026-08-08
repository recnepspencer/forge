//! Exact governed-input carriage shared by undo and redo admission.

use crate::domain_computation::primary_graph::WorthQueryRetainedGovernedInput;

use super::{WorthQueryUndoDenial, WorthQueryUndoDenialKind};

pub(crate) fn require_original_governed_input(
    input: Option<&WorthQueryRetainedGovernedInput>,
) -> Result<&WorthQueryRetainedGovernedInput, WorthQueryUndoDenial> {
    bound_original_governed_input(input).ok_or_else(|| {
        WorthQueryUndoDenial::new(WorthQueryUndoDenialKind::OriginalGovernedInputRequired)
    })
}

pub(crate) fn bound_original_governed_input(
    input: Option<&WorthQueryRetainedGovernedInput>,
) -> Option<&WorthQueryRetainedGovernedInput> {
    input.filter(|input| input.has_governed_identity())
}
