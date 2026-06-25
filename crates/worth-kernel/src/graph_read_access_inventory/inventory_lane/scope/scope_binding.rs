use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::scope_expectation::WorthGraphReadAccessScopeExpectation;
use super::scope_family::WorthGraphReadAccessScopeFamily;
use super::scope_kind::WorthGraphReadAccessScopeKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessScopeBinding {
    source_path: String,
    scope_kind: WorthGraphReadAccessScopeKind,
    scope_family: WorthGraphReadAccessScopeFamily,
    scope_expectation: WorthGraphReadAccessScopeExpectation,
    selected_obligation_index: Option<usize>,
    authority_digest: Option<String>,
    touch_descriptor_digest: Option<String>,
    execution_proof_digest: Option<String>,
    selected_registration_digest: Option<String>,
    adoption_manifest_digest: Option<String>,
    certification_boundary: Option<String>,
}

impl WorthGraphReadAccessScopeBinding {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn selected_obligation(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
        selected_registration_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::SelectedObligation,
            scope_family,
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            Some(selected_registration_digest.into()),
            None,
            None,
        )
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_inventory::inventory_lane) fn touched_authority_digest(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::TouchedAuthorityDigest,
            scope_family,
            WorthGraphReadAccessScopeExpectation::FutureExecutionReceiptExpectation,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn declaration_touched_authority_digest(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::TouchedAuthorityDigest,
            scope_family,
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_touch_descriptor_digest(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        scope_family: WorthGraphReadAccessScopeFamily,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::TouchDescriptorDigest,
            scope_family,
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn topology_read_proof(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::TopologyReadProof,
            WorthGraphReadAccessScopeFamily::TopologyReadLedger,
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_inventory::inventory_lane) fn preview_declaration_candidate(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::TouchDescriptorDigest,
            WorthGraphReadAccessScopeFamily::TopologyReadLedger,
            WorthGraphReadAccessScopeExpectation::PreviewDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_inventory::inventory_lane) fn branch_declaration_candidate(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::TouchDescriptorDigest,
            WorthGraphReadAccessScopeFamily::TopologyReadLedger,
            WorthGraphReadAccessScopeExpectation::BranchDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_inventory::inventory_lane) fn spatial_declaration_authority(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::SpatialContinuationProof,
            WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation,
            WorthGraphReadAccessScopeExpectation::MilestoneSevenDeclarationCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn spatial_continuation_proof(
        source_path: impl Into<String>,
        selected_obligation_index: usize,
        authority_digest: impl Into<String>,
        touch_descriptor_digest: impl Into<String>,
        execution_proof_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::SpatialContinuationProof,
            WorthGraphReadAccessScopeFamily::PlanarBooleanContinuation,
            WorthGraphReadAccessScopeExpectation::QueryAccessRequirementCandidateInput,
            Some(selected_obligation_index),
            Some(authority_digest.into()),
            Some(touch_descriptor_digest.into()),
            Some(execution_proof_digest.into()),
            None,
            None,
            None,
        )
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn deleted_graph_read_source(
        source_path: impl Into<String>,
        adoption_manifest_digest: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::DeletedGraphReadSource,
            WorthGraphReadAccessScopeFamily::DeletedGraphReadSource,
            WorthGraphReadAccessScopeExpectation::DeletionOnlyResidue,
            None,
            None,
            None,
            None,
            None,
            Some(adoption_manifest_digest.into()),
            None,
        )
    }

    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_certification_boundary(
        source_path: impl Into<String>,
        certification_boundary: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::CertificationOnlyBoundary,
            WorthGraphReadAccessScopeFamily::CertificationBoundary,
            WorthGraphReadAccessScopeExpectation::CertificationOnlyBoundary,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(certification_boundary.into()),
        )
    }

    #[cfg(test)]
    pub(in crate::graph_read_access_inventory::inventory_lane) fn out_of_scope_non_graph_read(
        source_path: impl Into<String>,
        boundary: impl Into<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        Self::with_required_evidence(
            source_path,
            WorthGraphReadAccessScopeKind::OutOfScopeNonGraphRead,
            WorthGraphReadAccessScopeFamily::NonGraphReadBoundary,
            WorthGraphReadAccessScopeExpectation::NonGraphReadBoundary,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(boundary.into()),
        )
    }

    fn with_required_evidence(
        source_path: impl Into<String>,
        scope_kind: WorthGraphReadAccessScopeKind,
        scope_family: WorthGraphReadAccessScopeFamily,
        scope_expectation: WorthGraphReadAccessScopeExpectation,
        selected_obligation_index: Option<usize>,
        authority_digest: Option<String>,
        touch_descriptor_digest: Option<String>,
        execution_proof_digest: Option<String>,
        selected_registration_digest: Option<String>,
        adoption_manifest_digest: Option<String>,
        certification_boundary: Option<String>,
    ) -> Result<Self, WorthGraphReadAccessInventoryError> {
        let binding = Self {
            source_path: require_non_empty(source_path.into())?,
            scope_kind,
            scope_family,
            scope_expectation,
            selected_obligation_index,
            authority_digest,
            touch_descriptor_digest,
            execution_proof_digest,
            selected_registration_digest,
            adoption_manifest_digest,
            certification_boundary,
        };
        binding.require_present_evidence()?;
        Ok(binding)
    }

    fn require_present_evidence(&self) -> Result<(), WorthGraphReadAccessInventoryError> {
        for evidence in [
            self.authority_digest.as_deref(),
            self.touch_descriptor_digest.as_deref(),
            self.execution_proof_digest.as_deref(),
            self.selected_registration_digest.as_deref(),
            self.adoption_manifest_digest.as_deref(),
            self.certification_boundary.as_deref(),
        ] {
            if evidence == Some("") {
                return Err(error(
                    WorthGraphReadAccessInventoryErrorKind::MissingScopeEvidence,
                ));
            }
        }

        let has_evidence = self.authority_digest.is_some()
            || self.touch_descriptor_digest.is_some()
            || self.execution_proof_digest.is_some()
            || self.selected_registration_digest.is_some()
            || self.adoption_manifest_digest.is_some()
            || self.certification_boundary.is_some();
        if !has_evidence {
            return Err(error(
                WorthGraphReadAccessInventoryErrorKind::MissingScopeEvidence,
            ));
        }
        Ok(())
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn scope_kind(&self) -> WorthGraphReadAccessScopeKind {
        self.scope_kind
    }

    pub const fn scope_family(&self) -> WorthGraphReadAccessScopeFamily {
        self.scope_family
    }

    pub const fn scope_expectation(&self) -> WorthGraphReadAccessScopeExpectation {
        self.scope_expectation
    }

    pub const fn selected_obligation_index(&self) -> Option<usize> {
        self.selected_obligation_index
    }

    pub fn authority_digest(&self) -> Option<&str> {
        self.authority_digest.as_deref()
    }

    pub fn touch_descriptor_digest(&self) -> Option<&str> {
        self.touch_descriptor_digest.as_deref()
    }

    pub fn execution_proof_digest(&self) -> Option<&str> {
        self.execution_proof_digest.as_deref()
    }

    pub fn selected_registration_digest(&self) -> Option<&str> {
        self.selected_registration_digest.as_deref()
    }

    pub fn adoption_manifest_digest(&self) -> Option<&str> {
        self.adoption_manifest_digest.as_deref()
    }

    pub fn certification_boundary(&self) -> Option<&str> {
        self.certification_boundary.as_deref()
    }
}

fn require_non_empty(value: String) -> Result<String, WorthGraphReadAccessInventoryError> {
    if value.is_empty() {
        return Err(error(
            WorthGraphReadAccessInventoryErrorKind::MissingScopeEvidence,
        ));
    }
    Ok(value)
}

const fn error(kind: WorthGraphReadAccessInventoryErrorKind) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::new(kind)
}
