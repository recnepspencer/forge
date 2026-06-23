pub(crate) fn canonical_parameter_bits(parameter: f64) -> u64 {
    if parameter == 0.0 {
        0.0f64.to_bits()
    } else {
        parameter.to_bits()
    }
}
