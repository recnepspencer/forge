pub(crate) fn canonical_parameter_range(parameter_range: [f64; 2]) -> [f64; 2] {
    [
        canonical_parameter_value(parameter_range[0]),
        canonical_parameter_value(parameter_range[1]),
    ]
}

pub(crate) fn canonical_parameter_value(parameter: f64) -> f64 {
    if parameter == 0.0 {
        0.0
    } else {
        parameter
    }
}
