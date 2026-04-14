use forge_relational::facade::runtime::RelationalReadView;

use crate::data::topology_view::WorthTopologyView;
use crate::validators::error::WorthTopologyValidationError;
use crate::validators::{
    loop_wiring, naming, radial, reference_integrity, shell_closure, vertex_branching,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyValidator;

impl WorthTopologyValidator {
    pub fn validate(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
        reference_integrity::validate(view)?;
        loop_wiring::validate(view)?;
        radial::validate(view)?;
        shell_closure::validate(view)?;
        vertex_branching::validate(view)?;
        Ok(())
    }

    pub fn validate_named_truth(
        read_view: &RelationalReadView,
    ) -> Result<(), WorthTopologyValidationError> {
        naming::validate_named_topology_truth(read_view)
    }
}

pub fn validate_topology_view(
    view: &WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
    WorthTopologyValidator::validate(view)
}

pub fn validate_named_topology_truth(
    read_view: &RelationalReadView,
) -> Result<(), WorthTopologyValidationError> {
    WorthTopologyValidator::validate_named_truth(read_view)
}
