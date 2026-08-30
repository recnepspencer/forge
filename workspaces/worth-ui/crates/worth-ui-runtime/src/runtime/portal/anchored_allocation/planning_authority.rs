#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAdmittedPortalPlanningAuthority {
    contract: super::UiAdmittedPortalAnchorContract,
}

impl UiAdmittedPortalPlanningAuthority {
    pub(crate) fn seal(
        basis: &crate::evidence::UiMeasurementBasis,
        neighborhood: &crate::evidence::UiAllocationNeighborhood,
        constraint_set: &crate::evidence::UiAllocationConstraintSet,
    ) -> Option<Self> {
        let input = constraint_set.portal_anchor_planning_input()?;
        if input.posture()
            != crate::evidence::UiPortalAnchorPlanningInputPosture::AdmittedPlanningTimeOnly
            || !input.is_planning_time_only()
            || input.neighborhood_identity_digest() != neighborhood.identity().identity_digest()
        {
            return None;
        }
        let result = basis.evidence_inputs().iter().find_map(|evidence| {
            evidence.as_host_measurement_result().filter(|result| {
                result.evidence_category()
                    == crate::evidence::UiMeasurementEvidenceCategory::PortalAnchorRect
            })
        })?;
        let identity = super::UiPortalAnchorIdentity::from_measurement_result(result)?;
        Some(Self {
            contract: super::UiAdmittedPortalAnchorContract::seal(
                identity,
                basis,
                neighborhood,
                input.identity_digest(),
                input.source_generation_digest()?,
                result.authority_witness(),
            ),
        })
    }

    pub(crate) fn bind(
        &self,
        basis: &crate::evidence::UiMeasurementBasis,
    ) -> Option<super::UiAdmittedPortalAnchorContract> {
        self.contract
            .matches_basis(basis)
            .then(|| self.contract.clone())
    }
}
