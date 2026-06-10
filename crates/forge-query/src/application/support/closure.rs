use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentityScheme, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::identity_boundary_inventory::{
    scan_format_digest_residue_paths, scan_raw_session_admission_residue_paths,
    scan_string_matching_residue_paths, EXACT_ZERO_FORMAT_DIGEST_PATHS,
    EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS, EXACT_ZERO_STRING_MATCHING_PATHS,
    EVIDENCE_IDENTITY_COVERED_SURFACES, SESSION_LABEL_ORDINARY_ENTRYPOINTS,
    STOP_CLASS_COVERED_CONTRACTS,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryMilestoneClosureStatus {
    Closed,
}

impl ForgeQueryMilestoneClosureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryFolkloreResidueStatus {
    ZeroFolkloreResidue,
    FolkloreResidueRemaining(Vec<String>),
}

impl ForgeQueryFolkloreResidueStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ZeroFolkloreResidue => "zero-folklore-residue",
            Self::FolkloreResidueRemaining(paths) if paths.is_empty() => "zero-folklore-residue",
            Self::FolkloreResidueRemaining(_) => "folklore-residue-remaining",
        }
    }

    pub fn remaining_paths(&self) -> &[String] {
        match self {
            Self::ZeroFolkloreResidue => &[],
            Self::FolkloreResidueRemaining(paths) => paths.as_slice(),
        }
    }

    pub fn is_zero(&self) -> bool {
        matches!(self, Self::ZeroFolkloreResidue)
    }

    pub(crate) fn derived() -> Self {
        let mut remaining = scan_format_digest_residue_paths()
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        remaining.extend(
            scan_string_matching_residue_paths()
                .into_iter()
                .map(str::to_string),
        );
        remaining.extend(
            scan_raw_session_admission_residue_paths()
                .into_iter()
                .map(str::to_string),
        );
        remaining.sort();
        remaining.dedup();
        if remaining.is_empty() {
            Self::ZeroFolkloreResidue
        } else {
            Self::FolkloreResidueRemaining(remaining)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEvidenceIdentityBoundaryClosure {
    scheme: ForgeQueryEvidenceIdentityScheme,
    status: ForgeQueryMilestoneClosureStatus,
    closure_digest: String,
}

impl ForgeQueryEvidenceIdentityBoundaryClosure {
    pub(crate) fn closed(support_matrix_digest: &str) -> Self {
        let scheme = ForgeQueryEvidenceIdentityScheme::V1;
        let status = ForgeQueryMilestoneClosureStatus::Closed;
        let closure_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ApplicationEvidenceIdentityBoundaryClosure,
        )
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("scheme"), scheme.as_str())
        .field_identity(
            ForgeQueryEvidenceTag::new("support_matrix_digest"),
            support_matrix_digest,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("covered_surface"),
            EVIDENCE_IDENTITY_COVERED_SURFACES.iter().copied(),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            scheme,
            status,
            closure_digest,
        }
    }

    pub fn scheme(&self) -> ForgeQueryEvidenceIdentityScheme {
        self.scheme
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }

    pub fn covered_surfaces(&self) -> &'static [&'static str] {
        EVIDENCE_IDENTITY_COVERED_SURFACES
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryStopClassBoundaryClosure {
    accessor: &'static str,
    status: ForgeQueryMilestoneClosureStatus,
    closure_digest: String,
}

impl ForgeQueryStopClassBoundaryClosure {
    pub(crate) fn closed() -> Self {
        let accessor = "ForgeQueryRuntimeError::stop_class()";
        let status = ForgeQueryMilestoneClosureStatus::Closed;
        let closure_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ApplicationStopClassBoundaryClosure,
        )
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("accessor"), accessor)
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("covered_contract"),
            STOP_CLASS_COVERED_CONTRACTS.iter().copied(),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            accessor,
            status,
            closure_digest,
        }
    }

    pub fn accessor(&self) -> &'static str {
        self.accessor
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }

    pub fn covered_contracts(&self) -> &'static [&'static str] {
        STOP_CLASS_COVERED_CONTRACTS
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQuerySessionLabelBoundaryClosure {
    entry_label_type: &'static str,
    collision_stop_class: &'static str,
    status: ForgeQueryMilestoneClosureStatus,
    closure_digest: String,
}

impl ForgeQuerySessionLabelBoundaryClosure {
    pub(crate) fn closed() -> Self {
        let entry_label_type = "ForgeQuerySessionLabel";
        let collision_stop_class = "ForgeQueryStopClass::SessionLabelCollision";
        let status = ForgeQueryMilestoneClosureStatus::Closed;
        let closure_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ApplicationSessionLabelBoundaryClosure,
        )
        .field_shape(ForgeQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(
            ForgeQueryEvidenceTag::new("entry_label_type"),
            entry_label_type,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("collision_stop_class"),
            collision_stop_class,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("ordinary_entrypoint"),
            SESSION_LABEL_ORDINARY_ENTRYPOINTS.iter().copied(),
        )
        .seal()
        .as_str()
        .to_string();
        Self {
            entry_label_type,
            collision_stop_class,
            status,
            closure_digest,
        }
    }

    pub fn entry_label_type(&self) -> &'static str {
        self.entry_label_type
    }

    pub fn collision_stop_class(&self) -> &'static str {
        self.collision_stop_class
    }

    pub fn status(&self) -> ForgeQueryMilestoneClosureStatus {
        self.status
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }

    pub fn ordinary_entrypoints(&self) -> &'static [&'static str] {
        SESSION_LABEL_ORDINARY_ENTRYPOINTS
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIdentityBoundaryClosure {
    evidence_identity: ForgeQueryEvidenceIdentityBoundaryClosure,
    stop_class: ForgeQueryStopClassBoundaryClosure,
    session_label: ForgeQuerySessionLabelBoundaryClosure,
    residue_status: ForgeQueryFolkloreResidueStatus,
    hostile_matrix_digest: String,
    closure_digest: String,
}

impl ForgeQueryIdentityBoundaryClosure {
    pub(crate) fn closed(support_matrix_digest: &str, hostile_matrix_digest: &str) -> Self {
        let evidence_identity =
            ForgeQueryEvidenceIdentityBoundaryClosure::closed(support_matrix_digest);
        let stop_class = ForgeQueryStopClassBoundaryClosure::closed();
        let session_label = ForgeQuerySessionLabelBoundaryClosure::closed();
        let residue_status = ForgeQueryFolkloreResidueStatus::derived();
        let mut closure_builder = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::ApplicationIdentityBoundaryClosure,
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("evidence_identity_closure_digest"),
            evidence_identity.closure_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("stop_class_closure_digest"),
            stop_class.closure_digest(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_closure_digest"),
            session_label.closure_digest(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("residue_status"),
            residue_status.as_str(),
        )
        .field_identity(
            ForgeQueryEvidenceTag::new("hostile_matrix_digest"),
            hostile_matrix_digest,
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("exact_zero_format_digest_path"),
            EXACT_ZERO_FORMAT_DIGEST_PATHS.iter().copied(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("exact_zero_string_matching_path"),
            EXACT_ZERO_STRING_MATCHING_PATHS.iter().copied(),
        )
        .field_identity_sequence(
            ForgeQueryEvidenceTag::new("exact_zero_raw_session_admission_path"),
            EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS.iter().copied(),
        );
        if let ForgeQueryFolkloreResidueStatus::FolkloreResidueRemaining(paths) = &residue_status {
            closure_builder = closure_builder.field_identity_sequence(
                ForgeQueryEvidenceTag::new("folklore_residue_path"),
                paths.iter().map(String::as_str),
            );
        }
        let closure_digest = closure_builder.seal().as_str().to_string();
        Self {
            evidence_identity,
            stop_class,
            session_label,
            residue_status,
            hostile_matrix_digest: hostile_matrix_digest.to_string(),
            closure_digest,
        }
    }

    pub fn evidence_identity(&self) -> &ForgeQueryEvidenceIdentityBoundaryClosure {
        &self.evidence_identity
    }

    pub fn stop_class(&self) -> &ForgeQueryStopClassBoundaryClosure {
        &self.stop_class
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabelBoundaryClosure {
        &self.session_label
    }

    pub fn residue_status(&self) -> &ForgeQueryFolkloreResidueStatus {
        &self.residue_status
    }

    pub fn hostile_matrix_digest(&self) -> &str {
        &self.hostile_matrix_digest
    }

    pub fn exact_zero_format_digest_paths(&self) -> &'static [&'static str] {
        EXACT_ZERO_FORMAT_DIGEST_PATHS
    }

    pub fn exact_zero_string_matching_paths(&self) -> &'static [&'static str] {
        EXACT_ZERO_STRING_MATCHING_PATHS
    }

    pub fn exact_zero_raw_session_admission_paths(&self) -> &'static [&'static str] {
        EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }
}
