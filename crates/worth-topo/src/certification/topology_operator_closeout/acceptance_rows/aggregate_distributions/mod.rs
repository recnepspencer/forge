mod aggregate_distribution_accessors;
mod aggregate_distribution_rows;

pub(in crate::certification::topology_operator_closeout) use aggregate_distribution_rows::{
    build_family_coverage_rows, build_naming_distribution_rows, build_rejection_distribution_rows,
    ensure_hostile_distribution_rows,
};
