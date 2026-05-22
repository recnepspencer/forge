use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus, ForgeQueryWorkspace,
};

use crate::construction::authoring::{primitive_construction_authoring, WorthKernelAuthorityError};
use crate::construction::digest::digest_owned_parts;

const AUDITED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 6] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
    ForgeQueryRuntimeFacadeFamily::BranchPreview,
    ForgeQueryRuntimeFacadeFamily::Read,
    ForgeQueryRuntimeFacadeFamily::Intent,
    ForgeQueryRuntimeFacadeFamily::Temporal,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionQueryBoundaryUsagePosture {
    RequiredNow,
    NeighborNotRequiredNow,
    DeferredFutureNeighbor,
}

impl PrimitiveConstructionQueryBoundaryUsagePosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiredNow => "required_now",
            Self::NeighborNotRequiredNow => "neighbor_not_required_now",
            Self::DeferredFutureNeighbor => "deferred_future_neighbor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionQueryBoundaryGapStatus {
    Closed,
    NotRequiredForCurrentPath,
    DeferredUnsupportedNeighbor,
}

impl PrimitiveConstructionQueryBoundaryGapStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::NotRequiredForCurrentPath => "not_required_for_current_path",
            Self::DeferredUnsupportedNeighbor => "deferred_unsupported_neighbor",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryBoundaryGapRowReport {
    family: ForgeQueryRuntimeFacadeFamily,
    usage_posture: PrimitiveConstructionQueryBoundaryUsagePosture,
    gap_status: PrimitiveConstructionQueryBoundaryGapStatus,
    support_status: ForgeQueryRuntimeFamilySupportStatus,
    contract_digest: String,
    row_digest: String,
}

