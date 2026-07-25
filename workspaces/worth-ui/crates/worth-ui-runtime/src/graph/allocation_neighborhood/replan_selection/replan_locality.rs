#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiReplanLocalityProof {
    graph_generation: crate::graph::UiGraphGeneration,
    measurement_basis_generation: crate::evidence::UiMeasurementBasisGeneration,
    target_graph_node_identity: crate::graph::UiGraphNodeIdentity,
    graph_membership_probes: u16,
    replacement_active_artifact_digest: Option<u64>,
    replacement_candidate_artifact_digest: Option<u64>,
    affected_handle_count: Option<u16>,
}

type RetainedReplacementLineage = Option<(
    std::rc::Rc<crate::runtime::WorthUiReplacementImpactClassification>,
    std::rc::Rc<crate::runtime::WorthUiRuntimeImpactNarrowing>,
)>;

pub(super) fn retain_replacement_lineage(
    target: &crate::graph::UiAdmittedAllocationInvalidationTarget,
) -> Result<RetainedReplacementLineage, super::UiReplanLocalityDenial> {
    match (&target.replacement_impact, &target.impact_narrowing) {
        (Some(impact), Some(narrowing)) => Ok(Some((
            std::rc::Rc::clone(impact),
            std::rc::Rc::clone(narrowing),
        ))),
        (None, None) => Ok(None),
        _ => Err(super::UiReplanLocalityDenial::IncompleteReplacementLineage),
    }
}

impl UiReplanLocalityProof {
    pub(super) fn from_target(
        target: &crate::graph::UiAdmittedAllocationInvalidationTarget,
    ) -> Result<Self, super::UiReplanLocalityDenial> {
        let lineage = retain_replacement_lineage(target)?;
        Ok(Self {
            graph_generation: target.graph_generation(),
            measurement_basis_generation: target.measurement_basis_generation(),
            target_graph_node_identity: target.graph_node_identity(),
            graph_membership_probes: target.graph_membership_probes(),
            replacement_active_artifact_digest: lineage
                .as_ref()
                .map(|(impact, _)| impact.active_artifact_digest()),
            replacement_candidate_artifact_digest: lineage
                .as_ref()
                .map(|(impact, _)| impact.candidate_artifact_digest()),
            affected_handle_count: lineage
                .as_ref()
                .map(|(_, narrowing)| u16::try_from(narrowing.affected_handle_count()))
                .transpose()
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
    pub fn replacement_active_artifact_digest(&self) -> Option<u64> {
        self.replacement_active_artifact_digest
    }
    pub fn replacement_candidate_artifact_digest(&self) -> Option<u64> {
        self.replacement_candidate_artifact_digest
    }
    pub fn affected_handle_count(&self) -> Option<u16> {
        self.affected_handle_count
    }
}
