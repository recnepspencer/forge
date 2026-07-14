use crate::failure::{StoreError, StoreErrorKind};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::{Deserialize, Serialize};

use super::{
    core::{BulkIngestSourceRequest, BulkPlanKind, BulkSourceMember, BulkTransformRequest},
    utils::{
        canonicalize_members, ensure_program_identity, stable_json_digest, BULK_FAMILY_VERSION,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenBulkSourceManifest {
    family_version: u32,
    program_id: String,
    source_identity: String,
    target_branch_scope: BranchId,
    ordered_members: Vec<BulkSourceMember>,
    manifest_digest: String,
}

impl FrozenBulkSourceManifest {
    pub fn freeze(request: BulkIngestSourceRequest) -> Result<Self, StoreError> {
        ensure_program_identity(request.program_id(), request.source_identity())?;
        let ordered_members = canonicalize_members(request.source_members().to_vec())?;

        #[derive(Serialize)]
        struct DigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            source_identity: &'a str,
            target_branch_scope: &'a BranchId,
            ordered_members: &'a [BulkSourceMember],
        }
        let manifest_digest = stable_json_digest(
            "bulk ingest source manifest",
            &DigestInput {
                family_version: BULK_FAMILY_VERSION,
                program_id: request.program_id(),
                source_identity: request.source_identity(),
                target_branch_scope: request.target_branch_scope(),
                ordered_members: &ordered_members,
            },
        )?;

        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id().to_string(),
            source_identity: request.source_identity().to_string(),
            target_branch_scope: request.target_branch_scope().clone(),
            ordered_members,
            manifest_digest,
        })
    }
    pub fn family_version(&self) -> u32 {
        self.family_version
    }
    pub fn kind(&self) -> BulkPlanKind {
        BulkPlanKind::Ingest
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
    pub fn ordered_members(&self) -> &[BulkSourceMember] {
        &self.ordered_members
    }
    pub fn manifest_digest(&self) -> &str {
        &self.manifest_digest
    }
    pub(crate) fn has_valid_digest(&self) -> Result<bool, StoreError> {
        #[derive(Serialize)]
        struct DigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            source_identity: &'a str,
            target_branch_scope: &'a BranchId,
            ordered_members: &'a [BulkSourceMember],
        }
        Ok(self.manifest_digest
            == stable_json_digest(
                "bulk ingest source manifest",
                &DigestInput {
                    family_version: self.family_version,
                    program_id: &self.program_id,
                    source_identity: &self.source_identity,
                    target_branch_scope: &self.target_branch_scope,
                    ordered_members: &self.ordered_members,
                },
            )?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformBasis {
    family_version: u32,
    program_id: String,
    transform_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: CommitId,
    basis_digest: String,
}

impl FrozenTransformBasis {
    pub fn freeze(request: &BulkTransformRequest) -> Result<Self, StoreError> {
        ensure_program_identity(request.program_id(), request.transform_identity())?;
        #[derive(Serialize)]
        struct DigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
        }
        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id().to_string(),
            transform_identity: request.transform_identity().to_string(),
            target_branch_scope: request.target_branch_scope().clone(),
            basis_commit_id: request.basis_commit_id(),
            basis_digest: stable_json_digest(
                "bulk transform basis",
                &DigestInput {
                    family_version: BULK_FAMILY_VERSION,
                    program_id: request.program_id(),
                    transform_identity: request.transform_identity(),
                    target_branch_scope: request.target_branch_scope(),
                    basis_commit_id: request.basis_commit_id(),
                },
            )?,
        })
    }
    pub fn family_version(&self) -> u32 {
        self.family_version
    }
    pub fn kind(&self) -> BulkPlanKind {
        BulkPlanKind::Transform
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
    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }
    pub(crate) fn has_valid_digest(&self) -> Result<bool, StoreError> {
        #[derive(Serialize)]
        struct DigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
        }
        Ok(self.basis_digest
            == stable_json_digest(
                "bulk transform basis",
                &DigestInput {
                    family_version: self.family_version,
                    program_id: &self.program_id,
                    transform_identity: &self.transform_identity,
                    target_branch_scope: &self.target_branch_scope,
                    basis_commit_id: self.basis_commit_id,
                },
            )?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrozenTransformTargetPartition {
    family_version: u32,
    program_id: String,
    transform_identity: String,
    target_branch_scope: BranchId,
    basis_commit_id: CommitId,
    ordered_members: Vec<BulkSourceMember>,
    partition_digest: String,
}

impl FrozenTransformTargetPartition {
    pub fn freeze(
        request: &BulkTransformRequest,
        basis: &FrozenTransformBasis,
    ) -> Result<Self, StoreError> {
        if basis.program_id() != request.program_id()
            || basis.transform_identity() != request.transform_identity()
            || basis.target_branch_scope() != request.target_branch_scope()
            || basis.basis_commit_id() != request.basis_commit_id()
        {
            return Err(StoreError::new(
                StoreErrorKind::BulkTransformBasisDrift,
                "bulk transform target partition request drifted from the frozen transform basis",
            ));
        }
        let ordered_members = canonicalize_members(request.target_members().to_vec())?;
        #[derive(Serialize)]
        struct DigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
            ordered_members: &'a [BulkSourceMember],
        }
        Ok(Self {
            family_version: BULK_FAMILY_VERSION,
            program_id: request.program_id().to_string(),
            transform_identity: request.transform_identity().to_string(),
            target_branch_scope: request.target_branch_scope().clone(),
            basis_commit_id: request.basis_commit_id(),
            ordered_members: ordered_members.clone(),
            partition_digest: stable_json_digest(
                "bulk transform target partition",
                &DigestInput {
                    family_version: BULK_FAMILY_VERSION,
                    program_id: request.program_id(),
                    transform_identity: request.transform_identity(),
                    target_branch_scope: request.target_branch_scope(),
                    basis_commit_id: request.basis_commit_id(),
                    ordered_members: &ordered_members,
                },
            )?,
        })
    }
    pub fn family_version(&self) -> u32 {
        self.family_version
    }
    pub fn kind(&self) -> BulkPlanKind {
        BulkPlanKind::Transform
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
    pub fn ordered_members(&self) -> &[BulkSourceMember] {
        &self.ordered_members
    }
    pub fn partition_digest(&self) -> &str {
        &self.partition_digest
    }
    pub(crate) fn has_valid_digest(&self) -> Result<bool, StoreError> {
        #[derive(Serialize)]
        struct DigestInput<'a> {
            family_version: u32,
            program_id: &'a str,
            transform_identity: &'a str,
            target_branch_scope: &'a BranchId,
            basis_commit_id: CommitId,
            ordered_members: &'a [BulkSourceMember],
        }
        Ok(self.partition_digest
            == stable_json_digest(
                "bulk transform target partition",
                &DigestInput {
                    family_version: self.family_version,
                    program_id: &self.program_id,
                    transform_identity: &self.transform_identity,
                    target_branch_scope: &self.target_branch_scope,
                    basis_commit_id: self.basis_commit_id,
                    ordered_members: &self.ordered_members,
                },
            )?)
    }
}