impl PrimitiveConstructionQueryBoundaryGapRowReport {
    fn new(
        family: ForgeQueryRuntimeFacadeFamily,
        usage_posture: PrimitiveConstructionQueryBoundaryUsagePosture,
        gap_status: PrimitiveConstructionQueryBoundaryGapStatus,
        support_status: ForgeQueryRuntimeFamilySupportStatus,
        contract_digest: &str,
    ) -> Self {
        let row_digest = digest_owned_parts(&[
            format!("{family:?}"),
            usage_posture.as_str().to_string(),
            gap_status.as_str().to_string(),
            format!("{support_status:?}"),
            contract_digest.to_string(),
        ]);
        Self {
            family,
            usage_posture,
            gap_status,
            support_status,
            contract_digest: contract_digest.to_string(),
            row_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn usage_posture(&self) -> PrimitiveConstructionQueryBoundaryUsagePosture {
        self.usage_posture
    }

    pub fn gap_status(&self) -> PrimitiveConstructionQueryBoundaryGapStatus {
        self.gap_status
    }

    pub fn support_status(&self) -> ForgeQueryRuntimeFamilySupportStatus {
        self.support_status
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryBoundaryGapRegister {
    rows: Vec<PrimitiveConstructionQueryBoundaryGapRowReport>,
    unresolved_gap_count: usize,
    report_digest: String,
}

impl PrimitiveConstructionQueryBoundaryGapRegister {
    pub fn rows(&self) -> &[PrimitiveConstructionQueryBoundaryGapRowReport] {
        &self.rows
    }

    pub fn unresolved_gap_count(&self) -> usize {
        self.unresolved_gap_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn prepare_primitive_construction_query_boundary_gap_register(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionQueryBoundaryGapRegister, WorthKernelAuthorityError> {
    let (required_contracts, branch_preview_supported) = {
        let session = primitive_construction_authoring(workspace)?;
        let authority_report = session.authority_chain_report();
        (
            authority_report.required_query_family_contracts().to_vec(),
            session
                .admit_query_family(ForgeQueryRuntimeFacadeFamily::BranchPreview)
                .is_ok(),
        )
    };
    let public_api_contract = workspace.public_api_contract();
    let rows = AUDITED_QUERY_FAMILIES
        .iter()
        .map(|family| {
            let contract = public_api_contract
                .family(*family)
                .cloned()
                .expect("audited query family should exist");
            let usage_posture = match family {
                ForgeQueryRuntimeFacadeFamily::Write
                | ForgeQueryRuntimeFacadeFamily::Inspect
                | ForgeQueryRuntimeFacadeFamily::BranchPreview => {
                    PrimitiveConstructionQueryBoundaryUsagePosture::RequiredNow
                }
                ForgeQueryRuntimeFacadeFamily::Read | ForgeQueryRuntimeFacadeFamily::Intent => {
                    PrimitiveConstructionQueryBoundaryUsagePosture::NeighborNotRequiredNow
                }
                ForgeQueryRuntimeFacadeFamily::Temporal => {
                    PrimitiveConstructionQueryBoundaryUsagePosture::DeferredFutureNeighbor
                }
                _ => PrimitiveConstructionQueryBoundaryUsagePosture::DeferredFutureNeighbor,
            };
            let required_now = required_contracts
                .iter()
                .any(|contract| contract.family() == *family)
                || (*family == ForgeQueryRuntimeFacadeFamily::BranchPreview
                    && branch_preview_supported);
            let gap_status = if required_now {
                PrimitiveConstructionQueryBoundaryGapStatus::Closed
            } else if contract.status() == ForgeQueryRuntimeFamilySupportStatus::Supported {
                PrimitiveConstructionQueryBoundaryGapStatus::NotRequiredForCurrentPath
            } else {
                PrimitiveConstructionQueryBoundaryGapStatus::DeferredUnsupportedNeighbor
            };
            PrimitiveConstructionQueryBoundaryGapRowReport::new(
                *family,
                usage_posture,
                gap_status,
                contract.status(),
                contract.contract_digest(),
            )
        })
        .collect::<Vec<_>>();
    let unresolved_gap_count = rows
        .iter()
        .filter(|row| {
            row.gap_status()
                == PrimitiveConstructionQueryBoundaryGapStatus::DeferredUnsupportedNeighbor
        })
        .count();
    let report_digest = digest_owned_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(format!(
                "unresolved-gap-count:{unresolved_gap_count}"
            )))
            .collect::<Vec<_>>(),
    );
    Ok(PrimitiveConstructionQueryBoundaryGapRegister {
        rows,
        unresolved_gap_count,
        report_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        prepare_primitive_construction_query_boundary_gap_register,
        PrimitiveConstructionQueryBoundaryGapStatus,
        PrimitiveConstructionQueryBoundaryUsagePosture,
    };
    use forge_query::facade::{
        ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus,
    };
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn query_boundary_gap_register_distinguishes_closed_runtime_families_from_deferred_neighbors() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-gap-register".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_boundary_gap_register(&mut workspace)
            .expect("query boundary gap register");
        let write = report
            .rows()
            .iter()
            .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Write)
            .expect("write row");
        let branch_preview = report
            .rows()
            .iter()
            .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::BranchPreview)
            .expect("branch preview row");
        let temporal = report
            .rows()
            .iter()
            .find(|row| row.family() == ForgeQueryRuntimeFacadeFamily::Temporal)
            .expect("temporal row");

        assert_eq!(
            write.usage_posture(),
            PrimitiveConstructionQueryBoundaryUsagePosture::RequiredNow
        );
        assert_eq!(
            write.gap_status(),
            PrimitiveConstructionQueryBoundaryGapStatus::Closed
        );
        assert_eq!(
            branch_preview.gap_status(),
            PrimitiveConstructionQueryBoundaryGapStatus::Closed
        );
        assert_eq!(
            temporal.usage_posture(),
            PrimitiveConstructionQueryBoundaryUsagePosture::DeferredFutureNeighbor
        );
        assert_eq!(
            temporal.gap_status(),
            PrimitiveConstructionQueryBoundaryGapStatus::DeferredUnsupportedNeighbor
        );
        assert_ne!(
            temporal.support_status(),
            ForgeQueryRuntimeFamilySupportStatus::Supported
        );
        assert!(report.unresolved_gap_count() >= 1);
        assert!(!report.report_digest().is_empty());
    }
}
