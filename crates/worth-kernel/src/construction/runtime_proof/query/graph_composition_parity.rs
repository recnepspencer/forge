use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};
use topology::facade::TopologyConstructionQueryMutationSurface;

use crate::construction::authoring::{
    primitive_construction_authoring, PrimitiveConstructionQueryEntryError,
    WorthKernelAuthorityError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionQueryGraphCompositionParityReport {
    family: PrimitiveConstructionFamily,
    query_contract_digest: String,
    topology_query_receipt_surface: TopologyConstructionQueryMutationSurface,
    artifact_surface: TopologyConstructionQueryMutationSurface,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    parity_verified: bool,
    report_digest: String,
}

impl PrimitiveConstructionQueryGraphCompositionParityReport {
    fn new(
        family: PrimitiveConstructionFamily,
        query_contract_digest: String,
        topology_query_envelope_surface: TopologyConstructionQueryMutationSurface,
        artifact_surface: TopologyConstructionQueryMutationSurface,
        required_query_families: &[ForgeQueryRuntimeFacadeFamily],
    ) -> Self {
        let parity_verified = topology_query_envelope_surface
            == TopologyConstructionQueryMutationSurface::ComposeGraph
            && artifact_surface == TopologyConstructionQueryMutationSurface::ComposeGraph
            && required_query_families
                == [
                    ForgeQueryRuntimeFacadeFamily::Write,
                    ForgeQueryRuntimeFacadeFamily::Inspect,
                ];
        let report_digest = digest_owned_parts(&[
            family.as_str().to_string(),
            query_contract_digest.clone(),
            topology_query_envelope_surface.as_str().to_string(),
            artifact_surface.as_str().to_string(),
            required_query_families
                .iter()
                .map(|family| format!("{family:?}"))
                .collect::<Vec<_>>()
                .join("|"),
            parity_verified.to_string(),
        ]);
        Self {
            family,
            query_contract_digest,
            topology_query_receipt_surface: topology_query_envelope_surface,
            artifact_surface,
            required_query_families: required_query_families.to_vec(),
            parity_verified,
            report_digest,
        }
    }

    pub fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub fn topology_query_receipt_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.topology_query_receipt_surface
    }

    pub fn query_contract_digest(&self) -> &str {
        &self.query_contract_digest
    }

    pub fn artifact_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.artifact_surface
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn parity_verified(&self) -> bool {
        self.parity_verified
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[derive(Debug)]
pub enum PrimitiveConstructionQueryGraphCompositionParityError {
    Authority(WorthKernelAuthorityError),
    QueryEntry(PrimitiveConstructionQueryEntryError),
    QueryRuntime(ForgeQueryRuntimeError),
}

impl std::fmt::Display for PrimitiveConstructionQueryGraphCompositionParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Authority(error) => write!(f, "{error:?}"),
            Self::QueryEntry(error) => write!(f, "{error}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryGraphCompositionParityError {}

pub fn prepare_primitive_construction_query_graph_composition_parity_report(
    workspace: &mut ForgeQueryWorkspace,
    intent: impl Into<PrimitiveConstructionIntent>,
) -> Result<
    PrimitiveConstructionQueryGraphCompositionParityReport,
    PrimitiveConstructionQueryGraphCompositionParityError,
> {
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Write)
        .map_err(PrimitiveConstructionQueryGraphCompositionParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let prepared = {
        let mut session = primitive_construction_authoring(workspace)
            .map_err(PrimitiveConstructionQueryGraphCompositionParityError::Authority)?;
        session
            .prepare_result(intent)
            .map_err(PrimitiveConstructionQueryGraphCompositionParityError::QueryEntry)?
    };
    let artifact = prepared.canonical_artifact();
    let topology_query_envelope = prepared
        .evidence()
        .topology_query_handoff()
        .topology_query_envelope();
    Ok(PrimitiveConstructionQueryGraphCompositionParityReport::new(
        prepared.family(),
        query_contract_digest,
        topology_query_envelope.mutation_surface(),
        artifact.mutation_surface(),
        topology_query_envelope.required_query_families(),
    ))
}

#[cfg(test)]
mod tests {
    use super::prepare_primitive_construction_query_graph_composition_parity_report;
    use crate::construction::{
        OrthotopeSpec, PrimitiveConstructionFamily, PrimitiveConstructionIntent,
    };
    use forge_query::facade::ForgeQueryRuntimeFacadeFamily;
    use topology::facade::{
        milestone_one_runtime_builder, topology_runtime, TopologyConstructionQueryMutationSurface,
        TopologyRuntimeAdapters,
    };

    #[test]
    fn graph_composition_parity_report_locks_compose_graph_as_the_write_surface() {
        let runtime = milestone_one_runtime_builder()
            .expect("runtime builder")
            .build();
        let mut workspace = topology_runtime(
            TopologyRuntimeAdapters::current_head(runtime),
            "worth-kernel.query-graph-composition".to_string(),
        )
        .expect("workspace");
        let report = prepare_primitive_construction_query_graph_composition_parity_report(
            &mut workspace,
            PrimitiveConstructionIntent::orthotope(OrthotopeSpec {
                half_extents: [1.0, 2.0, 3.0],
            }),
        )
        .expect("graph composition parity report");

        assert_eq!(report.family(), PrimitiveConstructionFamily::Orthotope);
        assert_eq!(
            report.topology_query_receipt_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(
            report.artifact_surface(),
            TopologyConstructionQueryMutationSurface::ComposeGraph
        );
        assert_eq!(
            report.required_query_families(),
            &[
                ForgeQueryRuntimeFacadeFamily::Write,
                ForgeQueryRuntimeFacadeFamily::Inspect,
            ]
        );
        assert!(!report.query_contract_digest().is_empty());
        assert!(report.parity_verified());
        assert!(!report.report_digest().is_empty());
    }
}
