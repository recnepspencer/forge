use worth_query::facade::{
    consumer_kit::WorthQuerySupportSnapshotRow, WorthQueryRuntimePublicApiFamilyContract,
};

use super::{
    WorthServerDirectDeclaration, WorthServerDirectDeclarationSourceKind,
    WorthServerDirectDeclarationSourceSupportStatus, WorthServerDirectViewShape,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerDirectSupportSnapshot {
    declaration: WorthServerDirectDeclaration,
    source_kind: WorthServerDirectDeclarationSourceKind,
    source_support_status: WorthServerDirectDeclarationSourceSupportStatus,
    source_support_reason: String,
    read_family_row: WorthQuerySupportSnapshotRow,
    read_family_contract: Option<WorthQueryRuntimePublicApiFamilyContract>,
    support_matrix_digest: String,
    support_snapshot_digest: String,
    read_family_pin_report_digest: String,
    read_family_pin_satisfied: bool,
    support_posture_digest: String,
}

impl WorthServerDirectSupportSnapshot {
    pub(crate) fn new(
        declaration: WorthServerDirectDeclaration,
        read_family_row: WorthQuerySupportSnapshotRow,
        read_family_contract: Option<WorthQueryRuntimePublicApiFamilyContract>,
        support_matrix_digest: String,
        support_snapshot_digest: String,
        read_family_pin_report_digest: String,
        read_family_pin_satisfied: bool,
    ) -> Self {
        let source_kind = declaration.source().kind();
        let source_support_status = declaration.source().support_status();
        let source_support_reason = declaration.source().support_reason().to_string();
        let support_posture_digest = format!(
            "worth-server-direct-support-v2|source:{}|source_status:{}|reason:{}|view_shape:{}|read_row:{}|read_live_row:{}|read_contract:{}|matrix:{}|snapshot:{}|pin_report:{}|pin_satisfied:{}",
            source_kind.as_str(),
            source_support_status.as_str(),
            source_support_reason,
            declaration.view_shape().as_str(),
            read_family_row.snapshot_row_digest(),
            read_family_row.live_row_digest(),
            read_family_contract
                .as_ref()
                .map(|contract| contract.contract_digest())
                .unwrap_or("none"),
            support_matrix_digest,
            support_snapshot_digest,
            read_family_pin_report_digest,
            read_family_pin_satisfied,
        );
        Self {
            declaration,
            source_kind,
            source_support_status,
            source_support_reason,
            read_family_row,
            read_family_contract,
            support_matrix_digest,
            support_snapshot_digest,
            read_family_pin_report_digest,
            read_family_pin_satisfied,
            support_posture_digest,
        }
    }

    pub fn declaration(&self) -> &WorthServerDirectDeclaration {
        &self.declaration
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

    pub fn read_family_row(&self) -> &WorthQuerySupportSnapshotRow {
        &self.read_family_row
    }

    pub fn read_family_contract(&self) -> Option<&WorthQueryRuntimePublicApiFamilyContract> {
        self.read_family_contract.as_ref()
    }

    pub fn support_matrix_digest(&self) -> &str {
        &self.support_matrix_digest
    }

    pub fn support_snapshot_digest(&self) -> &str {
        &self.support_snapshot_digest
    }

    pub fn read_family_pin_report_digest(&self) -> &str {
        &self.read_family_pin_report_digest
    }

    pub fn read_family_pin_satisfied(&self) -> bool {
        self.read_family_pin_satisfied
    }

    pub fn support_posture_digest(&self) -> &str {
        &self.support_posture_digest
    }

    pub fn view_shape(&self) -> WorthServerDirectViewShape {
        self.declaration.view_shape()
    }

    pub fn is_admitted_now(&self) -> bool {
        self.source_support_status == WorthServerDirectDeclarationSourceSupportStatus::Supported
            && self.read_family_contract.is_some()
            && self.read_family_pin_satisfied
    }
}
