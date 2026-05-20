use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryWorkspace,
};
use topology::facade::{topology_construction_authority, TopologyConstructionAuthority};
use worth_spatial::facade::{construction_birth_authority, SpatialConstructionBirthAuthority};

const REQUIRED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 2] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
];
const REPORTED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 3] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
    ForgeQueryRuntimeFacadeFamily::BranchPreview,
];

#[derive(Debug)]
pub enum WorthKernelAuthorityError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl From<ForgeQueryRuntimeError> for WorthKernelAuthorityError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::QueryRuntime(value)
    }
}

pub struct PrimitiveConstructionAuthoringSession<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
    spatial_authority: SpatialConstructionBirthAuthority,
    topology_authority: TopologyConstructionAuthority,
    required_query_family_contracts: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
    query_gap_rows: Vec<PrimitiveConstructionQueryGapRow>,
}

impl<'a> PrimitiveConstructionAuthoringSession<'a> {
    pub(crate) fn new(
        workspace: &'a mut ForgeQueryWorkspace,
    ) -> Result<Self, WorthKernelAuthorityError> {
        let public_api_contract = workspace.public_api_contract();
        let mut required_query_family_contracts = Vec::with_capacity(REQUIRED_QUERY_FAMILIES.len());
        for family in REQUIRED_QUERY_FAMILIES {
            required_query_family_contracts.push(workspace.admit_public_api_family(family)?);
        }
        let query_gap_rows = REPORTED_QUERY_FAMILIES
            .into_iter()
            .filter_map(|family| {
                let contract = public_api_contract
                    .family(family)
                    .cloned()
                    .expect("reported query family should exist in public api contract");
                if REQUIRED_QUERY_FAMILIES.contains(&family) {
                    return None;
                }
                (contract.status() != ForgeQueryRuntimeFamilySupportStatus::Supported)
                    .then(|| PrimitiveConstructionQueryGapRow::new(contract))
            })
            .collect::<Vec<_>>();
        Ok(Self {
            workspace,
            spatial_authority: construction_birth_authority(),
            topology_authority: topology_construction_authority(),
            required_query_family_contracts,
            query_gap_rows,
        })
    }

    pub fn workspace_name(&self) -> &str {
        self.workspace.name()
    }

