use super::super::closeout::PrimitiveConstructionCorpusRequiredScenarioInventory;
use forge_query::facade::ForgeQueryWorkspace;

use super::builder::{
    build_compound_parity_report_from_siege,
    prepare_primitive_construction_compound_adversarial_siege_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
};
use super::report::PrimitiveConstructionCompoundMilestoneCloseoutReport;

pub fn prepare_primitive_construction_compound_milestone_closeout_report(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<
    PrimitiveConstructionCompoundMilestoneCloseoutReport,
    PrimitiveConstructionCompoundAdversarialSiegeError,
> {
    let siege = prepare_primitive_construction_compound_adversarial_siege_report(workspace)?;
    let parity = build_compound_parity_report_from_siege(&siege)?;
    Ok(PrimitiveConstructionCompoundMilestoneCloseoutReport::new(
        siege,
        parity,
        required_closeout_scenario_inventory(),
    ))
}

fn required_closeout_scenario_inventory() -> PrimitiveConstructionCorpusRequiredScenarioInventory {
    PrimitiveConstructionCorpusRequiredScenarioInventory::new([
        "orthotope_direct_stable",
        "orthotope_boundary_neighbor_rejected",
        "regular_prism_direct_stable",
        "regular_prism_boundary_neighbor_rejected",
        "pyramid_direct_stable_comparison",
        "pyramid_threshold_admitted_exact_support",
        "pyramid_threshold_rejected_neighbor",
        "pyramid_semantic_exhaustion",
        "simplex_world_collapsed_admitted_local_or_exact",
        "simplex_world_collapsed_threshold_rejected",
        "simplex_world_collapsed_explicit_exhaustion",
        "sheet_patch_reorient_grazing_workplane",
        "wire_open_endpoint_graze",
        "wire_open_motion_relocation",
        "mixed_topology_class_batch",
    ])
}
