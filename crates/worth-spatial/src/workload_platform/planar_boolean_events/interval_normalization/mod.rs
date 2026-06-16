mod collapsed_interval;
mod parameter_range;
mod relation_interval_normalization;

pub(crate) use collapsed_interval::interval_has_collapsed;
pub(crate) use parameter_range::canonical_parameter_range;
pub(crate) use relation_interval_normalization::normalized_parameter_range;
