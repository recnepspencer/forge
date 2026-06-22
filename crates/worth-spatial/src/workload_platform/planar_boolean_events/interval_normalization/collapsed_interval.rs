pub(crate) fn interval_has_collapsed(parameter_range: [f64; 2]) -> bool {
    parameter_range[0] == parameter_range[1]
}

#[cfg(test)]
mod tests {
    use super::interval_has_collapsed;

    #[test]
    fn collapsed_interval_after_normalization_denies_instead_of_becoming_overlap() {
        assert!(interval_has_collapsed([0.5, 0.5]));
        assert!(!interval_has_collapsed([0.5, 1.0]));
    }
}
