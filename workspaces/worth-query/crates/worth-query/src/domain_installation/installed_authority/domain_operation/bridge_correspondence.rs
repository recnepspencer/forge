use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryInstalledConditionalDependencyAuthority, WorthQuerySemanticLocality,
};
use worth_runtime_bridge::facade::{
    BridgeCorrespondenceDenial, BridgeCorrespondenceDenialKind,
    BridgeSemanticCorrespondenceRegistration, BridgeSemanticDependencyCandidate,
    BridgeSemanticDependencyCandidateParts, BridgeSemanticLocality,
    BridgeSignalAspectTargetDeclaration, RelationalBridgeRecordIdentityParts,
};

use super::WorthQueryInstalledDomainOperation;
use crate::domain_installation::WorthQueryInstalledGraphParticipation;

impl<D, O, F> WorthQueryInstalledDomainOperation<D, O, F> {
    pub(crate) fn semantic_correspondence_candidate<G: 'static>(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        graph: &WorthQueryInstalledGraphParticipation<G>,
        source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    ) -> Result<BridgeSemanticDependencyCandidate, BridgeCorrespondenceDenial> {
        self.semantic_correspondence_candidate_with_observation(
            location,
            dependency_ordinal,
            graph,
            source_record_identity,
            source_record_identity,
        )
    }

    fn semantic_correspondence_candidate_with_observation<G: 'static>(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        graph: &WorthQueryInstalledGraphParticipation<G>,
        source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        observation_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    ) -> Result<BridgeSemanticDependencyCandidate, BridgeCorrespondenceDenial> {
        let dependency = self.installed_conditional_dependency(location, dependency_ordinal)?;
        self.admit_dependency_graph(dependency.dependency().graph_read_role().as_str(), graph)?;
        BridgeSemanticDependencyCandidate::admit(bridge_candidate_parts(
            &dependency,
            graph,
            source_record_identity,
            observation_record_identity,
        ))
    }

    pub fn semantic_correspondence_registration<G: 'static>(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        graph: &WorthQueryInstalledGraphParticipation<G>,
        source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        targets: Vec<BridgeSignalAspectTargetDeclaration>,
    ) -> Result<BridgeSemanticCorrespondenceRegistration, BridgeCorrespondenceDenial> {
        let dependency = self.semantic_correspondence_candidate(
            location,
            dependency_ordinal,
            graph,
            source_record_identity,
        )?;
        BridgeSemanticCorrespondenceRegistration::new(dependency, targets)
    }

    pub(crate) fn semantic_correspondence_registration_with_observation<G: 'static>(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
        graph: &WorthQueryInstalledGraphParticipation<G>,
        source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        observation_record_identity: Option<RelationalBridgeRecordIdentityParts>,
        targets: Vec<BridgeSignalAspectTargetDeclaration>,
    ) -> Result<BridgeSemanticCorrespondenceRegistration, BridgeCorrespondenceDenial> {
        let dependency = self.semantic_correspondence_candidate_with_observation(
            location,
            dependency_ordinal,
            graph,
            source_record_identity,
            observation_record_identity,
        )?;
        BridgeSemanticCorrespondenceRegistration::new(dependency, targets)
    }

    fn installed_conditional_dependency(
        &self,
        location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        dependency_ordinal: usize,
    ) -> Result<WorthQueryInstalledConditionalDependencyAuthority, BridgeCorrespondenceDenial> {
        if !self.domain_authority.is_current_installation_generation() {
            return Err(BridgeCorrespondenceDenial::without_admission(
                BridgeCorrespondenceDenialKind::StaleSourceInstallation,
            ));
        }
        self.operation_authority
            .conditional_dependency(location, dependency_ordinal)
            .map_err(|_| {
                BridgeCorrespondenceDenial::without_admission(
                    BridgeCorrespondenceDenialKind::PortableDependencyNotOwnedByOperation,
                )
            })
    }

    fn admit_dependency_graph<G: 'static>(
        &self,
        graph_role: &str,
        graph: &WorthQueryInstalledGraphParticipation<G>,
    ) -> Result<(), BridgeCorrespondenceDenial> {
        let operation_owns_role = self.graph_bindings.iter().any(|binding| {
            binding.role == graph_role && binding.graph_marker == std::any::TypeId::of::<G>()
        });
        let graph_matches = graph.record.definition.role == graph_role
            && graph.record.runtime_authority == self.domain_authority.runtime_authority().as_u64();
        if operation_owns_role && graph_matches {
            return Ok(());
        }
        Err(BridgeCorrespondenceDenial::without_admission(
            BridgeCorrespondenceDenialKind::GraphParticipationNotOwnedByOperation,
        ))
    }
}

fn bridge_candidate_parts<G>(
    authority: &WorthQueryInstalledConditionalDependencyAuthority,
    graph: &WorthQueryInstalledGraphParticipation<G>,
    source_record_identity: Option<RelationalBridgeRecordIdentityParts>,
    observation_record_identity: Option<RelationalBridgeRecordIdentityParts>,
) -> BridgeSemanticDependencyCandidateParts {
    let dependency = authority.dependency();
    BridgeSemanticDependencyCandidateParts {
        source_installation_identity: source_installation_identity(authority),
        source_basis: Arc::from(authority.operation_canonical_identity()),
        source_runtime_authority: authority.runtime_ordinal(),
        source_installation_generation: authority.generation().ordinal(),
        source_authority_binding_identity: Arc::from(authority.authority_binding_identity()),
        source_stage_identity: authority.location().stage_identity().map(Arc::from),
        source_node_identity: Arc::from(authority.location().node_identity()),
        dependency_ordinal: authority.dependency_ordinal(),
        declared_graph_role: Arc::from(dependency.graph_read_role().as_str()),
        graph_participation_identity: Arc::from(
            graph.record.installation_authority.authority_identity(),
        ),
        graph_adapter_identity: Arc::from(graph.record.installation_authority.provider_identity()),
        source_record_identity,
        observation_record_identity,
        contract: dependency.contract().clone(),
        projection_mask: dependency.projection_mask().clone(),
        binding: dependency.binding().clone(),
        locality: lower_locality(dependency.locality()),
        relevant_changes: dependency.relevant_changes().to_vec(),
    }
}

fn source_installation_identity(
    authority: &WorthQueryInstalledConditionalDependencyAuthority,
) -> Arc<str> {
    Arc::from(format!(
        "{}|generation={}|operation={}|node={}|dependency={}",
        authority.owner(),
        authority.generation().ordinal(),
        authority.operation_slot(),
        authority.location().node_identity(),
        authority.dependency_ordinal(),
    ))
}

fn lower_locality(locality: &WorthQuerySemanticLocality) -> BridgeSemanticLocality {
    match locality {
        WorthQuerySemanticLocality::SourceRecord => BridgeSemanticLocality::SourceRecord,
        WorthQuerySemanticLocality::SourcePartition(role) => {
            BridgeSemanticLocality::SourcePartition(role.clone())
        }
        WorthQuerySemanticLocality::WholeLogicalGraph => BridgeSemanticLocality::WholeLogicalGraph,
    }
}
