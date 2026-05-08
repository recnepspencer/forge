use forge_relational::facade::runtime::RelationalReadView;
use serde::{Deserialize, Serialize};

use crate::interpretation::InterpretedTopologyView;
use crate::materialization::MaterializedTopologyView;
use crate::validators::error::TopologyValidationError;
use crate::validators::{loop_wiring, naming, ownership, radial, shell_closure, vertex_branching};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyValidationPhase {
    Truth,
    DerivedMaterialization,
    DerivedInterpretation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TopologyValidationInputClass {
    RelationalTruthView,
    MaterializedTopologyView,
    InterpretedTopologyView,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyValidationRow {
    pub validator: String,
    pub phase: TopologyValidationPhase,
    pub input_class: TopologyValidationInputClass,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyValidationReport {
    pub rows: Vec<TopologyValidationRow>,
}

pub type DerivedTopologyValidationReport = TopologyValidationReport;

#[derive(Debug, Default, Clone, Copy)]
pub struct TopologyValidator;

impl TopologyValidator {
    pub fn validate(view: &MaterializedTopologyView) -> Result<(), TopologyValidationError> {
        Self::materialized_validation_report(view).map(|_| ())
    }

    pub fn materialized_validation_report(
        view: &MaterializedTopologyView,
    ) -> Result<TopologyValidationReport, TopologyValidationError> {
        let mut rows = Vec::new();

        ownership::validate(view)?;
        rows.push(validation_row_with(
            "ownership",
            TopologyValidationPhase::DerivedMaterialization,
            TopologyValidationInputClass::MaterializedTopologyView,
        ));
        loop_wiring::validate(view)?;
        rows.push(validation_row_with(
            "loop_wiring",
            TopologyValidationPhase::DerivedMaterialization,
            TopologyValidationInputClass::MaterializedTopologyView,
        ));

        Ok(TopologyValidationReport { rows })
    }

    pub fn derived_validation_report(
        materialized: &MaterializedTopologyView,
        interpreted: &InterpretedTopologyView,
    ) -> Result<DerivedTopologyValidationReport, TopologyValidationError> {
        let mut rows = Self::materialized_validation_report(materialized)?.rows;
        radial::validate(interpreted)?;
        rows.push(validation_row_with(
            "radial",
            TopologyValidationPhase::DerivedInterpretation,
            TopologyValidationInputClass::InterpretedTopologyView,
        ));
        shell_closure::validate(interpreted)?;
        rows.push(validation_row_with(
            "shell_closure",
            TopologyValidationPhase::DerivedInterpretation,
            TopologyValidationInputClass::InterpretedTopologyView,
        ));
        vertex_branching::validate(interpreted)?;
        rows.push(validation_row_with(
            "vertex_branching",
            TopologyValidationPhase::DerivedInterpretation,
            TopologyValidationInputClass::InterpretedTopologyView,
        ));

        Ok(TopologyValidationReport { rows })
    }

    pub fn validate_named_truth(
        read_view: &RelationalReadView,
    ) -> Result<(), TopologyValidationError> {
        naming::validate_named_topology_truth(read_view)
    }
}

fn validation_row_with(
    validator: &'static str,
    phase: TopologyValidationPhase,
    input_class: TopologyValidationInputClass,
) -> TopologyValidationRow {
    TopologyValidationRow {
        validator: validator.to_string(),
        phase,
        input_class,
        status: "passed".to_string(),
    }
}

pub fn validate_topology_view(
    view: &crate::data::topology_view::TopologyView,
) -> Result<(), TopologyValidationError> {
    let materialized = MaterializedTopologyView::whole_view(view.clone());
    let interpreted = crate::interpretation::interpret_topology_view(&materialized);
    TopologyValidator::derived_validation_report(&materialized, &interpreted).map(|_| ())
}

pub fn topology_validation_report(
    view: &crate::data::topology_view::TopologyView,
) -> Result<TopologyValidationReport, TopologyValidationError> {
    let materialized = MaterializedTopologyView::whole_view(view.clone());
    let interpreted = crate::interpretation::interpret_topology_view(&materialized);
    TopologyValidator::derived_validation_report(&materialized, &interpreted)
}

pub fn validate_materialized_topology(
    view: &MaterializedTopologyView,
) -> Result<DerivedTopologyValidationReport, TopologyValidationError> {
    TopologyValidator::materialized_validation_report(view)
}

pub fn validate_interpreted_topology(
    materialized: &MaterializedTopologyView,
    interpreted: &InterpretedTopologyView,
) -> Result<DerivedTopologyValidationReport, TopologyValidationError> {
    TopologyValidator::derived_validation_report(materialized, interpreted)
}

pub fn validate_named_topology_truth(
    read_view: &RelationalReadView,
) -> Result<(), TopologyValidationError> {
    TopologyValidator::validate_named_truth(read_view)
}
