use forge_relational::facade::runtime::RelationalReadView;
use serde::{Deserialize, Serialize};

use crate::interpretation::InterpretedTopologyView;
use crate::materialization::MaterializedTopologyView;
use crate::validators::error::WorthTopologyValidationError;
use crate::validators::{loop_wiring, naming, ownership, radial, shell_closure, vertex_branching};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthTopologyValidationPhase {
    Truth,
    DerivedMaterialization,
    DerivedInterpretation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthTopologyValidationInputClass {
    RelationalTruthView,
    MaterializedTopologyView,
    InterpretedTopologyView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyValidationRow {
    pub validator: String,
    pub phase: WorthTopologyValidationPhase,
    pub input_class: WorthTopologyValidationInputClass,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTopologyValidationReport {
    pub rows: Vec<WorthTopologyValidationRow>,
}

pub type DerivedTopologyValidationReport = WorthTopologyValidationReport;

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthTopologyValidator;

impl WorthTopologyValidator {
    pub fn validate(view: &MaterializedTopologyView) -> Result<(), WorthTopologyValidationError> {
        Self::materialized_validation_report(view).map(|_| ())
    }

    pub fn materialized_validation_report(
        view: &MaterializedTopologyView,
    ) -> Result<WorthTopologyValidationReport, WorthTopologyValidationError> {
        let mut rows = Vec::new();

        ownership::validate(view)?;
        rows.push(validation_row_with(
            "ownership",
            WorthTopologyValidationPhase::DerivedMaterialization,
            WorthTopologyValidationInputClass::MaterializedTopologyView,
        ));
        loop_wiring::validate(view)?;
        rows.push(validation_row_with(
            "loop_wiring",
            WorthTopologyValidationPhase::DerivedMaterialization,
            WorthTopologyValidationInputClass::MaterializedTopologyView,
        ));

        Ok(WorthTopologyValidationReport { rows })
    }

    pub fn derived_validation_report(
        materialized: &MaterializedTopologyView,
        interpreted: &InterpretedTopologyView,
    ) -> Result<DerivedTopologyValidationReport, WorthTopologyValidationError> {
        let mut rows = Self::materialized_validation_report(materialized)?.rows;
        radial::validate(interpreted)?;
        rows.push(validation_row_with(
            "radial",
            WorthTopologyValidationPhase::DerivedInterpretation,
            WorthTopologyValidationInputClass::InterpretedTopologyView,
        ));
        shell_closure::validate(interpreted)?;
        rows.push(validation_row_with(
            "shell_closure",
            WorthTopologyValidationPhase::DerivedInterpretation,
            WorthTopologyValidationInputClass::InterpretedTopologyView,
        ));
        vertex_branching::validate(interpreted)?;
        rows.push(validation_row_with(
            "vertex_branching",
            WorthTopologyValidationPhase::DerivedInterpretation,
            WorthTopologyValidationInputClass::InterpretedTopologyView,
        ));

        Ok(WorthTopologyValidationReport { rows })
    }

    pub fn validate_named_truth(
        read_view: &RelationalReadView,
    ) -> Result<(), WorthTopologyValidationError> {
        naming::validate_named_topology_truth(read_view)
    }
}

fn validation_row_with(
    validator: &'static str,
    phase: WorthTopologyValidationPhase,
    input_class: WorthTopologyValidationInputClass,
) -> WorthTopologyValidationRow {
    WorthTopologyValidationRow {
        validator: validator.to_string(),
        phase,
        input_class,
        status: "passed".to_string(),
    }
}

pub fn validate_topology_view(
    view: &crate::data::topology_view::WorthTopologyView,
) -> Result<(), WorthTopologyValidationError> {
    let materialized = MaterializedTopologyView::whole_view(view.clone());
    let interpreted = crate::interpretation::interpret_topology_view(&materialized);
    WorthTopologyValidator::derived_validation_report(&materialized, &interpreted).map(|_| ())
}

pub fn topology_validation_report(
    view: &crate::data::topology_view::WorthTopologyView,
) -> Result<WorthTopologyValidationReport, WorthTopologyValidationError> {
    let materialized = MaterializedTopologyView::whole_view(view.clone());
    let interpreted = crate::interpretation::interpret_topology_view(&materialized);
    WorthTopologyValidator::derived_validation_report(&materialized, &interpreted)
}

pub fn validate_materialized_topology(
    view: &MaterializedTopologyView,
) -> Result<DerivedTopologyValidationReport, WorthTopologyValidationError> {
    WorthTopologyValidator::materialized_validation_report(view)
}

pub fn validate_interpreted_topology(
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
) -> Result<DerivedTopologyValidationReport, WorthTopologyValidationError> {
    WorthTopologyValidator::derived_validation_report(materialized, interpreted)
}

pub fn validate_named_topology_truth(
    read_view: &RelationalReadView,
) -> Result<(), WorthTopologyValidationError> {
    WorthTopologyValidator::validate_named_truth(read_view)
}
