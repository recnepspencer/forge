#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiReplanLocalityProof {
    graph_generation: crate::graph::UiGraphGeneration,
    measurement_basis_generation: crate::evidence::UiMeasurementBasisGeneration,
    target_graph_node_identity: crate::graph::UiGraphNodeIdentity,
    graph_membership_probes: u16,
    replacement_active_artifact_digest: u64,
    replacement_candidate_artifact_digest: u64,
    affected_handle_count: u16,
}

type RetainedReplacementLineage = (
    std::rc::Rc<crate::runtime::WorthUiReplacementImpactClassification>,
    std::rc::Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>,
);

pub(super) fn retain_replacement_lineage(
    target: &crate::graph::UiAdmittedAllocationInvalidationTarget,
) -> Result<RetainedReplacementLineage, super::UiReplanLocalityDenial> {
    Ok((
        std::rc::Rc::clone(
            target
                .replacement_impact
                .as_ref()
                .ok_or(super::UiReplanLocalityDenial::MissingReplacementImpact)?,
        ),
        std::rc::Rc::clone(
            target
                .impact_narrowing
                .as_ref()
                .ok_or(super::UiReplanLocalityDenial::MissingReplacementImpact)?,
        ),
    ))
}

impl UiReplanLocalityProof {
    pub(super) fn from_target(
        target: &crate::graph::UiAdmittedAllocationInvalidationTarget,
    ) -> Result<Self, super::UiReplanLocalityDenial> {
        let impact = target
            .replacement_impact()
            .ok_or(super::UiReplanLocalityDenial::MissingReplacementImpact)?;
        let narrowing = target
            .impact_narrowing()
            .ok_or(super::UiReplanLocalityDenial::MissingReplacementImpact)?;
        Ok(Self {
            graph_generation: target.graph_generation(),
            measurement_basis_generation: target.measurement_basis_generation(),
            target_graph_node_identity: target.graph_node_identity(),
            graph_membership_probes: target.graph_membership_probes(),
            replacement_active_artifact_digest: impact.active_artifact_digest(),
            replacement_candidate_artifact_digest: impact.candidate_artifact_digest(),
            affected_handle_count: u16::try_from(narrowing.affected_handle_count())
                .map_err(|_| super::UiReplanLocalityDenial::CounterExhausted)?,
        })
    }

    pub fn graph_generation(&self) -> crate::graph::UiGraphGeneration {
        self.graph_generation
    }
    pub fn measurement_basis_generation(&self) -> crate::evidence::UiMeasurementBasisGeneration {
        self.measurement_basis_generation
    }
    pub fn target_graph_node_identity(&self) -> crate::graph::UiGraphNodeIdentity {
        self.target_graph_node_identity
    }
    pub fn graph_membership_probes(&self) -> u16 {
        self.graph_membership_probes
    }
    pub fn replacement_active_artifact_digest(&self) -> u64 {
        self.replacement_active_artifact_digest
    }
    pub fn replacement_candidate_artifact_digest(&self) -> u64 {
        self.replacement_candidate_artifact_digest
    }
    pub fn affected_handle_count(&self) -> u16 {
        self.affected_handle_count
    }
}
