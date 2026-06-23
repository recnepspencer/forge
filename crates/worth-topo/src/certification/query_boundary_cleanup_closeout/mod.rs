use crate::certification::error::TopologyCertificationError;
use crate::certification::shared::digest_rows;
use crate::certification::DeterministicDigest;

mod rows;
mod support;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyQueryBoundaryCleanupArea {
    OperatorPath,
    SnapshotSurfaces,
    ReadViewDecode,
    BasisAdapter,
    PublicFacade,
    DerivedValidationRehome,
}

impl TopologyQueryBoundaryCleanupArea {
    pub const ALL: [Self; 6] = [
        Self::OperatorPath,
        Self::SnapshotSurfaces,
        Self::ReadViewDecode,
        Self::BasisAdapter,
        Self::PublicFacade,
        Self::DerivedValidationRehome,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::OperatorPath => "operator_path",
            Self::SnapshotSurfaces => "snapshot_surfaces",
            Self::ReadViewDecode => "read_view_decode",
            Self::BasisAdapter => "basis_adapter",
            Self::PublicFacade => "public_facade",
            Self::DerivedValidationRehome => "derived_validation_rehome",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopologyQueryBoundaryCleanupStatus {
    Closed,
    Gap,
}

impl TopologyQueryBoundaryCleanupStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Gap => "gap",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBoundaryCleanupRow {
    area: TopologyQueryBoundaryCleanupArea,
    status: TopologyQueryBoundaryCleanupStatus,
    reason: String,
    designated_survivor: Option<String>,
    row_digest: DeterministicDigest,
}

impl TopologyQueryBoundaryCleanupRow {
    pub fn area(&self) -> TopologyQueryBoundaryCleanupArea {
        self.area
    }

    pub fn status(&self) -> TopologyQueryBoundaryCleanupStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn designated_survivor(&self) -> Option<&str> {
        self.designated_survivor.as_deref()
    }

    pub fn row_digest(&self) -> &DeterministicDigest {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyQueryBoundaryCleanupCloseoutReport {
    rows: Vec<TopologyQueryBoundaryCleanupRow>,
    cleanup_complete: bool,
    closeout_digest: DeterministicDigest,
}

impl TopologyQueryBoundaryCleanupCloseoutReport {
    pub fn rows(&self) -> &[TopologyQueryBoundaryCleanupRow] {
        &self.rows
    }

    pub fn cleanup_complete(&self) -> bool {
        self.cleanup_complete
    }

    pub fn status(
        &self,
        area: TopologyQueryBoundaryCleanupArea,
    ) -> TopologyQueryBoundaryCleanupStatus {
        self.rows
            .iter()
            .find(|row| row.area() == area)
            .map(TopologyQueryBoundaryCleanupRow::status)
            .unwrap_or_else(|| panic!("cleanup closeout rows should cover every declared area"))
    }

    pub fn closeout_digest(&self) -> &DeterministicDigest {
        &self.closeout_digest
    }
}

pub fn certify_topology_query_boundary_cleanup_closeout(
) -> Result<TopologyQueryBoundaryCleanupCloseoutReport, TopologyCertificationError> {
    let rows = vec![
        rows::certify_operator_path_row()?,
        rows::certify_snapshot_surfaces_row()?,
        rows::certify_read_view_decode_row()?,
        rows::certify_basis_adapter_row()?,
        rows::certify_public_facade_row()?,
        rows::certify_derived_validation_rehome_row()?,
    ];
    let cleanup_complete = rows
        .iter()
        .all(|row| row.status() == TopologyQueryBoundaryCleanupStatus::Closed);
    let closeout_digest = digest_rows(rows.iter().map(|row| {
        format!(
            "{}:{}:{}:{}:{}",
            row.area().as_str(),
            row.status().as_str(),
            row.reason(),
            row.designated_survivor().unwrap_or("none"),
            row.row_digest().digest_hex
        )
    }));
    let report = TopologyQueryBoundaryCleanupCloseoutReport {
        rows,
        cleanup_complete,
        closeout_digest,
    };
    if !report.cleanup_complete() {
        return Err(TopologyCertificationError::ReadView(
            "worth-topo query boundary cleanup closeout contains gap rows".to_string(),
        ));
    }
    Ok(report)
}

#[cfg(test)]
mod tests;
