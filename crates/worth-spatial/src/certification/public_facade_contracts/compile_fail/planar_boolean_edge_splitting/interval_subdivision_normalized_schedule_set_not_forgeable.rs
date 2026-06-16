use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizationCounters,
};

fn main() {
    let _ = PlanarBooleanIntervalSubdivisionNormalizedScheduleSet::new(
        "set".to_string(),
        "endpoint set".to_string(),
        vec![],
        PlanarBooleanIntervalSubdivisionNormalizationCounters::default(),
    );
}
