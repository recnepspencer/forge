use forge_query::facade::ForgeQueryWorkspace;

mod report;

pub use report::PrimitiveConstructionCompoundMilestoneCloseoutReport;

use super::builder::{
    prepare_primitive_construction_compound_parity_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
};
use super::parity::compound_parity_registry;

pub fn prepare_primitive_construction_compound_milestone_closeout_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundMilestoneCloseoutReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let parity = prepare_primitive_construction_compound_parity_report(workspace)?;
    Ok(PrimitiveConstructionCompoundMilestoneCloseoutReport::new(
        parity,
        compound_parity_registry().required_scenario_inventory(),
    ))
}
