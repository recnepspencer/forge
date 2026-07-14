use std::sync::atomic::{AtomicU64, Ordering};

use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::WorthQuerySnapshotIdentity;
use crate::session_label::WorthQuerySessionLabel;

use super::{
    WorthQueryEffectPolicy, WorthQueryInspection, WorthQueryPreviewBasisAdmission,
    WorthQueryRuntime, WorthQueryRuntimeError, WorthQueryRuntimeFacadeFamily,
    WorthQueryWriteCommand, WorthQueryWriteReceipt,
};

static NEXT_RUNTIME_AUTHORITY_IDENTITY: AtomicU64 = AtomicU64::new(1);

/// Process-local identity for one concrete runtime authority owner.
///
/// This is intentionally not a workspace name or a consumer-authored digest.
/// Only the runtime builder can mint it, so ordinary contexts cannot be moved
/// between otherwise similar workspaces.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct WorthQueryRuntimeAuthorityIdentity(u64);

impl WorthQueryRuntimeAuthorityIdentity {
    pub(crate) fn mint() -> Self {
        Self(NEXT_RUNTIME_AUTHORITY_IDENTITY.fetch_add(1, Ordering::Relaxed))
    }

    fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryOrdinaryAuthorityFamily {
    Mutation,
    ReadOnlyPreview,
    PromotionPreview,
    Writeback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryOrdinaryAuthorityDrift {
    Current,
    ForeignOwner,
    StaleSnapshot,
}

impl WorthQueryOrdinaryAuthorityFamily {
    fn as_str(self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::ReadOnlyPreview => "read-only-preview",
            Self::PromotionPreview => "promotion-preview",
            Self::Writeback => "writeback",
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryOrdinaryAuthorityAdmission {
    family: WorthQueryOrdinaryAuthorityFamily,
    runtime_identity: WorthQueryRuntimeAuthorityIdentity,
    snapshot_identity: WorthQuerySnapshotIdentity,
    session_label: Option<WorthQuerySessionLabel>,
    preview_basis: Option<WorthQueryPreviewBasisAdmission>,
    admission_identity: WorthQueryEvidenceIdentity,
}

pub(crate) struct WorthQueryLowerRuntimeMutationExecution {
    request_identity: WorthQueryEvidenceIdentity,
    handoff_identity: WorthQueryEvidenceIdentity,
    receipt_identity: WorthQueryEvidenceIdentity,
    inspection_identity: Option<WorthQueryEvidenceIdentity>,
    receipt: WorthQueryWriteReceipt,
}

impl WorthQueryLowerRuntimeMutationExecution {
    pub(crate) fn request_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.request_identity
    }

    pub(crate) fn handoff_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.handoff_identity
    }

    pub(crate) fn receipt_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.receipt_identity
    }

    pub(crate) fn inspection_identity(&self) -> Option<&WorthQueryEvidenceIdentity> {
        self.inspection_identity.as_ref()
    }

    pub(crate) fn into_receipt(self) -> WorthQueryWriteReceipt {
        self.receipt
    }
}

impl WorthQueryOrdinaryAuthorityAdmission {
    pub(crate) fn family(&self) -> WorthQueryOrdinaryAuthorityFamily {
        self.family
    }

    pub(crate) fn snapshot_identity(&self) -> &WorthQuerySnapshotIdentity {
        &self.snapshot_identity
    }

    pub(crate) fn session_label(&self) -> Option<&WorthQuerySessionLabel> {
        self.session_label.as_ref()
    }

    pub(crate) fn preview_basis(&self) -> Option<&WorthQueryPreviewBasisAdmission> {
        self.preview_basis.as_ref()
    }

    pub(crate) fn into_preview_basis(self) -> Option<WorthQueryPreviewBasisAdmission> {
        self.preview_basis
    }

    pub(crate) fn admission_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.admission_identity
    }
}

