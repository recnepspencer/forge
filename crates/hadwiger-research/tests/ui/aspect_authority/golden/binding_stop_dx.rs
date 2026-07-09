use worth_query::facade::WORTHQueryBindingMissingRequiredAspect;
use hadwiger_research::facade::{AspectClosureStop, HadwigerAspectKind};

fn binding_stop_dx() {
    let stop = AspectClosureStop::query_missing_required_aspect(
        HadwigerAspectKind::UnitDistanceEmbedding,
        WORTHQueryBindingMissingRequiredAspect::new("unit-distance aspect missing"),
    );

    assert!(stop.is_query_owned());
    assert_eq!(stop.reason(), "unit-distance aspect missing");
}

fn main() {
    let _ = binding_stop_dx as fn();
}
