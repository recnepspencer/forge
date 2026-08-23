#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiHostMeasurementReplanConsequence {
    neighborhood_identity_digest: u64,
    predecessor_basis_identity_digest: u64,
    measurement: crate::evidence::UiMeasurementResult,
}

impl Eq for UiHostMeasurementReplanConsequence {}

impl UiHostMeasurementReplanConsequence {
    pub(in crate::graph::allocation_neighborhood) fn seal(
        target: &crate::graph::UiAdmittedAllocationInvalidationTargetSet,
        measurement: &crate::evidence::UiMeasurementResult,
    ) -> Result<Self, super::UiReplanLocalityDenial> {
        let predecessor = target
            .primary()
            .allocation_plan()
            .ok_or(super::UiReplanLocalityDenial::MissingAdmittedCandidate)?
            .candidate()
            .measurement_basis();
        predecessor
            .succeed_host_measurement_result(measurement)
            .map_err(|_| super::UiReplanLocalityDenial::HostMeasurementSuccessorDenied)?;
        Ok(Self {
            neighborhood_identity_digest: target
                .primary()
                .neighborhood_identity()
                .identity_digest(),
            predecessor_basis_identity_digest: predecessor.identity_digest(),
            measurement: measurement.clone(),
        })
    }

    pub(crate) fn neighborhood_identity_digest(&self) -> u64 {
        self.neighborhood_identity_digest
    }

    pub(crate) fn predecessor_basis_identity_digest(&self) -> u64 {
        self.predecessor_basis_identity_digest
    }

    pub(crate) fn measurement(&self) -> &crate::evidence::UiMeasurementResult {
        &self.measurement
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        let (_, source_generation, source_order) = self.measurement.host_source_position();
        crate::declaration::stable_text_digest("worth-ui.host-measurement-consequence")
            ^ self.neighborhood_identity_digest.rotate_left(11)
            ^ self.predecessor_basis_identity_digest.rotate_left(23)
            ^ crate::evidence::measurement_result_identity_digest(&self.measurement).rotate_left(37)
            ^ source_generation.rotate_left(47)
            ^ source_order.rotate_left(53)
    }
}
