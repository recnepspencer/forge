use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceIdentityScheme,
    WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};

use super::identity_boundary_hostile_matrix::WorthQueryIdentityBoundaryHostileMatrixArtifact;
use super::identity_boundary_inventory::{
    scan_format_digest_residue_paths, scan_lower_runtime_identity_shim_paths,
    scan_raw_session_admission_residue_paths, scan_string_carried_session_identity_residue_paths,
    EVIDENCE_IDENTITY_COVERED_SURFACES, EXACT_ZERO_FORMAT_DIGEST_PATHS,
    EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS, EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS,
    SESSION_LABEL_ORDINARY_ENTRYPOINTS, STOP_CLASS_COVERED_CONTRACTS,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryMilestoneClosureStatus {
    Open,
    Partial,
    Closed,
}

impl WorthQueryMilestoneClosureStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Partial => "partial",
            Self::Closed => "closed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryFolkloreResidueStatus {
    ZeroFolkloreResidue,
    FolkloreResidueRemaining(Vec<String>),
}

impl WorthQueryFolkloreResidueStatus {
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
            scan_raw_session_admission_residue_paths()
                .into_iter()
                .map(str::to_string),
        );
        remaining.extend(
            scan_string_carried_session_identity_residue_paths()
                .into_iter()
                .map(str::to_string),
        );
        remaining.extend(
            scan_lower_runtime_identity_shim_paths()
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
pub struct WorthQueryEvidenceIdentityBoundaryClosure {
    scheme: WorthQueryEvidenceIdentityScheme,
    status: WorthQueryMilestoneClosureStatus,
    closure_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryEvidenceIdentityBoundaryClosure {
    pub(crate) fn derived(
        support_matrix_digest: &str,
        ordinary_surface_available: bool,
        hostile_matrix_certified: bool,
        residue_clean: bool,
    ) -> Self {
        let scheme = WorthQueryEvidenceIdentityScheme::V1;
        let status = derive_boundary_status(
            ordinary_surface_available,
            residue_clean,
            hostile_matrix_certified,
        );
        let closure_identity = worth_query_evidence_identity(
            WorthQueryEvidenceScope::ApplicationEvidenceIdentityBoundaryClosure,
        )
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(WorthQueryEvidenceTag::new("scheme"), scheme.as_str())
        .field_value(
            WorthQueryEvidenceTag::new("support_matrix_digest"),
            support_matrix_digest,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("covered_surface"),
            EVIDENCE_IDENTITY_COVERED_SURFACES.iter().copied(),
        )
        .seal();
        Self {
            scheme,
            status,
            closure_identity,
        }
    }

    pub fn scheme(&self) -> WorthQueryEvidenceIdentityScheme {
        self.scheme
    }

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closure_identity
    }

    pub fn covered_surfaces(&self) -> &'static [&'static str] {
        EVIDENCE_IDENTITY_COVERED_SURFACES
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryStopClassBoundaryClosure {
    accessor: &'static str,
    status: WorthQueryMilestoneClosureStatus,
    closure_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryStopClassBoundaryClosure {
    pub(crate) fn derived(
        ordinary_surface_available: bool,
        hostile_matrix_certified: bool,
        residue_clean: bool,
    ) -> Self {
        let accessor = "WorthQueryRuntimeError::stop_class()";
        let status = derive_boundary_status(
            ordinary_surface_available,
            residue_clean,
            hostile_matrix_certified,
        );
        let closure_identity = worth_query_evidence_identity(
            WorthQueryEvidenceScope::ApplicationStopClassBoundaryClosure,
        )
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(WorthQueryEvidenceTag::new("accessor"), accessor)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("covered_contract"),
            STOP_CLASS_COVERED_CONTRACTS.iter().copied(),
        )
        .seal();
        Self {
            accessor,
            status,
            closure_identity,
        }
    }

    pub fn accessor(&self) -> &'static str {
        self.accessor
    }

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closure_identity
    }

    pub fn covered_contracts(&self) -> &'static [&'static str] {
        STOP_CLASS_COVERED_CONTRACTS
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySessionLabelBoundaryClosure {
    entry_label_type: &'static str,
    collision_stop_class: &'static str,
    status: WorthQueryMilestoneClosureStatus,
    closure_identity: WorthQueryEvidenceIdentity,
}

impl WorthQuerySessionLabelBoundaryClosure {
    pub(crate) fn derived(
        ordinary_surface_available: bool,
        hostile_matrix_certified: bool,
        residue_clean: bool,
    ) -> Self {
        let entry_label_type = "WorthQuerySessionLabel";
        let collision_stop_class = "WorthQueryStopClass::SessionLabelCollision";
        let status = derive_boundary_status(
            ordinary_surface_available,
            residue_clean,
            hostile_matrix_certified,
        );
        let closure_identity = worth_query_evidence_identity(
            WorthQueryEvidenceScope::ApplicationSessionLabelBoundaryClosure,
        )
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("entry_label_type"),
            entry_label_type,
        )
        .field_shape(
            WorthQueryEvidenceTag::new("collision_stop_class"),
            collision_stop_class,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("ordinary_entrypoint"),
            SESSION_LABEL_ORDINARY_ENTRYPOINTS.iter().copied(),
        )
        .seal();
        Self {
            entry_label_type,
            collision_stop_class,
            status,
            closure_identity,
        }
    }

    pub fn entry_label_type(&self) -> &'static str {
        self.entry_label_type
    }

    pub fn collision_stop_class(&self) -> &'static str {
        self.collision_stop_class
    }

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closure_identity
    }

    pub fn ordinary_entrypoints(&self) -> &'static [&'static str] {
        SESSION_LABEL_ORDINARY_ENTRYPOINTS
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIdentityBoundaryClosure {
    status: WorthQueryMilestoneClosureStatus,
    evidence_identity: WorthQueryEvidenceIdentityBoundaryClosure,
    stop_class: WorthQueryStopClassBoundaryClosure,
    session_label: WorthQuerySessionLabelBoundaryClosure,
    residue_status: WorthQueryFolkloreResidueStatus,
    hostile_matrix_certified: bool,
    hostile_matrix_digest: String,
    closure_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryIdentityBoundaryClosure {
    pub(crate) fn derived(
        support_matrix_digest: &str,
        hostile_matrix: &WorthQueryIdentityBoundaryHostileMatrixArtifact,
        evidence_identity_surface_available: bool,
        stop_class_surface_available: bool,
        session_label_surface_available: bool,
    ) -> Self {
        let hostile_matrix_certified = hostile_matrix.certified();
        let hostile_matrix_digest = hostile_matrix.artifact_digest();
        let format_digest_residue_paths = scan_format_digest_residue_paths();
        let raw_session_admission_residue_paths = scan_raw_session_admission_residue_paths();
        let string_carried_session_identity_residue_paths =
            scan_string_carried_session_identity_residue_paths();
        let evidence_identity_residue_clean = format_digest_residue_paths.is_empty();
        let session_label_residue_clean = raw_session_admission_residue_paths.is_empty()
            && string_carried_session_identity_residue_paths.is_empty();
        let evidence_identity = WorthQueryEvidenceIdentityBoundaryClosure::derived(
            support_matrix_digest,
            evidence_identity_surface_available,
            hostile_matrix_certified,
            evidence_identity_residue_clean,
        );
        let stop_class = WorthQueryStopClassBoundaryClosure::derived(
            stop_class_surface_available,
            hostile_matrix_certified,
            true,
        );
        let session_label = WorthQuerySessionLabelBoundaryClosure::derived(
            session_label_surface_available,
            hostile_matrix_certified,
            session_label_residue_clean,
        );
        let residue_status = WorthQueryFolkloreResidueStatus::derived();
        let status = derive_closure_status(
            evidence_identity.status(),
            stop_class.status(),
            session_label.status(),
            residue_status.is_zero(),
        );
        let mut closure_builder = worth_query_evidence_identity(
            WorthQueryEvidenceScope::ApplicationIdentityBoundaryClosure,
        )
        .field_shape(WorthQueryEvidenceTag::new("status"), status.as_str())
        .field_value(
            WorthQueryEvidenceTag::new("evidence_identity_closure_digest"),
            evidence_identity.closure_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("stop_class_closure_digest"),
            stop_class.closure_digest(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("session_label_closure_digest"),
            session_label.closure_digest(),
        )
        .field_shape(
            WorthQueryEvidenceTag::new("residue_status"),
            residue_status.as_str(),
        )
        .field_bool(
            WorthQueryEvidenceTag::new("hostile_matrix_certified"),
            hostile_matrix_certified,
        )
        .field_value(
            WorthQueryEvidenceTag::new("hostile_matrix_digest"),
            hostile_matrix_digest,
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("exact_zero_format_digest_path"),
            EXACT_ZERO_FORMAT_DIGEST_PATHS.iter().copied(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("exact_zero_raw_session_admission_path"),
            EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS.iter().copied(),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("exact_zero_string_carried_session_identity_path"),
            EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS
                .iter()
                .copied(),
        );
        if let WorthQueryFolkloreResidueStatus::FolkloreResidueRemaining(paths) = &residue_status {
            closure_builder = closure_builder.field_value_sequence(
                WorthQueryEvidenceTag::new("folklore_residue_path"),
                paths.iter().map(String::as_str),
            );
        }
        let closure_identity = closure_builder.seal();
        Self {
            status,
            evidence_identity,
            stop_class,
            session_label,
            residue_status,
            hostile_matrix_certified,
            hostile_matrix_digest: hostile_matrix_digest.to_string(),
            closure_identity,
        }
    }

    pub fn status(&self) -> WorthQueryMilestoneClosureStatus {
        self.status
    }

    pub fn evidence_identity(&self) -> &WorthQueryEvidenceIdentityBoundaryClosure {
        &self.evidence_identity
    }

    pub fn stop_class(&self) -> &WorthQueryStopClassBoundaryClosure {
        &self.stop_class
    }

    pub fn session_label(&self) -> &WorthQuerySessionLabelBoundaryClosure {
        &self.session_label
    }

    pub fn residue_status(&self) -> &WorthQueryFolkloreResidueStatus {
        &self.residue_status
    }

    pub fn hostile_matrix_digest(&self) -> &str {
        &self.hostile_matrix_digest
    }

    pub fn hostile_matrix_certified(&self) -> bool {
        self.hostile_matrix_certified
    }

    pub fn exact_zero_format_digest_paths(&self) -> &'static [&'static str] {
        EXACT_ZERO_FORMAT_DIGEST_PATHS
    }

    pub fn exact_zero_raw_session_admission_paths(&self) -> &'static [&'static str] {
        EXACT_ZERO_RAW_SESSION_ADMISSION_PATHS
    }

    pub fn exact_zero_string_carried_session_identity_paths(&self) -> &'static [&'static str] {
        EXACT_ZERO_STRING_CARRIED_SESSION_IDENTITY_PATHS
    }

    pub fn closure_digest(&self) -> &str {
        self.closure_identity.as_str()
    }

    pub fn closure_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.closure_identity
    }
}

