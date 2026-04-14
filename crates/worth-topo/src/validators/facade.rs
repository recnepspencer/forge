use forge_relational::facade::runtime::RelationalReadView;
use serde::{Deserialize, Serialize};

use crate::data::topology_view::WorthTopologyView;
use crate::validators::error::WorthTopologyValidationError;
use crate::validators::{
    loop_wiring, naming, radial, reference_integrity, shell_closure, vertex_branching,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyValidationRow {
    pub validator: String,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyValidationReport {
    pub rows: Vec<WorthTopologyValidationRow>,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyValidator;

impl WorthTopologyValidator {
    pub fn validate(view: &WorthTopologyView) -> Result<(), WorthTopologyValidationError> {
        Self::validation_report(view).map(|_| ())
    }

    pub fn validation_report(
        view: &WorthTopologyView,
    ) -> Result<WorthTopologyValidationReport, WorthTopologyValidationError> {
        let mut rows = Vec::new();

        reference_integrity::validate(view)?;
        rows.push(validation_row("reference_integrity"));
        loop_wiring::validate(view)?;
        rows.push(validation_row("loop_wiring"));
        radial::validate(view)?;
        rows.push(validation_row("radial"));
        shell_closure::validate(view)?;
        rows.push(validation_row("shell_closure"));
        vertex_branching::validate(view)?;
        rows.push(validation_row("vertex_branching"));

        Ok(WorthTopologyValidationReport { rows })
    }

    pub fn validate_named_truth(
        read_view: &RelationalReadView,
    ) -> Result<(), WorthTopologyValidationError> {
        naming::validate_named_topology_truth(read_view)
    }
}

fn validation_row(validator: &'static str) -> WorthTopologyValidationRow {
    WorthTopologyValidationRow {
        validator: validator.to_string(),
        status: "passed".to_string(),
    }
}

pub fn validate_topology_view(
    view: &WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
    WorthTopologyValidator::validate(view)
}

pub fn topology_validation_report(
    view: &WorthTopologyView,
) -> Result<WorthTopologyValidationReport, WorthTopologyValidationError> {
    WorthTopologyValidator::validation_report(view)
}

pub fn validate_named_topology_truth(
    read_view: &RelationalReadView,
) -> Result<(), WorthTopologyValidationError> {
    WorthTopologyValidator::validate_named_truth(read_view)
}
