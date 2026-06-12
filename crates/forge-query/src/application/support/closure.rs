use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentityScheme, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryFolkloreResidueStatus {
    ZeroFolkloreResidue,
}

impl ForgeQueryFolkloreResidueStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ZeroFolkloreResidue => "zero-folklore-residue",
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
            [
                ForgeQueryEvidenceScope::RuntimePublicSupportMatrixRow.as_str(),
                ForgeQueryEvidenceScope::RuntimePublicSupportMatrix.as_str(),
                ForgeQueryEvidenceScope::RuntimePublicApiFamilyContract.as_str(),
                ForgeQueryEvidenceScope::RuntimePublicApiContract.as_str(),
                ForgeQueryEvidenceScope::RuntimePublicApiTranscriptEvidence.as_str(),
                ForgeQueryEvidenceScope::RuntimeStateSnapshot.as_str(),
                ForgeQueryEvidenceScope::PreviewBasisAdmission.as_str(),
                ForgeQueryEvidenceScope::BranchBasisAdmission.as_str(),
                ForgeQueryEvidenceScope::PreviewIntentAdmission.as_str(),
                ForgeQueryEvidenceScope::PreviewIntentReceipt.as_str(),
                ForgeQueryEvidenceScope::BranchIntentAdmission.as_str(),
                ForgeQueryEvidenceScope::BranchIntentReceipt.as_str(),
                ForgeQueryEvidenceScope::IntentDenialEvidence.as_str(),
                ForgeQueryEvidenceScope::ApplicationSupportReport.as_str(),
            ],
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
            [
                "typed-family-admission-denial",
                "typed-preview-promotion-stop",
                "typed-session-label-collision-stop",
            ],
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIdentityBoundaryClosure {
    evidence_identity: ForgeQueryEvidenceIdentityBoundaryClosure,
    stop_class: ForgeQueryStopClassBoundaryClosure,
    session_label: ForgeQuerySessionLabelBoundaryClosure,
    residue_status: ForgeQueryFolkloreResidueStatus,
    closure_digest: String,
}

impl ForgeQueryIdentityBoundaryClosure {
    pub(crate) fn closed(support_matrix_digest: &str) -> Self {
        let evidence_identity =
            ForgeQueryEvidenceIdentityBoundaryClosure::closed(support_matrix_digest);
        let stop_class = ForgeQueryStopClassBoundaryClosure::closed();
        let session_label = ForgeQuerySessionLabelBoundaryClosure::closed();
        let residue_status = ForgeQueryFolkloreResidueStatus::ZeroFolkloreResidue;
        let closure_digest = forge_query_evidence_identity(
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
        .seal()
        .as_str()
        .to_string();
        Self {
            evidence_identity,
            stop_class,
            session_label,
            residue_status,
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

    pub fn residue_status(&self) -> ForgeQueryFolkloreResidueStatus {
        self.residue_status
    }

    pub fn closure_digest(&self) -> &str {
        &self.closure_digest
    }
}
