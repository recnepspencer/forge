use forge_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BulkPlanKind {
    Ingest,
    Transform,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkSourceMember {
    pub(super) member_id: String,
    pub(super) width_units: u64,
}

impl BulkSourceMember {
    pub fn new(member_id: impl Into<String>, width_units: u64) -> Self {
        Self {
            member_id: member_id.into(),
            width_units,
        }
    }

    pub fn member_id(&self) -> &str {
        &self.member_id
    }

    pub fn width_units(&self) -> u64 {
        self.width_units
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkIngestSourceRequest {
    program_id: String,
    source_identity: String,
    target_branch_scope: BranchId,
    source_members: Vec<BulkSourceMember>,
}

impl BulkIngestSourceRequest {
    pub fn new(
        program_id: impl Into<String>,
        source_identity: impl Into<String>,
        target_branch_scope: BranchId,
        source_members: Vec<BulkSourceMember>,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            source_identity: source_identity.into(),
            target_branch_scope,
            source_members,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }
    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }
    pub fn source_members(&self) -> &[BulkSourceMember] {
        &self.source_members
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BulkTransformRequest {
    program_id: String,
    transform_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: CommitId,
    target_members: Vec<BulkSourceMember>,
}

impl BulkTransformRequest {
    pub fn new(
        program_id: impl Into<String>,
        transform_identity: impl Into<String>,
        target_branch_scope: BranchId,
        basis_commit_id: CommitId,
        target_members: Vec<BulkSourceMember>,
    ) -> Self {
        Self {
            program_id: program_id.into(),
            transform_identity: transform_identity.into(),
            target_branch_scope,
            basis_commit_id,
            target_members,
        }
    }

    pub fn program_id(&self) -> &str {
        &self.program_id
    }
    pub fn transform_identity(&self) -> &str {
        &self.transform_identity
    }
    pub fn target_branch_scope(&self) -> &BranchId {
        &self.target_branch_scope
    }
    pub fn basis_commit_id(&self) -> CommitId {
        self.basis_commit_id
    }
    pub fn target_members(&self) -> &[BulkSourceMember] {
        &self.target_members
    }
}
