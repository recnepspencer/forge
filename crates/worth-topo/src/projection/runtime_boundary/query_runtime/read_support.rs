use super::contracts::TopologyRuntimeSupport;
use crate::projection::TopologyDomainQueryRequestFamily;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TopologyQueryReadFamilySupportStatus {
    Denied,
    Admitted,
}

impl TopologyQueryReadFamilySupportStatus {
    pub fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyRuntimeReadFamilySupportRow {
    family: TopologyDomainQueryRequestFamily,
    status: TopologyQueryReadFamilySupportStatus,
    reason: String,
    row_digest: String,
}

impl TopologyRuntimeReadFamilySupportRow {
    pub fn family(&self) -> TopologyDomainQueryRequestFamily {
        self.family
    }

    pub fn status(&self) -> TopologyQueryReadFamilySupportStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub(super) fn admitted(
        family: TopologyDomainQueryRequestFamily,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        Self {
            family,
            status: TopologyQueryReadFamilySupportStatus::Admitted,
            row_digest: support_row_digest(
                family,
                TopologyQueryReadFamilySupportStatus::Admitted,
                &reason,
            ),
            reason,
        }
    }
}

impl TopologyRuntimeSupport {
    pub fn query_read_family_support_rows(&self) -> &[TopologyRuntimeReadFamilySupportRow] {
        &self.query_read_family_support_rows
    }

    pub fn query_read_family_support_status(
        &self,
        family: TopologyDomainQueryRequestFamily,
    ) -> TopologyQueryReadFamilySupportStatus {
        self.query_read_family_support_rows
            .iter()
            .find(|row| row.family == family)
            .map(TopologyRuntimeReadFamilySupportRow::status)
            .unwrap_or_else(|| {
                panic!(" runtime read-family support rows should cover every declared family")
            })
    }
}

pub(super) fn current_head_query_read_family_support_rows(
) -> Vec<TopologyRuntimeReadFamilySupportRow> {
    TopologyDomainQueryRequestFamily::ALL
        .into_iter()
        .map(|family| {
            TopologyRuntimeReadFamilySupportRow::admitted(
                family,
                match family {
                    TopologyDomainQueryRequestFamily::HalfEdgeSharedVertexNeighborhood => {
                        "current-head bridge-backed runtime admits the shared-vertex topology-domain read family"
                    }
                    TopologyDomainQueryRequestFamily::HalfEdgeRadialNeighborhood => {
                        "current-head bridge-backed runtime admits the radial topology-domain read family"
                    }
                    TopologyDomainQueryRequestFamily::LoopCycleNeighborhood => {
                        "current-head bridge-backed runtime admits the loop-cycle topology-domain read family"
                    }
                    TopologyDomainQueryRequestFamily::LocalRewireNeighborhood => {
                        "current-head bridge-backed runtime admits the local-rewire topology-domain read family"
                    }
                },
            )
        })
        .collect()
}

pub(super) fn snapshot_query_read_family_support_rows() -> Vec<TopologyRuntimeReadFamilySupportRow>
{
    TopologyDomainQueryRequestFamily::ALL
        .into_iter()
        .map(|family| {
            TopologyRuntimeReadFamilySupportRow::admitted(
                family,
                "snapshot read-only runtime admits the topology-domain read family through an admitted historical query basis context",
            )
        })
        .collect()
}

fn support_row_digest(
    family: TopologyDomainQueryRequestFamily,
    status: TopologyQueryReadFamilySupportStatus,
    reason: &str,
) -> String {
    format!("family={family:?};status={status:?};reason={reason}",)
}
