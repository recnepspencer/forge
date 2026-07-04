use schema::facade::platform::authority::touched_graph_parity_closeout::TouchedGraphParityFamilyKind;

use crate::workload_composition::touched_graph_parity_closeout::family_contributors::{
    KernelTouchedGraphParityCoverageContributor, KernelTouchedGraphParityCoverageError,
    KernelTouchedGraphParityQuerySurfaceKind,
};

use super::error::{
    PublicProjectionContributorCatalogError, PublicProjectionContributorCatalogErrorKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicProjectionContributorRowKind {
    PublicProof,
    DerivedDiagnostics,
}

impl PublicProjectionContributorRowKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicProof => "public-proof",
            Self::DerivedDiagnostics => "derived-diagnostics",
        }
    }

    pub const fn family_kind(self) -> TouchedGraphParityFamilyKind {
        match self {
            Self::PublicProof => TouchedGraphParityFamilyKind::PublicProof,
            Self::DerivedDiagnostics => TouchedGraphParityFamilyKind::DerivedDiagnostics,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicProjectionContributorCatalogRow {
    kind: PublicProjectionContributorRowKind,
    family_kind: TouchedGraphParityFamilyKind,
    current_packet_or_identity_source: &'static str,
    carried_projection_authority_source: &'static str,
    ordinary_path_live_caller_surface: &'static str,
    ordinary_path_live_caller_path: &'static str,
    selected_route_identity_digest: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_witness_identity_digest: Option<String>,
    proof_chain_digest: Option<String>,
    milestone_fifteen_seed_digest: Option<String>,
    residue_digest: Option<String>,
    source_firewall_digest: Option<String>,
    decision_trace_identity_digest: Option<String>,
    selected_identity_fields_produced: &'static [&'static str],
    coverage_contributor: KernelTouchedGraphParityCoverageContributor,
}

impl PublicProjectionContributorCatalogRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        kind: PublicProjectionContributorRowKind,
        current_packet_or_identity_source: &'static str,
        carried_projection_authority_source: &'static str,
        ordinary_path_live_caller_surface: &'static str,
        ordinary_path_live_caller_path: &'static str,
        selected_route_identity_digest: String,
        selected_family_identity: String,
        selected_product_identity_digest: String,
        selected_witness_identity_digest: Option<String>,
        proof_chain_digest: Option<String>,
        milestone_fifteen_seed_digest: Option<String>,
        residue_digest: Option<String>,
        source_firewall_digest: Option<String>,
        decision_trace_identity_digest: Option<String>,
        selected_identity_fields_produced: &'static [&'static str],
        current_surface: &'static str,
        source_path: &'static str,
    ) -> Result<Self, PublicProjectionContributorCatalogError> {
        if current_packet_or_identity_source.is_empty()
            || carried_projection_authority_source.is_empty()
            || ordinary_path_live_caller_surface.is_empty()
            || ordinary_path_live_caller_path.is_empty()
            || selected_route_identity_digest.is_empty()
            || selected_family_identity.is_empty()
            || selected_product_identity_digest.is_empty()
            || selected_identity_fields_produced.is_empty()
        {
            return Err(PublicProjectionContributorCatalogError::new(
                PublicProjectionContributorCatalogErrorKind::MissingCarriedIdentity,
                "public projection contributor row requires selected-route, family, product, and caller authority",
            ));
        }

        if matches!(kind, PublicProjectionContributorRowKind::PublicProof)
            && (proof_chain_digest.as_deref().unwrap_or_default().is_empty()
                || milestone_fifteen_seed_digest
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                || residue_digest.as_deref().unwrap_or_default().is_empty()
                || source_firewall_digest
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty())
        {
            return Err(PublicProjectionContributorCatalogError::new(
                PublicProjectionContributorCatalogErrorKind::MissingCarriedIdentity,
                "public-proof contributor row must carry proof-chain, seed, residue, and source-firewall digests",
            ));
        }

        if matches!(kind, PublicProjectionContributorRowKind::DerivedDiagnostics)
            && decision_trace_identity_digest
                .as_deref()
                .unwrap_or_default()
                .is_empty()
        {
            return Err(PublicProjectionContributorCatalogError::new(
                PublicProjectionContributorCatalogErrorKind::MissingCarriedIdentity,
                "derived-diagnostics contributor row must carry the selected decision-trace identity",
            ));
        }

        Ok(Self {
            kind,
            family_kind: kind.family_kind(),
            current_packet_or_identity_source,
            carried_projection_authority_source,
            ordinary_path_live_caller_surface,
            ordinary_path_live_caller_path,
            selected_route_identity_digest,
            selected_family_identity,
            selected_product_identity_digest,
            selected_witness_identity_digest,
            proof_chain_digest,
            milestone_fifteen_seed_digest,
            residue_digest,
            source_firewall_digest,
            decision_trace_identity_digest,
            selected_identity_fields_produced,
            coverage_contributor: KernelTouchedGraphParityCoverageContributor::new(
                current_surface,
                source_path,
                current_packet_or_identity_source,
                carried_projection_authority_source,
                "public_projection_consumer",
                source_path,
                selected_identity_fields_produced,
                KernelTouchedGraphParityQuerySurfaceKind::ConsumerResidue,
                ordinary_path_live_caller_surface,
                ordinary_path_live_caller_path,
            ),
        })
    }

    pub const fn kind(&self) -> PublicProjectionContributorRowKind {
        self.kind
    }

    pub const fn family_kind(&self) -> TouchedGraphParityFamilyKind {
        self.family_kind
    }

    pub const fn current_packet_or_identity_source(&self) -> &'static str {
        self.current_packet_or_identity_source
    }

    pub const fn carried_projection_authority_source(&self) -> &'static str {
        self.carried_projection_authority_source
    }

    pub const fn ordinary_path_live_caller_surface(&self) -> &'static str {
        self.ordinary_path_live_caller_surface
    }

    pub const fn ordinary_path_live_caller_path(&self) -> &'static str {
        self.ordinary_path_live_caller_path
    }

    pub fn selected_route_identity_digest(&self) -> &str {
        &self.selected_route_identity_digest
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn selected_witness_identity_digest(&self) -> Option<&str> {
        self.selected_witness_identity_digest.as_deref()
    }

    pub fn proof_chain_digest(&self) -> Option<&str> {
        self.proof_chain_digest.as_deref()
    }

    pub fn milestone_fifteen_seed_digest(&self) -> Option<&str> {
        self.milestone_fifteen_seed_digest.as_deref()
    }

    pub fn residue_digest(&self) -> Option<&str> {
        self.residue_digest.as_deref()
    }

    pub fn source_firewall_digest(&self) -> Option<&str> {
        self.source_firewall_digest.as_deref()
    }

    pub fn decision_trace_identity_digest(&self) -> Option<&str> {
        self.decision_trace_identity_digest.as_deref()
    }

    pub const fn selected_identity_fields_produced(&self) -> &'static [&'static str] {
        self.selected_identity_fields_produced
    }

    pub const fn coverage_contributor(&self) -> &KernelTouchedGraphParityCoverageContributor {
        &self.coverage_contributor
    }

    #[cfg(test)]
    pub(crate) fn with_test_authority_override(
        mut self,
        selected_route_identity_digest: &str,
        selected_family_identity: &str,
        selected_product_identity_digest: &str,
        selected_witness_identity_digest: Option<&str>,
        proof_chain_digest: Option<&str>,
        milestone_fifteen_seed_digest: Option<&str>,
        residue_digest: Option<&str>,
        source_firewall_digest: Option<&str>,
        decision_trace_identity_digest: Option<&str>,
    ) -> Self {
        self.selected_route_identity_digest = selected_route_identity_digest.to_string();
        self.selected_family_identity = selected_family_identity.to_string();
        self.selected_product_identity_digest = selected_product_identity_digest.to_string();
        self.selected_witness_identity_digest =
            selected_witness_identity_digest.map(str::to_string);
        self.proof_chain_digest = proof_chain_digest.map(str::to_string);
        self.milestone_fifteen_seed_digest = milestone_fifteen_seed_digest.map(str::to_string);
        self.residue_digest = residue_digest.map(str::to_string);
        self.source_firewall_digest = source_firewall_digest.map(str::to_string);
        self.decision_trace_identity_digest = decision_trace_identity_digest.map(str::to_string);
        self
    }
}

pub(crate) fn public_projection_family_coverage_contributor_rows_from_catalog(
    rows: &[PublicProjectionContributorCatalogRow],
) -> Result<Vec<KernelTouchedGraphParityCoverageContributor>, KernelTouchedGraphParityCoverageError>
{
    Ok(rows
        .iter()
        .map(|row| row.coverage_contributor().clone())
        .collect())
}
