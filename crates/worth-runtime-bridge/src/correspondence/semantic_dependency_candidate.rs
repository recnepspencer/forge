use std::sync::Arc;

use worth_foundational::facade::{
    AspectBinding, AspectContract, AspectMask, AuthoritativeAspectChangeKind, ProjectionMask,
};
use worth_query_installation::facade::WorthQueryInstalledConditionalDependencyAuthority;
use worth_query_installation::facade::WorthQueryInstalledGraphParticipationAuthority;

use super::{BridgeCorrespondenceDenial, BridgeCorrespondenceDenialKind};

pub use worth_query_installation::facade::WorthQuerySemanticLocality as BridgeSemanticLocality;

#[derive(Debug, Clone)]
pub struct BridgeSemanticDependencyCandidate {
    pub(crate) query_authority: Arc<WorthQueryInstalledConditionalDependencyAuthority>,
    pub(crate) query_installation_identity: Arc<str>,
    pub(crate) query_basis: Arc<str>,
    pub(crate) query_runtime_authority: u64,
    pub(crate) query_installation_generation: u64,
    pub(crate) declared_graph_role: Arc<str>,
    pub(crate) graph_authority: Arc<WorthQueryInstalledGraphParticipationAuthority>,
    pub(crate) graph_participation_identity: Arc<str>,
    pub(crate) graph_adapter_identity: Arc<str>,
    pub(crate) source_record_identity:
        Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
    pub(crate) contract: AspectContract,
    pub(crate) projection_mask: AspectMask<ProjectionMask>,
    pub(crate) binding: AspectBinding,
    pub(crate) locality: BridgeSemanticLocality,
    pub(crate) relevant_changes: Vec<AuthoritativeAspectChangeKind>,
}

impl BridgeSemanticDependencyCandidate {
    /// Joins an opaque Query-installed dependency to one runtime graph-binding
    /// projection. Portable semantic fields cannot be supplied independently.
    pub fn from_query_authority(
        query_authority: WorthQueryInstalledConditionalDependencyAuthority,
        graph_authority: Arc<WorthQueryInstalledGraphParticipationAuthority>,
        source_record_identity: Option<
            crate::relational_identity::RelationalBridgeRecordIdentityParts,
        >,
    ) -> Result<Self, BridgeCorrespondenceDenial> {
        let query_authority = Arc::new(query_authority);
        let dependency = query_authority.dependency();
        let contract = dependency.contract().clone();
        let projection_mask = dependency.projection_mask().clone();
        let binding = dependency.binding().clone();
        let locality = dependency.locality().clone();
        let relevant_changes = dependency.relevant_changes().to_vec();
        contract
            .admits_projection_mask(&projection_mask)
            .map_err(|_| {
                BridgeCorrespondenceDenial::without_admission(
                    BridgeCorrespondenceDenialKind::ProjectionMaskNotAdmitted,
                )
            })?;
        let query_installation_identity: Arc<str> = Arc::from(format!(
            "{}|generation={}|operation={}|node={}|dependency={}",
            query_authority.owner(),
            query_authority.generation().ordinal(),
            query_authority.operation_slot(),
            query_authority.location().node_identity(),
            query_authority.dependency_ordinal(),
        ));
        let query_basis: Arc<str> = Arc::from(query_authority.operation_canonical_identity());
        let query_runtime_authority = query_authority.runtime_ordinal();
        let query_installation_generation = query_authority.generation().ordinal();
        let declared_graph_role: Arc<str> = Arc::from(dependency.graph_read_role().as_str());
        if graph_authority.runtime_ordinal() != query_runtime_authority
            || graph_authority.role() != declared_graph_role.as_ref()
        {
            return Err(BridgeCorrespondenceDenial::without_admission(
                BridgeCorrespondenceDenialKind::GraphParticipationNotOwnedByOperation,
            ));
        }
        let graph_participation_identity: Arc<str> =
            Arc::from(graph_authority.authority_identity());
        let graph_adapter_identity: Arc<str> = Arc::from(graph_authority.provider_identity());
        if query_installation_identity.trim().is_empty()
            || query_basis.trim().is_empty()
            || query_runtime_authority == 0
            || query_installation_generation == 0
            || declared_graph_role.trim().is_empty()
            || graph_participation_identity.trim().is_empty()
            || graph_adapter_identity.trim().is_empty()
            || relevant_changes.is_empty()
            || matches!(locality, BridgeSemanticLocality::SourceRecord)
                != source_record_identity.is_some()
        {
            return Err(BridgeCorrespondenceDenial::without_admission(
                BridgeCorrespondenceDenialKind::InvalidPortableDependency,
            ));
        }
        Ok(Self {
            query_authority,
            query_installation_identity,
            query_basis,
            query_runtime_authority,
            query_installation_generation,
            declared_graph_role,
            graph_authority,
            graph_participation_identity,
            graph_adapter_identity,
            source_record_identity,
            contract,
            projection_mask,
            binding,
            locality,
            relevant_changes,
        })
    }

