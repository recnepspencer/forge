use super::denial::{
    PlanarBooleanSplitPersistentNamingDenial, PlanarBooleanSplitPersistentNamingDenialKind,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    overlap_edge_chains::PlanarBooleanOverlapEdgeChainSet,
    split_chain_validation::PlanarBooleanSplitChainValidationReceipt,
    split_edge_fragments::PlanarBooleanSplitEdgeFragmentSet,
    split_vertex_identity::PlanarBooleanSplitVertexIdentitySet,
};
use topology::facade::{NamingAttachmentReport, TopologyCurrentHeadQueryBasisEvidence};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitPersistentNamingQueryBasis {
    topology_query_domain_identity: String,
    persistent_name_live_view_identity: String,
    naming_attachment_report_identity: String,
}

impl PlanarBooleanSplitPersistentNamingQueryBasis {
    pub fn from_topology_query_artifacts(
        topology_domain_handle: &TopologyCurrentHeadQueryBasisEvidence,
        naming_attachment_report: &NamingAttachmentReport,
    ) -> Result<Self, PlanarBooleanSplitPersistentNamingDenial> {
        if !naming_attachment_report.fully_named {
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference,
                topology_domain_handle.handle_identity_digest(),
                "split persistent naming requires a fully attached Query persistent-name report",
            ));
        }
        if !report_has_complete_query_attachment_evidence(naming_attachment_report) {
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference,
                topology_domain_handle.handle_identity_digest(),
                "split persistent naming requires Query persistent-name attachment evidence",
            ));
        }
        let query_basis = Self {
            topology_query_domain_identity: topology_domain_handle
                .handle_identity_digest()
                .to_string(),
            persistent_name_live_view_identity: topology_domain_handle
                .support_snapshot_digest()
                .to_string(),
            naming_attachment_report_identity: naming_attachment_report_identity(
                naming_attachment_report,
            ),
        };
        query_basis.validate_query_authority()?;
        Ok(query_basis)
    }

    #[cfg(test)]
    pub(crate) fn from_query_runtime(
        topology_query_domain_identity: impl Into<String>,
        persistent_name_live_view_identity: impl Into<String>,
        naming_attachment_report_identity: impl Into<String>,
    ) -> Self {
        Self {
            topology_query_domain_identity: topology_query_domain_identity.into(),
            persistent_name_live_view_identity: persistent_name_live_view_identity.into(),
            naming_attachment_report_identity: naming_attachment_report_identity.into(),
        }
    }

    pub(crate) fn validate_query_authority(
        &self,
    ) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
        for value in [
            self.topology_query_domain_identity(),
            self.persistent_name_live_view_identity(),
            self.naming_attachment_report_identity(),
        ] {
            if value.is_empty() {
                return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                    PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference,
                    "split-persistent-naming-query-basis",
                    "split persistent naming requires non-empty Query/topology basis identities",
                ));
            }
            if contains_geometry_or_display_authority(value) {
                return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                    PlanarBooleanSplitPersistentNamingDenialKind::GeometryOrDisplayAuthorityRejected,
                    value,
                    "split persistent naming Query basis must not be minted from geometry, display, or debug authority",
                ));
            }
        }
        Ok(())
    }

    pub fn topology_query_domain_identity(&self) -> &str {
        &self.topology_query_domain_identity
    }
    pub fn persistent_name_live_view_identity(&self) -> &str {
        &self.persistent_name_live_view_identity
    }
    pub fn naming_attachment_report_identity(&self) -> &str {
        &self.naming_attachment_report_identity
    }
}

pub struct PlanarBooleanSplitPersistentNamingInput<'a> {
    split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    query_basis: PlanarBooleanSplitPersistentNamingQueryBasis,
}

impl<'a> PlanarBooleanSplitPersistentNamingInput<'a> {
    pub fn new(
        split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        query_basis: PlanarBooleanSplitPersistentNamingQueryBasis,
    ) -> Self {
        Self {
            split_chain_validation,
            split_fragments,
            split_vertices,
            overlap_chains,
            query_basis,
        }
    }