fn derive_boundary_status(
    ordinary_surface_available: bool,
    residue_clean: bool,
    hostile_matrix_certified: bool,
) -> WorthQueryMilestoneClosureStatus {
    if ordinary_surface_available && residue_clean && hostile_matrix_certified {
        WorthQueryMilestoneClosureStatus::Closed
    } else if ordinary_surface_available || residue_clean || hostile_matrix_certified {
        WorthQueryMilestoneClosureStatus::Partial
    } else {
        WorthQueryMilestoneClosureStatus::Open
    }
}

fn derive_closure_status(
    evidence_identity: WorthQueryMilestoneClosureStatus,
    stop_class: WorthQueryMilestoneClosureStatus,
    session_label: WorthQueryMilestoneClosureStatus,
    residue_clean: bool,
) -> WorthQueryMilestoneClosureStatus {
    if residue_clean
        && evidence_identity == WorthQueryMilestoneClosureStatus::Closed
        && stop_class == WorthQueryMilestoneClosureStatus::Closed
        && session_label == WorthQueryMilestoneClosureStatus::Closed
    {
        WorthQueryMilestoneClosureStatus::Closed
    } else if evidence_identity == WorthQueryMilestoneClosureStatus::Open
        && stop_class == WorthQueryMilestoneClosureStatus::Open
        && session_label == WorthQueryMilestoneClosureStatus::Open
        && !residue_clean
    {
        WorthQueryMilestoneClosureStatus::Open
    } else {
        WorthQueryMilestoneClosureStatus::Partial
    }
}
