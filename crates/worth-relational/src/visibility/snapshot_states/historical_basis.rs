use std::sync::Arc;

use crate::branch::RelationalBranchRoot;
use crate::history::data::BranchId;
use crate::identity::data::VersionId;
use crate::runtime::RelationalRuntime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HistoricalVisibilityDenial {
    UnknownVersion,
    AuthoringBranchUnavailable,
    MvccIntervalUnavailable,
    CertificationReconstructionRequired,
}

#[derive(Clone, Debug)]
pub(crate) struct HistoricalVisibilityBasis {
    branch_id: BranchId,
    version_id: VersionId,
    root: Option<Arc<RelationalBranchRoot>>,
    coverage: HistoricalVisibilityCoverage,
}

#[derive(Clone, Debug)]
enum HistoricalVisibilityCoverage {
    EmptyGenesis,
    RetainedInterval {
        source_root_id: u64,
        source_version: VersionId,
    },
}

impl HistoricalVisibilityBasis {
    pub(crate) fn resolve(
        runtime: &RelationalRuntime,
        version_id: VersionId,
    ) -> Result<Self, HistoricalVisibilityDenial> {
        let branch_id = crate::visibility::branch_scope::branch_for_version(runtime, version_id)
            .ok_or(HistoricalVisibilityDenial::UnknownVersion)?;
        if let Some(root) = runtime
            .history
            .commit_catalog
            .find_by_version(version_id)
            .and_then(|artifact| artifact.linked_root())
        {
            return Ok(Self {
                branch_id,
                version_id,
                coverage: HistoricalVisibilityCoverage::RetainedInterval {
                    source_root_id: root.id(),
                    source_version: version_id,
                },
                root: Some(root),
            });
        }
        let cell = runtime
            .history
            .branch_cell(&branch_id)
            .ok_or(HistoricalVisibilityDenial::AuthoringBranchUnavailable)?;
        let Some(root) = cell.root().cloned() else {
            if version_id.is_zero() && runtime.history().latest_commit().is_none() {
                return Ok(Self {
                    branch_id,
                    version_id,
                    root: None,
                    coverage: HistoricalVisibilityCoverage::EmptyGenesis,
                });
            }
            return Err(HistoricalVisibilityDenial::CertificationReconstructionRequired);
        };
        let source_version = match root.axes() {
            Some(axes) => VersionId(axes.storage_version),
            None if version_id.is_zero() && root.id() == 0 && root.descriptor().is_none() => {
                VersionId(0)
            }
            None => return Err(HistoricalVisibilityDenial::MvccIntervalUnavailable),
        };
        if source_version.as_u64() < version_id.as_u64() {
            return Err(HistoricalVisibilityDenial::MvccIntervalUnavailable);
        }
        let source_root_id = root.id();
        Ok(Self {
            branch_id,
            version_id,
            root: Some(root),
            coverage: HistoricalVisibilityCoverage::RetainedInterval {
                source_root_id,
                source_version,
            },
        })
    }

    pub(crate) fn branch_id(&self) -> &BranchId {
        &self.branch_id
    }

    pub(crate) const fn version_id(&self) -> VersionId {
        self.version_id
    }

    pub(crate) fn root(&self) -> Option<&Arc<RelationalBranchRoot>> {
        self.root.as_ref()
    }

    pub(crate) fn source_root_id(&self) -> Option<u64> {
        match self.coverage {
            HistoricalVisibilityCoverage::EmptyGenesis => None,
            HistoricalVisibilityCoverage::RetainedInterval { source_root_id, .. } => {
                Some(source_root_id)
            }
        }
    }

    pub(crate) fn schema_commitment(&self) -> Option<[u8; 32]> {
        self.root
            .as_ref()
            .map(|root| root.schema_authority().registry().authority_digest_bytes())
    }

    pub(crate) fn source_version(&self) -> VersionId {
        match self.coverage {
            HistoricalVisibilityCoverage::EmptyGenesis => VersionId(0),
            HistoricalVisibilityCoverage::RetainedInterval { source_version, .. } => source_version,
        }
    }
}