    pub(crate) fn split_chain_validation(&self) -> &PlanarBooleanSplitChainValidationReceipt {
        self.split_chain_validation
    }
    pub(crate) fn split_fragments(&self) -> &PlanarBooleanSplitEdgeFragmentSet {
        self.split_fragments
    }
    pub(crate) fn split_vertices(&self) -> &PlanarBooleanSplitVertexIdentitySet {
        self.split_vertices
    }
    pub(crate) fn overlap_chains(&self) -> &PlanarBooleanOverlapEdgeChainSet {
        self.overlap_chains
    }
    pub(crate) fn query_basis(&self) -> &PlanarBooleanSplitPersistentNamingQueryBasis {
        &self.query_basis
    }

    pub(crate) fn validate_product_lineage(
        &self,
    ) -> Result<(), PlanarBooleanSplitPersistentNamingDenial> {
        self.query_basis().validate_query_authority()?;
        if !self
            .split_chain_validation()
            .certifies_split_chain_integrity()
        {
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::SplitChainValidationNotCertified,
                self.split_chain_validation().receipt_identity(),
                "persistent naming requires certified split-chain validation",
            ));
        }
        if self
            .split_chain_validation()
            .split_edge_fragment_set_identity()
            != self.split_fragments().fragment_set_identity()
        {
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::ForeignFragmentSet,
                self.split_fragments().fragment_set_identity(),
                "persistent naming requires the split-chain validated fragment set",
            ));
        }
        if self
            .split_chain_validation()
            .overlap_edge_chain_set_identity()
            != self.overlap_chains().chain_set_identity()
        {
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::ForeignOverlapChainSet,
                self.overlap_chains().chain_set_identity(),
                "persistent naming requires the split-chain validated overlap-chain set",
            ));
        }
        if self.split_fragments().split_vertex_identity_set_identity()
            != self.split_vertices().split_vertex_identity_set_identity()
        {
            return Err(PlanarBooleanSplitPersistentNamingDenial::new(
                PlanarBooleanSplitPersistentNamingDenialKind::ForeignSplitVertexSet,
                self.split_vertices().split_vertex_identity_set_identity(),
                "persistent naming requires the fragment-authoritative split vertex set",
            ));
        }
        Ok(())
    }
}

fn naming_attachment_report_identity(report: &NamingAttachmentReport) -> String {
    let mut parts = vec![
        "planar-boolean-split-query-naming-attachment-report".to_string(),
        format!("fully-named:{}", report.fully_named),
    ];
    parts.extend(report.orphan_persistent_name_ids.iter().map(|identity| {
        format!(
            "orphan:{}:{}:{}",
            identity.partition_value(),
            identity.local_slot_value(),
            identity.generation_value()
        )
    }));
    for attachment in &report.attachments {
        parts.push(format!(
            "attachment:{}:{}:{}:{}",
            attachment.topology_entity_id.partition_value(),
            attachment.topology_entity_id.local_slot_value(),
            attachment.topology_entity_id.generation_value(),
            attachment.topology_kind_name
        ));
        let mut names = attachment
            .attached_persistent_name_ids
            .iter()
            .map(|identity| {
                format!(
                    "name:{}:{}:{}",
                    identity.partition_value(),
                    identity.local_slot_value(),
                    identity.generation_value()
                )
            })
            .collect::<Vec<_>>();
        names.sort();
        parts.extend(names);
    }
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn report_has_complete_query_attachment_evidence(report: &NamingAttachmentReport) -> bool {
    !report.attachments.is_empty()
        && report.orphan_persistent_name_ids.is_empty()
        && report
            .attachments
            .iter()
            .all(|attachment| !attachment.attached_persistent_name_ids.is_empty())
}

pub(crate) fn contains_geometry_or_display_authority(value: &str) -> bool {
    value.contains("coordinate:") || value.contains("display:") || value.contains("debug:")
}
