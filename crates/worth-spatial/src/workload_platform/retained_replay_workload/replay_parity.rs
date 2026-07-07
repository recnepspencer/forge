use crate::spatial_compiled_product_family::{
    SpatialCompiledProductConsumer, SpatialCompiledProductFamilyIdentity,
};
use crate::workload_platform::compiled_product_admission::{
    SpatialCompiledProductAdmissionErrorKind, SpatialCompiledProductAdmissionWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityAdmissionProvenance {
    source_authority_digest: String,
    locality_footprint_digest: String,
    evidence_support_digest: String,
    family_digest: String,
    authority_truth_identity_digest: String,
    equivalence_policy_identity_digest: String,
    prior_proof_identity_digest: Option<String>,
    compiled_product_identity_digest: String,
}

impl ReplayParityAdmissionProvenance {
    pub(crate) fn new(
        source_authority_digest: String,
        locality_footprint_digest: String,
        evidence_support_digest: String,
        family_digest: String,
        authority_truth_identity_digest: String,
        equivalence_policy_identity_digest: String,
        prior_proof_identity_digest: Option<String>,
        compiled_product_identity_digest: String,
    ) -> Self {
        Self {
            source_authority_digest,
            locality_footprint_digest,
            evidence_support_digest,
            family_digest,
            authority_truth_identity_digest,
            equivalence_policy_identity_digest,
            prior_proof_identity_digest,
            compiled_product_identity_digest,
        }
    }

    pub fn source_authority_digest(&self) -> &str {
        &self.source_authority_digest
    }

    pub fn locality_footprint_digest(&self) -> &str {
        &self.locality_footprint_digest
    }

    pub fn evidence_support_digest(&self) -> &str {
        &self.evidence_support_digest
    }

    pub fn family_digest(&self) -> &str {
        &self.family_digest
    }

    pub fn authority_truth_identity_digest(&self) -> &str {
        &self.authority_truth_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn prior_proof_identity_digest(&self) -> Option<&str> {
        self.prior_proof_identity_digest.as_deref()
    }

    pub fn compiled_product_identity_digest(&self) -> &str {
        &self.compiled_product_identity_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParitySpatialAdmissionCause {
    BroadEvidenceScanDenied,
    FamilyCatalogDenied,
    WrongAuthorityBasis,
    WrongReceiptFamily,
    WrongSupportPosture,
}

impl From<SpatialCompiledProductAdmissionErrorKind> for ReplayParitySpatialAdmissionCause {
    fn from(value: SpatialCompiledProductAdmissionErrorKind) -> Self {
        match value {
            SpatialCompiledProductAdmissionErrorKind::BroadEvidenceScanDenied => {
                Self::BroadEvidenceScanDenied
            }
            SpatialCompiledProductAdmissionErrorKind::FamilyCatalogDenied => {
                Self::FamilyCatalogDenied
            }
            SpatialCompiledProductAdmissionErrorKind::WrongAuthorityBasis => {
                Self::WrongAuthorityBasis
            }
            SpatialCompiledProductAdmissionErrorKind::WrongReceiptFamily => {
                Self::WrongReceiptFamily
            }
            SpatialCompiledProductAdmissionErrorKind::WrongSupportPosture => {
                Self::WrongSupportPosture
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParityErrorKind {
    SpatialAdmission,
    FamilySelection,
    IdentityLowering,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityError {
    kind: ReplayParityErrorKind,
    spatial_admission_cause: Option<ReplayParitySpatialAdmissionCause>,
    detail: String,
}

impl ReplayParityError {
    pub(crate) fn new(
        kind: ReplayParityErrorKind,
        spatial_admission_cause: Option<ReplayParitySpatialAdmissionCause>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            spatial_admission_cause,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> ReplayParityErrorKind {
        self.kind
    }

    pub fn spatial_admission_cause(&self) -> Option<ReplayParitySpatialAdmissionCause> {
        self.spatial_admission_cause
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayParityKind {
    LiveRetainedReplayedProjectionMatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityRow {
    kind: ReplayParityKind,
    parity_identity: String,
    human_parity: String,
}

impl ReplayParityRow {
    pub(crate) fn new(
        kind: ReplayParityKind,
        parity_identity: impl Into<String>,
        human_parity: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            parity_identity: parity_identity.into(),
            human_parity: human_parity.into(),
        }
    }

    pub fn kind(&self) -> ReplayParityKind {
        self.kind
    }

    pub fn parity_identity(&self) -> &str {
        &self.parity_identity
    }

    pub fn human_parity(&self) -> &str {
        &self.human_parity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayParityReport {
    admitted_consumer: SpatialCompiledProductConsumer,
    selected_family_identity: SpatialCompiledProductFamilyIdentity,
    admission_witness: SpatialCompiledProductAdmissionWitness,
    admission_provenance: ReplayParityAdmissionProvenance,
    rows: Vec<ReplayParityRow>,
}

impl ReplayParityReport {
    pub(crate) fn from_lowered(
        admitted_consumer: SpatialCompiledProductConsumer,
        selected_family_identity: SpatialCompiledProductFamilyIdentity,
        admission_witness: SpatialCompiledProductAdmissionWitness,
        admission_provenance: ReplayParityAdmissionProvenance,
        rows: Vec<ReplayParityRow>,
    ) -> Self {
        Self {
            admitted_consumer,
            selected_family_identity,
            admission_witness,
            admission_provenance,
            rows,
        }
    }

    pub fn rows(&self) -> &[ReplayParityRow] {
        &self.rows
    }

    pub fn admitted_consumer(&self) -> SpatialCompiledProductConsumer {
        self.admitted_consumer
    }

    pub fn selected_family_identity(&self) -> SpatialCompiledProductFamilyIdentity {
        self.selected_family_identity
    }

    #[cfg(test)]
    pub(crate) fn admission_witness(&self) -> &SpatialCompiledProductAdmissionWitness {
        &self.admission_witness
    }

    pub fn admission_provenance(&self) -> &ReplayParityAdmissionProvenance {
        &self.admission_provenance
    }

    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}
