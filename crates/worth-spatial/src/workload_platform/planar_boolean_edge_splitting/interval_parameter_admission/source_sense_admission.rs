use crate::workload_platform::planar_boolean_events::PlanarBooleanSourceIntervalSense;

use super::denial::PlanarBooleanSplitIntervalAdmissionDenial;
use super::range_domain::SplitIntervalRangeDomain;

pub(crate) fn admit_source_sense_ordered_range(
    candidate_identity: &str,
    range_domain: SplitIntervalRangeDomain,
    source_sense: PlanarBooleanSourceIntervalSense,
) -> Result<[f64; 2], PlanarBooleanSplitIntervalAdmissionDenial> {
    let source_parameter_range = range_domain.source_parameter_range();
    require_source_sense_matches_range(candidate_identity, source_parameter_range, source_sense)?;
    Ok(ordered_admitted_range(source_parameter_range))
}

fn require_source_sense_matches_range(
    candidate_identity: &str,
    source_parameter_range: [f64; 2],
    source_sense: PlanarBooleanSourceIntervalSense,
) -> Result<(), PlanarBooleanSplitIntervalAdmissionDenial> {
    match source_sense {
        PlanarBooleanSourceIntervalSense::Forward
            if source_parameter_range[0] < source_parameter_range[1] =>
        {
            Ok(())
        }
        PlanarBooleanSourceIntervalSense::Reversed
            if source_parameter_range[0] > source_parameter_range[1] =>
        {
            Ok(())
        }
        _ => Err(
            PlanarBooleanSplitIntervalAdmissionDenial::contradictory_interval_sense(
                candidate_identity,
                "interval split source sense must agree with the source parameter range",
            ),
        ),
    }
}

fn ordered_admitted_range(source_parameter_range: [f64; 2]) -> [f64; 2] {
    if source_parameter_range[0] < source_parameter_range[1] {
        source_parameter_range
    } else {
        [source_parameter_range[1], source_parameter_range[0]]
    }
}
