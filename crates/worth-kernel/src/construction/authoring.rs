use forge_query::facade::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryWorkspace,
};
use worth_spatial::facade::witness_catalog::SpatialWitnessCatalog;

use crate::construction::authoring_authority::is_supported_optional_query_family;
use crate::construction::authoring_entry::PrimitiveConstructionAuthoringEntry;
use crate::construction::authoring_input::{
    PrimitiveConstructionAuthoringInput, PrimitiveConstructionCatalogAuthoringInput,
};
use crate::construction::result::PrimitiveConstructionResultError;
use crate::construction::PrimitiveConstructionSpatialIntentError;

pub use crate::construction::authoring_authority::{
    PrimitiveConstructionAuthorityChainReport, PrimitiveConstructionQueryGapRow,
    WorthKernelAuthorityError,
};

const REQUIRED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 2] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
];
const REPORTED_QUERY_FAMILIES: [ForgeQueryRuntimeFacadeFamily; 3] = [
    ForgeQueryRuntimeFacadeFamily::Write,
    ForgeQueryRuntimeFacadeFamily::Inspect,
    ForgeQueryRuntimeFacadeFamily::BranchPreview,
];

pub struct PrimitiveConstructionAuthoringSession<'a> {
    workspace: &'a mut ForgeQueryWorkspace,
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
                (!is_supported_optional_query_family(&contract))
                    .then(|| PrimitiveConstructionQueryGapRow::new(contract))
            })
            .collect::<Vec<_>>();
        Ok(Self {
            workspace,
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

    pub fn author<I: PrimitiveConstructionAuthoringInput>(
        &mut self,
        intent: I,
    ) -> Result<PrimitiveConstructionAuthoringEntry, PrimitiveConstructionQueryEntryError> {
        self.require_query_construction_entry()?;
        let intent = intent
            .lower_for_query_entry()
            .map_err(PrimitiveConstructionQueryEntryError::Lowering)?;
        Ok(PrimitiveConstructionAuthoringEntry::new(intent))
    }

    pub fn author_with_catalog<
        I: PrimitiveConstructionCatalogAuthoringInput,
        C: SpatialWitnessCatalog,
    >(
        &mut self,
        intent: I,
        catalog: &C,
    ) -> Result<PrimitiveConstructionAuthoringEntry, PrimitiveConstructionQueryEntryError> {
        self.require_query_construction_entry()?;
        let intent = intent
            .lower_for_query_entry_with_catalog(catalog)
            .map_err(PrimitiveConstructionQueryEntryError::Lowering)?;
        Ok(PrimitiveConstructionAuthoringEntry::new(intent))
    }

    fn require_query_construction_entry(&self) -> Result<(), WorthKernelAuthorityError> {
        for family in REQUIRED_QUERY_FAMILIES {
            self.admit_query_family(family)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryEntryError {
    Authority(WorthKernelAuthorityError),
    Lowering(PrimitiveConstructionSpatialIntentError),
    Result(PrimitiveConstructionResultError),
}

impl From<WorthKernelAuthorityError> for PrimitiveConstructionQueryEntryError {
    fn from(value: WorthKernelAuthorityError) -> Self {
        Self::Authority(value)
    }
}

impl std::fmt::Display for PrimitiveConstructionQueryEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error:?}"),
            Self::Lowering(error) => write!(f, "{error}"),
            Self::Result(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryEntryError {}

pub fn primitive_construction_authoring(
    workspace: &mut ForgeQueryWorkspace,
) -> Result<PrimitiveConstructionAuthoringSession<'_>, WorthKernelAuthorityError> {
    PrimitiveConstructionAuthoringSession::new(workspace)
}

#[cfg(test)]
mod tests {
    use super::primitive_construction_authoring;
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyConstructionQueryMutationSurface,
        TopologyRuntimeAdapters,
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
            report.lower_layer_birth_boundary_name(),
            "worth-spatial.bindings.primitive-birth"
        );
        assert_eq!(
            report.lower_layer_birth_boundary_scope(),
            "primitive_construction_birth_planning_and_consequence"
        );
        assert_eq!(
            report.topology_construction_admitted_handoff_name(),
            "worth-topo.query-native-construction-admitted-handoff"
        );
        assert_eq!(
            report.topology_mutation_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(report.required_query_family_contracts().len(), 2);
        assert!(report.query_gap_rows().is_empty());
        assert!(report
            .report_digest()
            .starts_with("worth-kernel.v1:artifact-identity:sha256:"));
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
