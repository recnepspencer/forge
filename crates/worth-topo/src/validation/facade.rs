use forge_relational::facade::runtime::RelationalReadView;
use serde::{Deserialize, Serialize};

use crate::derived_topology::materialized_graph::MaterializedTopologyView;
use crate::derived_topology::traversal_views::InterpretedTopologyView;
use crate::validation::error::TopologyValidationError;
use crate::validation::rule_identity::{
    loop_wiring_rule, ownership_rule, radial_rings_rule, shell_closure_rule, vertex_disks_rule,
};
use crate::validation::TopologyValidationRuleIdentity;
use crate::validation::{
    loop_wiring, naming, ownership, radial_rings, shell_closure, vertex_disks,
};

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
    pub rule_identity: TopologyValidationRuleIdentity,
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
    pub fn materialized_validation_report(
        view: &MaterializedTopologyView,
    ) -> Result<TopologyValidationReport, TopologyValidationError> {
        let mut rows = Vec::new();

        ownership::validate(view)?;
        rows.push(validation_row_with(
            ownership_rule(),
            TopologyValidationPhase::DerivedMaterialization,
            TopologyValidationInputClass::MaterializedTopologyView,
        ));
        loop_wiring::validate(view)?;
        rows.push(validation_row_with(
            loop_wiring_rule(),
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
        radial_rings::validate(interpreted)?;
        rows.push(validation_row_with(
            radial_rings_rule(),
            TopologyValidationPhase::DerivedInterpretation,
            TopologyValidationInputClass::InterpretedTopologyView,
        ));
        shell_closure::validate(interpreted)?;
        rows.push(validation_row_with(
            shell_closure_rule(),
            TopologyValidationPhase::DerivedInterpretation,
            TopologyValidationInputClass::InterpretedTopologyView,
        ));
        vertex_disks::validate(interpreted)?;
        rows.push(validation_row_with(
            vertex_disks_rule(),
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
    rule_identity: TopologyValidationRuleIdentity,
    phase: TopologyValidationPhase,
    input_class: TopologyValidationInputClass,
) -> TopologyValidationRow {
    TopologyValidationRow {
        validator: rule_identity.name().to_string(),
        rule_identity,
        phase,
        input_class,
        status: "passed".to_string(),
    }
}

#[cfg(test)]
pub fn validate_topology_view(
    view: &crate::brep::topology_graph::TopologyView,
) -> Result<(), TopologyValidationError> {
    let materialized = MaterializedTopologyView::whole_view(view.clone());
    let interpreted =
        crate::derived_topology::traversal_views::interpret_topology_view(&materialized);
    TopologyValidator::derived_validation_report(&materialized, &interpreted).map(|_| ())
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
