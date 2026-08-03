use worth_ui::facade::query_binding::UiScalarProjectionObservation;

fn invalid(observation: &UiScalarProjectionObservation) {
    let _copy: UiScalarProjectionObservation = observation.clone();
}

fn main() {}
