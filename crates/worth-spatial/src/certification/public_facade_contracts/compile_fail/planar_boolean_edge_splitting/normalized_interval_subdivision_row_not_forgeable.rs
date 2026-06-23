use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanMicroIntervalAction, PlanarBooleanNormalizedIntervalSubdivisionRow,
};

fn main() {
    let _ = PlanarBooleanNormalizedIntervalSubdivisionRow::new(
        "subdivision".to_string(),
        "event".to_string(),
        vec!["candidate".to_string()],
        "source".to_string(),
        "carrier".to_string(),
        [0.0, 1.0],
        "source interval".to_string(),
        [0.0, 1.0],
        worth_spatial::facade::planar_boolean_events::PlanarBooleanSourceIntervalSense::Forward,
        "normalized interval".to_string(),
        [0.0, 1.0],
        worth_spatial::facade::planar_boolean_events::PlanarBooleanIntervalEventKind::PartialOverlap,
        "frame".to_string(),
        "precision".to_string(),
        PlanarBooleanMicroIntervalAction::Retain,
        vec![],
        vec![],
    );
}