    pub fn query_front_door(&self) -> &'static str {
        "ForgeQueryWorkspace"
    }

    pub fn authority_chain_report(&self) -> PrimitiveConstructionAuthorityChainReport {
        PrimitiveConstructionAuthorityChainReport::new(
            self.workspace.name().to_string(),
            self.spatial_authority.clone(),
            self.topology_authority.clone(),
            self.required_query_family_contracts.clone(),
            self.query_gap_rows.clone(),
        )
    }

    pub fn admit_query_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<ForgeQueryRuntimePublicApiFamilyContract, WorthKernelAuthorityError> {
        self.workspace
            .admit_public_api_family(family)
            .map_err(Into::into)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryGapRow {
    family: ForgeQueryRuntimeFacadeFamily,
    reason: String,
    evidence: Vec<String>,
    contract_digest: String,
}

impl PrimitiveConstructionQueryGapRow {
    fn new(contract: ForgeQueryRuntimePublicApiFamilyContract) -> Self {
        Self {
            family: contract.family(),
            reason: contract.reason().unwrap_or("unsupported").to_string(),
            evidence: contract
                .evidence()
                .iter()
                .map(|evidence| evidence.to_string())
                .collect(),
            contract_digest: contract.contract_digest().to_string(),
        }
    }

    pub fn family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.family
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn evidence(&self) -> &[String] {
        &self.evidence
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionAuthorityChainReport {
    workspace_name: String,
    query_front_door: &'static str,
    kernel_boundary_name: &'static str,
    kernel_authority_scope: &'static str,
    spatial_authority: SpatialConstructionBirthAuthority,
    topology_authority: TopologyConstructionAuthority,
    required_query_family_contracts: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
    query_gap_rows: Vec<PrimitiveConstructionQueryGapRow>,
    report_digest: String,
}

impl PrimitiveConstructionAuthorityChainReport {
    fn new(
        workspace_name: String,
        spatial_authority: SpatialConstructionBirthAuthority,
        topology_authority: TopologyConstructionAuthority,
        required_query_family_contracts: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
        query_gap_rows: Vec<PrimitiveConstructionQueryGapRow>,
    ) -> Self {
        let kernel_boundary_name = "worth-kernel.primitive-construction-authoring";
        let kernel_authority_scope = "primitive_construction_orchestration";
        let query_front_door = "ForgeQueryWorkspace";
        let mut parts = vec![
            workspace_name.clone(),
            query_front_door.to_string(),
            kernel_boundary_name.to_string(),
            kernel_authority_scope.to_string(),
            spatial_authority.authority_digest().to_string(),
            topology_authority.authority_digest().to_string(),
        ];
        parts.extend(
            required_query_family_contracts
                .iter()
                .map(|contract| contract.contract_digest().to_string()),
        );
        parts.extend(
            query_gap_rows
                .iter()
                .map(|row| row.contract_digest().to_string()),
        );
        let report_digest = digest_parts(&parts);
        Self {
            workspace_name,
            query_front_door,
            kernel_boundary_name,
            kernel_authority_scope,
            spatial_authority,
            topology_authority,
            required_query_family_contracts,
            query_gap_rows,
            report_digest,
        }
    }

    pub fn workspace_name(&self) -> &str {
        &self.workspace_name
    }

    pub fn query_front_door(&self) -> &str {
        self.query_front_door
    }

    pub fn kernel_boundary_name(&self) -> &str {
        self.kernel_boundary_name
    }

    pub fn kernel_authority_scope(&self) -> &str {
        self.kernel_authority_scope
    }

    pub fn spatial_authority(&self) -> &SpatialConstructionBirthAuthority {
        &self.spatial_authority
    }

    pub fn topology_authority(&self) -> &TopologyConstructionAuthority {
        &self.topology_authority
    }

    pub fn required_query_family_contracts(&self) -> &[ForgeQueryRuntimePublicApiFamilyContract] {
        &self.required_query_family_contracts
    }

    pub fn query_gap_rows(&self) -> &[PrimitiveConstructionQueryGapRow] {
        &self.query_gap_rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn primitive_construction_authoring(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionAuthoringSession<'_>, WorthKernelAuthorityError> {
    PrimitiveConstructionAuthoringSession::new(workspace)
}

fn digest_parts(parts: &[String]) -> String {
    let mut hasher = DefaultHasher::new();
    for part in parts {
        part.hash(&mut hasher);
    }
    format!("{:016x}", hasher.finish())
}

#[cfg(test)]
mod tests {
    use super::primitive_construction_authoring;
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters,
    };

    #[test]
    fn authoring_session_reports_kernel_spatial_topology_chain() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.phase-one".to_string(),
        )
        .expect("workspace");
        let session = primitive_construction_authoring(&mut workspace).expect("authoring session");
        let report = session.authority_chain_report();

        assert_eq!(report.workspace_name(), "worth-kernel.phase-one");
        assert_eq!(report.query_front_door(), "ForgeQueryWorkspace");
        assert_eq!(
            report.kernel_boundary_name(),
            "worth-kernel.primitive-construction-authoring"
        );
        assert_eq!(
            report.spatial_authority().boundary_name(),
            "worth-spatial.construction-birth-authority"
        );
        assert_eq!(
            report.topology_authority().boundary_name(),
            "worth-topo.construction-authority"
        );
        assert_eq!(report.required_query_family_contracts().len(), 2);
        assert!(report.query_gap_rows().is_empty());
        assert!(!report.report_digest().is_empty());
    }

    #[test]
    fn authoring_session_surfaces_query_gap_denials_without_local_workaround() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.phase-one-gap".to_string(),
        )
        .expect("workspace");
        let session = primitive_construction_authoring(&mut workspace).expect("authoring session");
        let error = session
            .admit_query_family(ForgeQueryRuntimeFacadeFamily::Temporal)
            .expect_err("temporal family should remain unsupported here");

        let message = match error {
            super::WorthKernelAuthorityError::QueryRuntime(inner) => inner.to_string(),
        };
        assert!(message.contains("temporal"));
    }
}
