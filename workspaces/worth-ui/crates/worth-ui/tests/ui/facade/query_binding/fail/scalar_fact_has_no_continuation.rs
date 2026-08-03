use worth_ui::facade::query_binding::UiScalarProjectionObservation;

fn invalid(observation: &UiScalarProjectionObservation) {
    let _ = observation.fact().continuation();
}

fn main() {}
