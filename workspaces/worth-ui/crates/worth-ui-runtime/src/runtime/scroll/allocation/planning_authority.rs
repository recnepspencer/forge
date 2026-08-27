use super::{
    UiAdmittedScrollExtentSource, UiAdmittedScrollOwnedContract, UiAdmittedScrollQuerySource,
    UiScrollOffsetAllocationPosture, UiScrollVirtualizationPosture,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiAdmittedScrollPlanningAuthority {
    measurement_basis_identity_digest: u64,
    measurement_basis_generation: crate::evidence::UiMeasurementBasisGeneration,
    planning_input_identity_digest: u64,
    source_evidence_identity_digest: u64,
    source_generation_digest: u64,
    graph_authority: crate::graph::UiGraphScrollPlanningAuthority,
}

impl UiAdmittedScrollPlanningAuthority {
    pub(crate) fn seal(
        planning_basis: &crate::runtime::WorthUiAllocationPlanningBasis,
    ) -> Result<Option<Self>, super::UiScrollContractAdmissionDenial> {
        let Some(graph_authority) = planning_basis.scroll_authority() else {
            return Ok(None);
        };
        let Some(input) = planning_basis
            .allocation_constraint_set()
            .and_then(crate::evidence::UiAllocationConstraintSet::scroll_owner_planning_input)
        else {
            return Ok(None);
        };
        if input.posture()
            != crate::evidence::UiScrollOwnerPlanningInputPosture::AdmittedPlanningTimeOnly
            || !input.is_planning_time_only()
        {
            return Err(super::UiScrollContractAdmissionDenial::PlanningPostureMismatch);
        }
        if graph_authority.neighborhood_identity()
            != planning_basis.allocation_neighborhood().identity()
        {
            return Err(super::UiScrollContractAdmissionDenial::NeighborhoodMismatch);
        }
        Ok(Some(Self {
            measurement_basis_identity_digest: planning_basis.measurement_basis().identity_digest(),
            measurement_basis_generation: planning_basis.measurement_basis().generation(),
            planning_input_identity_digest: input.identity_digest(),
            source_evidence_identity_digest: input
                .source_evidence_identity_digest()
                .ok_or(super::UiScrollContractAdmissionDenial::SourceEvidenceMissing)?,
            source_generation_digest: input
                .source_generation_digest()
                .ok_or(super::UiScrollContractAdmissionDenial::SourceGenerationMissing)?,
            graph_authority: graph_authority.clone(),
        }))
    }

    pub(crate) fn committed_sources(
        &self,
    ) -> Box<[crate::runtime::UiCommittedScrollActivationSource]> {
        self.graph_authority
            .host_sources()
            .iter()
            .map(
                |witness| crate::runtime::UiCommittedScrollActivationSource::Host {
                    witness: *witness,
                    contract: self
                        .bind(UiAdmittedScrollExtentSource::HostViewport { witness: *witness }),
                },
            )
            .chain(self.graph_authority.query_sources().iter().map(|mapping| {
                crate::runtime::UiCommittedScrollActivationSource::Query {
                    contract: self.bind(UiAdmittedScrollExtentSource::QueryContent(
                        UiAdmittedScrollQuerySource {
                            source_key: mapping.source_key().clone(),
                            target: mapping.target(),
                        },
                    )),
                }
            }))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn bind(&self, source: UiAdmittedScrollExtentSource) -> UiAdmittedScrollOwnedContract {
        UiAdmittedScrollOwnedContract {
            neighborhood_identity: self.graph_authority.neighborhood_identity().clone(),
            graph_generation: self
                .graph_authority
                .neighborhood_identity()
                .graph_generation(),
            measurement_basis_identity_digest: self.measurement_basis_identity_digest,
            measurement_basis_generation: self.measurement_basis_generation,
            coordinate_ownership: self
                .graph_authority
                .neighborhood_identity()
                .layout_operator_contract_identity(),
            planning_input_identity_digest: self.planning_input_identity_digest,
            source_evidence_identity_digest: self.source_evidence_identity_digest,
            source_generation_digest: self.source_generation_digest,
            source,
            virtualization: UiScrollVirtualizationPosture::NonVirtualized,
            offset_allocation: UiScrollOffsetAllocationPosture::ProjectedInteractionOnly,
        }
    }
}
