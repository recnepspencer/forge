use forge_query::facade::{
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryRuntimePublicSupportMatrixRow,
};

use super::{
    ForgeServerDirectDeclaration, ForgeServerDirectDeclarationSourceKind,
    ForgeServerDirectDeclarationSourceSupportStatus, ForgeServerDirectViewShape,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerDirectSupportSnapshot {
    declaration: ForgeServerDirectDeclaration,
    source_kind: ForgeServerDirectDeclarationSourceKind,
    source_support_status: ForgeServerDirectDeclarationSourceSupportStatus,
    source_support_reason: String,
    read_family_row: ForgeQueryRuntimePublicSupportMatrixRow,
    read_family_contract: Option<ForgeQueryRuntimePublicApiFamilyContract>,
    support_matrix_digest: String,
    support_posture_digest: String,
}

impl ForgeServerDirectSupportSnapshot {
    pub(crate) fn new(
        declaration: ForgeServerDirectDeclaration,
        read_family_row: ForgeQueryRuntimePublicSupportMatrixRow,
        read_family_contract: Option<ForgeQueryRuntimePublicApiFamilyContract>,
        support_matrix_digest: String,
    ) -> Self {
        let source_kind = declaration.source().kind();
        let source_support_status = declaration.source().support_status();
        let source_support_reason = declaration.source().support_reason().to_string();
        let support_posture_digest = format!(
            "forge-server-direct-support-v1|source:{}|source_status:{}|reason:{}|view_shape:{}|read_row:{}|read_contract:{}|matrix:{}",
            source_kind.as_str(),
            source_support_status.as_str(),
            source_support_reason,
            declaration.view_shape().as_str(),
            read_family_row.row_digest(),
            read_family_contract
                .as_ref()
                .map(|contract| contract.contract_digest())
                .unwrap_or("none"),
            support_matrix_digest,
        );
        Self {
            declaration,
            source_kind,
            source_support_status,
            source_support_reason,
            read_family_row,
            read_family_contract,
            support_matrix_digest,
            support_posture_digest,
        }
    }

    pub fn declaration(&self) -> &ForgeServerDirectDeclaration {
        &self.declaration
    }

    pub fn source_kind(&self) -> ForgeServerDirectDeclarationSourceKind {
        self.source_kind
    }

    pub fn source_support_status(&self) -> ForgeServerDirectDeclarationSourceSupportStatus {
        self.source_support_status
    }

    pub fn source_support_reason(&self) -> &str {
        &self.source_support_reason
    }

    pub fn read_family_row(&self) -> &ForgeQueryRuntimePublicSupportMatrixRow {
        &self.read_family_row
    }

    pub fn read_family_contract(&self) -> Option<&ForgeQueryRuntimePublicApiFamilyContract> {
        self.read_family_contract.as_ref()
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn support_posture_digest(&self) -> &str {
        &self.support_posture_digest
    }

    pub fn view_shape(&self) -> ForgeServerDirectViewShape {
        self.declaration.view_shape()
    }

    pub fn is_admitted_now(&self) -> bool {
        self.source_support_status == ForgeServerDirectDeclarationSourceSupportStatus::Supported
            && self.read_family_contract.is_some()
    }
}
