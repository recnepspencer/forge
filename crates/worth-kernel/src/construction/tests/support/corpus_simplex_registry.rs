use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

pub(crate) fn required_simplex_ladder_scenarios() -> &'static [&'static str] {
    &[
        "simplex_world_collapsed_admitted_local_or_exact",
        "simplex_world_collapsed_threshold_rejected",
        "simplex_world_collapsed_explicit_exhaustion",
    ]
}

pub(crate) fn required_simplex_exhaustion_witness_kinds(
) -> &'static [PrimitiveRealizationExhaustionWitnessKind] {
    &[
        PrimitiveRealizationExhaustionWitnessKind::ZeroScaleSimplexSupportCollapse,
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse,
    ]
}