impl WorthQueryRuntime {
    pub(crate) fn capture_ordinary_mutation_authority(
        &self,
    ) -> Result<WorthQueryOrdinaryAuthorityAdmission, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Write)?;
        Ok(self.ordinary_authority_admission(
            WorthQueryOrdinaryAuthorityFamily::Mutation,
            None,
            None,
        ))
    }

    pub(crate) fn admit_ordinary_rich_inspection(&self) -> Result<(), WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Inspect)
    }

    pub(crate) fn capture_ordinary_writeback_authority(
        &self,
    ) -> Result<WorthQueryOrdinaryAuthorityAdmission, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::Effect)?;
        self.backend.admit_query_writeback_authority()?;
        Ok(self.ordinary_authority_admission(
            WorthQueryOrdinaryAuthorityFamily::Writeback,
            None,
            None,
        ))
    }

    pub(crate) fn capture_ordinary_preview_authority(
        &self,
        label: WorthQuerySessionLabel,
        effect_policy: WorthQueryEffectPolicy,
    ) -> Result<WorthQueryOrdinaryAuthorityAdmission, WorthQueryRuntimeError> {
        self.admit_facade_family(WorthQueryRuntimeFacadeFamily::BranchPreview)?;
        let preview_basis =
            self.backend
                .admit_preview_basis(&label, effect_policy, &self.evidence_authority)?;
        let family = match effect_policy {
            WorthQueryEffectPolicy::DeriveOnly => {
                WorthQueryOrdinaryAuthorityFamily::ReadOnlyPreview
            }
            WorthQueryEffectPolicy::SandboxedWriteIntent => {
                WorthQueryOrdinaryAuthorityFamily::PromotionPreview
            }
            WorthQueryEffectPolicy::Muted
            | WorthQueryEffectPolicy::Redirected
            | WorthQueryEffectPolicy::AuthoritativeAllowed => {
                return Err(WorthQueryRuntimeError::Workspace(
                    crate::memory_workspace::WorthQueryWorkspaceError::new(
                        "ordinary preview contexts cannot admit external-effect authority",
                    ),
                ));
            }
        };
        Ok(self.ordinary_authority_admission(family, Some(label), Some(preview_basis)))
    }

    pub(crate) fn ordinary_authority_drift(
        &self,
        admission: &WorthQueryOrdinaryAuthorityAdmission,
    ) -> WorthQueryOrdinaryAuthorityDrift {
        if admission.runtime_identity != self.authority_identity {
            WorthQueryOrdinaryAuthorityDrift::ForeignOwner
        } else if admission.snapshot_identity != self.current_snapshot_identity() {
            WorthQueryOrdinaryAuthorityDrift::StaleSnapshot
        } else {
            WorthQueryOrdinaryAuthorityDrift::Current
        }
    }

    pub(crate) fn execute_ordinary_authoritative_mutation(
        &mut self,
        command: WorthQueryWriteCommand,
        materialize_inspection: bool,
    ) -> Result<WorthQueryLowerRuntimeMutationExecution, WorthQueryRuntimeError> {
        let admitted = self.write_intent(command).admit()?;
        let request_identity =
            workflow_lower_identity("request", admitted.handoff().request_digest());
        let handoff_identity =
            workflow_lower_identity("handoff", admitted.handoff().handoff_digest());
        let receipt = admitted.execute()?;
        let receipt_identity = receipt.commit_evidence_identity().clone();
        let inspection_identity = if materialize_inspection {
            let inspection = match self.inspect(&receipt)? {
                WorthQueryInspection::WriteReceipt(inspection) => inspection,
                other => panic!("expected write receipt inspection, got {other:?}"),
            };
            Some(inspection.inspection_identity().clone())
        } else {
            None
        };
        Ok(WorthQueryLowerRuntimeMutationExecution {
            request_identity,
            handoff_identity,
            receipt_identity,
            inspection_identity,
            receipt,
        })
    }

    fn ordinary_authority_admission(
        &self,
        family: WorthQueryOrdinaryAuthorityFamily,
        session_label: Option<WorthQuerySessionLabel>,
        preview_basis: Option<WorthQueryPreviewBasisAdmission>,
    ) -> WorthQueryOrdinaryAuthorityAdmission {
        let snapshot_identity = self.current_snapshot_identity();
        let mut identity =
            WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowContextBinding)
                .field_shape(
                    WorthQueryEvidenceTag::new("role"),
                    "ordinary-authority-admission",
                )
                .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
                .field_value(
                    WorthQueryEvidenceTag::new("runtime_authority"),
                    self.authority_identity.as_u64().to_string(),
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("snapshot"),
                    &snapshot_identity.evidence_identity(),
                );
        if let Some(label) = session_label.as_ref() {
            identity = identity.field_value(
                WorthQueryEvidenceTag::new("session_label"),
                label.identity_digest().as_str(),
            );
        }
        if let Some(basis) = preview_basis.as_ref() {
            identity = identity.field_value(
                WorthQueryEvidenceTag::new("preview_basis"),
                basis.admission_digest().as_str(),
            );
        }
        WorthQueryOrdinaryAuthorityAdmission {
            family,
            runtime_identity: self.authority_identity,
            snapshot_identity,
            session_label,
            preview_basis,
            admission_identity: identity.seal(),
        }
    }
}

fn workflow_lower_identity(role: &'static str, digest: &str) -> WorthQueryEvidenceIdentity {
    WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::WorkflowMutationLowering)
        .field_shape(WorthQueryEvidenceTag::new("role"), role)
        .field_value(WorthQueryEvidenceTag::new("digest"), digest)
        .seal()
}
