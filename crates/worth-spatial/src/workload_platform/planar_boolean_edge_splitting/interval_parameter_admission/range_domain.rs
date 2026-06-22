use super::denial::PlanarBooleanSplitIntervalAdmissionDenial;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SplitIntervalRangeDomain {
    source_parameter_range: [f64; 2],
}

impl SplitIntervalRangeDomain {
    pub(crate) fn new(
        candidate_identity: &str,
        source_parameter_range: [f64; 2],
    ) -> Result<Self, PlanarBooleanSplitIntervalAdmissionDenial> {
        require_finite_interval(candidate_identity, source_parameter_range)?;
        require_in_source_edge_domain(candidate_identity, source_parameter_range)?;
        require_non_collapsed_interval(candidate_identity, source_parameter_range)?;
        Ok(Self {
            source_parameter_range,
        })
    }

    pub(crate) fn source_parameter_range(self) -> [f64; 2] {
        self.source_parameter_range
    }
}

fn require_finite_interval(
    candidate_identity: &str,
    source_parameter_range: [f64; 2],
) -> Result<(), PlanarBooleanSplitIntervalAdmissionDenial> {
    if source_parameter_range[0].is_finite() && source_parameter_range[1].is_finite() {
        Ok(())
    } else {
        Err(PlanarBooleanSplitIntervalAdmissionDenial::non_finite_range(
            candidate_identity,
            "interval split parameter range must be finite",
        ))
    }
}

fn require_in_source_edge_domain(
    candidate_identity: &str,
    source_parameter_range: [f64; 2],
) -> Result<(), PlanarBooleanSplitIntervalAdmissionDenial> {
    if (0.0..=1.0).contains(&source_parameter_range[0])
        && (0.0..=1.0).contains(&source_parameter_range[1])
    {
        Ok(())
    } else {
        Err(
            PlanarBooleanSplitIntervalAdmissionDenial::out_of_domain_range(
                candidate_identity,
                "interval split parameter range must be inside the source-edge domain",
            ),
        )
    }
}

fn require_non_collapsed_interval(
    candidate_identity: &str,
    source_parameter_range: [f64; 2],
) -> Result<(), PlanarBooleanSplitIntervalAdmissionDenial> {
    if source_parameter_range[0] != source_parameter_range[1] {
        Ok(())
    } else {
        Err(
            PlanarBooleanSplitIntervalAdmissionDenial::collapsed_interval(
                candidate_identity,
                "interval split parameter range must not collapse to a point",
            ),
        )
    }
}
