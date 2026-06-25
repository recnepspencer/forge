use super::super::digest::digest_parts;
use crate::runtime::{
    WorthUiCompositionRootKind, WorthUiCompositionRootReceipt, WorthUiRuntimeFactId,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionRootMountDenialCode {
    MissingPageSlot,
    InvalidSurfaceId,
    MissingSurface,
    MissingComponentInstance,
    MissingPortalEntry,
    MissingCollectionItem,
    MissingDiagnosticPanel,
    DuplicateRootIdentity,
    MosaicPlacementDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootMountDenial {
    code: WorthUiCompositionRootMountDenialCode,
    root_kind: WorthUiCompositionRootKind,
    root_authority_identity: String,
    subject: String,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    denial_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionRootMountReport {
    denials: Vec<WorthUiCompositionRootMountDenial>,
    denial_set_digest: u64,
}

impl WorthUiCompositionRootMountDenial {
    pub(crate) fn new(
        code: WorthUiCompositionRootMountDenialCode,
        root: &WorthUiCompositionRootReceipt,
        subject: impl Into<String>,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let subject = subject.into();
        let denial_digest = digest_parts([
            code.token(),
            root.kind().token(),
            root.authority_identity(),
            subject.as_str(),
        ]);
        Self {
            code,
            root_kind: root.kind(),
            root_authority_identity: root.authority_identity().to_owned(),
            subject,
            consumed_facts,
            denial_digest,
        }
    }

    pub fn code(&self) -> WorthUiCompositionRootMountDenialCode {
        self.code
    }

    pub fn root_kind(&self) -> WorthUiCompositionRootKind {
        self.root_kind
    }

    pub fn root_authority_identity(&self) -> &str {
        &self.root_authority_identity
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }
}

impl WorthUiCompositionRootMountReport {
    pub(crate) fn denied(denials: Vec<WorthUiCompositionRootMountDenial>) -> Self {
        let denial_set_digest = digest_parts(denials.iter().map(|denial| {
            format!(
                "{}:{}:{}",
                denial.code().token(),
                denial.root_kind().token(),
                denial.subject()
            )
        }));
        Self {
            denials,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiCompositionRootMountDenial] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

impl WorthUiCompositionRootMountDenialCode {
    pub const fn token(self) -> &'static str {
        match self {
            Self::MissingPageSlot => "composition_root_mount.missing_page_slot",
            Self::InvalidSurfaceId => "composition_root_mount.invalid_surface_id",
            Self::MissingSurface => "composition_root_mount.missing_surface",
            Self::MissingComponentInstance => "composition_root_mount.missing_component_instance",
            Self::MissingPortalEntry => "composition_root_mount.missing_portal_entry",
            Self::MissingCollectionItem => "composition_root_mount.missing_collection_item",
            Self::MissingDiagnosticPanel => "composition_root_mount.missing_diagnostic_panel",
            Self::DuplicateRootIdentity => "composition_root_mount.duplicate_root_identity",
            Self::MosaicPlacementDenied => "composition_root_mount.mosaic_placement_denied",
        }
    }
}
