use crate::construction::{
    OrthotopeSpec, PrimitiveConstructionFamily, PrimitiveConstructionIntent, RegularPrismSpec,
    RegularPyramidSpec, ShellWithHoleSpec, SimplexSolidSpec, WireBodySpec,
};

use super::replay_siege_report::PrimitiveConstructionCorpusParameterRole;

#[derive(Clone)]
pub(super) struct PrimitiveConstructionCorpusScenario {
    pub(super) scenario_id: &'static str,
    pub(super) family: PrimitiveConstructionFamily,
    pub(super) parameter_role: PrimitiveConstructionCorpusParameterRole,
    pub(super) intent: PrimitiveConstructionIntent,
}

pub(super) fn primitive_construction_corpus() -> Vec<PrimitiveConstructionCorpusScenario> {
    vec![
        scenario(
            "simplex_minimal",
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec { scale: 1.0 }),
        ),
        scenario(
            "simplex_generic",
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec { scale: 2.5 })
                .at([1.0, -2.0, 0.5]),
        ),
        scenario(
            "simplex_stress",
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec { scale: 10.0 })
                .at([8.0, 8.0, 8.0]),
        ),
        scenario(
            "simplex_threshold_admitted",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec {
                scale: f64::MIN_POSITIVE,
            }),
        ),
        scenario(
            "simplex_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec { scale: 0.0 }),
        ),
        scenario(
            "simplex_rejected",
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
            PrimitiveConstructionIntent::simplex_solid(SimplexSolidSpec { scale: 1.0 }).at([
                f64::NAN,
                0.0,
                0.0,
            ]),
        ),
        scenario(
            "orthotope_minimal",
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 1.0, 1.0],
            }),
        ),
        scenario(
            "orthotope_generic",
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            })
            .at([1.0, 2.0, 3.0]),
        ),
        scenario(
            "orthotope_stress",
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [4.0, 7.0, 9.0],
            })
            .at([10.0, -5.0, 3.0]),
        ),
        scenario(
            "orthotope_threshold_admitted",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [f64::MIN_POSITIVE, f64::MIN_POSITIVE, f64::MIN_POSITIVE],
            }),
        ),
        scenario(
            "orthotope_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 0.0, 2.0],
            }),
        ),
        scenario(
            "orthotope_rejected",
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 1.0, 1.0],
            })
            .at([f64::NAN, 0.0, 0.0]),
        ),
        scenario(
            "regular_prism_minimal",
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            }),
        ),
        scenario(
            "regular_prism_generic",
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 6,
                radius: 1.0,
                height: 2.0,
            })
            .at([1.0, -1.0, 0.5]),
        ),
        scenario(
            "regular_prism_stress",
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 12,
                radius: 2.0,
                height: 4.0,
            })
            .at([12.0, -8.0, 4.0]),
        ),
        scenario(
            "regular_prism_threshold_admitted",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 3,
                radius: 1.0e-150,
                height: 1.0e-150,
            }),
        ),
        scenario(
            "regular_prism_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 3,
                radius: 0.0,
                height: 2.0,
            }),
        ),
        scenario(
            "regular_prism_rejected",
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
            PrimitiveConstructionIntent::regular_prism(RegularPrismSpec {
                sides: 2,
                radius: 1.0,
                height: 2.0,
            }),
        ),
        scenario(
            "regular_pyramid_minimal",
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 1.0,
            }),
        ),
        scenario(
            "regular_pyramid_generic",
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 5,
                radius: 1.0,
                height: 2.0,
            })
            .at([0.5, 1.5, -0.5]),
        ),
        scenario(
            "regular_pyramid_stress",
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 4,
                radius: 2f64.powi(500),
                height: 2f64.powi(501),
            })
            .at([2f64.powi(548), -2f64.powi(548), 2f64.powi(548)]),
        ),
        scenario(
            "regular_pyramid_threshold_admitted",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0e-200,
                height: 1.0e-200,
            }),
        ),
        scenario(
            "regular_pyramid_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 3,
                radius: 1.0,
                height: 0.0,
            }),
        ),
        scenario(
            "regular_pyramid_rejected",
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
            PrimitiveConstructionIntent::regular_pyramid(RegularPyramidSpec {
                sides: 2,
                radius: 1.0,
                height: 2.0,
            }),
        ),
        scenario(
            "wire_body_minimal",
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 3 }),
        ),
        scenario(
            "wire_body_generic",
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 8 }),
        ),
        scenario(
            "wire_body_stress",
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 32 }),
        ),
        scenario(
            "wire_body_threshold_admitted",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 4 }),
        ),
        scenario(
            "wire_body_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 2 }),
        ),
        scenario(
            "wire_body_rejected",
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
            PrimitiveConstructionIntent::wire_body(WireBodySpec { edge_count: 0 }),
        ),
        scenario(
            "shell_with_hole_minimal",
            PrimitiveConstructionCorpusParameterRole::MinimalAdmitted,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 3,
                hole_loop_edge_counts: vec![3],
            }),
        ),
        scenario(
            "shell_with_hole_generic",
            PrimitiveConstructionCorpusParameterRole::GenericAdmitted,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3, 4],
            }),
        ),
        scenario(
            "shell_with_hole_stress",
            PrimitiveConstructionCorpusParameterRole::StressAdmitted,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 12,
                hole_loop_edge_counts: vec![3, 4, 5, 6],
            }),
        ),
        scenario(
            "shell_with_hole_threshold_admitted",
            PrimitiveConstructionCorpusParameterRole::ThresholdAdmitted,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![3],
            }),
        ),
        scenario(
            "shell_with_hole_threshold_rejected",
            PrimitiveConstructionCorpusParameterRole::ThresholdRejected,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![2],
            }),
        ),
        scenario(
            "shell_with_hole_rejected",
            PrimitiveConstructionCorpusParameterRole::ExplicitRejected,
            PrimitiveConstructionIntent::shell_with_hole(ShellWithHoleSpec {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: Vec::new(),
            }),
        ),
    ]
}

fn scenario(
    scenario_id: &'static str,
    parameter_role: PrimitiveConstructionCorpusParameterRole,
    intent: PrimitiveConstructionIntent,
) -> PrimitiveConstructionCorpusScenario {
    PrimitiveConstructionCorpusScenario {
        scenario_id,
        family: intent.family(),
        parameter_role,
        intent,
    }
}