    pub fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn declared_graph_role(&self) -> &str {
        &self.declared_graph_role
    }

    pub fn graph_participation_identity(&self) -> &str {
        &self.graph_participation_identity
    }

    pub fn graph_adapter_identity(&self) -> &str {
        &self.graph_adapter_identity
    }

    pub fn projection_mask(&self) -> &AspectMask<ProjectionMask> {
        &self.projection_mask
    }

    pub fn binding(&self) -> &AspectBinding {
        &self.binding
    }

    pub fn locality(&self) -> &BridgeSemanticLocality {
        &self.locality
    }

    pub fn relevant_changes(&self) -> &[AuthoritativeAspectChangeKind] {
        &self.relevant_changes
    }

    pub fn conditional_node_location(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryConditionalNodeLocation {
        self.query_authority.location()
    }

    pub(crate) fn dependency_ordinal(&self) -> usize {
        self.query_authority.dependency_ordinal()
    }

    pub(crate) fn matches_declared_dependency(
        &self,
        dependency: &worth_query_installation::facade::WorthQuerySemanticTruthDependency,
    ) -> bool {
        self.query_authority.dependency() == dependency
    }

    pub(crate) fn canonical_registration_key(&self) -> String {
        let locality = match &self.locality {
            BridgeSemanticLocality::SourceRecord => "record".to_string(),
            BridgeSemanticLocality::SourcePartition(partition) => {
                format!("partition:{}", partition.as_str())
            }
            BridgeSemanticLocality::WholeLogicalGraph => "graph".to_string(),
        };
        let mask = if self.projection_mask.is_whole_aspect() {
            "whole".to_string()
        } else {
            self.projection_mask
                .paths()
                .iter()
                .map(|path| {
                    path.fields()
                        .iter()
                        .map(|field| field.as_str())
                        .collect::<Vec<_>>()
                        .join(".")
                })
                .collect::<Vec<_>>()
                .join("|")
        };
        let changes = self
            .relevant_changes
            .iter()
            .map(|change| change.canonical_name())
            .collect::<Vec<_>>()
            .join("|");
        [
            self.query_installation_identity.to_string(),
            self.query_basis.to_string(),
            self.query_runtime_authority.to_string(),
            self.query_installation_generation.to_string(),
            self.query_authority.authority_binding_identity(),
            self.query_authority
                .location()
                .stage_identity()
                .unwrap_or("operation")
                .to_string(),
            self.query_authority.location().node_identity().to_string(),
            self.query_authority.dependency_ordinal().to_string(),
            self.declared_graph_role.to_string(),
            self.graph_participation_identity.to_string(),
            self.graph_adapter_identity.to_string(),
            source_record_identity_token(self.source_record_identity),
            self.contract.key().as_str().to_string(),
            self.contract.identity().0.to_string(),
            self.contract.revision().0.to_string(),
            mask,
            self.binding.canonical_name(),
            locality,
            changes,
        ]
        .into_iter()
        .map(|field| format!("{}:{field}", field.len()))
        .collect()
    }

    pub(crate) fn authority_registration_key(&self) -> String {
        [
            self.query_runtime_authority.to_string(),
            self.query_installation_generation.to_string(),
            self.query_authority.authority_binding_identity(),
            self.graph_participation_identity.to_string(),
            self.graph_adapter_identity.to_string(),
            source_record_identity_token(self.source_record_identity),
        ]
        .into_iter()
        .map(|field| format!("{}:{field}", field.len()))
        .collect()
    }
}

impl PartialEq for BridgeSemanticDependencyCandidate {
    fn eq(&self, other: &Self) -> bool {
        self.query_authority.authority_binding_identity()
            == other.query_authority.authority_binding_identity()
            && self.query_installation_identity == other.query_installation_identity
            && self.query_basis == other.query_basis
            && self.query_runtime_authority == other.query_runtime_authority
            && self.query_installation_generation == other.query_installation_generation
            && self.declared_graph_role == other.declared_graph_role
            && self.graph_authority == other.graph_authority
            && self.graph_participation_identity == other.graph_participation_identity
            && self.graph_adapter_identity == other.graph_adapter_identity
            && self.source_record_identity == other.source_record_identity
            && self.contract == other.contract
            && self.projection_mask == other.projection_mask
            && self.binding == other.binding
            && self.locality == other.locality
            && self.relevant_changes == other.relevant_changes
    }
}

impl Eq for BridgeSemanticDependencyCandidate {}

fn source_record_identity_token(
    identity: Option<crate::relational_identity::RelationalBridgeRecordIdentityParts>,
) -> String {
    identity
        .map(|identity| identity.bridge_entity_identity())
        .unwrap_or_else(|| "not-record-local".to_string())
}
