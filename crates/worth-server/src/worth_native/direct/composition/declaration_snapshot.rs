use crate::{
    WorthServerAdmittedDirectDeclaration, WorthServerDirectDeclaration,
    WorthServerDirectDeclarationSourceKind, WorthServerDirectDeclarationSourceSupportStatus,
    WorthServerDirectViewShape,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectDeclarationSnapshot {
    declaration: WorthServerDirectDeclaration,
    declaration_digest: String,
    workspace_name: String,
    source_kind: WorthServerDirectDeclarationSourceKind,
    source_support_status: WorthServerDirectDeclarationSourceSupportStatus,
    source_support_reason: String,
    view_shape: WorthServerDirectViewShape,
    support_matrix_digest: String,
    support_posture_digest: String,
    family_contract_digest: String,
    canonical_digest: String,
}

impl WorthServerDirectDeclarationSnapshot {
    pub(crate) fn from_admitted(declaration: &WorthServerAdmittedDirectDeclaration) -> Self {
        let support_snapshot = declaration.support_snapshot();
        let family_contract_digest = declaration
            .query_family_contract()
            .contract_digest()
            .to_string();
        let canonical_digest = format!(
            "worth-server-direct-declaration-snapshot-v1|declaration:{}|workspace:{}|support:{}|family:{}",
            declaration.declaration_digest(),
            declaration.workspace_name(),
            support_snapshot.support_posture_digest(),
            family_contract_digest,
        );
        Self {
            declaration: declaration.declaration().clone(),
            declaration_digest: declaration.declaration_digest().to_string(),
            workspace_name: declaration.workspace_name().to_string(),
            source_kind: support_snapshot.source_kind(),
            source_support_status: support_snapshot.source_support_status(),
            source_support_reason: support_snapshot.source_support_reason().to_string(),
            view_shape: support_snapshot.view_shape(),
            support_matrix_digest: support_snapshot.support_matrix_digest().to_string(),
            support_posture_digest: support_snapshot.support_posture_digest().to_string(),
            family_contract_digest,
            canonical_digest,
        }
    }

    pub fn declaration(&self) -> &WorthServerDirectDeclaration {
        &self.declaration
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn source_kind(&self) -> WorthServerDirectDeclarationSourceKind {
        self.source_kind
    }

    pub fn source_support_status(&self) -> WorthServerDirectDeclarationSourceSupportStatus {
        self.source_support_status
    }

    pub fn source_support_reason(&self) -> &str {
        &self.source_support_reason
    }

    pub fn view_shape(&self) -> WorthServerDirectViewShape {
        self.view_shape
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn support_posture_digest(&self) -> &str {
        &self.support_posture_digest
    }

    pub fn family_contract_digest(&self) -> &str {
        &self.family_contract_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
