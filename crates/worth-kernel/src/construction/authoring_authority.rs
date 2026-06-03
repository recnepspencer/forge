use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimePublicApiFamilyContract,
};
use topology::facade::TopologyConstructionQueryMutationSurface;

use crate::construction::digest::{digest_owned_parts_with_scope, ConstructionDigestScope};

const TOPOLOGY_CONSTRUCTION_ADMITTED_HANDOFF_NAME: &str =
    "worth-topo.query-native-construction-admitted-handoff";
const SPATIAL_BINDINGS_BOUNDARY_NAME: &str = "worth-spatial.bindings.primitive-birth";
const SPATIAL_BINDINGS_BOUNDARY_SCOPE: &str =
    "primitive_construction_birth_planning_and_consequence";

#[derive(Debug)]
pub enum WorthKernelAuthorityError {
    QueryRuntime(ForgeQueryRuntimeError),
}

impl From<ForgeQueryRuntimeError> for WorthKernelAuthorityError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::QueryRuntime(value)
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
    pub(crate) fn new(contract: ForgeQueryRuntimePublicApiFamilyContract) -> Self {
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
    lower_layer_birth_boundary_name: &'static str,
    lower_layer_birth_boundary_scope: &'static str,
    lower_layer_birth_boundary_digest: String,
    topology_construction_admitted_handoff_name: &'static str,
    topology_mutation_surface: TopologyConstructionQueryMutationSurface,
    required_query_family_contracts: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
    query_gap_rows: Vec<PrimitiveConstructionQueryGapRow>,
    report_digest: String,
}

impl PrimitiveConstructionAuthorityChainReport {
    pub(crate) fn new(
        workspace_name: String,
        required_query_family_contracts: Vec<ForgeQueryRuntimePublicApiFamilyContract>,
        query_gap_rows: Vec<PrimitiveConstructionQueryGapRow>,
    ) -> Self {
        let kernel_boundary_name = "worth-kernel.primitive-construction-authoring";
        let kernel_authority_scope = "primitive_construction_orchestration";
        let query_front_door = "ForgeQueryWorkspace";
        let topology_construction_admitted_handoff_name =
            TOPOLOGY_CONSTRUCTION_ADMITTED_HANDOFF_NAME;
        let topology_mutation_surface = TopologyConstructionQueryMutationSurface::ComposeGraph;
        let lower_layer_birth_boundary_name = SPATIAL_BINDINGS_BOUNDARY_NAME;
        let lower_layer_birth_boundary_scope = SPATIAL_BINDINGS_BOUNDARY_SCOPE;
        let lower_layer_birth_boundary_digest = digest_owned_parts_with_scope(
            ConstructionDigestScope::ArtifactIdentity,
            &[
                lower_layer_birth_boundary_name.to_string(),
                lower_layer_birth_boundary_scope.to_string(),
            ],
        );
        let mut parts = vec![
            workspace_name.clone(),
            query_front_door.to_string(),
            kernel_boundary_name.to_string(),
            kernel_authority_scope.to_string(),
            lower_layer_birth_boundary_digest.clone(),
            topology_construction_admitted_handoff_name.to_string(),
            topology_mutation_surface.as_str().to_string(),
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
        let report_digest =
            digest_owned_parts_with_scope(ConstructionDigestScope::ArtifactIdentity, &parts);
        Self {
            workspace_name,
            query_front_door,
            kernel_boundary_name,
            kernel_authority_scope,
            lower_layer_birth_boundary_name,
            lower_layer_birth_boundary_scope,
            lower_layer_birth_boundary_digest,
            topology_construction_admitted_handoff_name,
            topology_mutation_surface,
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

    pub fn lower_layer_birth_boundary_name(&self) -> &str {
        self.lower_layer_birth_boundary_name
    }

    pub fn lower_layer_birth_boundary_scope(&self) -> &str {
        self.lower_layer_birth_boundary_scope
    }

    pub fn lower_layer_birth_boundary_digest(&self) -> &str {
        &self.lower_layer_birth_boundary_digest
    }

    pub fn topology_construction_admitted_handoff_name(&self) -> &str {
        self.topology_construction_admitted_handoff_name
    }

    pub fn topology_mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.topology_mutation_surface
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

pub(crate) fn is_supported_optional_query_family(
    contract: &ForgeQueryRuntimePublicApiFamilyContract,
) -> bool {
    contract.status() == ForgeQueryRuntimeFamilySupportStatus::Supported
}
